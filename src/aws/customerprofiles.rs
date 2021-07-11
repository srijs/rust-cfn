//! Types for the `CustomerProfiles` service.

/// The [`AWS::CustomerProfiles::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html) resource type.
#[derive(Debug, Default)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Debug, Default)]
pub struct DomainProperties {
    /// Property [`DeadLetterQueueUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-deadletterqueueurl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dead_letter_queue_url: Option<::Value<String>>,
    /// Property [`DefaultEncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-defaultencryptionkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_encryption_key: Option<::Value<String>>,
    /// Property [`DefaultExpirationDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-defaultexpirationdays).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_expiration_days: Option<::Value<u32>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-domain.html#cfn-customerprofiles-domain-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DomainProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref dead_letter_queue_url) = self.dead_letter_queue_url {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeadLetterQueueUrl", dead_letter_queue_url)?;
        }
        if let Some(ref default_encryption_key) = self.default_encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultEncryptionKey", default_encryption_key)?;
        }
        if let Some(ref default_expiration_days) = self.default_expiration_days {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultExpirationDays", default_expiration_days)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DomainProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DomainProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DomainProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dead_letter_queue_url: Option<::Value<String>> = None;
                let mut default_encryption_key: Option<::Value<String>> = None;
                let mut default_expiration_days: Option<::Value<u32>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeadLetterQueueUrl" => {
                            dead_letter_queue_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultEncryptionKey" => {
                            default_encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultExpirationDays" => {
                            default_expiration_days = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DomainProperties {
                    dead_letter_queue_url: dead_letter_queue_url,
                    default_encryption_key: default_encryption_key,
                    default_expiration_days: default_expiration_days,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Domain {
    type Properties = DomainProperties;
    const TYPE: &'static str = "AWS::CustomerProfiles::Domain";
    fn properties(&self) -> &DomainProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Domain {}

impl From<DomainProperties> for Domain {
    fn from(properties: DomainProperties) -> Domain {
        Domain { properties }
    }
}

/// The [`AWS::CustomerProfiles::Integration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html) resource type.
#[derive(Debug, Default)]
pub struct Integration {
    properties: IntegrationProperties
}

/// Properties for the `Integration` resource.
#[derive(Debug, Default)]
pub struct IntegrationProperties {
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`FlowDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-flowdefinition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub flow_definition: Option<::Value<self::integration::FlowDefinition>>,
    /// Property [`ObjectTypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-objecttypename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub object_type_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-integration.html#cfn-customerprofiles-integration-uri).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub uri: Option<::Value<String>>,
}

impl ::serde::Serialize for IntegrationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref flow_definition) = self.flow_definition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowDefinition", flow_definition)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectTypeName", &self.object_type_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref uri) = self.uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uri", uri)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IntegrationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegrationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IntegrationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IntegrationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain_name: Option<::Value<String>> = None;
                let mut flow_definition: Option<::Value<self::integration::FlowDefinition>> = None;
                let mut object_type_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut uri: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlowDefinition" => {
                            flow_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ObjectTypeName" => {
                            object_type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Uri" => {
                            uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IntegrationProperties {
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    flow_definition: flow_definition,
                    object_type_name: object_type_name.ok_or(::serde::de::Error::missing_field("ObjectTypeName"))?,
                    tags: tags,
                    uri: uri,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Integration {
    type Properties = IntegrationProperties;
    const TYPE: &'static str = "AWS::CustomerProfiles::Integration";
    fn properties(&self) -> &IntegrationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IntegrationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Integration {}

impl From<IntegrationProperties> for Integration {
    fn from(properties: IntegrationProperties) -> Integration {
        Integration { properties }
    }
}

/// The [`AWS::CustomerProfiles::ObjectType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html) resource type.
#[derive(Debug, Default)]
pub struct ObjectType {
    properties: ObjectTypeProperties
}

/// Properties for the `ObjectType` resource.
#[derive(Debug, Default)]
pub struct ObjectTypeProperties {
    /// Property [`AllowProfileCreation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-allowprofilecreation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_profile_creation: Option<::Value<bool>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`EncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-encryptionkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_key: Option<::Value<String>>,
    /// Property [`ExpirationDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-expirationdays).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub expiration_days: Option<::Value<u32>>,
    /// Property [`Fields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-fields).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fields: Option<::ValueList<self::object_type::FieldMap>>,
    /// Property [`Keys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-keys).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub keys: Option<::ValueList<self::object_type::KeyMap>>,
    /// Property [`ObjectTypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-objecttypename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub object_type_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-customerprofiles-objecttype.html#cfn-customerprofiles-objecttype-templateid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ObjectTypeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allow_profile_creation) = self.allow_profile_creation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowProfileCreation", allow_profile_creation)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref encryption_key) = self.encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKey", encryption_key)?;
        }
        if let Some(ref expiration_days) = self.expiration_days {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpirationDays", expiration_days)?;
        }
        if let Some(ref fields) = self.fields {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", fields)?;
        }
        if let Some(ref keys) = self.keys {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Keys", keys)?;
        }
        if let Some(ref object_type_name) = self.object_type_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectTypeName", object_type_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref template_id) = self.template_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateId", template_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ObjectTypeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ObjectTypeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ObjectTypeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ObjectTypeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allow_profile_creation: Option<::Value<bool>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut encryption_key: Option<::Value<String>> = None;
                let mut expiration_days: Option<::Value<u32>> = None;
                let mut fields: Option<::ValueList<self::object_type::FieldMap>> = None;
                let mut keys: Option<::ValueList<self::object_type::KeyMap>> = None;
                let mut object_type_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut template_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowProfileCreation" => {
                            allow_profile_creation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionKey" => {
                            encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExpirationDays" => {
                            expiration_days = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Fields" => {
                            fields = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Keys" => {
                            keys = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ObjectTypeName" => {
                            object_type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateId" => {
                            template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ObjectTypeProperties {
                    allow_profile_creation: allow_profile_creation,
                    description: description,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    encryption_key: encryption_key,
                    expiration_days: expiration_days,
                    fields: fields,
                    keys: keys,
                    object_type_name: object_type_name,
                    tags: tags,
                    template_id: template_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ObjectType {
    type Properties = ObjectTypeProperties;
    const TYPE: &'static str = "AWS::CustomerProfiles::ObjectType";
    fn properties(&self) -> &ObjectTypeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ObjectTypeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ObjectType {}

impl From<ObjectTypeProperties> for ObjectType {
    fn from(properties: ObjectTypeProperties) -> ObjectType {
        ObjectType { properties }
    }
}

pub mod integration {
    //! Property types for the `Integration` resource.

    /// The [`AWS::CustomerProfiles::Integration.ConnectorOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectorOperator {
        /// Property [`Marketo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html#cfn-customerprofiles-integration-connectoroperator-marketo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub marketo: Option<::Value<String>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html#cfn-customerprofiles-integration-connectoroperator-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<String>>,
        /// Property [`Salesforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html#cfn-customerprofiles-integration-connectoroperator-salesforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salesforce: Option<::Value<String>>,
        /// Property [`ServiceNow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html#cfn-customerprofiles-integration-connectoroperator-servicenow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_now: Option<::Value<String>>,
        /// Property [`Zendesk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-connectoroperator.html#cfn-customerprofiles-integration-connectoroperator-zendesk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zendesk: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConnectorOperator {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref marketo) = self.marketo {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Marketo", marketo)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            if let Some(ref salesforce) = self.salesforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Salesforce", salesforce)?;
            }
            if let Some(ref service_now) = self.service_now {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNow", service_now)?;
            }
            if let Some(ref zendesk) = self.zendesk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Zendesk", zendesk)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectorOperator {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorOperator, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectorOperator;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectorOperator")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut marketo: Option<::Value<String>> = None;
                    let mut s3: Option<::Value<String>> = None;
                    let mut salesforce: Option<::Value<String>> = None;
                    let mut service_now: Option<::Value<String>> = None;
                    let mut zendesk: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Marketo" => {
                                marketo = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Salesforce" => {
                                salesforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNow" => {
                                service_now = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Zendesk" => {
                                zendesk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectorOperator {
                        marketo: marketo,
                        s3: s3,
                        salesforce: salesforce,
                        service_now: service_now,
                        zendesk: zendesk,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.FlowDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct FlowDefinition {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`FlowName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-flowname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flow_name: ::Value<String>,
        /// Property [`KmsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-kmsarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_arn: ::Value<String>,
        /// Property [`SourceFlowConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-sourceflowconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_flow_config: ::Value<SourceFlowConfig>,
        /// Property [`Tasks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-tasks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tasks: ::ValueList<Task>,
        /// Property [`TriggerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-flowdefinition.html#cfn-customerprofiles-integration-flowdefinition-triggerconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trigger_config: ::Value<TriggerConfig>,
    }

    impl ::codec::SerializeValue for FlowDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowName", &self.flow_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsArn", &self.kms_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFlowConfig", &self.source_flow_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tasks", &self.tasks)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerConfig", &self.trigger_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FlowDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FlowDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FlowDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FlowDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut flow_name: Option<::Value<String>> = None;
                    let mut kms_arn: Option<::Value<String>> = None;
                    let mut source_flow_config: Option<::Value<SourceFlowConfig>> = None;
                    let mut tasks: Option<::ValueList<Task>> = None;
                    let mut trigger_config: Option<::Value<TriggerConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FlowName" => {
                                flow_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsArn" => {
                                kms_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceFlowConfig" => {
                                source_flow_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tasks" => {
                                tasks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TriggerConfig" => {
                                trigger_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FlowDefinition {
                        description: description,
                        flow_name: flow_name.ok_or(::serde::de::Error::missing_field("FlowName"))?,
                        kms_arn: kms_arn.ok_or(::serde::de::Error::missing_field("KmsArn"))?,
                        source_flow_config: source_flow_config.ok_or(::serde::de::Error::missing_field("SourceFlowConfig"))?,
                        tasks: tasks.ok_or(::serde::de::Error::missing_field("Tasks"))?,
                        trigger_config: trigger_config.ok_or(::serde::de::Error::missing_field("TriggerConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.IncrementalPullConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-incrementalpullconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct IncrementalPullConfig {
        /// Property [`DatetimeTypeFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-incrementalpullconfig.html#cfn-customerprofiles-integration-incrementalpullconfig-datetimetypefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datetime_type_field_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IncrementalPullConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref datetime_type_field_name) = self.datetime_type_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatetimeTypeFieldName", datetime_type_field_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IncrementalPullConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IncrementalPullConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IncrementalPullConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IncrementalPullConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut datetime_type_field_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatetimeTypeFieldName" => {
                                datetime_type_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IncrementalPullConfig {
                        datetime_type_field_name: datetime_type_field_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.MarketoSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-marketosourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct MarketoSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-marketosourceproperties.html#cfn-customerprofiles-integration-marketosourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for MarketoSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MarketoSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MarketoSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MarketoSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MarketoSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MarketoSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.S3SourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-s3sourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct S3SourceProperties {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-s3sourceproperties.html#cfn-customerprofiles-integration-s3sourceproperties-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-s3sourceproperties.html#cfn-customerprofiles-integration-s3sourceproperties-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3SourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3SourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3SourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3SourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3SourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3SourceProperties {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        bucket_prefix: bucket_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.SalesforceSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-salesforcesourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceSourceProperties {
        /// Property [`EnableDynamicFieldUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-salesforcesourceproperties.html#cfn-customerprofiles-integration-salesforcesourceproperties-enabledynamicfieldupdate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_dynamic_field_update: Option<::Value<bool>>,
        /// Property [`IncludeDeletedRecords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-salesforcesourceproperties.html#cfn-customerprofiles-integration-salesforcesourceproperties-includedeletedrecords).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_deleted_records: Option<::Value<bool>>,
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-salesforcesourceproperties.html#cfn-customerprofiles-integration-salesforcesourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for SalesforceSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_dynamic_field_update) = self.enable_dynamic_field_update {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableDynamicFieldUpdate", enable_dynamic_field_update)?;
            }
            if let Some(ref include_deleted_records) = self.include_deleted_records {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeDeletedRecords", include_deleted_records)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_dynamic_field_update: Option<::Value<bool>> = None;
                    let mut include_deleted_records: Option<::Value<bool>> = None;
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableDynamicFieldUpdate" => {
                                enable_dynamic_field_update = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeDeletedRecords" => {
                                include_deleted_records = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceSourceProperties {
                        enable_dynamic_field_update: enable_dynamic_field_update,
                        include_deleted_records: include_deleted_records,
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.ScheduledTriggerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ScheduledTriggerProperties {
        /// Property [`DataPullMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-datapullmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_pull_mode: Option<::Value<String>>,
        /// Property [`FirstExecutionFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-firstexecutionfrom).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub first_execution_from: Option<::Value<f64>>,
        /// Property [`ScheduleEndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-scheduleendtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_end_time: Option<::Value<f64>>,
        /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-scheduleexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_expression: ::Value<String>,
        /// Property [`ScheduleOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-scheduleoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_offset: Option<::Value<u32>>,
        /// Property [`ScheduleStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-schedulestarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_start_time: Option<::Value<f64>>,
        /// Property [`Timezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-scheduledtriggerproperties.html#cfn-customerprofiles-integration-scheduledtriggerproperties-timezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timezone: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ScheduledTriggerProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_pull_mode) = self.data_pull_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataPullMode", data_pull_mode)?;
            }
            if let Some(ref first_execution_from) = self.first_execution_from {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirstExecutionFrom", first_execution_from)?;
            }
            if let Some(ref schedule_end_time) = self.schedule_end_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleEndTime", schedule_end_time)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", &self.schedule_expression)?;
            if let Some(ref schedule_offset) = self.schedule_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleOffset", schedule_offset)?;
            }
            if let Some(ref schedule_start_time) = self.schedule_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleStartTime", schedule_start_time)?;
            }
            if let Some(ref timezone) = self.timezone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timezone", timezone)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScheduledTriggerProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduledTriggerProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScheduledTriggerProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScheduledTriggerProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_pull_mode: Option<::Value<String>> = None;
                    let mut first_execution_from: Option<::Value<f64>> = None;
                    let mut schedule_end_time: Option<::Value<f64>> = None;
                    let mut schedule_expression: Option<::Value<String>> = None;
                    let mut schedule_offset: Option<::Value<u32>> = None;
                    let mut schedule_start_time: Option<::Value<f64>> = None;
                    let mut timezone: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataPullMode" => {
                                data_pull_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirstExecutionFrom" => {
                                first_execution_from = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleEndTime" => {
                                schedule_end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleExpression" => {
                                schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleOffset" => {
                                schedule_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleStartTime" => {
                                schedule_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timezone" => {
                                timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScheduledTriggerProperties {
                        data_pull_mode: data_pull_mode,
                        first_execution_from: first_execution_from,
                        schedule_end_time: schedule_end_time,
                        schedule_expression: schedule_expression.ok_or(::serde::de::Error::missing_field("ScheduleExpression"))?,
                        schedule_offset: schedule_offset,
                        schedule_start_time: schedule_start_time,
                        timezone: timezone,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.ServiceNowSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-servicenowsourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceNowSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-servicenowsourceproperties.html#cfn-customerprofiles-integration-servicenowsourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for ServiceNowSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceNowSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceNowSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceNowSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceNowSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceNowSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.SourceConnectorProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceConnectorProperties {
        /// Property [`Marketo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html#cfn-customerprofiles-integration-sourceconnectorproperties-marketo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub marketo: Option<::Value<MarketoSourceProperties>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html#cfn-customerprofiles-integration-sourceconnectorproperties-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<S3SourceProperties>>,
        /// Property [`Salesforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html#cfn-customerprofiles-integration-sourceconnectorproperties-salesforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salesforce: Option<::Value<SalesforceSourceProperties>>,
        /// Property [`ServiceNow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html#cfn-customerprofiles-integration-sourceconnectorproperties-servicenow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_now: Option<::Value<ServiceNowSourceProperties>>,
        /// Property [`Zendesk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceconnectorproperties.html#cfn-customerprofiles-integration-sourceconnectorproperties-zendesk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zendesk: Option<::Value<ZendeskSourceProperties>>,
    }

    impl ::codec::SerializeValue for SourceConnectorProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref marketo) = self.marketo {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Marketo", marketo)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            if let Some(ref salesforce) = self.salesforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Salesforce", salesforce)?;
            }
            if let Some(ref service_now) = self.service_now {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNow", service_now)?;
            }
            if let Some(ref zendesk) = self.zendesk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Zendesk", zendesk)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceConnectorProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceConnectorProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceConnectorProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceConnectorProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut marketo: Option<::Value<MarketoSourceProperties>> = None;
                    let mut s3: Option<::Value<S3SourceProperties>> = None;
                    let mut salesforce: Option<::Value<SalesforceSourceProperties>> = None;
                    let mut service_now: Option<::Value<ServiceNowSourceProperties>> = None;
                    let mut zendesk: Option<::Value<ZendeskSourceProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Marketo" => {
                                marketo = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Salesforce" => {
                                salesforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNow" => {
                                service_now = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Zendesk" => {
                                zendesk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceConnectorProperties {
                        marketo: marketo,
                        s3: s3,
                        salesforce: salesforce,
                        service_now: service_now,
                        zendesk: zendesk,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.SourceFlowConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceFlowConfig {
        /// Property [`ConnectorProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html#cfn-customerprofiles-integration-sourceflowconfig-connectorprofilename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_profile_name: Option<::Value<String>>,
        /// Property [`ConnectorType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html#cfn-customerprofiles-integration-sourceflowconfig-connectortype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_type: ::Value<String>,
        /// Property [`IncrementalPullConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html#cfn-customerprofiles-integration-sourceflowconfig-incrementalpullconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub incremental_pull_config: Option<::Value<IncrementalPullConfig>>,
        /// Property [`SourceConnectorProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-sourceflowconfig.html#cfn-customerprofiles-integration-sourceflowconfig-sourceconnectorproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_connector_properties: ::Value<SourceConnectorProperties>,
    }

    impl ::codec::SerializeValue for SourceFlowConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connector_profile_name) = self.connector_profile_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorProfileName", connector_profile_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorType", &self.connector_type)?;
            if let Some(ref incremental_pull_config) = self.incremental_pull_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncrementalPullConfig", incremental_pull_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceConnectorProperties", &self.source_connector_properties)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceFlowConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceFlowConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceFlowConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceFlowConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connector_profile_name: Option<::Value<String>> = None;
                    let mut connector_type: Option<::Value<String>> = None;
                    let mut incremental_pull_config: Option<::Value<IncrementalPullConfig>> = None;
                    let mut source_connector_properties: Option<::Value<SourceConnectorProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectorProfileName" => {
                                connector_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorType" => {
                                connector_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncrementalPullConfig" => {
                                incremental_pull_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceConnectorProperties" => {
                                source_connector_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceFlowConfig {
                        connector_profile_name: connector_profile_name,
                        connector_type: connector_type.ok_or(::serde::de::Error::missing_field("ConnectorType"))?,
                        incremental_pull_config: incremental_pull_config,
                        source_connector_properties: source_connector_properties.ok_or(::serde::de::Error::missing_field("SourceConnectorProperties"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.Task`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html) property type.
    #[derive(Debug, Default)]
    pub struct Task {
        /// Property [`ConnectorOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html#cfn-customerprofiles-integration-task-connectoroperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_operator: Option<::Value<ConnectorOperator>>,
        /// Property [`DestinationField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html#cfn-customerprofiles-integration-task-destinationfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_field: Option<::Value<String>>,
        /// Property [`SourceFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html#cfn-customerprofiles-integration-task-sourcefields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_fields: ::ValueList<String>,
        /// Property [`TaskProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html#cfn-customerprofiles-integration-task-taskproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_properties: Option<::ValueList<TaskPropertiesMap>>,
        /// Property [`TaskType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-task.html#cfn-customerprofiles-integration-task-tasktype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Task {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connector_operator) = self.connector_operator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorOperator", connector_operator)?;
            }
            if let Some(ref destination_field) = self.destination_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationField", destination_field)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFields", &self.source_fields)?;
            if let Some(ref task_properties) = self.task_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskProperties", task_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskType", &self.task_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Task {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Task, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Task;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Task")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connector_operator: Option<::Value<ConnectorOperator>> = None;
                    let mut destination_field: Option<::Value<String>> = None;
                    let mut source_fields: Option<::ValueList<String>> = None;
                    let mut task_properties: Option<::ValueList<TaskPropertiesMap>> = None;
                    let mut task_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectorOperator" => {
                                connector_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationField" => {
                                destination_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceFields" => {
                                source_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskProperties" => {
                                task_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskType" => {
                                task_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Task {
                        connector_operator: connector_operator,
                        destination_field: destination_field,
                        source_fields: source_fields.ok_or(::serde::de::Error::missing_field("SourceFields"))?,
                        task_properties: task_properties,
                        task_type: task_type.ok_or(::serde::de::Error::missing_field("TaskType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.TaskPropertiesMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-taskpropertiesmap.html) property type.
    #[derive(Debug, Default)]
    pub struct TaskPropertiesMap {
        /// Property [`OperatorPropertyKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-taskpropertiesmap.html#cfn-customerprofiles-integration-taskpropertiesmap-operatorpropertykey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operator_property_key: ::Value<String>,
        /// Property [`Property`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-taskpropertiesmap.html#cfn-customerprofiles-integration-taskpropertiesmap-property).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property: ::Value<String>,
    }

    impl ::codec::SerializeValue for TaskPropertiesMap {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperatorPropertyKey", &self.operator_property_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Property", &self.property)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TaskPropertiesMap {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskPropertiesMap, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TaskPropertiesMap;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TaskPropertiesMap")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut operator_property_key: Option<::Value<String>> = None;
                    let mut property: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OperatorPropertyKey" => {
                                operator_property_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Property" => {
                                property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskPropertiesMap {
                        operator_property_key: operator_property_key.ok_or(::serde::de::Error::missing_field("OperatorPropertyKey"))?,
                        property: property.ok_or(::serde::de::Error::missing_field("Property"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.TriggerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TriggerConfig {
        /// Property [`TriggerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerconfig.html#cfn-customerprofiles-integration-triggerconfig-triggerproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trigger_properties: Option<::Value<TriggerProperties>>,
        /// Property [`TriggerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerconfig.html#cfn-customerprofiles-integration-triggerconfig-triggertype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trigger_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for TriggerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref trigger_properties) = self.trigger_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerProperties", trigger_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerType", &self.trigger_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TriggerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TriggerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TriggerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TriggerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut trigger_properties: Option<::Value<TriggerProperties>> = None;
                    let mut trigger_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TriggerProperties" => {
                                trigger_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TriggerType" => {
                                trigger_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TriggerConfig {
                        trigger_properties: trigger_properties,
                        trigger_type: trigger_type.ok_or(::serde::de::Error::missing_field("TriggerType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.TriggerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct TriggerProperties {
        /// Property [`Scheduled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-triggerproperties.html#cfn-customerprofiles-integration-triggerproperties-scheduled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scheduled: Option<::Value<ScheduledTriggerProperties>>,
    }

    impl ::codec::SerializeValue for TriggerProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref scheduled) = self.scheduled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scheduled", scheduled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TriggerProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TriggerProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TriggerProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TriggerProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut scheduled: Option<::Value<ScheduledTriggerProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Scheduled" => {
                                scheduled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TriggerProperties {
                        scheduled: scheduled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::Integration.ZendeskSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-zendesksourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ZendeskSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-integration-zendesksourceproperties.html#cfn-customerprofiles-integration-zendesksourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for ZendeskSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ZendeskSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ZendeskSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ZendeskSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ZendeskSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ZendeskSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod object_type {
    //! Property types for the `ObjectType` resource.

    /// The [`AWS::CustomerProfiles::ObjectType.FieldMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-fieldmap.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldMap {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-fieldmap.html#cfn-customerprofiles-objecttype-fieldmap-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`ObjectTypeField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-fieldmap.html#cfn-customerprofiles-objecttype-fieldmap-objecttypefield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_type_field: Option<::Value<ObjectTypeField>>,
    }

    impl ::codec::SerializeValue for FieldMap {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref object_type_field) = self.object_type_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectTypeField", object_type_field)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldMap {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldMap, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldMap;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldMap")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut object_type_field: Option<::Value<ObjectTypeField>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectTypeField" => {
                                object_type_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldMap {
                        name: name,
                        object_type_field: object_type_field,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::ObjectType.KeyMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-keymap.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyMap {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-keymap.html#cfn-customerprofiles-objecttype-keymap-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`ObjectTypeKeyList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-keymap.html#cfn-customerprofiles-objecttype-keymap-objecttypekeylist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_type_key_list: Option<::ValueList<ObjectTypeKey>>,
    }

    impl ::codec::SerializeValue for KeyMap {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref object_type_key_list) = self.object_type_key_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectTypeKeyList", object_type_key_list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyMap {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyMap, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyMap;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyMap")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut object_type_key_list: Option<::ValueList<ObjectTypeKey>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectTypeKeyList" => {
                                object_type_key_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyMap {
                        name: name,
                        object_type_key_list: object_type_key_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::ObjectType.ObjectTypeField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypefield.html) property type.
    #[derive(Debug, Default)]
    pub struct ObjectTypeField {
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypefield.html#cfn-customerprofiles-objecttype-objecttypefield-contenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_type: Option<::Value<String>>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypefield.html#cfn-customerprofiles-objecttype-objecttypefield-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: Option<::Value<String>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypefield.html#cfn-customerprofiles-objecttype-objecttypefield-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ObjectTypeField {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content_type) = self.content_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", content_type)?;
            }
            if let Some(ref source) = self.source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", source)?;
            }
            if let Some(ref target) = self.target {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", target)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ObjectTypeField {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ObjectTypeField, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ObjectTypeField;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ObjectTypeField")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_type: Option<::Value<String>> = None;
                    let mut source: Option<::Value<String>> = None;
                    let mut target: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ObjectTypeField {
                        content_type: content_type,
                        source: source,
                        target: target,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CustomerProfiles::ObjectType.ObjectTypeKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypekey.html) property type.
    #[derive(Debug, Default)]
    pub struct ObjectTypeKey {
        /// Property [`FieldNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypekey.html#cfn-customerprofiles-objecttype-objecttypekey-fieldnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_names: Option<::ValueList<String>>,
        /// Property [`StandardIdentifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-customerprofiles-objecttype-objecttypekey.html#cfn-customerprofiles-objecttype-objecttypekey-standardidentifiers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub standard_identifiers: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ObjectTypeKey {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref field_names) = self.field_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldNames", field_names)?;
            }
            if let Some(ref standard_identifiers) = self.standard_identifiers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StandardIdentifiers", standard_identifiers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ObjectTypeKey {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ObjectTypeKey, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ObjectTypeKey;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ObjectTypeKey")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_names: Option<::ValueList<String>> = None;
                    let mut standard_identifiers: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldNames" => {
                                field_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StandardIdentifiers" => {
                                standard_identifiers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ObjectTypeKey {
                        field_names: field_names,
                        standard_identifiers: standard_identifiers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
