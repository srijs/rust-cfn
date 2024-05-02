//! Types for the `CleanRooms` service.

/// The [`AWS::CleanRooms::AnalysisTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-analysistemplate.html) resource type.
#[derive(Debug, Default)]
pub struct AnalysisTemplate {
    properties: AnalysisTemplateProperties
}

/// Properties for the `AnalysisTemplate` resource.
#[derive(Debug, Default)]
pub struct AnalysisTemplateProperties {
    /// Property [`AnalysisParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-analysistemplate.html#cfn-cleanrooms-analysistemplate-analysisparameters).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub analysis_parameters: Option<::ValueList<self::analysis_template::AnalysisParameter>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-analysistemplate.html#cfn-cleanrooms-analysistemplate-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-analysistemplate.html#cfn-cleanrooms-analysistemplate-format).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub format: ::Value<String>,
    /// Property [`MembershipIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-analysistemplate.html#cfn-cleanrooms-analysistemplate-membershipidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub membership_identifier: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-analysistemplate.html#cfn-cleanrooms-analysistemplate-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-analysistemplate.html#cfn-cleanrooms-analysistemplate-source).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source: ::Value<self::analysis_template::AnalysisSource>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-analysistemplate.html#cfn-cleanrooms-analysistemplate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AnalysisTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref analysis_parameters) = self.analysis_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnalysisParameters", analysis_parameters)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", &self.format)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MembershipIdentifier", &self.membership_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AnalysisTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AnalysisTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AnalysisTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut analysis_parameters: Option<::ValueList<self::analysis_template::AnalysisParameter>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut format: Option<::Value<String>> = None;
                let mut membership_identifier: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut source: Option<::Value<self::analysis_template::AnalysisSource>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AnalysisParameters" => {
                            analysis_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Format" => {
                            format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MembershipIdentifier" => {
                            membership_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Source" => {
                            source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AnalysisTemplateProperties {
                    analysis_parameters: analysis_parameters,
                    description: description,
                    format: format.ok_or(::serde::de::Error::missing_field("Format"))?,
                    membership_identifier: membership_identifier.ok_or(::serde::de::Error::missing_field("MembershipIdentifier"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AnalysisTemplate {
    type Properties = AnalysisTemplateProperties;
    const TYPE: &'static str = "AWS::CleanRooms::AnalysisTemplate";
    fn properties(&self) -> &AnalysisTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AnalysisTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AnalysisTemplate {}

impl From<AnalysisTemplateProperties> for AnalysisTemplate {
    fn from(properties: AnalysisTemplateProperties) -> AnalysisTemplate {
        AnalysisTemplate { properties }
    }
}

/// The [`AWS::CleanRooms::Collaboration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html) resource type.
#[derive(Debug, Default)]
pub struct Collaboration {
    properties: CollaborationProperties
}

/// Properties for the `Collaboration` resource.
#[derive(Debug, Default)]
pub struct CollaborationProperties {
    /// Property [`CreatorDisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html#cfn-cleanrooms-collaboration-creatordisplayname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub creator_display_name: ::Value<String>,
    /// Property [`CreatorMemberAbilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html#cfn-cleanrooms-collaboration-creatormemberabilities).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub creator_member_abilities: ::ValueList<String>,
    /// Property [`CreatorPaymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html#cfn-cleanrooms-collaboration-creatorpaymentconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub creator_payment_configuration: Option<::Value<self::collaboration::PaymentConfiguration>>,
    /// Property [`DataEncryptionMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html#cfn-cleanrooms-collaboration-dataencryptionmetadata).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_encryption_metadata: Option<::Value<self::collaboration::DataEncryptionMetadata>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html#cfn-cleanrooms-collaboration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`Members`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html#cfn-cleanrooms-collaboration-members).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub members: ::ValueList<self::collaboration::MemberSpecification>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html#cfn-cleanrooms-collaboration-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`QueryLogStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html#cfn-cleanrooms-collaboration-querylogstatus).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub query_log_status: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-collaboration.html#cfn-cleanrooms-collaboration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CollaborationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatorDisplayName", &self.creator_display_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatorMemberAbilities", &self.creator_member_abilities)?;
        if let Some(ref creator_payment_configuration) = self.creator_payment_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatorPaymentConfiguration", creator_payment_configuration)?;
        }
        if let Some(ref data_encryption_metadata) = self.data_encryption_metadata {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataEncryptionMetadata", data_encryption_metadata)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Members", &self.members)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryLogStatus", &self.query_log_status)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CollaborationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CollaborationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CollaborationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CollaborationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut creator_display_name: Option<::Value<String>> = None;
                let mut creator_member_abilities: Option<::ValueList<String>> = None;
                let mut creator_payment_configuration: Option<::Value<self::collaboration::PaymentConfiguration>> = None;
                let mut data_encryption_metadata: Option<::Value<self::collaboration::DataEncryptionMetadata>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut members: Option<::ValueList<self::collaboration::MemberSpecification>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut query_log_status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CreatorDisplayName" => {
                            creator_display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CreatorMemberAbilities" => {
                            creator_member_abilities = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CreatorPaymentConfiguration" => {
                            creator_payment_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataEncryptionMetadata" => {
                            data_encryption_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Members" => {
                            members = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryLogStatus" => {
                            query_log_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CollaborationProperties {
                    creator_display_name: creator_display_name.ok_or(::serde::de::Error::missing_field("CreatorDisplayName"))?,
                    creator_member_abilities: creator_member_abilities.ok_or(::serde::de::Error::missing_field("CreatorMemberAbilities"))?,
                    creator_payment_configuration: creator_payment_configuration,
                    data_encryption_metadata: data_encryption_metadata,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    members: members.ok_or(::serde::de::Error::missing_field("Members"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    query_log_status: query_log_status.ok_or(::serde::de::Error::missing_field("QueryLogStatus"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Collaboration {
    type Properties = CollaborationProperties;
    const TYPE: &'static str = "AWS::CleanRooms::Collaboration";
    fn properties(&self) -> &CollaborationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CollaborationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Collaboration {}

impl From<CollaborationProperties> for Collaboration {
    fn from(properties: CollaborationProperties) -> Collaboration {
        Collaboration { properties }
    }
}

/// The [`AWS::CleanRooms::ConfiguredTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtable.html) resource type.
#[derive(Debug, Default)]
pub struct ConfiguredTable {
    properties: ConfiguredTableProperties
}

/// Properties for the `ConfiguredTable` resource.
#[derive(Debug, Default)]
pub struct ConfiguredTableProperties {
    /// Property [`AllowedColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtable.html#cfn-cleanrooms-configuredtable-allowedcolumns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub allowed_columns: ::ValueList<String>,
    /// Property [`AnalysisMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtable.html#cfn-cleanrooms-configuredtable-analysismethod).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub analysis_method: ::Value<String>,
    /// Property [`AnalysisRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtable.html#cfn-cleanrooms-configuredtable-analysisrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub analysis_rules: Option<::ValueList<self::configured_table::AnalysisRule>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtable.html#cfn-cleanrooms-configuredtable-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtable.html#cfn-cleanrooms-configuredtable-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`TableReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtable.html#cfn-cleanrooms-configuredtable-tablereference).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub table_reference: ::Value<self::configured_table::TableReference>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtable.html#cfn-cleanrooms-configuredtable-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ConfiguredTableProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedColumns", &self.allowed_columns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnalysisMethod", &self.analysis_method)?;
        if let Some(ref analysis_rules) = self.analysis_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnalysisRules", analysis_rules)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableReference", &self.table_reference)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfiguredTableProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfiguredTableProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfiguredTableProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfiguredTableProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allowed_columns: Option<::ValueList<String>> = None;
                let mut analysis_method: Option<::Value<String>> = None;
                let mut analysis_rules: Option<::ValueList<self::configured_table::AnalysisRule>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut table_reference: Option<::Value<self::configured_table::TableReference>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowedColumns" => {
                            allowed_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AnalysisMethod" => {
                            analysis_method = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AnalysisRules" => {
                            analysis_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableReference" => {
                            table_reference = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfiguredTableProperties {
                    allowed_columns: allowed_columns.ok_or(::serde::de::Error::missing_field("AllowedColumns"))?,
                    analysis_method: analysis_method.ok_or(::serde::de::Error::missing_field("AnalysisMethod"))?,
                    analysis_rules: analysis_rules,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    table_reference: table_reference.ok_or(::serde::de::Error::missing_field("TableReference"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfiguredTable {
    type Properties = ConfiguredTableProperties;
    const TYPE: &'static str = "AWS::CleanRooms::ConfiguredTable";
    fn properties(&self) -> &ConfiguredTableProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfiguredTableProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfiguredTable {}

impl From<ConfiguredTableProperties> for ConfiguredTable {
    fn from(properties: ConfiguredTableProperties) -> ConfiguredTable {
        ConfiguredTable { properties }
    }
}

/// The [`AWS::CleanRooms::ConfiguredTableAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtableassociation.html) resource type.
#[derive(Debug, Default)]
pub struct ConfiguredTableAssociation {
    properties: ConfiguredTableAssociationProperties
}

/// Properties for the `ConfiguredTableAssociation` resource.
#[derive(Debug, Default)]
pub struct ConfiguredTableAssociationProperties {
    /// Property [`ConfiguredTableIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtableassociation.html#cfn-cleanrooms-configuredtableassociation-configuredtableidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub configured_table_identifier: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtableassociation.html#cfn-cleanrooms-configuredtableassociation-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`MembershipIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtableassociation.html#cfn-cleanrooms-configuredtableassociation-membershipidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub membership_identifier: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtableassociation.html#cfn-cleanrooms-configuredtableassociation-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtableassociation.html#cfn-cleanrooms-configuredtableassociation-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-configuredtableassociation.html#cfn-cleanrooms-configuredtableassociation-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ConfiguredTableAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfiguredTableIdentifier", &self.configured_table_identifier)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MembershipIdentifier", &self.membership_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfiguredTableAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfiguredTableAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfiguredTableAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfiguredTableAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut configured_table_identifier: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut membership_identifier: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfiguredTableIdentifier" => {
                            configured_table_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MembershipIdentifier" => {
                            membership_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ConfiguredTableAssociationProperties {
                    configured_table_identifier: configured_table_identifier.ok_or(::serde::de::Error::missing_field("ConfiguredTableIdentifier"))?,
                    description: description,
                    membership_identifier: membership_identifier.ok_or(::serde::de::Error::missing_field("MembershipIdentifier"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfiguredTableAssociation {
    type Properties = ConfiguredTableAssociationProperties;
    const TYPE: &'static str = "AWS::CleanRooms::ConfiguredTableAssociation";
    fn properties(&self) -> &ConfiguredTableAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfiguredTableAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfiguredTableAssociation {}

impl From<ConfiguredTableAssociationProperties> for ConfiguredTableAssociation {
    fn from(properties: ConfiguredTableAssociationProperties) -> ConfiguredTableAssociation {
        ConfiguredTableAssociation { properties }
    }
}

/// The [`AWS::CleanRooms::Membership`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-membership.html) resource type.
#[derive(Debug, Default)]
pub struct Membership {
    properties: MembershipProperties
}

/// Properties for the `Membership` resource.
#[derive(Debug, Default)]
pub struct MembershipProperties {
    /// Property [`CollaborationIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-membership.html#cfn-cleanrooms-membership-collaborationidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub collaboration_identifier: ::Value<String>,
    /// Property [`DefaultResultConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-membership.html#cfn-cleanrooms-membership-defaultresultconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_result_configuration: Option<::Value<self::membership::MembershipProtectedQueryResultConfiguration>>,
    /// Property [`PaymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-membership.html#cfn-cleanrooms-membership-paymentconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub payment_configuration: Option<::Value<self::membership::MembershipPaymentConfiguration>>,
    /// Property [`QueryLogStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-membership.html#cfn-cleanrooms-membership-querylogstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub query_log_status: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanrooms-membership.html#cfn-cleanrooms-membership-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for MembershipProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollaborationIdentifier", &self.collaboration_identifier)?;
        if let Some(ref default_result_configuration) = self.default_result_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResultConfiguration", default_result_configuration)?;
        }
        if let Some(ref payment_configuration) = self.payment_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PaymentConfiguration", payment_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryLogStatus", &self.query_log_status)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MembershipProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MembershipProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MembershipProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MembershipProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut collaboration_identifier: Option<::Value<String>> = None;
                let mut default_result_configuration: Option<::Value<self::membership::MembershipProtectedQueryResultConfiguration>> = None;
                let mut payment_configuration: Option<::Value<self::membership::MembershipPaymentConfiguration>> = None;
                let mut query_log_status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CollaborationIdentifier" => {
                            collaboration_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultResultConfiguration" => {
                            default_result_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PaymentConfiguration" => {
                            payment_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryLogStatus" => {
                            query_log_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MembershipProperties {
                    collaboration_identifier: collaboration_identifier.ok_or(::serde::de::Error::missing_field("CollaborationIdentifier"))?,
                    default_result_configuration: default_result_configuration,
                    payment_configuration: payment_configuration,
                    query_log_status: query_log_status.ok_or(::serde::de::Error::missing_field("QueryLogStatus"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Membership {
    type Properties = MembershipProperties;
    const TYPE: &'static str = "AWS::CleanRooms::Membership";
    fn properties(&self) -> &MembershipProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MembershipProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Membership {}

impl From<MembershipProperties> for Membership {
    fn from(properties: MembershipProperties) -> Membership {
        Membership { properties }
    }
}

pub mod analysis_template {
    //! Property types for the `AnalysisTemplate` resource.

    /// The [`AWS::CleanRooms::AnalysisTemplate.AnalysisParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysisparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalysisParameter {
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysisparameter.html#cfn-cleanrooms-analysistemplate-analysisparameter-defaultvalue).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub default_value: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysisparameter.html#cfn-cleanrooms-analysistemplate-analysisparameter-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysisparameter.html#cfn-cleanrooms-analysistemplate-analysisparameter-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for AnalysisParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalysisParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalysisParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalysisParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_value: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalysisParameter {
                        default_value: default_value,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::AnalysisTemplate.AnalysisSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysisschema.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalysisSchema {
        /// Property [`ReferencedTables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysisschema.html#cfn-cleanrooms-analysistemplate-analysisschema-referencedtables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub referenced_tables: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AnalysisSchema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferencedTables", &self.referenced_tables)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalysisSchema {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisSchema, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalysisSchema;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalysisSchema")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut referenced_tables: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReferencedTables" => {
                                referenced_tables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalysisSchema {
                        referenced_tables: referenced_tables.ok_or(::serde::de::Error::missing_field("ReferencedTables"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::AnalysisTemplate.AnalysisSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysissource.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalysisSource {
        /// Property [`Text`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-analysistemplate-analysissource.html#cfn-cleanrooms-analysistemplate-analysissource-text).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub text: ::Value<String>,
    }

    impl ::codec::SerializeValue for AnalysisSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Text", &self.text)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalysisSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalysisSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalysisSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut text: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Text" => {
                                text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalysisSource {
                        text: text.ok_or(::serde::de::Error::missing_field("Text"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod collaboration {
    //! Property types for the `Collaboration` resource.

    /// The [`AWS::CleanRooms::Collaboration.DataEncryptionMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-dataencryptionmetadata.html) property type.
    #[derive(Debug, Default)]
    pub struct DataEncryptionMetadata {
        /// Property [`AllowCleartext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-dataencryptionmetadata.html#cfn-cleanrooms-collaboration-dataencryptionmetadata-allowcleartext).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub allow_cleartext: ::Value<bool>,
        /// Property [`AllowDuplicates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-dataencryptionmetadata.html#cfn-cleanrooms-collaboration-dataencryptionmetadata-allowduplicates).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub allow_duplicates: ::Value<bool>,
        /// Property [`AllowJoinsOnColumnsWithDifferentNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-dataencryptionmetadata.html#cfn-cleanrooms-collaboration-dataencryptionmetadata-allowjoinsoncolumnswithdifferentnames).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub allow_joins_on_columns_with_different_names: ::Value<bool>,
        /// Property [`PreserveNulls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-dataencryptionmetadata.html#cfn-cleanrooms-collaboration-dataencryptionmetadata-preservenulls).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub preserve_nulls: ::Value<bool>,
    }

    impl ::codec::SerializeValue for DataEncryptionMetadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowCleartext", &self.allow_cleartext)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowDuplicates", &self.allow_duplicates)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowJoinsOnColumnsWithDifferentNames", &self.allow_joins_on_columns_with_different_names)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreserveNulls", &self.preserve_nulls)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataEncryptionMetadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataEncryptionMetadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataEncryptionMetadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataEncryptionMetadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_cleartext: Option<::Value<bool>> = None;
                    let mut allow_duplicates: Option<::Value<bool>> = None;
                    let mut allow_joins_on_columns_with_different_names: Option<::Value<bool>> = None;
                    let mut preserve_nulls: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowCleartext" => {
                                allow_cleartext = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowDuplicates" => {
                                allow_duplicates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowJoinsOnColumnsWithDifferentNames" => {
                                allow_joins_on_columns_with_different_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreserveNulls" => {
                                preserve_nulls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataEncryptionMetadata {
                        allow_cleartext: allow_cleartext.ok_or(::serde::de::Error::missing_field("AllowCleartext"))?,
                        allow_duplicates: allow_duplicates.ok_or(::serde::de::Error::missing_field("AllowDuplicates"))?,
                        allow_joins_on_columns_with_different_names: allow_joins_on_columns_with_different_names.ok_or(::serde::de::Error::missing_field("AllowJoinsOnColumnsWithDifferentNames"))?,
                        preserve_nulls: preserve_nulls.ok_or(::serde::de::Error::missing_field("PreserveNulls"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::Collaboration.MemberSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-memberspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct MemberSpecification {
        /// Property [`AccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-memberspecification.html#cfn-cleanrooms-collaboration-memberspecification-accountid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub account_id: ::Value<String>,
        /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-memberspecification.html#cfn-cleanrooms-collaboration-memberspecification-displayname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub display_name: ::Value<String>,
        /// Property [`MemberAbilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-memberspecification.html#cfn-cleanrooms-collaboration-memberspecification-memberabilities).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub member_abilities: ::ValueList<String>,
        /// Property [`PaymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-memberspecification.html#cfn-cleanrooms-collaboration-memberspecification-paymentconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub payment_configuration: Option<::Value<PaymentConfiguration>>,
    }

    impl ::codec::SerializeValue for MemberSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountId", &self.account_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", &self.display_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberAbilities", &self.member_abilities)?;
            if let Some(ref payment_configuration) = self.payment_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PaymentConfiguration", payment_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MemberSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MemberSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MemberSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MemberSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut account_id: Option<::Value<String>> = None;
                    let mut display_name: Option<::Value<String>> = None;
                    let mut member_abilities: Option<::ValueList<String>> = None;
                    let mut payment_configuration: Option<::Value<PaymentConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccountId" => {
                                account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisplayName" => {
                                display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MemberAbilities" => {
                                member_abilities = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PaymentConfiguration" => {
                                payment_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MemberSpecification {
                        account_id: account_id.ok_or(::serde::de::Error::missing_field("AccountId"))?,
                        display_name: display_name.ok_or(::serde::de::Error::missing_field("DisplayName"))?,
                        member_abilities: member_abilities.ok_or(::serde::de::Error::missing_field("MemberAbilities"))?,
                        payment_configuration: payment_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::Collaboration.PaymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-paymentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct PaymentConfiguration {
        /// Property [`QueryCompute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-paymentconfiguration.html#cfn-cleanrooms-collaboration-paymentconfiguration-querycompute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub query_compute: ::Value<QueryComputePaymentConfig>,
    }

    impl ::codec::SerializeValue for PaymentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryCompute", &self.query_compute)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PaymentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PaymentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PaymentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PaymentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut query_compute: Option<::Value<QueryComputePaymentConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "QueryCompute" => {
                                query_compute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PaymentConfiguration {
                        query_compute: query_compute.ok_or(::serde::de::Error::missing_field("QueryCompute"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::Collaboration.QueryComputePaymentConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-querycomputepaymentconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QueryComputePaymentConfig {
        /// Property [`IsResponsible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-collaboration-querycomputepaymentconfig.html#cfn-cleanrooms-collaboration-querycomputepaymentconfig-isresponsible).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub is_responsible: ::Value<bool>,
    }

    impl ::codec::SerializeValue for QueryComputePaymentConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsResponsible", &self.is_responsible)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueryComputePaymentConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueryComputePaymentConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueryComputePaymentConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueryComputePaymentConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut is_responsible: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IsResponsible" => {
                                is_responsible = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueryComputePaymentConfig {
                        is_responsible: is_responsible.ok_or(::serde::de::Error::missing_field("IsResponsible"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod configured_table {
    //! Property types for the `ConfiguredTable` resource.

    /// The [`AWS::CleanRooms::ConfiguredTable.AggregateColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-aggregatecolumn.html) property type.
    #[derive(Debug, Default)]
    pub struct AggregateColumn {
        /// Property [`ColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-aggregatecolumn.html#cfn-cleanrooms-configuredtable-aggregatecolumn-columnnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_names: ::ValueList<String>,
        /// Property [`Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-aggregatecolumn.html#cfn-cleanrooms-configuredtable-aggregatecolumn-function).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function: ::Value<String>,
    }

    impl ::codec::SerializeValue for AggregateColumn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnNames", &self.column_names)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Function", &self.function)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AggregateColumn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AggregateColumn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AggregateColumn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AggregateColumn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_names: Option<::ValueList<String>> = None;
                    let mut function: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnNames" => {
                                column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Function" => {
                                function = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AggregateColumn {
                        column_names: column_names.ok_or(::serde::de::Error::missing_field("ColumnNames"))?,
                        function: function.ok_or(::serde::de::Error::missing_field("Function"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::ConfiguredTable.AggregationConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-aggregationconstraint.html) property type.
    #[derive(Debug, Default)]
    pub struct AggregationConstraint {
        /// Property [`ColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-aggregationconstraint.html#cfn-cleanrooms-configuredtable-aggregationconstraint-columnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_name: ::Value<String>,
        /// Property [`Minimum`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-aggregationconstraint.html#cfn-cleanrooms-configuredtable-aggregationconstraint-minimum).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimum: ::Value<f64>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-aggregationconstraint.html#cfn-cleanrooms-configuredtable-aggregationconstraint-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for AggregationConstraint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnName", &self.column_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Minimum", &self.minimum)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AggregationConstraint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AggregationConstraint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AggregationConstraint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AggregationConstraint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_name: Option<::Value<String>> = None;
                    let mut minimum: Option<::Value<f64>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnName" => {
                                column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Minimum" => {
                                minimum = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AggregationConstraint {
                        column_name: column_name.ok_or(::serde::de::Error::missing_field("ColumnName"))?,
                        minimum: minimum.ok_or(::serde::de::Error::missing_field("Minimum"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::ConfiguredTable.AnalysisRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrule.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalysisRule {
        /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrule.html#cfn-cleanrooms-configuredtable-analysisrule-policy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy: ::Value<ConfiguredTableAnalysisRulePolicy>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrule.html#cfn-cleanrooms-configuredtable-analysisrule-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for AnalysisRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", &self.policy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalysisRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalysisRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalysisRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy: Option<::Value<ConfiguredTableAnalysisRulePolicy>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Policy" => {
                                policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalysisRule {
                        policy: policy.ok_or(::serde::de::Error::missing_field("Policy"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::ConfiguredTable.AnalysisRuleAggregation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisruleaggregation.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalysisRuleAggregation {
        /// Property [`AggregateColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisruleaggregation.html#cfn-cleanrooms-configuredtable-analysisruleaggregation-aggregatecolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aggregate_columns: ::ValueList<AggregateColumn>,
        /// Property [`AllowedJoinOperators`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisruleaggregation.html#cfn-cleanrooms-configuredtable-analysisruleaggregation-allowedjoinoperators).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_join_operators: Option<::ValueList<String>>,
        /// Property [`DimensionColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisruleaggregation.html#cfn-cleanrooms-configuredtable-analysisruleaggregation-dimensioncolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension_columns: ::ValueList<String>,
        /// Property [`JoinColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisruleaggregation.html#cfn-cleanrooms-configuredtable-analysisruleaggregation-joincolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub join_columns: ::ValueList<String>,
        /// Property [`JoinRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisruleaggregation.html#cfn-cleanrooms-configuredtable-analysisruleaggregation-joinrequired).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub join_required: Option<::Value<String>>,
        /// Property [`OutputConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisruleaggregation.html#cfn-cleanrooms-configuredtable-analysisruleaggregation-outputconstraints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_constraints: ::ValueList<AggregationConstraint>,
        /// Property [`ScalarFunctions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisruleaggregation.html#cfn-cleanrooms-configuredtable-analysisruleaggregation-scalarfunctions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scalar_functions: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AnalysisRuleAggregation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregateColumns", &self.aggregate_columns)?;
            if let Some(ref allowed_join_operators) = self.allowed_join_operators {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedJoinOperators", allowed_join_operators)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionColumns", &self.dimension_columns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JoinColumns", &self.join_columns)?;
            if let Some(ref join_required) = self.join_required {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JoinRequired", join_required)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputConstraints", &self.output_constraints)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalarFunctions", &self.scalar_functions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalysisRuleAggregation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisRuleAggregation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalysisRuleAggregation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalysisRuleAggregation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aggregate_columns: Option<::ValueList<AggregateColumn>> = None;
                    let mut allowed_join_operators: Option<::ValueList<String>> = None;
                    let mut dimension_columns: Option<::ValueList<String>> = None;
                    let mut join_columns: Option<::ValueList<String>> = None;
                    let mut join_required: Option<::Value<String>> = None;
                    let mut output_constraints: Option<::ValueList<AggregationConstraint>> = None;
                    let mut scalar_functions: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AggregateColumns" => {
                                aggregate_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowedJoinOperators" => {
                                allowed_join_operators = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DimensionColumns" => {
                                dimension_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JoinColumns" => {
                                join_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JoinRequired" => {
                                join_required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputConstraints" => {
                                output_constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScalarFunctions" => {
                                scalar_functions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalysisRuleAggregation {
                        aggregate_columns: aggregate_columns.ok_or(::serde::de::Error::missing_field("AggregateColumns"))?,
                        allowed_join_operators: allowed_join_operators,
                        dimension_columns: dimension_columns.ok_or(::serde::de::Error::missing_field("DimensionColumns"))?,
                        join_columns: join_columns.ok_or(::serde::de::Error::missing_field("JoinColumns"))?,
                        join_required: join_required,
                        output_constraints: output_constraints.ok_or(::serde::de::Error::missing_field("OutputConstraints"))?,
                        scalar_functions: scalar_functions.ok_or(::serde::de::Error::missing_field("ScalarFunctions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::ConfiguredTable.AnalysisRuleCustom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrulecustom.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalysisRuleCustom {
        /// Property [`AllowedAnalyses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrulecustom.html#cfn-cleanrooms-configuredtable-analysisrulecustom-allowedanalyses).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_analyses: ::ValueList<String>,
        /// Property [`AllowedAnalysisProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrulecustom.html#cfn-cleanrooms-configuredtable-analysisrulecustom-allowedanalysisproviders).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_analysis_providers: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AnalysisRuleCustom {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedAnalyses", &self.allowed_analyses)?;
            if let Some(ref allowed_analysis_providers) = self.allowed_analysis_providers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedAnalysisProviders", allowed_analysis_providers)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalysisRuleCustom {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisRuleCustom, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalysisRuleCustom;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalysisRuleCustom")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_analyses: Option<::ValueList<String>> = None;
                    let mut allowed_analysis_providers: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedAnalyses" => {
                                allowed_analyses = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowedAnalysisProviders" => {
                                allowed_analysis_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalysisRuleCustom {
                        allowed_analyses: allowed_analyses.ok_or(::serde::de::Error::missing_field("AllowedAnalyses"))?,
                        allowed_analysis_providers: allowed_analysis_providers,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::ConfiguredTable.AnalysisRuleList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrulelist.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalysisRuleList {
        /// Property [`AllowedJoinOperators`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrulelist.html#cfn-cleanrooms-configuredtable-analysisrulelist-allowedjoinoperators).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_join_operators: Option<::ValueList<String>>,
        /// Property [`JoinColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrulelist.html#cfn-cleanrooms-configuredtable-analysisrulelist-joincolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub join_columns: ::ValueList<String>,
        /// Property [`ListColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-analysisrulelist.html#cfn-cleanrooms-configuredtable-analysisrulelist-listcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub list_columns: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AnalysisRuleList {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_join_operators) = self.allowed_join_operators {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedJoinOperators", allowed_join_operators)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JoinColumns", &self.join_columns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ListColumns", &self.list_columns)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalysisRuleList {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisRuleList, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalysisRuleList;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalysisRuleList")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_join_operators: Option<::ValueList<String>> = None;
                    let mut join_columns: Option<::ValueList<String>> = None;
                    let mut list_columns: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedJoinOperators" => {
                                allowed_join_operators = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JoinColumns" => {
                                join_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ListColumns" => {
                                list_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalysisRuleList {
                        allowed_join_operators: allowed_join_operators,
                        join_columns: join_columns.ok_or(::serde::de::Error::missing_field("JoinColumns"))?,
                        list_columns: list_columns.ok_or(::serde::de::Error::missing_field("ListColumns"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::ConfiguredTable.ConfiguredTableAnalysisRulePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-configuredtableanalysisrulepolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfiguredTableAnalysisRulePolicy {
        /// Property [`V1`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-configuredtableanalysisrulepolicy.html#cfn-cleanrooms-configuredtable-configuredtableanalysisrulepolicy-v1).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub v1: ::Value<ConfiguredTableAnalysisRulePolicyV1>,
    }

    impl ::codec::SerializeValue for ConfiguredTableAnalysisRulePolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "V1", &self.v1)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfiguredTableAnalysisRulePolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfiguredTableAnalysisRulePolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfiguredTableAnalysisRulePolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfiguredTableAnalysisRulePolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut v1: Option<::Value<ConfiguredTableAnalysisRulePolicyV1>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "V1" => {
                                v1 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfiguredTableAnalysisRulePolicy {
                        v1: v1.ok_or(::serde::de::Error::missing_field("V1"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::ConfiguredTable.ConfiguredTableAnalysisRulePolicyV1`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-configuredtableanalysisrulepolicyv1.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfiguredTableAnalysisRulePolicyV1 {
        /// Property [`Aggregation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-configuredtableanalysisrulepolicyv1.html#cfn-cleanrooms-configuredtable-configuredtableanalysisrulepolicyv1-aggregation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aggregation: Option<::Value<AnalysisRuleAggregation>>,
        /// Property [`Custom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-configuredtableanalysisrulepolicyv1.html#cfn-cleanrooms-configuredtable-configuredtableanalysisrulepolicyv1-custom).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom: Option<::Value<AnalysisRuleCustom>>,
        /// Property [`List`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-configuredtableanalysisrulepolicyv1.html#cfn-cleanrooms-configuredtable-configuredtableanalysisrulepolicyv1-list).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub list: Option<::Value<AnalysisRuleList>>,
    }

    impl ::codec::SerializeValue for ConfiguredTableAnalysisRulePolicyV1 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aggregation) = self.aggregation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Aggregation", aggregation)?;
            }
            if let Some(ref custom) = self.custom {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Custom", custom)?;
            }
            if let Some(ref list) = self.list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "List", list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfiguredTableAnalysisRulePolicyV1 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfiguredTableAnalysisRulePolicyV1, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfiguredTableAnalysisRulePolicyV1;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfiguredTableAnalysisRulePolicyV1")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aggregation: Option<::Value<AnalysisRuleAggregation>> = None;
                    let mut custom: Option<::Value<AnalysisRuleCustom>> = None;
                    let mut list: Option<::Value<AnalysisRuleList>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Aggregation" => {
                                aggregation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Custom" => {
                                custom = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "List" => {
                                list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfiguredTableAnalysisRulePolicyV1 {
                        aggregation: aggregation,
                        custom: custom,
                        list: list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::ConfiguredTable.GlueTableReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-gluetablereference.html) property type.
    #[derive(Debug, Default)]
    pub struct GlueTableReference {
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-gluetablereference.html#cfn-cleanrooms-configuredtable-gluetablereference-databasename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-gluetablereference.html#cfn-cleanrooms-configuredtable-gluetablereference-tablename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for GlueTableReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GlueTableReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GlueTableReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GlueTableReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GlueTableReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database_name: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GlueTableReference {
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::ConfiguredTable.TableReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-tablereference.html) property type.
    #[derive(Debug, Default)]
    pub struct TableReference {
        /// Property [`Glue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-configuredtable-tablereference.html#cfn-cleanrooms-configuredtable-tablereference-glue).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub glue: ::Value<GlueTableReference>,
    }

    impl ::codec::SerializeValue for TableReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Glue", &self.glue)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut glue: Option<::Value<GlueTableReference>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Glue" => {
                                glue = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableReference {
                        glue: glue.ok_or(::serde::de::Error::missing_field("Glue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod membership {
    //! Property types for the `Membership` resource.

    /// The [`AWS::CleanRooms::Membership.MembershipPaymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershippaymentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MembershipPaymentConfiguration {
        /// Property [`QueryCompute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershippaymentconfiguration.html#cfn-cleanrooms-membership-membershippaymentconfiguration-querycompute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_compute: ::Value<MembershipQueryComputePaymentConfig>,
    }

    impl ::codec::SerializeValue for MembershipPaymentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryCompute", &self.query_compute)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MembershipPaymentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MembershipPaymentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MembershipPaymentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MembershipPaymentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut query_compute: Option<::Value<MembershipQueryComputePaymentConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "QueryCompute" => {
                                query_compute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MembershipPaymentConfiguration {
                        query_compute: query_compute.ok_or(::serde::de::Error::missing_field("QueryCompute"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::Membership.MembershipProtectedQueryOutputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipprotectedqueryoutputconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MembershipProtectedQueryOutputConfiguration {
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipprotectedqueryoutputconfiguration.html#cfn-cleanrooms-membership-membershipprotectedqueryoutputconfiguration-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: ::Value<ProtectedQueryS3OutputConfiguration>,
    }

    impl ::codec::SerializeValue for MembershipProtectedQueryOutputConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", &self.s3)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MembershipProtectedQueryOutputConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MembershipProtectedQueryOutputConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MembershipProtectedQueryOutputConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MembershipProtectedQueryOutputConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3: Option<::Value<ProtectedQueryS3OutputConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MembershipProtectedQueryOutputConfiguration {
                        s3: s3.ok_or(::serde::de::Error::missing_field("S3"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::Membership.MembershipProtectedQueryResultConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipprotectedqueryresultconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MembershipProtectedQueryResultConfiguration {
        /// Property [`OutputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipprotectedqueryresultconfiguration.html#cfn-cleanrooms-membership-membershipprotectedqueryresultconfiguration-outputconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_configuration: ::Value<MembershipProtectedQueryOutputConfiguration>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipprotectedqueryresultconfiguration.html#cfn-cleanrooms-membership-membershipprotectedqueryresultconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MembershipProtectedQueryResultConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputConfiguration", &self.output_configuration)?;
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MembershipProtectedQueryResultConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MembershipProtectedQueryResultConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MembershipProtectedQueryResultConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MembershipProtectedQueryResultConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut output_configuration: Option<::Value<MembershipProtectedQueryOutputConfiguration>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OutputConfiguration" => {
                                output_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MembershipProtectedQueryResultConfiguration {
                        output_configuration: output_configuration.ok_or(::serde::de::Error::missing_field("OutputConfiguration"))?,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::Membership.MembershipQueryComputePaymentConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipquerycomputepaymentconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MembershipQueryComputePaymentConfig {
        /// Property [`IsResponsible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-membershipquerycomputepaymentconfig.html#cfn-cleanrooms-membership-membershipquerycomputepaymentconfig-isresponsible).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_responsible: ::Value<bool>,
    }

    impl ::codec::SerializeValue for MembershipQueryComputePaymentConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsResponsible", &self.is_responsible)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MembershipQueryComputePaymentConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MembershipQueryComputePaymentConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MembershipQueryComputePaymentConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MembershipQueryComputePaymentConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut is_responsible: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IsResponsible" => {
                                is_responsible = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MembershipQueryComputePaymentConfig {
                        is_responsible: is_responsible.ok_or(::serde::de::Error::missing_field("IsResponsible"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CleanRooms::Membership.ProtectedQueryS3OutputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-protectedquerys3outputconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ProtectedQueryS3OutputConfiguration {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-protectedquerys3outputconfiguration.html#cfn-cleanrooms-membership-protectedquerys3outputconfiguration-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-protectedquerys3outputconfiguration.html#cfn-cleanrooms-membership-protectedquerys3outputconfiguration-keyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_prefix: Option<::Value<String>>,
        /// Property [`ResultFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanrooms-membership-protectedquerys3outputconfiguration.html#cfn-cleanrooms-membership-protectedquerys3outputconfiguration-resultformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub result_format: ::Value<String>,
    }

    impl ::codec::SerializeValue for ProtectedQueryS3OutputConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref key_prefix) = self.key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPrefix", key_prefix)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResultFormat", &self.result_format)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProtectedQueryS3OutputConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProtectedQueryS3OutputConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProtectedQueryS3OutputConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProtectedQueryS3OutputConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key_prefix: Option<::Value<String>> = None;
                    let mut result_format: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyPrefix" => {
                                key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResultFormat" => {
                                result_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProtectedQueryS3OutputConfiguration {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key_prefix: key_prefix,
                        result_format: result_format.ok_or(::serde::de::Error::missing_field("ResultFormat"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
