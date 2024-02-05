//! Types for the `PCAConnectorAD` service.

/// The [`AWS::PCAConnectorAD::Connector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-connector.html) resource type.
#[derive(Debug, Default)]
pub struct Connector {
    properties: ConnectorProperties
}

/// Properties for the `Connector` resource.
#[derive(Debug, Default)]
pub struct ConnectorProperties {
    /// Property [`CertificateAuthorityArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-connector.html#cfn-pcaconnectorad-connector-certificateauthorityarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_authority_arn: ::Value<String>,
    /// Property [`DirectoryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-connector.html#cfn-pcaconnectorad-connector-directoryid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub directory_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-connector.html#cfn-pcaconnectorad-connector-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`VpcInformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-connector.html#cfn-pcaconnectorad-connector-vpcinformation).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_information: ::Value<self::connector::VpcInformation>,
}

impl ::serde::Serialize for ConnectorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateAuthorityArn", &self.certificate_authority_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryId", &self.directory_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcInformation", &self.vpc_information)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate_authority_arn: Option<::Value<String>> = None;
                let mut directory_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut vpc_information: Option<::Value<self::connector::VpcInformation>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateAuthorityArn" => {
                            certificate_authority_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DirectoryId" => {
                            directory_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcInformation" => {
                            vpc_information = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectorProperties {
                    certificate_authority_arn: certificate_authority_arn.ok_or(::serde::de::Error::missing_field("CertificateAuthorityArn"))?,
                    directory_id: directory_id.ok_or(::serde::de::Error::missing_field("DirectoryId"))?,
                    tags: tags,
                    vpc_information: vpc_information.ok_or(::serde::de::Error::missing_field("VpcInformation"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Connector {
    type Properties = ConnectorProperties;
    const TYPE: &'static str = "AWS::PCAConnectorAD::Connector";
    fn properties(&self) -> &ConnectorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Connector {}

impl From<ConnectorProperties> for Connector {
    fn from(properties: ConnectorProperties) -> Connector {
        Connector { properties }
    }
}

/// The [`AWS::PCAConnectorAD::DirectoryRegistration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-directoryregistration.html) resource type.
#[derive(Debug, Default)]
pub struct DirectoryRegistration {
    properties: DirectoryRegistrationProperties
}

/// Properties for the `DirectoryRegistration` resource.
#[derive(Debug, Default)]
pub struct DirectoryRegistrationProperties {
    /// Property [`DirectoryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-directoryregistration.html#cfn-pcaconnectorad-directoryregistration-directoryid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub directory_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-directoryregistration.html#cfn-pcaconnectorad-directoryregistration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for DirectoryRegistrationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryId", &self.directory_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DirectoryRegistrationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DirectoryRegistrationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DirectoryRegistrationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DirectoryRegistrationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut directory_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DirectoryId" => {
                            directory_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DirectoryRegistrationProperties {
                    directory_id: directory_id.ok_or(::serde::de::Error::missing_field("DirectoryId"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DirectoryRegistration {
    type Properties = DirectoryRegistrationProperties;
    const TYPE: &'static str = "AWS::PCAConnectorAD::DirectoryRegistration";
    fn properties(&self) -> &DirectoryRegistrationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DirectoryRegistrationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DirectoryRegistration {}

impl From<DirectoryRegistrationProperties> for DirectoryRegistration {
    fn from(properties: DirectoryRegistrationProperties) -> DirectoryRegistration {
        DirectoryRegistration { properties }
    }
}

/// The [`AWS::PCAConnectorAD::ServicePrincipalName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-serviceprincipalname.html) resource type.
#[derive(Debug, Default)]
pub struct ServicePrincipalName {
    properties: ServicePrincipalNameProperties
}

/// Properties for the `ServicePrincipalName` resource.
#[derive(Debug, Default)]
pub struct ServicePrincipalNameProperties {
    /// Property [`ConnectorArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-serviceprincipalname.html#cfn-pcaconnectorad-serviceprincipalname-connectorarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_arn: Option<::Value<String>>,
    /// Property [`DirectoryRegistrationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-serviceprincipalname.html#cfn-pcaconnectorad-serviceprincipalname-directoryregistrationarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub directory_registration_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for ServicePrincipalNameProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref connector_arn) = self.connector_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorArn", connector_arn)?;
        }
        if let Some(ref directory_registration_arn) = self.directory_registration_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryRegistrationArn", directory_registration_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServicePrincipalNameProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServicePrincipalNameProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServicePrincipalNameProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServicePrincipalNameProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connector_arn: Option<::Value<String>> = None;
                let mut directory_registration_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectorArn" => {
                            connector_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DirectoryRegistrationArn" => {
                            directory_registration_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServicePrincipalNameProperties {
                    connector_arn: connector_arn,
                    directory_registration_arn: directory_registration_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ServicePrincipalName {
    type Properties = ServicePrincipalNameProperties;
    const TYPE: &'static str = "AWS::PCAConnectorAD::ServicePrincipalName";
    fn properties(&self) -> &ServicePrincipalNameProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServicePrincipalNameProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ServicePrincipalName {}

impl From<ServicePrincipalNameProperties> for ServicePrincipalName {
    fn from(properties: ServicePrincipalNameProperties) -> ServicePrincipalName {
        ServicePrincipalName { properties }
    }
}

/// The [`AWS::PCAConnectorAD::Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-template.html) resource type.
#[derive(Debug, Default)]
pub struct Template {
    properties: TemplateProperties
}

/// Properties for the `Template` resource.
#[derive(Debug, Default)]
pub struct TemplateProperties {
    /// Property [`ConnectorArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-template.html#cfn-pcaconnectorad-template-connectorarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_arn: ::Value<String>,
    /// Property [`Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-template.html#cfn-pcaconnectorad-template-definition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub definition: ::Value<self::template::TemplateDefinition>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-template.html#cfn-pcaconnectorad-template-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ReenrollAllCertificateHolders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-template.html#cfn-pcaconnectorad-template-reenrollallcertificateholders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub reenroll_all_certificate_holders: Option<::Value<bool>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-template.html#cfn-pcaconnectorad-template-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for TemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorArn", &self.connector_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Definition", &self.definition)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref reenroll_all_certificate_holders) = self.reenroll_all_certificate_holders {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReenrollAllCertificateHolders", reenroll_all_certificate_holders)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connector_arn: Option<::Value<String>> = None;
                let mut definition: Option<::Value<self::template::TemplateDefinition>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut reenroll_all_certificate_holders: Option<::Value<bool>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectorArn" => {
                            connector_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Definition" => {
                            definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReenrollAllCertificateHolders" => {
                            reenroll_all_certificate_holders = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TemplateProperties {
                    connector_arn: connector_arn.ok_or(::serde::de::Error::missing_field("ConnectorArn"))?,
                    definition: definition.ok_or(::serde::de::Error::missing_field("Definition"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    reenroll_all_certificate_holders: reenroll_all_certificate_holders,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Template {
    type Properties = TemplateProperties;
    const TYPE: &'static str = "AWS::PCAConnectorAD::Template";
    fn properties(&self) -> &TemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Template {}

impl From<TemplateProperties> for Template {
    fn from(properties: TemplateProperties) -> Template {
        Template { properties }
    }
}

/// The [`AWS::PCAConnectorAD::TemplateGroupAccessControlEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-templategroupaccesscontrolentry.html) resource type.
#[derive(Debug, Default)]
pub struct TemplateGroupAccessControlEntry {
    properties: TemplateGroupAccessControlEntryProperties
}

/// Properties for the `TemplateGroupAccessControlEntry` resource.
#[derive(Debug, Default)]
pub struct TemplateGroupAccessControlEntryProperties {
    /// Property [`AccessRights`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-templategroupaccesscontrolentry.html#cfn-pcaconnectorad-templategroupaccesscontrolentry-accessrights).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_rights: ::Value<self::template_group_access_control_entry::AccessRights>,
    /// Property [`GroupDisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-templategroupaccesscontrolentry.html#cfn-pcaconnectorad-templategroupaccesscontrolentry-groupdisplayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub group_display_name: ::Value<String>,
    /// Property [`GroupSecurityIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-templategroupaccesscontrolentry.html#cfn-pcaconnectorad-templategroupaccesscontrolentry-groupsecurityidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub group_security_identifier: Option<::Value<String>>,
    /// Property [`TemplateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-templategroupaccesscontrolentry.html#cfn-pcaconnectorad-templategroupaccesscontrolentry-templatearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub template_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for TemplateGroupAccessControlEntryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessRights", &self.access_rights)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupDisplayName", &self.group_display_name)?;
        if let Some(ref group_security_identifier) = self.group_security_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupSecurityIdentifier", group_security_identifier)?;
        }
        if let Some(ref template_arn) = self.template_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateArn", template_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TemplateGroupAccessControlEntryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateGroupAccessControlEntryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TemplateGroupAccessControlEntryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TemplateGroupAccessControlEntryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_rights: Option<::Value<self::template_group_access_control_entry::AccessRights>> = None;
                let mut group_display_name: Option<::Value<String>> = None;
                let mut group_security_identifier: Option<::Value<String>> = None;
                let mut template_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessRights" => {
                            access_rights = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GroupDisplayName" => {
                            group_display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GroupSecurityIdentifier" => {
                            group_security_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateArn" => {
                            template_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TemplateGroupAccessControlEntryProperties {
                    access_rights: access_rights.ok_or(::serde::de::Error::missing_field("AccessRights"))?,
                    group_display_name: group_display_name.ok_or(::serde::de::Error::missing_field("GroupDisplayName"))?,
                    group_security_identifier: group_security_identifier,
                    template_arn: template_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TemplateGroupAccessControlEntry {
    type Properties = TemplateGroupAccessControlEntryProperties;
    const TYPE: &'static str = "AWS::PCAConnectorAD::TemplateGroupAccessControlEntry";
    fn properties(&self) -> &TemplateGroupAccessControlEntryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TemplateGroupAccessControlEntryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TemplateGroupAccessControlEntry {}

impl From<TemplateGroupAccessControlEntryProperties> for TemplateGroupAccessControlEntry {
    fn from(properties: TemplateGroupAccessControlEntryProperties) -> TemplateGroupAccessControlEntry {
        TemplateGroupAccessControlEntry { properties }
    }
}

pub mod connector {
    //! Property types for the `Connector` resource.

    /// The [`AWS::PCAConnectorAD::Connector.VpcInformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-connector-vpcinformation.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcInformation {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-connector-vpcinformation.html#cfn-pcaconnectorad-connector-vpcinformation-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcInformation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcInformation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcInformation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcInformation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcInformation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcInformation {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod template {
    //! Property types for the `Template` resource.

    /// The [`AWS::PCAConnectorAD::Template.ApplicationPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-applicationpolicies.html) property type.
    #[derive(Debug, Default)]
    pub struct ApplicationPolicies {
        /// Property [`Critical`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-applicationpolicies.html#cfn-pcaconnectorad-template-applicationpolicies-critical).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub critical: Option<::Value<bool>>,
        /// Property [`Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-applicationpolicies.html#cfn-pcaconnectorad-template-applicationpolicies-policies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policies: ::ValueList<ApplicationPolicy>,
    }

    impl ::codec::SerializeValue for ApplicationPolicies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref critical) = self.critical {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Critical", critical)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", &self.policies)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationPolicies {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationPolicies, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationPolicies;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationPolicies")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut critical: Option<::Value<bool>> = None;
                    let mut policies: Option<::ValueList<ApplicationPolicy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Critical" => {
                                critical = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Policies" => {
                                policies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationPolicies {
                        critical: critical,
                        policies: policies.ok_or(::serde::de::Error::missing_field("Policies"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.ApplicationPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-applicationpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ApplicationPolicy {
        /// Property [`PolicyObjectIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-applicationpolicy.html#cfn-pcaconnectorad-template-applicationpolicy-policyobjectidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_object_identifier: Option<::Value<String>>,
        /// Property [`PolicyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-applicationpolicy.html#cfn-pcaconnectorad-template-applicationpolicy-policytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ApplicationPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref policy_object_identifier) = self.policy_object_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyObjectIdentifier", policy_object_identifier)?;
            }
            if let Some(ref policy_type) = self.policy_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyType", policy_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_object_identifier: Option<::Value<String>> = None;
                    let mut policy_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyObjectIdentifier" => {
                                policy_object_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyType" => {
                                policy_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationPolicy {
                        policy_object_identifier: policy_object_identifier,
                        policy_type: policy_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.CertificateValidity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-certificatevalidity.html) property type.
    #[derive(Debug, Default)]
    pub struct CertificateValidity {
        /// Property [`RenewalPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-certificatevalidity.html#cfn-pcaconnectorad-template-certificatevalidity-renewalperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub renewal_period: ::Value<ValidityPeriod>,
        /// Property [`ValidityPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-certificatevalidity.html#cfn-pcaconnectorad-template-certificatevalidity-validityperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub validity_period: ::Value<ValidityPeriod>,
    }

    impl ::codec::SerializeValue for CertificateValidity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RenewalPeriod", &self.renewal_period)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidityPeriod", &self.validity_period)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CertificateValidity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateValidity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CertificateValidity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CertificateValidity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut renewal_period: Option<::Value<ValidityPeriod>> = None;
                    let mut validity_period: Option<::Value<ValidityPeriod>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RenewalPeriod" => {
                                renewal_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValidityPeriod" => {
                                validity_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CertificateValidity {
                        renewal_period: renewal_period.ok_or(::serde::de::Error::missing_field("RenewalPeriod"))?,
                        validity_period: validity_period.ok_or(::serde::de::Error::missing_field("ValidityPeriod"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.EnrollmentFlagsV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv2.html) property type.
    #[derive(Debug, Default)]
    pub struct EnrollmentFlagsV2 {
        /// Property [`EnableKeyReuseOnNtTokenKeysetStorageFull`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv2.html#cfn-pcaconnectorad-template-enrollmentflagsv2-enablekeyreuseonnttokenkeysetstoragefull).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_key_reuse_on_nt_token_keyset_storage_full: Option<::Value<bool>>,
        /// Property [`IncludeSymmetricAlgorithms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv2.html#cfn-pcaconnectorad-template-enrollmentflagsv2-includesymmetricalgorithms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_symmetric_algorithms: Option<::Value<bool>>,
        /// Property [`NoSecurityExtension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv2.html#cfn-pcaconnectorad-template-enrollmentflagsv2-nosecurityextension).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_security_extension: Option<::Value<bool>>,
        /// Property [`RemoveInvalidCertificateFromPersonalStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv2.html#cfn-pcaconnectorad-template-enrollmentflagsv2-removeinvalidcertificatefrompersonalstore).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_invalid_certificate_from_personal_store: Option<::Value<bool>>,
        /// Property [`UserInteractionRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv2.html#cfn-pcaconnectorad-template-enrollmentflagsv2-userinteractionrequired).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_interaction_required: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for EnrollmentFlagsV2 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_key_reuse_on_nt_token_keyset_storage_full) = self.enable_key_reuse_on_nt_token_keyset_storage_full {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableKeyReuseOnNtTokenKeysetStorageFull", enable_key_reuse_on_nt_token_keyset_storage_full)?;
            }
            if let Some(ref include_symmetric_algorithms) = self.include_symmetric_algorithms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeSymmetricAlgorithms", include_symmetric_algorithms)?;
            }
            if let Some(ref no_security_extension) = self.no_security_extension {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoSecurityExtension", no_security_extension)?;
            }
            if let Some(ref remove_invalid_certificate_from_personal_store) = self.remove_invalid_certificate_from_personal_store {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveInvalidCertificateFromPersonalStore", remove_invalid_certificate_from_personal_store)?;
            }
            if let Some(ref user_interaction_required) = self.user_interaction_required {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserInteractionRequired", user_interaction_required)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnrollmentFlagsV2 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnrollmentFlagsV2, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnrollmentFlagsV2;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnrollmentFlagsV2")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_key_reuse_on_nt_token_keyset_storage_full: Option<::Value<bool>> = None;
                    let mut include_symmetric_algorithms: Option<::Value<bool>> = None;
                    let mut no_security_extension: Option<::Value<bool>> = None;
                    let mut remove_invalid_certificate_from_personal_store: Option<::Value<bool>> = None;
                    let mut user_interaction_required: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableKeyReuseOnNtTokenKeysetStorageFull" => {
                                enable_key_reuse_on_nt_token_keyset_storage_full = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeSymmetricAlgorithms" => {
                                include_symmetric_algorithms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoSecurityExtension" => {
                                no_security_extension = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveInvalidCertificateFromPersonalStore" => {
                                remove_invalid_certificate_from_personal_store = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserInteractionRequired" => {
                                user_interaction_required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnrollmentFlagsV2 {
                        enable_key_reuse_on_nt_token_keyset_storage_full: enable_key_reuse_on_nt_token_keyset_storage_full,
                        include_symmetric_algorithms: include_symmetric_algorithms,
                        no_security_extension: no_security_extension,
                        remove_invalid_certificate_from_personal_store: remove_invalid_certificate_from_personal_store,
                        user_interaction_required: user_interaction_required,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.EnrollmentFlagsV3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv3.html) property type.
    #[derive(Debug, Default)]
    pub struct EnrollmentFlagsV3 {
        /// Property [`EnableKeyReuseOnNtTokenKeysetStorageFull`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv3.html#cfn-pcaconnectorad-template-enrollmentflagsv3-enablekeyreuseonnttokenkeysetstoragefull).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_key_reuse_on_nt_token_keyset_storage_full: Option<::Value<bool>>,
        /// Property [`IncludeSymmetricAlgorithms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv3.html#cfn-pcaconnectorad-template-enrollmentflagsv3-includesymmetricalgorithms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_symmetric_algorithms: Option<::Value<bool>>,
        /// Property [`NoSecurityExtension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv3.html#cfn-pcaconnectorad-template-enrollmentflagsv3-nosecurityextension).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_security_extension: Option<::Value<bool>>,
        /// Property [`RemoveInvalidCertificateFromPersonalStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv3.html#cfn-pcaconnectorad-template-enrollmentflagsv3-removeinvalidcertificatefrompersonalstore).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_invalid_certificate_from_personal_store: Option<::Value<bool>>,
        /// Property [`UserInteractionRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv3.html#cfn-pcaconnectorad-template-enrollmentflagsv3-userinteractionrequired).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_interaction_required: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for EnrollmentFlagsV3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_key_reuse_on_nt_token_keyset_storage_full) = self.enable_key_reuse_on_nt_token_keyset_storage_full {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableKeyReuseOnNtTokenKeysetStorageFull", enable_key_reuse_on_nt_token_keyset_storage_full)?;
            }
            if let Some(ref include_symmetric_algorithms) = self.include_symmetric_algorithms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeSymmetricAlgorithms", include_symmetric_algorithms)?;
            }
            if let Some(ref no_security_extension) = self.no_security_extension {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoSecurityExtension", no_security_extension)?;
            }
            if let Some(ref remove_invalid_certificate_from_personal_store) = self.remove_invalid_certificate_from_personal_store {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveInvalidCertificateFromPersonalStore", remove_invalid_certificate_from_personal_store)?;
            }
            if let Some(ref user_interaction_required) = self.user_interaction_required {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserInteractionRequired", user_interaction_required)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnrollmentFlagsV3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnrollmentFlagsV3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnrollmentFlagsV3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnrollmentFlagsV3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_key_reuse_on_nt_token_keyset_storage_full: Option<::Value<bool>> = None;
                    let mut include_symmetric_algorithms: Option<::Value<bool>> = None;
                    let mut no_security_extension: Option<::Value<bool>> = None;
                    let mut remove_invalid_certificate_from_personal_store: Option<::Value<bool>> = None;
                    let mut user_interaction_required: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableKeyReuseOnNtTokenKeysetStorageFull" => {
                                enable_key_reuse_on_nt_token_keyset_storage_full = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeSymmetricAlgorithms" => {
                                include_symmetric_algorithms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoSecurityExtension" => {
                                no_security_extension = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveInvalidCertificateFromPersonalStore" => {
                                remove_invalid_certificate_from_personal_store = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserInteractionRequired" => {
                                user_interaction_required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnrollmentFlagsV3 {
                        enable_key_reuse_on_nt_token_keyset_storage_full: enable_key_reuse_on_nt_token_keyset_storage_full,
                        include_symmetric_algorithms: include_symmetric_algorithms,
                        no_security_extension: no_security_extension,
                        remove_invalid_certificate_from_personal_store: remove_invalid_certificate_from_personal_store,
                        user_interaction_required: user_interaction_required,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.EnrollmentFlagsV4`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv4.html) property type.
    #[derive(Debug, Default)]
    pub struct EnrollmentFlagsV4 {
        /// Property [`EnableKeyReuseOnNtTokenKeysetStorageFull`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv4.html#cfn-pcaconnectorad-template-enrollmentflagsv4-enablekeyreuseonnttokenkeysetstoragefull).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_key_reuse_on_nt_token_keyset_storage_full: Option<::Value<bool>>,
        /// Property [`IncludeSymmetricAlgorithms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv4.html#cfn-pcaconnectorad-template-enrollmentflagsv4-includesymmetricalgorithms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_symmetric_algorithms: Option<::Value<bool>>,
        /// Property [`NoSecurityExtension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv4.html#cfn-pcaconnectorad-template-enrollmentflagsv4-nosecurityextension).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_security_extension: Option<::Value<bool>>,
        /// Property [`RemoveInvalidCertificateFromPersonalStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv4.html#cfn-pcaconnectorad-template-enrollmentflagsv4-removeinvalidcertificatefrompersonalstore).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_invalid_certificate_from_personal_store: Option<::Value<bool>>,
        /// Property [`UserInteractionRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv4.html#cfn-pcaconnectorad-template-enrollmentflagsv4-userinteractionrequired).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_interaction_required: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for EnrollmentFlagsV4 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_key_reuse_on_nt_token_keyset_storage_full) = self.enable_key_reuse_on_nt_token_keyset_storage_full {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableKeyReuseOnNtTokenKeysetStorageFull", enable_key_reuse_on_nt_token_keyset_storage_full)?;
            }
            if let Some(ref include_symmetric_algorithms) = self.include_symmetric_algorithms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeSymmetricAlgorithms", include_symmetric_algorithms)?;
            }
            if let Some(ref no_security_extension) = self.no_security_extension {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoSecurityExtension", no_security_extension)?;
            }
            if let Some(ref remove_invalid_certificate_from_personal_store) = self.remove_invalid_certificate_from_personal_store {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveInvalidCertificateFromPersonalStore", remove_invalid_certificate_from_personal_store)?;
            }
            if let Some(ref user_interaction_required) = self.user_interaction_required {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserInteractionRequired", user_interaction_required)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnrollmentFlagsV4 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnrollmentFlagsV4, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnrollmentFlagsV4;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnrollmentFlagsV4")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_key_reuse_on_nt_token_keyset_storage_full: Option<::Value<bool>> = None;
                    let mut include_symmetric_algorithms: Option<::Value<bool>> = None;
                    let mut no_security_extension: Option<::Value<bool>> = None;
                    let mut remove_invalid_certificate_from_personal_store: Option<::Value<bool>> = None;
                    let mut user_interaction_required: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableKeyReuseOnNtTokenKeysetStorageFull" => {
                                enable_key_reuse_on_nt_token_keyset_storage_full = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeSymmetricAlgorithms" => {
                                include_symmetric_algorithms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoSecurityExtension" => {
                                no_security_extension = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveInvalidCertificateFromPersonalStore" => {
                                remove_invalid_certificate_from_personal_store = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserInteractionRequired" => {
                                user_interaction_required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnrollmentFlagsV4 {
                        enable_key_reuse_on_nt_token_keyset_storage_full: enable_key_reuse_on_nt_token_keyset_storage_full,
                        include_symmetric_algorithms: include_symmetric_algorithms,
                        no_security_extension: no_security_extension,
                        remove_invalid_certificate_from_personal_store: remove_invalid_certificate_from_personal_store,
                        user_interaction_required: user_interaction_required,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.ExtensionsV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv2.html) property type.
    #[derive(Debug, Default)]
    pub struct ExtensionsV2 {
        /// Property [`ApplicationPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv2.html#cfn-pcaconnectorad-template-extensionsv2-applicationpolicies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_policies: Option<::Value<ApplicationPolicies>>,
        /// Property [`KeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv2.html#cfn-pcaconnectorad-template-extensionsv2-keyusage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_usage: ::Value<KeyUsage>,
    }

    impl ::codec::SerializeValue for ExtensionsV2 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_policies) = self.application_policies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationPolicies", application_policies)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyUsage", &self.key_usage)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExtensionsV2 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExtensionsV2, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExtensionsV2;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExtensionsV2")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_policies: Option<::Value<ApplicationPolicies>> = None;
                    let mut key_usage: Option<::Value<KeyUsage>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationPolicies" => {
                                application_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyUsage" => {
                                key_usage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExtensionsV2 {
                        application_policies: application_policies,
                        key_usage: key_usage.ok_or(::serde::de::Error::missing_field("KeyUsage"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.ExtensionsV3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv3.html) property type.
    #[derive(Debug, Default)]
    pub struct ExtensionsV3 {
        /// Property [`ApplicationPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv3.html#cfn-pcaconnectorad-template-extensionsv3-applicationpolicies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_policies: Option<::Value<ApplicationPolicies>>,
        /// Property [`KeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv3.html#cfn-pcaconnectorad-template-extensionsv3-keyusage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_usage: ::Value<KeyUsage>,
    }

    impl ::codec::SerializeValue for ExtensionsV3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_policies) = self.application_policies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationPolicies", application_policies)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyUsage", &self.key_usage)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExtensionsV3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExtensionsV3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExtensionsV3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExtensionsV3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_policies: Option<::Value<ApplicationPolicies>> = None;
                    let mut key_usage: Option<::Value<KeyUsage>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationPolicies" => {
                                application_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyUsage" => {
                                key_usage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExtensionsV3 {
                        application_policies: application_policies,
                        key_usage: key_usage.ok_or(::serde::de::Error::missing_field("KeyUsage"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.ExtensionsV4`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv4.html) property type.
    #[derive(Debug, Default)]
    pub struct ExtensionsV4 {
        /// Property [`ApplicationPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv4.html#cfn-pcaconnectorad-template-extensionsv4-applicationpolicies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_policies: Option<::Value<ApplicationPolicies>>,
        /// Property [`KeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv4.html#cfn-pcaconnectorad-template-extensionsv4-keyusage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_usage: ::Value<KeyUsage>,
    }

    impl ::codec::SerializeValue for ExtensionsV4 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_policies) = self.application_policies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationPolicies", application_policies)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyUsage", &self.key_usage)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExtensionsV4 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExtensionsV4, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExtensionsV4;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExtensionsV4")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_policies: Option<::Value<ApplicationPolicies>> = None;
                    let mut key_usage: Option<::Value<KeyUsage>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationPolicies" => {
                                application_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyUsage" => {
                                key_usage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExtensionsV4 {
                        application_policies: application_policies,
                        key_usage: key_usage.ok_or(::serde::de::Error::missing_field("KeyUsage"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.GeneralFlagsV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv2.html) property type.
    #[derive(Debug, Default)]
    pub struct GeneralFlagsV2 {
        /// Property [`AutoEnrollment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv2.html#cfn-pcaconnectorad-template-generalflagsv2-autoenrollment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_enrollment: Option<::Value<bool>>,
        /// Property [`MachineType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv2.html#cfn-pcaconnectorad-template-generalflagsv2-machinetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub machine_type: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for GeneralFlagsV2 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_enrollment) = self.auto_enrollment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoEnrollment", auto_enrollment)?;
            }
            if let Some(ref machine_type) = self.machine_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MachineType", machine_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeneralFlagsV2 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeneralFlagsV2, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeneralFlagsV2;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeneralFlagsV2")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_enrollment: Option<::Value<bool>> = None;
                    let mut machine_type: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoEnrollment" => {
                                auto_enrollment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MachineType" => {
                                machine_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeneralFlagsV2 {
                        auto_enrollment: auto_enrollment,
                        machine_type: machine_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.GeneralFlagsV3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv3.html) property type.
    #[derive(Debug, Default)]
    pub struct GeneralFlagsV3 {
        /// Property [`AutoEnrollment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv3.html#cfn-pcaconnectorad-template-generalflagsv3-autoenrollment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_enrollment: Option<::Value<bool>>,
        /// Property [`MachineType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv3.html#cfn-pcaconnectorad-template-generalflagsv3-machinetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub machine_type: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for GeneralFlagsV3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_enrollment) = self.auto_enrollment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoEnrollment", auto_enrollment)?;
            }
            if let Some(ref machine_type) = self.machine_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MachineType", machine_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeneralFlagsV3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeneralFlagsV3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeneralFlagsV3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeneralFlagsV3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_enrollment: Option<::Value<bool>> = None;
                    let mut machine_type: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoEnrollment" => {
                                auto_enrollment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MachineType" => {
                                machine_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeneralFlagsV3 {
                        auto_enrollment: auto_enrollment,
                        machine_type: machine_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.GeneralFlagsV4`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv4.html) property type.
    #[derive(Debug, Default)]
    pub struct GeneralFlagsV4 {
        /// Property [`AutoEnrollment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv4.html#cfn-pcaconnectorad-template-generalflagsv4-autoenrollment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_enrollment: Option<::Value<bool>>,
        /// Property [`MachineType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv4.html#cfn-pcaconnectorad-template-generalflagsv4-machinetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub machine_type: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for GeneralFlagsV4 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_enrollment) = self.auto_enrollment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoEnrollment", auto_enrollment)?;
            }
            if let Some(ref machine_type) = self.machine_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MachineType", machine_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeneralFlagsV4 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeneralFlagsV4, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeneralFlagsV4;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeneralFlagsV4")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_enrollment: Option<::Value<bool>> = None;
                    let mut machine_type: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoEnrollment" => {
                                auto_enrollment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MachineType" => {
                                machine_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeneralFlagsV4 {
                        auto_enrollment: auto_enrollment,
                        machine_type: machine_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.KeyUsage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusage.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyUsage {
        /// Property [`Critical`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusage.html#cfn-pcaconnectorad-template-keyusage-critical).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub critical: Option<::Value<bool>>,
        /// Property [`UsageFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusage.html#cfn-pcaconnectorad-template-keyusage-usageflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub usage_flags: ::Value<KeyUsageFlags>,
    }

    impl ::codec::SerializeValue for KeyUsage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref critical) = self.critical {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Critical", critical)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsageFlags", &self.usage_flags)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyUsage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyUsage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyUsage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyUsage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut critical: Option<::Value<bool>> = None;
                    let mut usage_flags: Option<::Value<KeyUsageFlags>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Critical" => {
                                critical = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UsageFlags" => {
                                usage_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyUsage {
                        critical: critical,
                        usage_flags: usage_flags.ok_or(::serde::de::Error::missing_field("UsageFlags"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.KeyUsageFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageflags.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyUsageFlags {
        /// Property [`DataEncipherment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageflags.html#cfn-pcaconnectorad-template-keyusageflags-dataencipherment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_encipherment: Option<::Value<bool>>,
        /// Property [`DigitalSignature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageflags.html#cfn-pcaconnectorad-template-keyusageflags-digitalsignature).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub digital_signature: Option<::Value<bool>>,
        /// Property [`KeyAgreement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageflags.html#cfn-pcaconnectorad-template-keyusageflags-keyagreement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_agreement: Option<::Value<bool>>,
        /// Property [`KeyEncipherment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageflags.html#cfn-pcaconnectorad-template-keyusageflags-keyencipherment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_encipherment: Option<::Value<bool>>,
        /// Property [`NonRepudiation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageflags.html#cfn-pcaconnectorad-template-keyusageflags-nonrepudiation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub non_repudiation: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for KeyUsageFlags {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_encipherment) = self.data_encipherment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataEncipherment", data_encipherment)?;
            }
            if let Some(ref digital_signature) = self.digital_signature {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DigitalSignature", digital_signature)?;
            }
            if let Some(ref key_agreement) = self.key_agreement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyAgreement", key_agreement)?;
            }
            if let Some(ref key_encipherment) = self.key_encipherment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyEncipherment", key_encipherment)?;
            }
            if let Some(ref non_repudiation) = self.non_repudiation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NonRepudiation", non_repudiation)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyUsageFlags {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyUsageFlags, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyUsageFlags;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyUsageFlags")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_encipherment: Option<::Value<bool>> = None;
                    let mut digital_signature: Option<::Value<bool>> = None;
                    let mut key_agreement: Option<::Value<bool>> = None;
                    let mut key_encipherment: Option<::Value<bool>> = None;
                    let mut non_repudiation: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataEncipherment" => {
                                data_encipherment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DigitalSignature" => {
                                digital_signature = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyAgreement" => {
                                key_agreement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyEncipherment" => {
                                key_encipherment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NonRepudiation" => {
                                non_repudiation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyUsageFlags {
                        data_encipherment: data_encipherment,
                        digital_signature: digital_signature,
                        key_agreement: key_agreement,
                        key_encipherment: key_encipherment,
                        non_repudiation: non_repudiation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.KeyUsageProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyUsageProperty {
        /// Property [`PropertyFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageproperty.html#cfn-pcaconnectorad-template-keyusageproperty-propertyflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_flags: Option<::Value<KeyUsagePropertyFlags>>,
        /// Property [`PropertyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageproperty.html#cfn-pcaconnectorad-template-keyusageproperty-propertytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KeyUsageProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref property_flags) = self.property_flags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyFlags", property_flags)?;
            }
            if let Some(ref property_type) = self.property_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyType", property_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyUsageProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyUsageProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyUsageProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyUsageProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut property_flags: Option<::Value<KeyUsagePropertyFlags>> = None;
                    let mut property_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PropertyFlags" => {
                                property_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyType" => {
                                property_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyUsageProperty {
                        property_flags: property_flags,
                        property_type: property_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.KeyUsagePropertyFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusagepropertyflags.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyUsagePropertyFlags {
        /// Property [`Decrypt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusagepropertyflags.html#cfn-pcaconnectorad-template-keyusagepropertyflags-decrypt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub decrypt: Option<::Value<bool>>,
        /// Property [`KeyAgreement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusagepropertyflags.html#cfn-pcaconnectorad-template-keyusagepropertyflags-keyagreement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_agreement: Option<::Value<bool>>,
        /// Property [`Sign`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusagepropertyflags.html#cfn-pcaconnectorad-template-keyusagepropertyflags-sign).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sign: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for KeyUsagePropertyFlags {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref decrypt) = self.decrypt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Decrypt", decrypt)?;
            }
            if let Some(ref key_agreement) = self.key_agreement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyAgreement", key_agreement)?;
            }
            if let Some(ref sign) = self.sign {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sign", sign)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyUsagePropertyFlags {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyUsagePropertyFlags, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyUsagePropertyFlags;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyUsagePropertyFlags")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut decrypt: Option<::Value<bool>> = None;
                    let mut key_agreement: Option<::Value<bool>> = None;
                    let mut sign: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Decrypt" => {
                                decrypt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyAgreement" => {
                                key_agreement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sign" => {
                                sign = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyUsagePropertyFlags {
                        decrypt: decrypt,
                        key_agreement: key_agreement,
                        sign: sign,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.PrivateKeyAttributesV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv2.html) property type.
    #[derive(Debug, Default)]
    pub struct PrivateKeyAttributesV2 {
        /// Property [`CryptoProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv2.html#cfn-pcaconnectorad-template-privatekeyattributesv2-cryptoproviders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crypto_providers: Option<::ValueList<String>>,
        /// Property [`KeySpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv2.html#cfn-pcaconnectorad-template-privatekeyattributesv2-keyspec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_spec: ::Value<String>,
        /// Property [`MinimalKeyLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv2.html#cfn-pcaconnectorad-template-privatekeyattributesv2-minimalkeylength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimal_key_length: ::Value<f64>,
    }

    impl ::codec::SerializeValue for PrivateKeyAttributesV2 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref crypto_providers) = self.crypto_providers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CryptoProviders", crypto_providers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySpec", &self.key_spec)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimalKeyLength", &self.minimal_key_length)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrivateKeyAttributesV2 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateKeyAttributesV2, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrivateKeyAttributesV2;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrivateKeyAttributesV2")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut crypto_providers: Option<::ValueList<String>> = None;
                    let mut key_spec: Option<::Value<String>> = None;
                    let mut minimal_key_length: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CryptoProviders" => {
                                crypto_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeySpec" => {
                                key_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinimalKeyLength" => {
                                minimal_key_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrivateKeyAttributesV2 {
                        crypto_providers: crypto_providers,
                        key_spec: key_spec.ok_or(::serde::de::Error::missing_field("KeySpec"))?,
                        minimal_key_length: minimal_key_length.ok_or(::serde::de::Error::missing_field("MinimalKeyLength"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.PrivateKeyAttributesV3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv3.html) property type.
    #[derive(Debug, Default)]
    pub struct PrivateKeyAttributesV3 {
        /// Property [`Algorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv3.html#cfn-pcaconnectorad-template-privatekeyattributesv3-algorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub algorithm: ::Value<String>,
        /// Property [`CryptoProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv3.html#cfn-pcaconnectorad-template-privatekeyattributesv3-cryptoproviders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crypto_providers: Option<::ValueList<String>>,
        /// Property [`KeySpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv3.html#cfn-pcaconnectorad-template-privatekeyattributesv3-keyspec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_spec: ::Value<String>,
        /// Property [`KeyUsageProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv3.html#cfn-pcaconnectorad-template-privatekeyattributesv3-keyusageproperty).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_usage_property: ::Value<KeyUsageProperty>,
        /// Property [`MinimalKeyLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv3.html#cfn-pcaconnectorad-template-privatekeyattributesv3-minimalkeylength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimal_key_length: ::Value<f64>,
    }

    impl ::codec::SerializeValue for PrivateKeyAttributesV3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", &self.algorithm)?;
            if let Some(ref crypto_providers) = self.crypto_providers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CryptoProviders", crypto_providers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySpec", &self.key_spec)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyUsageProperty", &self.key_usage_property)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimalKeyLength", &self.minimal_key_length)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrivateKeyAttributesV3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateKeyAttributesV3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrivateKeyAttributesV3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrivateKeyAttributesV3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm: Option<::Value<String>> = None;
                    let mut crypto_providers: Option<::ValueList<String>> = None;
                    let mut key_spec: Option<::Value<String>> = None;
                    let mut key_usage_property: Option<::Value<KeyUsageProperty>> = None;
                    let mut minimal_key_length: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Algorithm" => {
                                algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CryptoProviders" => {
                                crypto_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeySpec" => {
                                key_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyUsageProperty" => {
                                key_usage_property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinimalKeyLength" => {
                                minimal_key_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrivateKeyAttributesV3 {
                        algorithm: algorithm.ok_or(::serde::de::Error::missing_field("Algorithm"))?,
                        crypto_providers: crypto_providers,
                        key_spec: key_spec.ok_or(::serde::de::Error::missing_field("KeySpec"))?,
                        key_usage_property: key_usage_property.ok_or(::serde::de::Error::missing_field("KeyUsageProperty"))?,
                        minimal_key_length: minimal_key_length.ok_or(::serde::de::Error::missing_field("MinimalKeyLength"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.PrivateKeyAttributesV4`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv4.html) property type.
    #[derive(Debug, Default)]
    pub struct PrivateKeyAttributesV4 {
        /// Property [`Algorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv4.html#cfn-pcaconnectorad-template-privatekeyattributesv4-algorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub algorithm: Option<::Value<String>>,
        /// Property [`CryptoProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv4.html#cfn-pcaconnectorad-template-privatekeyattributesv4-cryptoproviders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crypto_providers: Option<::ValueList<String>>,
        /// Property [`KeySpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv4.html#cfn-pcaconnectorad-template-privatekeyattributesv4-keyspec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_spec: ::Value<String>,
        /// Property [`KeyUsageProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv4.html#cfn-pcaconnectorad-template-privatekeyattributesv4-keyusageproperty).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_usage_property: Option<::Value<KeyUsageProperty>>,
        /// Property [`MinimalKeyLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv4.html#cfn-pcaconnectorad-template-privatekeyattributesv4-minimalkeylength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimal_key_length: ::Value<f64>,
    }

    impl ::codec::SerializeValue for PrivateKeyAttributesV4 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref algorithm) = self.algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", algorithm)?;
            }
            if let Some(ref crypto_providers) = self.crypto_providers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CryptoProviders", crypto_providers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySpec", &self.key_spec)?;
            if let Some(ref key_usage_property) = self.key_usage_property {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyUsageProperty", key_usage_property)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimalKeyLength", &self.minimal_key_length)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrivateKeyAttributesV4 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateKeyAttributesV4, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrivateKeyAttributesV4;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrivateKeyAttributesV4")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm: Option<::Value<String>> = None;
                    let mut crypto_providers: Option<::ValueList<String>> = None;
                    let mut key_spec: Option<::Value<String>> = None;
                    let mut key_usage_property: Option<::Value<KeyUsageProperty>> = None;
                    let mut minimal_key_length: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Algorithm" => {
                                algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CryptoProviders" => {
                                crypto_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeySpec" => {
                                key_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyUsageProperty" => {
                                key_usage_property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinimalKeyLength" => {
                                minimal_key_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrivateKeyAttributesV4 {
                        algorithm: algorithm,
                        crypto_providers: crypto_providers,
                        key_spec: key_spec.ok_or(::serde::de::Error::missing_field("KeySpec"))?,
                        key_usage_property: key_usage_property,
                        minimal_key_length: minimal_key_length.ok_or(::serde::de::Error::missing_field("MinimalKeyLength"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.PrivateKeyFlagsV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv2.html) property type.
    #[derive(Debug, Default)]
    pub struct PrivateKeyFlagsV2 {
        /// Property [`ClientVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv2.html#cfn-pcaconnectorad-template-privatekeyflagsv2-clientversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_version: ::Value<String>,
        /// Property [`ExportableKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv2.html#cfn-pcaconnectorad-template-privatekeyflagsv2-exportablekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exportable_key: Option<::Value<bool>>,
        /// Property [`StrongKeyProtectionRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv2.html#cfn-pcaconnectorad-template-privatekeyflagsv2-strongkeyprotectionrequired).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub strong_key_protection_required: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for PrivateKeyFlagsV2 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientVersion", &self.client_version)?;
            if let Some(ref exportable_key) = self.exportable_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportableKey", exportable_key)?;
            }
            if let Some(ref strong_key_protection_required) = self.strong_key_protection_required {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StrongKeyProtectionRequired", strong_key_protection_required)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrivateKeyFlagsV2 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateKeyFlagsV2, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrivateKeyFlagsV2;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrivateKeyFlagsV2")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_version: Option<::Value<String>> = None;
                    let mut exportable_key: Option<::Value<bool>> = None;
                    let mut strong_key_protection_required: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientVersion" => {
                                client_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExportableKey" => {
                                exportable_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StrongKeyProtectionRequired" => {
                                strong_key_protection_required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrivateKeyFlagsV2 {
                        client_version: client_version.ok_or(::serde::de::Error::missing_field("ClientVersion"))?,
                        exportable_key: exportable_key,
                        strong_key_protection_required: strong_key_protection_required,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.PrivateKeyFlagsV3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv3.html) property type.
    #[derive(Debug, Default)]
    pub struct PrivateKeyFlagsV3 {
        /// Property [`ClientVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv3.html#cfn-pcaconnectorad-template-privatekeyflagsv3-clientversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_version: ::Value<String>,
        /// Property [`ExportableKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv3.html#cfn-pcaconnectorad-template-privatekeyflagsv3-exportablekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exportable_key: Option<::Value<bool>>,
        /// Property [`RequireAlternateSignatureAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv3.html#cfn-pcaconnectorad-template-privatekeyflagsv3-requirealternatesignaturealgorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_alternate_signature_algorithm: Option<::Value<bool>>,
        /// Property [`StrongKeyProtectionRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv3.html#cfn-pcaconnectorad-template-privatekeyflagsv3-strongkeyprotectionrequired).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub strong_key_protection_required: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for PrivateKeyFlagsV3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientVersion", &self.client_version)?;
            if let Some(ref exportable_key) = self.exportable_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportableKey", exportable_key)?;
            }
            if let Some(ref require_alternate_signature_algorithm) = self.require_alternate_signature_algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireAlternateSignatureAlgorithm", require_alternate_signature_algorithm)?;
            }
            if let Some(ref strong_key_protection_required) = self.strong_key_protection_required {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StrongKeyProtectionRequired", strong_key_protection_required)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrivateKeyFlagsV3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateKeyFlagsV3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrivateKeyFlagsV3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrivateKeyFlagsV3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_version: Option<::Value<String>> = None;
                    let mut exportable_key: Option<::Value<bool>> = None;
                    let mut require_alternate_signature_algorithm: Option<::Value<bool>> = None;
                    let mut strong_key_protection_required: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientVersion" => {
                                client_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExportableKey" => {
                                exportable_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireAlternateSignatureAlgorithm" => {
                                require_alternate_signature_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StrongKeyProtectionRequired" => {
                                strong_key_protection_required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrivateKeyFlagsV3 {
                        client_version: client_version.ok_or(::serde::de::Error::missing_field("ClientVersion"))?,
                        exportable_key: exportable_key,
                        require_alternate_signature_algorithm: require_alternate_signature_algorithm,
                        strong_key_protection_required: strong_key_protection_required,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.PrivateKeyFlagsV4`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv4.html) property type.
    #[derive(Debug, Default)]
    pub struct PrivateKeyFlagsV4 {
        /// Property [`ClientVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv4.html#cfn-pcaconnectorad-template-privatekeyflagsv4-clientversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_version: ::Value<String>,
        /// Property [`ExportableKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv4.html#cfn-pcaconnectorad-template-privatekeyflagsv4-exportablekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exportable_key: Option<::Value<bool>>,
        /// Property [`RequireAlternateSignatureAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv4.html#cfn-pcaconnectorad-template-privatekeyflagsv4-requirealternatesignaturealgorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_alternate_signature_algorithm: Option<::Value<bool>>,
        /// Property [`RequireSameKeyRenewal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv4.html#cfn-pcaconnectorad-template-privatekeyflagsv4-requiresamekeyrenewal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_same_key_renewal: Option<::Value<bool>>,
        /// Property [`StrongKeyProtectionRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv4.html#cfn-pcaconnectorad-template-privatekeyflagsv4-strongkeyprotectionrequired).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub strong_key_protection_required: Option<::Value<bool>>,
        /// Property [`UseLegacyProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv4.html#cfn-pcaconnectorad-template-privatekeyflagsv4-uselegacyprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_legacy_provider: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for PrivateKeyFlagsV4 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientVersion", &self.client_version)?;
            if let Some(ref exportable_key) = self.exportable_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportableKey", exportable_key)?;
            }
            if let Some(ref require_alternate_signature_algorithm) = self.require_alternate_signature_algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireAlternateSignatureAlgorithm", require_alternate_signature_algorithm)?;
            }
            if let Some(ref require_same_key_renewal) = self.require_same_key_renewal {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireSameKeyRenewal", require_same_key_renewal)?;
            }
            if let Some(ref strong_key_protection_required) = self.strong_key_protection_required {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StrongKeyProtectionRequired", strong_key_protection_required)?;
            }
            if let Some(ref use_legacy_provider) = self.use_legacy_provider {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseLegacyProvider", use_legacy_provider)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrivateKeyFlagsV4 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateKeyFlagsV4, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrivateKeyFlagsV4;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrivateKeyFlagsV4")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_version: Option<::Value<String>> = None;
                    let mut exportable_key: Option<::Value<bool>> = None;
                    let mut require_alternate_signature_algorithm: Option<::Value<bool>> = None;
                    let mut require_same_key_renewal: Option<::Value<bool>> = None;
                    let mut strong_key_protection_required: Option<::Value<bool>> = None;
                    let mut use_legacy_provider: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientVersion" => {
                                client_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExportableKey" => {
                                exportable_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireAlternateSignatureAlgorithm" => {
                                require_alternate_signature_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireSameKeyRenewal" => {
                                require_same_key_renewal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StrongKeyProtectionRequired" => {
                                strong_key_protection_required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseLegacyProvider" => {
                                use_legacy_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrivateKeyFlagsV4 {
                        client_version: client_version.ok_or(::serde::de::Error::missing_field("ClientVersion"))?,
                        exportable_key: exportable_key,
                        require_alternate_signature_algorithm: require_alternate_signature_algorithm,
                        require_same_key_renewal: require_same_key_renewal,
                        strong_key_protection_required: strong_key_protection_required,
                        use_legacy_provider: use_legacy_provider,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.SubjectNameFlagsV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html) property type.
    #[derive(Debug, Default)]
    pub struct SubjectNameFlagsV2 {
        /// Property [`RequireCommonName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html#cfn-pcaconnectorad-template-subjectnameflagsv2-requirecommonname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_common_name: Option<::Value<bool>>,
        /// Property [`RequireDirectoryPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html#cfn-pcaconnectorad-template-subjectnameflagsv2-requiredirectorypath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_directory_path: Option<::Value<bool>>,
        /// Property [`RequireDnsAsCn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html#cfn-pcaconnectorad-template-subjectnameflagsv2-requirednsascn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_dns_as_cn: Option<::Value<bool>>,
        /// Property [`RequireEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html#cfn-pcaconnectorad-template-subjectnameflagsv2-requireemail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_email: Option<::Value<bool>>,
        /// Property [`SanRequireDirectoryGuid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html#cfn-pcaconnectorad-template-subjectnameflagsv2-sanrequiredirectoryguid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_directory_guid: Option<::Value<bool>>,
        /// Property [`SanRequireDns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html#cfn-pcaconnectorad-template-subjectnameflagsv2-sanrequiredns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_dns: Option<::Value<bool>>,
        /// Property [`SanRequireDomainDns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html#cfn-pcaconnectorad-template-subjectnameflagsv2-sanrequiredomaindns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_domain_dns: Option<::Value<bool>>,
        /// Property [`SanRequireEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html#cfn-pcaconnectorad-template-subjectnameflagsv2-sanrequireemail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_email: Option<::Value<bool>>,
        /// Property [`SanRequireSpn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html#cfn-pcaconnectorad-template-subjectnameflagsv2-sanrequirespn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_spn: Option<::Value<bool>>,
        /// Property [`SanRequireUpn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html#cfn-pcaconnectorad-template-subjectnameflagsv2-sanrequireupn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_upn: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SubjectNameFlagsV2 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref require_common_name) = self.require_common_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireCommonName", require_common_name)?;
            }
            if let Some(ref require_directory_path) = self.require_directory_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireDirectoryPath", require_directory_path)?;
            }
            if let Some(ref require_dns_as_cn) = self.require_dns_as_cn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireDnsAsCn", require_dns_as_cn)?;
            }
            if let Some(ref require_email) = self.require_email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireEmail", require_email)?;
            }
            if let Some(ref san_require_directory_guid) = self.san_require_directory_guid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireDirectoryGuid", san_require_directory_guid)?;
            }
            if let Some(ref san_require_dns) = self.san_require_dns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireDns", san_require_dns)?;
            }
            if let Some(ref san_require_domain_dns) = self.san_require_domain_dns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireDomainDns", san_require_domain_dns)?;
            }
            if let Some(ref san_require_email) = self.san_require_email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireEmail", san_require_email)?;
            }
            if let Some(ref san_require_spn) = self.san_require_spn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireSpn", san_require_spn)?;
            }
            if let Some(ref san_require_upn) = self.san_require_upn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireUpn", san_require_upn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubjectNameFlagsV2 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubjectNameFlagsV2, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubjectNameFlagsV2;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubjectNameFlagsV2")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut require_common_name: Option<::Value<bool>> = None;
                    let mut require_directory_path: Option<::Value<bool>> = None;
                    let mut require_dns_as_cn: Option<::Value<bool>> = None;
                    let mut require_email: Option<::Value<bool>> = None;
                    let mut san_require_directory_guid: Option<::Value<bool>> = None;
                    let mut san_require_dns: Option<::Value<bool>> = None;
                    let mut san_require_domain_dns: Option<::Value<bool>> = None;
                    let mut san_require_email: Option<::Value<bool>> = None;
                    let mut san_require_spn: Option<::Value<bool>> = None;
                    let mut san_require_upn: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RequireCommonName" => {
                                require_common_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireDirectoryPath" => {
                                require_directory_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireDnsAsCn" => {
                                require_dns_as_cn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireEmail" => {
                                require_email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireDirectoryGuid" => {
                                san_require_directory_guid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireDns" => {
                                san_require_dns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireDomainDns" => {
                                san_require_domain_dns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireEmail" => {
                                san_require_email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireSpn" => {
                                san_require_spn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireUpn" => {
                                san_require_upn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubjectNameFlagsV2 {
                        require_common_name: require_common_name,
                        require_directory_path: require_directory_path,
                        require_dns_as_cn: require_dns_as_cn,
                        require_email: require_email,
                        san_require_directory_guid: san_require_directory_guid,
                        san_require_dns: san_require_dns,
                        san_require_domain_dns: san_require_domain_dns,
                        san_require_email: san_require_email,
                        san_require_spn: san_require_spn,
                        san_require_upn: san_require_upn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.SubjectNameFlagsV3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html) property type.
    #[derive(Debug, Default)]
    pub struct SubjectNameFlagsV3 {
        /// Property [`RequireCommonName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html#cfn-pcaconnectorad-template-subjectnameflagsv3-requirecommonname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_common_name: Option<::Value<bool>>,
        /// Property [`RequireDirectoryPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html#cfn-pcaconnectorad-template-subjectnameflagsv3-requiredirectorypath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_directory_path: Option<::Value<bool>>,
        /// Property [`RequireDnsAsCn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html#cfn-pcaconnectorad-template-subjectnameflagsv3-requirednsascn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_dns_as_cn: Option<::Value<bool>>,
        /// Property [`RequireEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html#cfn-pcaconnectorad-template-subjectnameflagsv3-requireemail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_email: Option<::Value<bool>>,
        /// Property [`SanRequireDirectoryGuid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html#cfn-pcaconnectorad-template-subjectnameflagsv3-sanrequiredirectoryguid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_directory_guid: Option<::Value<bool>>,
        /// Property [`SanRequireDns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html#cfn-pcaconnectorad-template-subjectnameflagsv3-sanrequiredns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_dns: Option<::Value<bool>>,
        /// Property [`SanRequireDomainDns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html#cfn-pcaconnectorad-template-subjectnameflagsv3-sanrequiredomaindns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_domain_dns: Option<::Value<bool>>,
        /// Property [`SanRequireEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html#cfn-pcaconnectorad-template-subjectnameflagsv3-sanrequireemail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_email: Option<::Value<bool>>,
        /// Property [`SanRequireSpn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html#cfn-pcaconnectorad-template-subjectnameflagsv3-sanrequirespn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_spn: Option<::Value<bool>>,
        /// Property [`SanRequireUpn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html#cfn-pcaconnectorad-template-subjectnameflagsv3-sanrequireupn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_upn: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SubjectNameFlagsV3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref require_common_name) = self.require_common_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireCommonName", require_common_name)?;
            }
            if let Some(ref require_directory_path) = self.require_directory_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireDirectoryPath", require_directory_path)?;
            }
            if let Some(ref require_dns_as_cn) = self.require_dns_as_cn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireDnsAsCn", require_dns_as_cn)?;
            }
            if let Some(ref require_email) = self.require_email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireEmail", require_email)?;
            }
            if let Some(ref san_require_directory_guid) = self.san_require_directory_guid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireDirectoryGuid", san_require_directory_guid)?;
            }
            if let Some(ref san_require_dns) = self.san_require_dns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireDns", san_require_dns)?;
            }
            if let Some(ref san_require_domain_dns) = self.san_require_domain_dns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireDomainDns", san_require_domain_dns)?;
            }
            if let Some(ref san_require_email) = self.san_require_email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireEmail", san_require_email)?;
            }
            if let Some(ref san_require_spn) = self.san_require_spn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireSpn", san_require_spn)?;
            }
            if let Some(ref san_require_upn) = self.san_require_upn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireUpn", san_require_upn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubjectNameFlagsV3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubjectNameFlagsV3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubjectNameFlagsV3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubjectNameFlagsV3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut require_common_name: Option<::Value<bool>> = None;
                    let mut require_directory_path: Option<::Value<bool>> = None;
                    let mut require_dns_as_cn: Option<::Value<bool>> = None;
                    let mut require_email: Option<::Value<bool>> = None;
                    let mut san_require_directory_guid: Option<::Value<bool>> = None;
                    let mut san_require_dns: Option<::Value<bool>> = None;
                    let mut san_require_domain_dns: Option<::Value<bool>> = None;
                    let mut san_require_email: Option<::Value<bool>> = None;
                    let mut san_require_spn: Option<::Value<bool>> = None;
                    let mut san_require_upn: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RequireCommonName" => {
                                require_common_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireDirectoryPath" => {
                                require_directory_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireDnsAsCn" => {
                                require_dns_as_cn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireEmail" => {
                                require_email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireDirectoryGuid" => {
                                san_require_directory_guid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireDns" => {
                                san_require_dns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireDomainDns" => {
                                san_require_domain_dns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireEmail" => {
                                san_require_email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireSpn" => {
                                san_require_spn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireUpn" => {
                                san_require_upn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubjectNameFlagsV3 {
                        require_common_name: require_common_name,
                        require_directory_path: require_directory_path,
                        require_dns_as_cn: require_dns_as_cn,
                        require_email: require_email,
                        san_require_directory_guid: san_require_directory_guid,
                        san_require_dns: san_require_dns,
                        san_require_domain_dns: san_require_domain_dns,
                        san_require_email: san_require_email,
                        san_require_spn: san_require_spn,
                        san_require_upn: san_require_upn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.SubjectNameFlagsV4`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html) property type.
    #[derive(Debug, Default)]
    pub struct SubjectNameFlagsV4 {
        /// Property [`RequireCommonName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html#cfn-pcaconnectorad-template-subjectnameflagsv4-requirecommonname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_common_name: Option<::Value<bool>>,
        /// Property [`RequireDirectoryPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html#cfn-pcaconnectorad-template-subjectnameflagsv4-requiredirectorypath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_directory_path: Option<::Value<bool>>,
        /// Property [`RequireDnsAsCn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html#cfn-pcaconnectorad-template-subjectnameflagsv4-requirednsascn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_dns_as_cn: Option<::Value<bool>>,
        /// Property [`RequireEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html#cfn-pcaconnectorad-template-subjectnameflagsv4-requireemail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_email: Option<::Value<bool>>,
        /// Property [`SanRequireDirectoryGuid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html#cfn-pcaconnectorad-template-subjectnameflagsv4-sanrequiredirectoryguid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_directory_guid: Option<::Value<bool>>,
        /// Property [`SanRequireDns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html#cfn-pcaconnectorad-template-subjectnameflagsv4-sanrequiredns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_dns: Option<::Value<bool>>,
        /// Property [`SanRequireDomainDns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html#cfn-pcaconnectorad-template-subjectnameflagsv4-sanrequiredomaindns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_domain_dns: Option<::Value<bool>>,
        /// Property [`SanRequireEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html#cfn-pcaconnectorad-template-subjectnameflagsv4-sanrequireemail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_email: Option<::Value<bool>>,
        /// Property [`SanRequireSpn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html#cfn-pcaconnectorad-template-subjectnameflagsv4-sanrequirespn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_spn: Option<::Value<bool>>,
        /// Property [`SanRequireUpn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html#cfn-pcaconnectorad-template-subjectnameflagsv4-sanrequireupn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub san_require_upn: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SubjectNameFlagsV4 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref require_common_name) = self.require_common_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireCommonName", require_common_name)?;
            }
            if let Some(ref require_directory_path) = self.require_directory_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireDirectoryPath", require_directory_path)?;
            }
            if let Some(ref require_dns_as_cn) = self.require_dns_as_cn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireDnsAsCn", require_dns_as_cn)?;
            }
            if let Some(ref require_email) = self.require_email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireEmail", require_email)?;
            }
            if let Some(ref san_require_directory_guid) = self.san_require_directory_guid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireDirectoryGuid", san_require_directory_guid)?;
            }
            if let Some(ref san_require_dns) = self.san_require_dns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireDns", san_require_dns)?;
            }
            if let Some(ref san_require_domain_dns) = self.san_require_domain_dns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireDomainDns", san_require_domain_dns)?;
            }
            if let Some(ref san_require_email) = self.san_require_email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireEmail", san_require_email)?;
            }
            if let Some(ref san_require_spn) = self.san_require_spn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireSpn", san_require_spn)?;
            }
            if let Some(ref san_require_upn) = self.san_require_upn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SanRequireUpn", san_require_upn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SubjectNameFlagsV4 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SubjectNameFlagsV4, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SubjectNameFlagsV4;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SubjectNameFlagsV4")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut require_common_name: Option<::Value<bool>> = None;
                    let mut require_directory_path: Option<::Value<bool>> = None;
                    let mut require_dns_as_cn: Option<::Value<bool>> = None;
                    let mut require_email: Option<::Value<bool>> = None;
                    let mut san_require_directory_guid: Option<::Value<bool>> = None;
                    let mut san_require_dns: Option<::Value<bool>> = None;
                    let mut san_require_domain_dns: Option<::Value<bool>> = None;
                    let mut san_require_email: Option<::Value<bool>> = None;
                    let mut san_require_spn: Option<::Value<bool>> = None;
                    let mut san_require_upn: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RequireCommonName" => {
                                require_common_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireDirectoryPath" => {
                                require_directory_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireDnsAsCn" => {
                                require_dns_as_cn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireEmail" => {
                                require_email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireDirectoryGuid" => {
                                san_require_directory_guid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireDns" => {
                                san_require_dns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireDomainDns" => {
                                san_require_domain_dns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireEmail" => {
                                san_require_email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireSpn" => {
                                san_require_spn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SanRequireUpn" => {
                                san_require_upn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SubjectNameFlagsV4 {
                        require_common_name: require_common_name,
                        require_directory_path: require_directory_path,
                        require_dns_as_cn: require_dns_as_cn,
                        require_email: require_email,
                        san_require_directory_guid: san_require_directory_guid,
                        san_require_dns: san_require_dns,
                        san_require_domain_dns: san_require_domain_dns,
                        san_require_email: san_require_email,
                        san_require_spn: san_require_spn,
                        san_require_upn: san_require_upn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.TemplateDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatedefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct TemplateDefinition {
        /// Property [`TemplateV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatedefinition.html#cfn-pcaconnectorad-template-templatedefinition-templatev2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub template_v2: Option<::Value<TemplateV2>>,
        /// Property [`TemplateV3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatedefinition.html#cfn-pcaconnectorad-template-templatedefinition-templatev3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub template_v3: Option<::Value<TemplateV3>>,
        /// Property [`TemplateV4`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatedefinition.html#cfn-pcaconnectorad-template-templatedefinition-templatev4).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub template_v4: Option<::Value<TemplateV4>>,
    }

    impl ::codec::SerializeValue for TemplateDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref template_v2) = self.template_v2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateV2", template_v2)?;
            }
            if let Some(ref template_v3) = self.template_v3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateV3", template_v3)?;
            }
            if let Some(ref template_v4) = self.template_v4 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateV4", template_v4)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TemplateDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TemplateDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TemplateDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut template_v2: Option<::Value<TemplateV2>> = None;
                    let mut template_v3: Option<::Value<TemplateV3>> = None;
                    let mut template_v4: Option<::Value<TemplateV4>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TemplateV2" => {
                                template_v2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemplateV3" => {
                                template_v3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemplateV4" => {
                                template_v4 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TemplateDefinition {
                        template_v2: template_v2,
                        template_v3: template_v3,
                        template_v4: template_v4,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.TemplateV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev2.html) property type.
    #[derive(Debug, Default)]
    pub struct TemplateV2 {
        /// Property [`CertificateValidity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev2.html#cfn-pcaconnectorad-template-templatev2-certificatevalidity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_validity: ::Value<CertificateValidity>,
        /// Property [`EnrollmentFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev2.html#cfn-pcaconnectorad-template-templatev2-enrollmentflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enrollment_flags: ::Value<EnrollmentFlagsV2>,
        /// Property [`Extensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev2.html#cfn-pcaconnectorad-template-templatev2-extensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub extensions: ::Value<ExtensionsV2>,
        /// Property [`GeneralFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev2.html#cfn-pcaconnectorad-template-templatev2-generalflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub general_flags: ::Value<GeneralFlagsV2>,
        /// Property [`PrivateKeyAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev2.html#cfn-pcaconnectorad-template-templatev2-privatekeyattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_key_attributes: ::Value<PrivateKeyAttributesV2>,
        /// Property [`PrivateKeyFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev2.html#cfn-pcaconnectorad-template-templatev2-privatekeyflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_key_flags: ::Value<PrivateKeyFlagsV2>,
        /// Property [`SubjectNameFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev2.html#cfn-pcaconnectorad-template-templatev2-subjectnameflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject_name_flags: ::Value<SubjectNameFlagsV2>,
        /// Property [`SupersededTemplates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev2.html#cfn-pcaconnectorad-template-templatev2-supersededtemplates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub superseded_templates: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for TemplateV2 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateValidity", &self.certificate_validity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnrollmentFlags", &self.enrollment_flags)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Extensions", &self.extensions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeneralFlags", &self.general_flags)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKeyAttributes", &self.private_key_attributes)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKeyFlags", &self.private_key_flags)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectNameFlags", &self.subject_name_flags)?;
            if let Some(ref superseded_templates) = self.superseded_templates {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupersededTemplates", superseded_templates)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TemplateV2 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateV2, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TemplateV2;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TemplateV2")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_validity: Option<::Value<CertificateValidity>> = None;
                    let mut enrollment_flags: Option<::Value<EnrollmentFlagsV2>> = None;
                    let mut extensions: Option<::Value<ExtensionsV2>> = None;
                    let mut general_flags: Option<::Value<GeneralFlagsV2>> = None;
                    let mut private_key_attributes: Option<::Value<PrivateKeyAttributesV2>> = None;
                    let mut private_key_flags: Option<::Value<PrivateKeyFlagsV2>> = None;
                    let mut subject_name_flags: Option<::Value<SubjectNameFlagsV2>> = None;
                    let mut superseded_templates: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateValidity" => {
                                certificate_validity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnrollmentFlags" => {
                                enrollment_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Extensions" => {
                                extensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GeneralFlags" => {
                                general_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateKeyAttributes" => {
                                private_key_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateKeyFlags" => {
                                private_key_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubjectNameFlags" => {
                                subject_name_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupersededTemplates" => {
                                superseded_templates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TemplateV2 {
                        certificate_validity: certificate_validity.ok_or(::serde::de::Error::missing_field("CertificateValidity"))?,
                        enrollment_flags: enrollment_flags.ok_or(::serde::de::Error::missing_field("EnrollmentFlags"))?,
                        extensions: extensions.ok_or(::serde::de::Error::missing_field("Extensions"))?,
                        general_flags: general_flags.ok_or(::serde::de::Error::missing_field("GeneralFlags"))?,
                        private_key_attributes: private_key_attributes.ok_or(::serde::de::Error::missing_field("PrivateKeyAttributes"))?,
                        private_key_flags: private_key_flags.ok_or(::serde::de::Error::missing_field("PrivateKeyFlags"))?,
                        subject_name_flags: subject_name_flags.ok_or(::serde::de::Error::missing_field("SubjectNameFlags"))?,
                        superseded_templates: superseded_templates,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.TemplateV3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html) property type.
    #[derive(Debug, Default)]
    pub struct TemplateV3 {
        /// Property [`CertificateValidity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html#cfn-pcaconnectorad-template-templatev3-certificatevalidity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_validity: ::Value<CertificateValidity>,
        /// Property [`EnrollmentFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html#cfn-pcaconnectorad-template-templatev3-enrollmentflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enrollment_flags: ::Value<EnrollmentFlagsV3>,
        /// Property [`Extensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html#cfn-pcaconnectorad-template-templatev3-extensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub extensions: ::Value<ExtensionsV3>,
        /// Property [`GeneralFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html#cfn-pcaconnectorad-template-templatev3-generalflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub general_flags: ::Value<GeneralFlagsV3>,
        /// Property [`HashAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html#cfn-pcaconnectorad-template-templatev3-hashalgorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_algorithm: ::Value<String>,
        /// Property [`PrivateKeyAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html#cfn-pcaconnectorad-template-templatev3-privatekeyattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_key_attributes: ::Value<PrivateKeyAttributesV3>,
        /// Property [`PrivateKeyFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html#cfn-pcaconnectorad-template-templatev3-privatekeyflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_key_flags: ::Value<PrivateKeyFlagsV3>,
        /// Property [`SubjectNameFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html#cfn-pcaconnectorad-template-templatev3-subjectnameflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject_name_flags: ::Value<SubjectNameFlagsV3>,
        /// Property [`SupersededTemplates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html#cfn-pcaconnectorad-template-templatev3-supersededtemplates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub superseded_templates: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for TemplateV3 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateValidity", &self.certificate_validity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnrollmentFlags", &self.enrollment_flags)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Extensions", &self.extensions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeneralFlags", &self.general_flags)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashAlgorithm", &self.hash_algorithm)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKeyAttributes", &self.private_key_attributes)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKeyFlags", &self.private_key_flags)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectNameFlags", &self.subject_name_flags)?;
            if let Some(ref superseded_templates) = self.superseded_templates {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupersededTemplates", superseded_templates)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TemplateV3 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateV3, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TemplateV3;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TemplateV3")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_validity: Option<::Value<CertificateValidity>> = None;
                    let mut enrollment_flags: Option<::Value<EnrollmentFlagsV3>> = None;
                    let mut extensions: Option<::Value<ExtensionsV3>> = None;
                    let mut general_flags: Option<::Value<GeneralFlagsV3>> = None;
                    let mut hash_algorithm: Option<::Value<String>> = None;
                    let mut private_key_attributes: Option<::Value<PrivateKeyAttributesV3>> = None;
                    let mut private_key_flags: Option<::Value<PrivateKeyFlagsV3>> = None;
                    let mut subject_name_flags: Option<::Value<SubjectNameFlagsV3>> = None;
                    let mut superseded_templates: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateValidity" => {
                                certificate_validity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnrollmentFlags" => {
                                enrollment_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Extensions" => {
                                extensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GeneralFlags" => {
                                general_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HashAlgorithm" => {
                                hash_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateKeyAttributes" => {
                                private_key_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateKeyFlags" => {
                                private_key_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubjectNameFlags" => {
                                subject_name_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupersededTemplates" => {
                                superseded_templates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TemplateV3 {
                        certificate_validity: certificate_validity.ok_or(::serde::de::Error::missing_field("CertificateValidity"))?,
                        enrollment_flags: enrollment_flags.ok_or(::serde::de::Error::missing_field("EnrollmentFlags"))?,
                        extensions: extensions.ok_or(::serde::de::Error::missing_field("Extensions"))?,
                        general_flags: general_flags.ok_or(::serde::de::Error::missing_field("GeneralFlags"))?,
                        hash_algorithm: hash_algorithm.ok_or(::serde::de::Error::missing_field("HashAlgorithm"))?,
                        private_key_attributes: private_key_attributes.ok_or(::serde::de::Error::missing_field("PrivateKeyAttributes"))?,
                        private_key_flags: private_key_flags.ok_or(::serde::de::Error::missing_field("PrivateKeyFlags"))?,
                        subject_name_flags: subject_name_flags.ok_or(::serde::de::Error::missing_field("SubjectNameFlags"))?,
                        superseded_templates: superseded_templates,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.TemplateV4`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html) property type.
    #[derive(Debug, Default)]
    pub struct TemplateV4 {
        /// Property [`CertificateValidity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html#cfn-pcaconnectorad-template-templatev4-certificatevalidity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_validity: ::Value<CertificateValidity>,
        /// Property [`EnrollmentFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html#cfn-pcaconnectorad-template-templatev4-enrollmentflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enrollment_flags: ::Value<EnrollmentFlagsV4>,
        /// Property [`Extensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html#cfn-pcaconnectorad-template-templatev4-extensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub extensions: ::Value<ExtensionsV4>,
        /// Property [`GeneralFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html#cfn-pcaconnectorad-template-templatev4-generalflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub general_flags: ::Value<GeneralFlagsV4>,
        /// Property [`HashAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html#cfn-pcaconnectorad-template-templatev4-hashalgorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_algorithm: Option<::Value<String>>,
        /// Property [`PrivateKeyAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html#cfn-pcaconnectorad-template-templatev4-privatekeyattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_key_attributes: ::Value<PrivateKeyAttributesV4>,
        /// Property [`PrivateKeyFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html#cfn-pcaconnectorad-template-templatev4-privatekeyflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_key_flags: ::Value<PrivateKeyFlagsV4>,
        /// Property [`SubjectNameFlags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html#cfn-pcaconnectorad-template-templatev4-subjectnameflags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject_name_flags: ::Value<SubjectNameFlagsV4>,
        /// Property [`SupersededTemplates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html#cfn-pcaconnectorad-template-templatev4-supersededtemplates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub superseded_templates: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for TemplateV4 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateValidity", &self.certificate_validity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnrollmentFlags", &self.enrollment_flags)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Extensions", &self.extensions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeneralFlags", &self.general_flags)?;
            if let Some(ref hash_algorithm) = self.hash_algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashAlgorithm", hash_algorithm)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKeyAttributes", &self.private_key_attributes)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKeyFlags", &self.private_key_flags)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectNameFlags", &self.subject_name_flags)?;
            if let Some(ref superseded_templates) = self.superseded_templates {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupersededTemplates", superseded_templates)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TemplateV4 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateV4, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TemplateV4;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TemplateV4")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_validity: Option<::Value<CertificateValidity>> = None;
                    let mut enrollment_flags: Option<::Value<EnrollmentFlagsV4>> = None;
                    let mut extensions: Option<::Value<ExtensionsV4>> = None;
                    let mut general_flags: Option<::Value<GeneralFlagsV4>> = None;
                    let mut hash_algorithm: Option<::Value<String>> = None;
                    let mut private_key_attributes: Option<::Value<PrivateKeyAttributesV4>> = None;
                    let mut private_key_flags: Option<::Value<PrivateKeyFlagsV4>> = None;
                    let mut subject_name_flags: Option<::Value<SubjectNameFlagsV4>> = None;
                    let mut superseded_templates: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateValidity" => {
                                certificate_validity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnrollmentFlags" => {
                                enrollment_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Extensions" => {
                                extensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GeneralFlags" => {
                                general_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HashAlgorithm" => {
                                hash_algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateKeyAttributes" => {
                                private_key_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateKeyFlags" => {
                                private_key_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubjectNameFlags" => {
                                subject_name_flags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupersededTemplates" => {
                                superseded_templates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TemplateV4 {
                        certificate_validity: certificate_validity.ok_or(::serde::de::Error::missing_field("CertificateValidity"))?,
                        enrollment_flags: enrollment_flags.ok_or(::serde::de::Error::missing_field("EnrollmentFlags"))?,
                        extensions: extensions.ok_or(::serde::de::Error::missing_field("Extensions"))?,
                        general_flags: general_flags.ok_or(::serde::de::Error::missing_field("GeneralFlags"))?,
                        hash_algorithm: hash_algorithm,
                        private_key_attributes: private_key_attributes.ok_or(::serde::de::Error::missing_field("PrivateKeyAttributes"))?,
                        private_key_flags: private_key_flags.ok_or(::serde::de::Error::missing_field("PrivateKeyFlags"))?,
                        subject_name_flags: subject_name_flags.ok_or(::serde::de::Error::missing_field("SubjectNameFlags"))?,
                        superseded_templates: superseded_templates,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PCAConnectorAD::Template.ValidityPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-validityperiod.html) property type.
    #[derive(Debug, Default)]
    pub struct ValidityPeriod {
        /// Property [`Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-validityperiod.html#cfn-pcaconnectorad-template-validityperiod-period).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub period: ::Value<f64>,
        /// Property [`PeriodType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-validityperiod.html#cfn-pcaconnectorad-template-validityperiod-periodtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub period_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for ValidityPeriod {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", &self.period)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeriodType", &self.period_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ValidityPeriod {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ValidityPeriod, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ValidityPeriod;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ValidityPeriod")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut period: Option<::Value<f64>> = None;
                    let mut period_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Period" => {
                                period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PeriodType" => {
                                period_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ValidityPeriod {
                        period: period.ok_or(::serde::de::Error::missing_field("Period"))?,
                        period_type: period_type.ok_or(::serde::de::Error::missing_field("PeriodType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod template_group_access_control_entry {
    //! Property types for the `TemplateGroupAccessControlEntry` resource.

    /// The [`AWS::PCAConnectorAD::TemplateGroupAccessControlEntry.AccessRights`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-templategroupaccesscontrolentry-accessrights.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessRights {
        /// Property [`AutoEnroll`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-templategroupaccesscontrolentry-accessrights.html#cfn-pcaconnectorad-templategroupaccesscontrolentry-accessrights-autoenroll).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_enroll: Option<::Value<String>>,
        /// Property [`Enroll`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-templategroupaccesscontrolentry-accessrights.html#cfn-pcaconnectorad-templategroupaccesscontrolentry-accessrights-enroll).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enroll: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AccessRights {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_enroll) = self.auto_enroll {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoEnroll", auto_enroll)?;
            }
            if let Some(ref enroll) = self.enroll {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enroll", enroll)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessRights {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessRights, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessRights;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessRights")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_enroll: Option<::Value<String>> = None;
                    let mut enroll: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoEnroll" => {
                                auto_enroll = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enroll" => {
                                enroll = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessRights {
                        auto_enroll: auto_enroll,
                        enroll: enroll,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
