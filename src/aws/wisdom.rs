//! Types for the `Wisdom` service.

/// The [`AWS::Wisdom::Assistant`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistant.html) resource type.
#[derive(Debug, Default)]
pub struct Assistant {
    properties: AssistantProperties
}

/// Properties for the `Assistant` resource.
#[derive(Debug, Default)]
pub struct AssistantProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistant.html#cfn-wisdom-assistant-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistant.html#cfn-wisdom-assistant-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ServerSideEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistant.html#cfn-wisdom-assistant-serversideencryptionconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_side_encryption_configuration: Option<::Value<self::assistant::ServerSideEncryptionConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistant.html#cfn-wisdom-assistant-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistant.html#cfn-wisdom-assistant-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for AssistantProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref server_side_encryption_configuration) = self.server_side_encryption_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerSideEncryptionConfiguration", server_side_encryption_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssistantProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssistantProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssistantProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssistantProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut server_side_encryption_configuration: Option<::Value<self::assistant::ServerSideEncryptionConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerSideEncryptionConfiguration" => {
                            server_side_encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(AssistantProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    server_side_encryption_configuration: server_side_encryption_configuration,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Assistant {
    type Properties = AssistantProperties;
    const TYPE: &'static str = "AWS::Wisdom::Assistant";
    fn properties(&self) -> &AssistantProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssistantProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Assistant {}

impl From<AssistantProperties> for Assistant {
    fn from(properties: AssistantProperties) -> Assistant {
        Assistant { properties }
    }
}

/// The [`AWS::Wisdom::AssistantAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistantassociation.html) resource type.
#[derive(Debug, Default)]
pub struct AssistantAssociation {
    properties: AssistantAssociationProperties
}

/// Properties for the `AssistantAssociation` resource.
#[derive(Debug, Default)]
pub struct AssistantAssociationProperties {
    /// Property [`AssistantId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistantassociation.html#cfn-wisdom-assistantassociation-assistantid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub assistant_id: ::Value<String>,
    /// Property [`Association`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistantassociation.html#cfn-wisdom-assistantassociation-association).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub association: ::Value<self::assistant_association::AssociationData>,
    /// Property [`AssociationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistantassociation.html#cfn-wisdom-assistantassociation-associationtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub association_type: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistantassociation.html#cfn-wisdom-assistantassociation-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AssistantAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssistantId", &self.assistant_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Association", &self.association)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociationType", &self.association_type)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssistantAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssistantAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssistantAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssistantAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut assistant_id: Option<::Value<String>> = None;
                let mut association: Option<::Value<self::assistant_association::AssociationData>> = None;
                let mut association_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssistantId" => {
                            assistant_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Association" => {
                            association = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssociationType" => {
                            association_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AssistantAssociationProperties {
                    assistant_id: assistant_id.ok_or(::serde::de::Error::missing_field("AssistantId"))?,
                    association: association.ok_or(::serde::de::Error::missing_field("Association"))?,
                    association_type: association_type.ok_or(::serde::de::Error::missing_field("AssociationType"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AssistantAssociation {
    type Properties = AssistantAssociationProperties;
    const TYPE: &'static str = "AWS::Wisdom::AssistantAssociation";
    fn properties(&self) -> &AssistantAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssistantAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AssistantAssociation {}

impl From<AssistantAssociationProperties> for AssistantAssociation {
    fn from(properties: AssistantAssociationProperties) -> AssistantAssociation {
        AssistantAssociation { properties }
    }
}

/// The [`AWS::Wisdom::KnowledgeBase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-knowledgebase.html) resource type.
#[derive(Debug, Default)]
pub struct KnowledgeBase {
    properties: KnowledgeBaseProperties
}

/// Properties for the `KnowledgeBase` resource.
#[derive(Debug, Default)]
pub struct KnowledgeBaseProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-knowledgebase.html#cfn-wisdom-knowledgebase-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`KnowledgeBaseType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-knowledgebase.html#cfn-wisdom-knowledgebase-knowledgebasetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub knowledge_base_type: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-knowledgebase.html#cfn-wisdom-knowledgebase-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RenderingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-knowledgebase.html#cfn-wisdom-knowledgebase-renderingconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rendering_configuration: Option<::Value<self::knowledge_base::RenderingConfiguration>>,
    /// Property [`ServerSideEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-knowledgebase.html#cfn-wisdom-knowledgebase-serversideencryptionconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_side_encryption_configuration: Option<::Value<self::knowledge_base::ServerSideEncryptionConfiguration>>,
    /// Property [`SourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-knowledgebase.html#cfn-wisdom-knowledgebase-sourceconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_configuration: Option<::Value<self::knowledge_base::SourceConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-knowledgebase.html#cfn-wisdom-knowledgebase-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for KnowledgeBaseProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KnowledgeBaseType", &self.knowledge_base_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref rendering_configuration) = self.rendering_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RenderingConfiguration", rendering_configuration)?;
        }
        if let Some(ref server_side_encryption_configuration) = self.server_side_encryption_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerSideEncryptionConfiguration", server_side_encryption_configuration)?;
        }
        if let Some(ref source_configuration) = self.source_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceConfiguration", source_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for KnowledgeBaseProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<KnowledgeBaseProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = KnowledgeBaseProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type KnowledgeBaseProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut knowledge_base_type: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut rendering_configuration: Option<::Value<self::knowledge_base::RenderingConfiguration>> = None;
                let mut server_side_encryption_configuration: Option<::Value<self::knowledge_base::ServerSideEncryptionConfiguration>> = None;
                let mut source_configuration: Option<::Value<self::knowledge_base::SourceConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KnowledgeBaseType" => {
                            knowledge_base_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RenderingConfiguration" => {
                            rendering_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerSideEncryptionConfiguration" => {
                            server_side_encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceConfiguration" => {
                            source_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(KnowledgeBaseProperties {
                    description: description,
                    knowledge_base_type: knowledge_base_type.ok_or(::serde::de::Error::missing_field("KnowledgeBaseType"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    rendering_configuration: rendering_configuration,
                    server_side_encryption_configuration: server_side_encryption_configuration,
                    source_configuration: source_configuration,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for KnowledgeBase {
    type Properties = KnowledgeBaseProperties;
    const TYPE: &'static str = "AWS::Wisdom::KnowledgeBase";
    fn properties(&self) -> &KnowledgeBaseProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut KnowledgeBaseProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for KnowledgeBase {}

impl From<KnowledgeBaseProperties> for KnowledgeBase {
    fn from(properties: KnowledgeBaseProperties) -> KnowledgeBase {
        KnowledgeBase { properties }
    }
}

pub mod assistant {
    //! Property types for the `Assistant` resource.

    /// The [`AWS::Wisdom::Assistant.ServerSideEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-assistant-serversideencryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerSideEncryptionConfiguration {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-assistant-serversideencryptionconfiguration.html#cfn-wisdom-assistant-serversideencryptionconfiguration-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServerSideEncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerSideEncryptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerSideEncryptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerSideEncryptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerSideEncryptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerSideEncryptionConfiguration {
                        kms_key_id: kms_key_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod assistant_association {
    //! Property types for the `AssistantAssociation` resource.

    /// The [`AWS::Wisdom::AssistantAssociation.AssociationData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-assistantassociation-associationdata.html) property type.
    #[derive(Debug, Default)]
    pub struct AssociationData {
        /// Property [`KnowledgeBaseId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-assistantassociation-associationdata.html#cfn-wisdom-assistantassociation-associationdata-knowledgebaseid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub knowledge_base_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for AssociationData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KnowledgeBaseId", &self.knowledge_base_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssociationData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssociationData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssociationData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssociationData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut knowledge_base_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KnowledgeBaseId" => {
                                knowledge_base_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssociationData {
                        knowledge_base_id: knowledge_base_id.ok_or(::serde::de::Error::missing_field("KnowledgeBaseId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod knowledge_base {
    //! Property types for the `KnowledgeBase` resource.

    /// The [`AWS::Wisdom::KnowledgeBase.AppIntegrationsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-appintegrationsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AppIntegrationsConfiguration {
        /// Property [`AppIntegrationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-appintegrationsconfiguration.html#cfn-wisdom-knowledgebase-appintegrationsconfiguration-appintegrationarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub app_integration_arn: ::Value<String>,
        /// Property [`ObjectFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-appintegrationsconfiguration.html#cfn-wisdom-knowledgebase-appintegrationsconfiguration-objectfields).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub object_fields: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AppIntegrationsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppIntegrationArn", &self.app_integration_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectFields", &self.object_fields)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AppIntegrationsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AppIntegrationsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AppIntegrationsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AppIntegrationsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_integration_arn: Option<::Value<String>> = None;
                    let mut object_fields: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppIntegrationArn" => {
                                app_integration_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectFields" => {
                                object_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AppIntegrationsConfiguration {
                        app_integration_arn: app_integration_arn.ok_or(::serde::de::Error::missing_field("AppIntegrationArn"))?,
                        object_fields: object_fields.ok_or(::serde::de::Error::missing_field("ObjectFields"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Wisdom::KnowledgeBase.RenderingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-renderingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct RenderingConfiguration {
        /// Property [`TemplateUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-renderingconfiguration.html#cfn-wisdom-knowledgebase-renderingconfiguration-templateuri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub template_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RenderingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref template_uri) = self.template_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateUri", template_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RenderingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RenderingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RenderingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RenderingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut template_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TemplateUri" => {
                                template_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RenderingConfiguration {
                        template_uri: template_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Wisdom::KnowledgeBase.ServerSideEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-serversideencryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerSideEncryptionConfiguration {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-serversideencryptionconfiguration.html#cfn-wisdom-knowledgebase-serversideencryptionconfiguration-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServerSideEncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerSideEncryptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerSideEncryptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerSideEncryptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerSideEncryptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerSideEncryptionConfiguration {
                        kms_key_id: kms_key_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Wisdom::KnowledgeBase.SourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-sourceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceConfiguration {
        /// Property [`AppIntegrations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-sourceconfiguration.html#cfn-wisdom-knowledgebase-sourceconfiguration-appintegrations).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub app_integrations: Option<::Value<AppIntegrationsConfiguration>>,
    }

    impl ::codec::SerializeValue for SourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref app_integrations) = self.app_integrations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppIntegrations", app_integrations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_integrations: Option<::Value<AppIntegrationsConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppIntegrations" => {
                                app_integrations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceConfiguration {
                        app_integrations: app_integrations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
