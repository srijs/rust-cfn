//! Types for the `Kendra` service.

/// The [`AWS::Kendra::DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html) resource type.
#[derive(Debug, Default)]
pub struct DataSource {
    properties: DataSourceProperties
}

/// Properties for the `DataSource` resource.
#[derive(Debug, Default)]
pub struct DataSourceProperties {
    /// Property [`CustomDocumentEnrichmentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html#cfn-kendra-datasource-customdocumentenrichmentconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_document_enrichment_configuration: Option<::Value<self::data_source::CustomDocumentEnrichmentConfiguration>>,
    /// Property [`DataSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html#cfn-kendra-datasource-datasourceconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_source_configuration: Option<::Value<self::data_source::DataSourceConfiguration>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html#cfn-kendra-datasource-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`IndexId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html#cfn-kendra-datasource-indexid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub index_id: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html#cfn-kendra-datasource-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html#cfn-kendra-datasource-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html#cfn-kendra-datasource-schedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html#cfn-kendra-datasource-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-datasource.html#cfn-kendra-datasource-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for DataSourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref custom_document_enrichment_configuration) = self.custom_document_enrichment_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomDocumentEnrichmentConfiguration", custom_document_enrichment_configuration)?;
        }
        if let Some(ref data_source_configuration) = self.data_source_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceConfiguration", data_source_configuration)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexId", &self.index_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        if let Some(ref schedule) = self.schedule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", schedule)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataSourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataSourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataSourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut custom_document_enrichment_configuration: Option<::Value<self::data_source::CustomDocumentEnrichmentConfiguration>> = None;
                let mut data_source_configuration: Option<::Value<self::data_source::DataSourceConfiguration>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut index_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut schedule: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CustomDocumentEnrichmentConfiguration" => {
                            custom_document_enrichment_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSourceConfiguration" => {
                            data_source_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IndexId" => {
                            index_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedule" => {
                            schedule = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(DataSourceProperties {
                    custom_document_enrichment_configuration: custom_document_enrichment_configuration,
                    data_source_configuration: data_source_configuration,
                    description: description,
                    index_id: index_id.ok_or(::serde::de::Error::missing_field("IndexId"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    role_arn: role_arn,
                    schedule: schedule,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataSource {
    type Properties = DataSourceProperties;
    const TYPE: &'static str = "AWS::Kendra::DataSource";
    fn properties(&self) -> &DataSourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataSourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataSource {}

impl From<DataSourceProperties> for DataSource {
    fn from(properties: DataSourceProperties) -> DataSource {
        DataSource { properties }
    }
}

/// The [`AWS::Kendra::Faq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-faq.html) resource type.
#[derive(Debug, Default)]
pub struct Faq {
    properties: FaqProperties
}

/// Properties for the `Faq` resource.
#[derive(Debug, Default)]
pub struct FaqProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-faq.html#cfn-kendra-faq-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FileFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-faq.html#cfn-kendra-faq-fileformat).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub file_format: Option<::Value<String>>,
    /// Property [`IndexId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-faq.html#cfn-kendra-faq-indexid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub index_id: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-faq.html#cfn-kendra-faq-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-faq.html#cfn-kendra-faq-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`S3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-faq.html#cfn-kendra-faq-s3path).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub s3_path: ::Value<self::faq::S3Path>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-faq.html#cfn-kendra-faq-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for FaqProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref file_format) = self.file_format {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileFormat", file_format)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexId", &self.index_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Path", &self.s3_path)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FaqProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FaqProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FaqProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FaqProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut file_format: Option<::Value<String>> = None;
                let mut index_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut s3_path: Option<::Value<self::faq::S3Path>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileFormat" => {
                            file_format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IndexId" => {
                            index_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Path" => {
                            s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FaqProperties {
                    description: description,
                    file_format: file_format,
                    index_id: index_id.ok_or(::serde::de::Error::missing_field("IndexId"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    s3_path: s3_path.ok_or(::serde::de::Error::missing_field("S3Path"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Faq {
    type Properties = FaqProperties;
    const TYPE: &'static str = "AWS::Kendra::Faq";
    fn properties(&self) -> &FaqProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FaqProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Faq {}

impl From<FaqProperties> for Faq {
    fn from(properties: FaqProperties) -> Faq {
        Faq { properties }
    }
}

/// The [`AWS::Kendra::Index`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html) resource type.
#[derive(Debug, Default)]
pub struct Index {
    properties: IndexProperties
}

/// Properties for the `Index` resource.
#[derive(Debug, Default)]
pub struct IndexProperties {
    /// Property [`CapacityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html#cfn-kendra-index-capacityunits).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub capacity_units: Option<::Value<self::index::CapacityUnitsConfiguration>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html#cfn-kendra-index-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DocumentMetadataConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html#cfn-kendra-index-documentmetadataconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub document_metadata_configurations: Option<::ValueList<self::index::DocumentMetadataConfiguration>>,
    /// Property [`Edition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html#cfn-kendra-index-edition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub edition: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html#cfn-kendra-index-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html#cfn-kendra-index-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`ServerSideEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html#cfn-kendra-index-serversideencryptionconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_side_encryption_configuration: Option<::Value<self::index::ServerSideEncryptionConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html#cfn-kendra-index-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserContextPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html#cfn-kendra-index-usercontextpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_context_policy: Option<::Value<String>>,
    /// Property [`UserTokenConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kendra-index.html#cfn-kendra-index-usertokenconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_token_configurations: Option<::ValueList<self::index::UserTokenConfiguration>>,
}

impl ::serde::Serialize for IndexProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref capacity_units) = self.capacity_units {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityUnits", capacity_units)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref document_metadata_configurations) = self.document_metadata_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentMetadataConfigurations", document_metadata_configurations)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Edition", &self.edition)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref server_side_encryption_configuration) = self.server_side_encryption_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerSideEncryptionConfiguration", server_side_encryption_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref user_context_policy) = self.user_context_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserContextPolicy", user_context_policy)?;
        }
        if let Some(ref user_token_configurations) = self.user_token_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserTokenConfigurations", user_token_configurations)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IndexProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IndexProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IndexProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IndexProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut capacity_units: Option<::Value<self::index::CapacityUnitsConfiguration>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut document_metadata_configurations: Option<::ValueList<self::index::DocumentMetadataConfiguration>> = None;
                let mut edition: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut server_side_encryption_configuration: Option<::Value<self::index::ServerSideEncryptionConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_context_policy: Option<::Value<String>> = None;
                let mut user_token_configurations: Option<::ValueList<self::index::UserTokenConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CapacityUnits" => {
                            capacity_units = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DocumentMetadataConfigurations" => {
                            document_metadata_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Edition" => {
                            edition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerSideEncryptionConfiguration" => {
                            server_side_encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserContextPolicy" => {
                            user_context_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserTokenConfigurations" => {
                            user_token_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IndexProperties {
                    capacity_units: capacity_units,
                    description: description,
                    document_metadata_configurations: document_metadata_configurations,
                    edition: edition.ok_or(::serde::de::Error::missing_field("Edition"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    server_side_encryption_configuration: server_side_encryption_configuration,
                    tags: tags,
                    user_context_policy: user_context_policy,
                    user_token_configurations: user_token_configurations,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Index {
    type Properties = IndexProperties;
    const TYPE: &'static str = "AWS::Kendra::Index";
    fn properties(&self) -> &IndexProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IndexProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Index {}

impl From<IndexProperties> for Index {
    fn from(properties: IndexProperties) -> Index {
        Index { properties }
    }
}

pub mod data_source {
    //! Property types for the `DataSource` resource.

    /// The [`AWS::Kendra::DataSource.AccessControlListConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-accesscontrollistconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessControlListConfiguration {
        /// Property [`KeyPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-accesscontrollistconfiguration.html#cfn-kendra-datasource-accesscontrollistconfiguration-keypath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AccessControlListConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key_path) = self.key_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyPath", key_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessControlListConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessControlListConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessControlListConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessControlListConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KeyPath" => {
                                key_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessControlListConfiguration {
                        key_path: key_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.AclConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-aclconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AclConfiguration {
        /// Property [`AllowedGroupsColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-aclconfiguration.html#cfn-kendra-datasource-aclconfiguration-allowedgroupscolumnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_groups_column_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for AclConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedGroupsColumnName", &self.allowed_groups_column_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AclConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AclConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AclConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AclConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_groups_column_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedGroupsColumnName" => {
                                allowed_groups_column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AclConfiguration {
                        allowed_groups_column_name: allowed_groups_column_name.ok_or(::serde::de::Error::missing_field("AllowedGroupsColumnName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ColumnConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-columnconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ColumnConfiguration {
        /// Property [`ChangeDetectingColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-columnconfiguration.html#cfn-kendra-datasource-columnconfiguration-changedetectingcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub change_detecting_columns: ::ValueList<String>,
        /// Property [`DocumentDataColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-columnconfiguration.html#cfn-kendra-datasource-columnconfiguration-documentdatacolumnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_data_column_name: ::Value<String>,
        /// Property [`DocumentIdColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-columnconfiguration.html#cfn-kendra-datasource-columnconfiguration-documentidcolumnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_id_column_name: ::Value<String>,
        /// Property [`DocumentTitleColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-columnconfiguration.html#cfn-kendra-datasource-columnconfiguration-documenttitlecolumnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_title_column_name: Option<::Value<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-columnconfiguration.html#cfn-kendra-datasource-columnconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
    }

    impl ::codec::SerializeValue for ColumnConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChangeDetectingColumns", &self.change_detecting_columns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentDataColumnName", &self.document_data_column_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentIdColumnName", &self.document_id_column_name)?;
            if let Some(ref document_title_column_name) = self.document_title_column_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentTitleColumnName", document_title_column_name)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColumnConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColumnConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColumnConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColumnConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut change_detecting_columns: Option<::ValueList<String>> = None;
                    let mut document_data_column_name: Option<::Value<String>> = None;
                    let mut document_id_column_name: Option<::Value<String>> = None;
                    let mut document_title_column_name: Option<::Value<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChangeDetectingColumns" => {
                                change_detecting_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentDataColumnName" => {
                                document_data_column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentIdColumnName" => {
                                document_id_column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentTitleColumnName" => {
                                document_title_column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ColumnConfiguration {
                        change_detecting_columns: change_detecting_columns.ok_or(::serde::de::Error::missing_field("ChangeDetectingColumns"))?,
                        document_data_column_name: document_data_column_name.ok_or(::serde::de::Error::missing_field("DocumentDataColumnName"))?,
                        document_id_column_name: document_id_column_name.ok_or(::serde::de::Error::missing_field("DocumentIdColumnName"))?,
                        document_title_column_name: document_title_column_name,
                        field_mappings: field_mappings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ConfluenceAttachmentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceattachmentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfluenceAttachmentConfiguration {
        /// Property [`AttachmentFieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceattachmentconfiguration.html#cfn-kendra-datasource-confluenceattachmentconfiguration-attachmentfieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attachment_field_mappings: Option<::ValueList<ConfluenceAttachmentToIndexFieldMapping>>,
        /// Property [`CrawlAttachments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceattachmentconfiguration.html#cfn-kendra-datasource-confluenceattachmentconfiguration-crawlattachments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawl_attachments: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ConfluenceAttachmentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attachment_field_mappings) = self.attachment_field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachmentFieldMappings", attachment_field_mappings)?;
            }
            if let Some(ref crawl_attachments) = self.crawl_attachments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlAttachments", crawl_attachments)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfluenceAttachmentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfluenceAttachmentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfluenceAttachmentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfluenceAttachmentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attachment_field_mappings: Option<::ValueList<ConfluenceAttachmentToIndexFieldMapping>> = None;
                    let mut crawl_attachments: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttachmentFieldMappings" => {
                                attachment_field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CrawlAttachments" => {
                                crawl_attachments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfluenceAttachmentConfiguration {
                        attachment_field_mappings: attachment_field_mappings,
                        crawl_attachments: crawl_attachments,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ConfluenceAttachmentToIndexFieldMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceattachmenttoindexfieldmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfluenceAttachmentToIndexFieldMapping {
        /// Property [`DataSourceFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceattachmenttoindexfieldmapping.html#cfn-kendra-datasource-confluenceattachmenttoindexfieldmapping-datasourcefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source_field_name: ::Value<String>,
        /// Property [`DateFieldFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceattachmenttoindexfieldmapping.html#cfn-kendra-datasource-confluenceattachmenttoindexfieldmapping-datefieldformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_field_format: Option<::Value<String>>,
        /// Property [`IndexFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceattachmenttoindexfieldmapping.html#cfn-kendra-datasource-confluenceattachmenttoindexfieldmapping-indexfieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_field_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ConfluenceAttachmentToIndexFieldMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceFieldName", &self.data_source_field_name)?;
            if let Some(ref date_field_format) = self.date_field_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateFieldFormat", date_field_format)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexFieldName", &self.index_field_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfluenceAttachmentToIndexFieldMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfluenceAttachmentToIndexFieldMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfluenceAttachmentToIndexFieldMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfluenceAttachmentToIndexFieldMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_source_field_name: Option<::Value<String>> = None;
                    let mut date_field_format: Option<::Value<String>> = None;
                    let mut index_field_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSourceFieldName" => {
                                data_source_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DateFieldFormat" => {
                                date_field_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexFieldName" => {
                                index_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfluenceAttachmentToIndexFieldMapping {
                        data_source_field_name: data_source_field_name.ok_or(::serde::de::Error::missing_field("DataSourceFieldName"))?,
                        date_field_format: date_field_format,
                        index_field_name: index_field_name.ok_or(::serde::de::Error::missing_field("IndexFieldName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ConfluenceBlogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceblogconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfluenceBlogConfiguration {
        /// Property [`BlogFieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceblogconfiguration.html#cfn-kendra-datasource-confluenceblogconfiguration-blogfieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub blog_field_mappings: Option<::ValueList<ConfluenceBlogToIndexFieldMapping>>,
    }

    impl ::codec::SerializeValue for ConfluenceBlogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref blog_field_mappings) = self.blog_field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlogFieldMappings", blog_field_mappings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfluenceBlogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfluenceBlogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfluenceBlogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfluenceBlogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut blog_field_mappings: Option<::ValueList<ConfluenceBlogToIndexFieldMapping>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlogFieldMappings" => {
                                blog_field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfluenceBlogConfiguration {
                        blog_field_mappings: blog_field_mappings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ConfluenceBlogToIndexFieldMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceblogtoindexfieldmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfluenceBlogToIndexFieldMapping {
        /// Property [`DataSourceFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceblogtoindexfieldmapping.html#cfn-kendra-datasource-confluenceblogtoindexfieldmapping-datasourcefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source_field_name: ::Value<String>,
        /// Property [`DateFieldFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceblogtoindexfieldmapping.html#cfn-kendra-datasource-confluenceblogtoindexfieldmapping-datefieldformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_field_format: Option<::Value<String>>,
        /// Property [`IndexFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceblogtoindexfieldmapping.html#cfn-kendra-datasource-confluenceblogtoindexfieldmapping-indexfieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_field_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ConfluenceBlogToIndexFieldMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceFieldName", &self.data_source_field_name)?;
            if let Some(ref date_field_format) = self.date_field_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateFieldFormat", date_field_format)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexFieldName", &self.index_field_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfluenceBlogToIndexFieldMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfluenceBlogToIndexFieldMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfluenceBlogToIndexFieldMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfluenceBlogToIndexFieldMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_source_field_name: Option<::Value<String>> = None;
                    let mut date_field_format: Option<::Value<String>> = None;
                    let mut index_field_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSourceFieldName" => {
                                data_source_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DateFieldFormat" => {
                                date_field_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexFieldName" => {
                                index_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfluenceBlogToIndexFieldMapping {
                        data_source_field_name: data_source_field_name.ok_or(::serde::de::Error::missing_field("DataSourceFieldName"))?,
                        date_field_format: date_field_format,
                        index_field_name: index_field_name.ok_or(::serde::de::Error::missing_field("IndexFieldName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ConfluenceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfluenceConfiguration {
        /// Property [`AttachmentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html#cfn-kendra-datasource-confluenceconfiguration-attachmentconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attachment_configuration: Option<::Value<ConfluenceAttachmentConfiguration>>,
        /// Property [`BlogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html#cfn-kendra-datasource-confluenceconfiguration-blogconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub blog_configuration: Option<::Value<ConfluenceBlogConfiguration>>,
        /// Property [`ExclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html#cfn-kendra-datasource-confluenceconfiguration-exclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclusion_patterns: Option<::ValueList<String>>,
        /// Property [`InclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html#cfn-kendra-datasource-confluenceconfiguration-inclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inclusion_patterns: Option<::ValueList<String>>,
        /// Property [`PageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html#cfn-kendra-datasource-confluenceconfiguration-pageconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub page_configuration: Option<::Value<ConfluencePageConfiguration>>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html#cfn-kendra-datasource-confluenceconfiguration-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: ::Value<String>,
        /// Property [`ServerUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html#cfn-kendra-datasource-confluenceconfiguration-serverurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_url: ::Value<String>,
        /// Property [`SpaceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html#cfn-kendra-datasource-confluenceconfiguration-spaceconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub space_configuration: Option<::Value<ConfluenceSpaceConfiguration>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html#cfn-kendra-datasource-confluenceconfiguration-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: ::Value<String>,
        /// Property [`VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluenceconfiguration.html#cfn-kendra-datasource-confluenceconfiguration-vpcconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_configuration: Option<::Value<DataSourceVpcConfiguration>>,
    }

    impl ::codec::SerializeValue for ConfluenceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attachment_configuration) = self.attachment_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachmentConfiguration", attachment_configuration)?;
            }
            if let Some(ref blog_configuration) = self.blog_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlogConfiguration", blog_configuration)?;
            }
            if let Some(ref exclusion_patterns) = self.exclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExclusionPatterns", exclusion_patterns)?;
            }
            if let Some(ref inclusion_patterns) = self.inclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InclusionPatterns", inclusion_patterns)?;
            }
            if let Some(ref page_configuration) = self.page_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PageConfiguration", page_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", &self.secret_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerUrl", &self.server_url)?;
            if let Some(ref space_configuration) = self.space_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpaceConfiguration", space_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
            if let Some(ref vpc_configuration) = self.vpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfiguration", vpc_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfluenceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfluenceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfluenceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfluenceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attachment_configuration: Option<::Value<ConfluenceAttachmentConfiguration>> = None;
                    let mut blog_configuration: Option<::Value<ConfluenceBlogConfiguration>> = None;
                    let mut exclusion_patterns: Option<::ValueList<String>> = None;
                    let mut inclusion_patterns: Option<::ValueList<String>> = None;
                    let mut page_configuration: Option<::Value<ConfluencePageConfiguration>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;
                    let mut server_url: Option<::Value<String>> = None;
                    let mut space_configuration: Option<::Value<ConfluenceSpaceConfiguration>> = None;
                    let mut version: Option<::Value<String>> = None;
                    let mut vpc_configuration: Option<::Value<DataSourceVpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttachmentConfiguration" => {
                                attachment_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BlogConfiguration" => {
                                blog_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExclusionPatterns" => {
                                exclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InclusionPatterns" => {
                                inclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PageConfiguration" => {
                                page_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerUrl" => {
                                server_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpaceConfiguration" => {
                                space_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfiguration" => {
                                vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfluenceConfiguration {
                        attachment_configuration: attachment_configuration,
                        blog_configuration: blog_configuration,
                        exclusion_patterns: exclusion_patterns,
                        inclusion_patterns: inclusion_patterns,
                        page_configuration: page_configuration,
                        secret_arn: secret_arn.ok_or(::serde::de::Error::missing_field("SecretArn"))?,
                        server_url: server_url.ok_or(::serde::de::Error::missing_field("ServerUrl"))?,
                        space_configuration: space_configuration,
                        version: version.ok_or(::serde::de::Error::missing_field("Version"))?,
                        vpc_configuration: vpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ConfluencePageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencepageconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfluencePageConfiguration {
        /// Property [`PageFieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencepageconfiguration.html#cfn-kendra-datasource-confluencepageconfiguration-pagefieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub page_field_mappings: Option<::ValueList<ConfluencePageToIndexFieldMapping>>,
    }

    impl ::codec::SerializeValue for ConfluencePageConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref page_field_mappings) = self.page_field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PageFieldMappings", page_field_mappings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfluencePageConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfluencePageConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfluencePageConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfluencePageConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut page_field_mappings: Option<::ValueList<ConfluencePageToIndexFieldMapping>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PageFieldMappings" => {
                                page_field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfluencePageConfiguration {
                        page_field_mappings: page_field_mappings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ConfluencePageToIndexFieldMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencepagetoindexfieldmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfluencePageToIndexFieldMapping {
        /// Property [`DataSourceFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencepagetoindexfieldmapping.html#cfn-kendra-datasource-confluencepagetoindexfieldmapping-datasourcefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source_field_name: ::Value<String>,
        /// Property [`DateFieldFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencepagetoindexfieldmapping.html#cfn-kendra-datasource-confluencepagetoindexfieldmapping-datefieldformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_field_format: Option<::Value<String>>,
        /// Property [`IndexFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencepagetoindexfieldmapping.html#cfn-kendra-datasource-confluencepagetoindexfieldmapping-indexfieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_field_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ConfluencePageToIndexFieldMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceFieldName", &self.data_source_field_name)?;
            if let Some(ref date_field_format) = self.date_field_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateFieldFormat", date_field_format)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexFieldName", &self.index_field_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfluencePageToIndexFieldMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfluencePageToIndexFieldMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfluencePageToIndexFieldMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfluencePageToIndexFieldMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_source_field_name: Option<::Value<String>> = None;
                    let mut date_field_format: Option<::Value<String>> = None;
                    let mut index_field_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSourceFieldName" => {
                                data_source_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DateFieldFormat" => {
                                date_field_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexFieldName" => {
                                index_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfluencePageToIndexFieldMapping {
                        data_source_field_name: data_source_field_name.ok_or(::serde::de::Error::missing_field("DataSourceFieldName"))?,
                        date_field_format: date_field_format,
                        index_field_name: index_field_name.ok_or(::serde::de::Error::missing_field("IndexFieldName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ConfluenceSpaceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespaceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfluenceSpaceConfiguration {
        /// Property [`CrawlArchivedSpaces`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespaceconfiguration.html#cfn-kendra-datasource-confluencespaceconfiguration-crawlarchivedspaces).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawl_archived_spaces: Option<::Value<bool>>,
        /// Property [`CrawlPersonalSpaces`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespaceconfiguration.html#cfn-kendra-datasource-confluencespaceconfiguration-crawlpersonalspaces).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawl_personal_spaces: Option<::Value<bool>>,
        /// Property [`ExcludeSpaces`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespaceconfiguration.html#cfn-kendra-datasource-confluencespaceconfiguration-excludespaces).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_spaces: Option<::ValueList<String>>,
        /// Property [`IncludeSpaces`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespaceconfiguration.html#cfn-kendra-datasource-confluencespaceconfiguration-includespaces).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_spaces: Option<::ValueList<String>>,
        /// Property [`SpaceFieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespaceconfiguration.html#cfn-kendra-datasource-confluencespaceconfiguration-spacefieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub space_field_mappings: Option<::ValueList<ConfluenceSpaceToIndexFieldMapping>>,
    }

    impl ::codec::SerializeValue for ConfluenceSpaceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref crawl_archived_spaces) = self.crawl_archived_spaces {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlArchivedSpaces", crawl_archived_spaces)?;
            }
            if let Some(ref crawl_personal_spaces) = self.crawl_personal_spaces {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlPersonalSpaces", crawl_personal_spaces)?;
            }
            if let Some(ref exclude_spaces) = self.exclude_spaces {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeSpaces", exclude_spaces)?;
            }
            if let Some(ref include_spaces) = self.include_spaces {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeSpaces", include_spaces)?;
            }
            if let Some(ref space_field_mappings) = self.space_field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpaceFieldMappings", space_field_mappings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfluenceSpaceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfluenceSpaceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfluenceSpaceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfluenceSpaceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut crawl_archived_spaces: Option<::Value<bool>> = None;
                    let mut crawl_personal_spaces: Option<::Value<bool>> = None;
                    let mut exclude_spaces: Option<::ValueList<String>> = None;
                    let mut include_spaces: Option<::ValueList<String>> = None;
                    let mut space_field_mappings: Option<::ValueList<ConfluenceSpaceToIndexFieldMapping>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CrawlArchivedSpaces" => {
                                crawl_archived_spaces = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CrawlPersonalSpaces" => {
                                crawl_personal_spaces = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeSpaces" => {
                                exclude_spaces = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeSpaces" => {
                                include_spaces = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpaceFieldMappings" => {
                                space_field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfluenceSpaceConfiguration {
                        crawl_archived_spaces: crawl_archived_spaces,
                        crawl_personal_spaces: crawl_personal_spaces,
                        exclude_spaces: exclude_spaces,
                        include_spaces: include_spaces,
                        space_field_mappings: space_field_mappings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ConfluenceSpaceToIndexFieldMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespacetoindexfieldmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfluenceSpaceToIndexFieldMapping {
        /// Property [`DataSourceFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespacetoindexfieldmapping.html#cfn-kendra-datasource-confluencespacetoindexfieldmapping-datasourcefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source_field_name: ::Value<String>,
        /// Property [`DateFieldFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespacetoindexfieldmapping.html#cfn-kendra-datasource-confluencespacetoindexfieldmapping-datefieldformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_field_format: Option<::Value<String>>,
        /// Property [`IndexFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-confluencespacetoindexfieldmapping.html#cfn-kendra-datasource-confluencespacetoindexfieldmapping-indexfieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_field_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ConfluenceSpaceToIndexFieldMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceFieldName", &self.data_source_field_name)?;
            if let Some(ref date_field_format) = self.date_field_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateFieldFormat", date_field_format)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexFieldName", &self.index_field_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfluenceSpaceToIndexFieldMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfluenceSpaceToIndexFieldMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfluenceSpaceToIndexFieldMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfluenceSpaceToIndexFieldMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_source_field_name: Option<::Value<String>> = None;
                    let mut date_field_format: Option<::Value<String>> = None;
                    let mut index_field_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSourceFieldName" => {
                                data_source_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DateFieldFormat" => {
                                date_field_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexFieldName" => {
                                index_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfluenceSpaceToIndexFieldMapping {
                        data_source_field_name: data_source_field_name.ok_or(::serde::de::Error::missing_field("DataSourceFieldName"))?,
                        date_field_format: date_field_format,
                        index_field_name: index_field_name.ok_or(::serde::de::Error::missing_field("IndexFieldName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ConnectionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-connectionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectionConfiguration {
        /// Property [`DatabaseHost`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-connectionconfiguration.html#cfn-kendra-datasource-connectionconfiguration-databasehost).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_host: ::Value<String>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-connectionconfiguration.html#cfn-kendra-datasource-connectionconfiguration-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`DatabasePort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-connectionconfiguration.html#cfn-kendra-datasource-connectionconfiguration-databaseport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_port: ::Value<u32>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-connectionconfiguration.html#cfn-kendra-datasource-connectionconfiguration-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: ::Value<String>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-connectionconfiguration.html#cfn-kendra-datasource-connectionconfiguration-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ConnectionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseHost", &self.database_host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabasePort", &self.database_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", &self.secret_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database_host: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut database_port: Option<::Value<u32>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatabaseHost" => {
                                database_host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabasePort" => {
                                database_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionConfiguration {
                        database_host: database_host.ok_or(::serde::de::Error::missing_field("DatabaseHost"))?,
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        database_port: database_port.ok_or(::serde::de::Error::missing_field("DatabasePort"))?,
                        secret_arn: secret_arn.ok_or(::serde::de::Error::missing_field("SecretArn"))?,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.CustomDocumentEnrichmentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-customdocumentenrichmentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomDocumentEnrichmentConfiguration {
        /// Property [`InlineConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-customdocumentenrichmentconfiguration.html#cfn-kendra-datasource-customdocumentenrichmentconfiguration-inlineconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inline_configurations: Option<::ValueList<InlineCustomDocumentEnrichmentConfiguration>>,
        /// Property [`PostExtractionHookConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-customdocumentenrichmentconfiguration.html#cfn-kendra-datasource-customdocumentenrichmentconfiguration-postextractionhookconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub post_extraction_hook_configuration: Option<::Value<HookConfiguration>>,
        /// Property [`PreExtractionHookConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-customdocumentenrichmentconfiguration.html#cfn-kendra-datasource-customdocumentenrichmentconfiguration-preextractionhookconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pre_extraction_hook_configuration: Option<::Value<HookConfiguration>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-customdocumentenrichmentconfiguration.html#cfn-kendra-datasource-customdocumentenrichmentconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CustomDocumentEnrichmentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref inline_configurations) = self.inline_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InlineConfigurations", inline_configurations)?;
            }
            if let Some(ref post_extraction_hook_configuration) = self.post_extraction_hook_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostExtractionHookConfiguration", post_extraction_hook_configuration)?;
            }
            if let Some(ref pre_extraction_hook_configuration) = self.pre_extraction_hook_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreExtractionHookConfiguration", pre_extraction_hook_configuration)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomDocumentEnrichmentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomDocumentEnrichmentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomDocumentEnrichmentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomDocumentEnrichmentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut inline_configurations: Option<::ValueList<InlineCustomDocumentEnrichmentConfiguration>> = None;
                    let mut post_extraction_hook_configuration: Option<::Value<HookConfiguration>> = None;
                    let mut pre_extraction_hook_configuration: Option<::Value<HookConfiguration>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InlineConfigurations" => {
                                inline_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostExtractionHookConfiguration" => {
                                post_extraction_hook_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreExtractionHookConfiguration" => {
                                pre_extraction_hook_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomDocumentEnrichmentConfiguration {
                        inline_configurations: inline_configurations,
                        post_extraction_hook_configuration: post_extraction_hook_configuration,
                        pre_extraction_hook_configuration: pre_extraction_hook_configuration,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.DataSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSourceConfiguration {
        /// Property [`ConfluenceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html#cfn-kendra-datasource-datasourceconfiguration-confluenceconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub confluence_configuration: Option<::Value<ConfluenceConfiguration>>,
        /// Property [`DatabaseConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html#cfn-kendra-datasource-datasourceconfiguration-databaseconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_configuration: Option<::Value<DatabaseConfiguration>>,
        /// Property [`GoogleDriveConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html#cfn-kendra-datasource-datasourceconfiguration-googledriveconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub google_drive_configuration: Option<::Value<GoogleDriveConfiguration>>,
        /// Property [`OneDriveConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html#cfn-kendra-datasource-datasourceconfiguration-onedriveconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub one_drive_configuration: Option<::Value<OneDriveConfiguration>>,
        /// Property [`S3Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html#cfn-kendra-datasource-datasourceconfiguration-s3configuration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_configuration: Option<::Value<S3DataSourceConfiguration>>,
        /// Property [`SalesforceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html#cfn-kendra-datasource-datasourceconfiguration-salesforceconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salesforce_configuration: Option<::Value<SalesforceConfiguration>>,
        /// Property [`ServiceNowConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html#cfn-kendra-datasource-datasourceconfiguration-servicenowconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_now_configuration: Option<::Value<ServiceNowConfiguration>>,
        /// Property [`SharePointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html#cfn-kendra-datasource-datasourceconfiguration-sharepointconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub share_point_configuration: Option<::Value<SharePointConfiguration>>,
        /// Property [`WebCrawlerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html#cfn-kendra-datasource-datasourceconfiguration-webcrawlerconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub web_crawler_configuration: Option<::Value<WebCrawlerConfiguration>>,
        /// Property [`WorkDocsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourceconfiguration.html#cfn-kendra-datasource-datasourceconfiguration-workdocsconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub work_docs_configuration: Option<::Value<WorkDocsConfiguration>>,
    }

    impl ::codec::SerializeValue for DataSourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref confluence_configuration) = self.confluence_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfluenceConfiguration", confluence_configuration)?;
            }
            if let Some(ref database_configuration) = self.database_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseConfiguration", database_configuration)?;
            }
            if let Some(ref google_drive_configuration) = self.google_drive_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GoogleDriveConfiguration", google_drive_configuration)?;
            }
            if let Some(ref one_drive_configuration) = self.one_drive_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OneDriveConfiguration", one_drive_configuration)?;
            }
            if let Some(ref s3_configuration) = self.s3_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Configuration", s3_configuration)?;
            }
            if let Some(ref salesforce_configuration) = self.salesforce_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SalesforceConfiguration", salesforce_configuration)?;
            }
            if let Some(ref service_now_configuration) = self.service_now_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNowConfiguration", service_now_configuration)?;
            }
            if let Some(ref share_point_configuration) = self.share_point_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SharePointConfiguration", share_point_configuration)?;
            }
            if let Some(ref web_crawler_configuration) = self.web_crawler_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebCrawlerConfiguration", web_crawler_configuration)?;
            }
            if let Some(ref work_docs_configuration) = self.work_docs_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkDocsConfiguration", work_docs_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut confluence_configuration: Option<::Value<ConfluenceConfiguration>> = None;
                    let mut database_configuration: Option<::Value<DatabaseConfiguration>> = None;
                    let mut google_drive_configuration: Option<::Value<GoogleDriveConfiguration>> = None;
                    let mut one_drive_configuration: Option<::Value<OneDriveConfiguration>> = None;
                    let mut s3_configuration: Option<::Value<S3DataSourceConfiguration>> = None;
                    let mut salesforce_configuration: Option<::Value<SalesforceConfiguration>> = None;
                    let mut service_now_configuration: Option<::Value<ServiceNowConfiguration>> = None;
                    let mut share_point_configuration: Option<::Value<SharePointConfiguration>> = None;
                    let mut web_crawler_configuration: Option<::Value<WebCrawlerConfiguration>> = None;
                    let mut work_docs_configuration: Option<::Value<WorkDocsConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfluenceConfiguration" => {
                                confluence_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseConfiguration" => {
                                database_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GoogleDriveConfiguration" => {
                                google_drive_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OneDriveConfiguration" => {
                                one_drive_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Configuration" => {
                                s3_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SalesforceConfiguration" => {
                                salesforce_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNowConfiguration" => {
                                service_now_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SharePointConfiguration" => {
                                share_point_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WebCrawlerConfiguration" => {
                                web_crawler_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkDocsConfiguration" => {
                                work_docs_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSourceConfiguration {
                        confluence_configuration: confluence_configuration,
                        database_configuration: database_configuration,
                        google_drive_configuration: google_drive_configuration,
                        one_drive_configuration: one_drive_configuration,
                        s3_configuration: s3_configuration,
                        salesforce_configuration: salesforce_configuration,
                        service_now_configuration: service_now_configuration,
                        share_point_configuration: share_point_configuration,
                        web_crawler_configuration: web_crawler_configuration,
                        work_docs_configuration: work_docs_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.DataSourceToIndexFieldMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourcetoindexfieldmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSourceToIndexFieldMapping {
        /// Property [`DataSourceFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourcetoindexfieldmapping.html#cfn-kendra-datasource-datasourcetoindexfieldmapping-datasourcefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source_field_name: ::Value<String>,
        /// Property [`DateFieldFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourcetoindexfieldmapping.html#cfn-kendra-datasource-datasourcetoindexfieldmapping-datefieldformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_field_format: Option<::Value<String>>,
        /// Property [`IndexFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourcetoindexfieldmapping.html#cfn-kendra-datasource-datasourcetoindexfieldmapping-indexfieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_field_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataSourceToIndexFieldMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceFieldName", &self.data_source_field_name)?;
            if let Some(ref date_field_format) = self.date_field_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateFieldFormat", date_field_format)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexFieldName", &self.index_field_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSourceToIndexFieldMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceToIndexFieldMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSourceToIndexFieldMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSourceToIndexFieldMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_source_field_name: Option<::Value<String>> = None;
                    let mut date_field_format: Option<::Value<String>> = None;
                    let mut index_field_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSourceFieldName" => {
                                data_source_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DateFieldFormat" => {
                                date_field_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexFieldName" => {
                                index_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSourceToIndexFieldMapping {
                        data_source_field_name: data_source_field_name.ok_or(::serde::de::Error::missing_field("DataSourceFieldName"))?,
                        date_field_format: date_field_format,
                        index_field_name: index_field_name.ok_or(::serde::de::Error::missing_field("IndexFieldName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.DataSourceVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourcevpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSourceVpcConfiguration {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourcevpcconfiguration.html#cfn-kendra-datasource-datasourcevpcconfiguration-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-datasourcevpcconfiguration.html#cfn-kendra-datasource-datasourcevpcconfiguration-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for DataSourceVpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSourceVpcConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceVpcConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSourceVpcConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSourceVpcConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSourceVpcConfiguration {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.DatabaseConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-databaseconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DatabaseConfiguration {
        /// Property [`AclConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-databaseconfiguration.html#cfn-kendra-datasource-databaseconfiguration-aclconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acl_configuration: Option<::Value<AclConfiguration>>,
        /// Property [`ColumnConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-databaseconfiguration.html#cfn-kendra-datasource-databaseconfiguration-columnconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_configuration: ::Value<ColumnConfiguration>,
        /// Property [`ConnectionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-databaseconfiguration.html#cfn-kendra-datasource-databaseconfiguration-connectionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_configuration: ::Value<ConnectionConfiguration>,
        /// Property [`DatabaseEngineType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-databaseconfiguration.html#cfn-kendra-datasource-databaseconfiguration-databaseenginetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_engine_type: ::Value<String>,
        /// Property [`SqlConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-databaseconfiguration.html#cfn-kendra-datasource-databaseconfiguration-sqlconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sql_configuration: Option<::Value<SqlConfiguration>>,
        /// Property [`VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-databaseconfiguration.html#cfn-kendra-datasource-databaseconfiguration-vpcconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_configuration: Option<::Value<DataSourceVpcConfiguration>>,
    }

    impl ::codec::SerializeValue for DatabaseConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref acl_configuration) = self.acl_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AclConfiguration", acl_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnConfiguration", &self.column_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionConfiguration", &self.connection_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseEngineType", &self.database_engine_type)?;
            if let Some(ref sql_configuration) = self.sql_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqlConfiguration", sql_configuration)?;
            }
            if let Some(ref vpc_configuration) = self.vpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfiguration", vpc_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatabaseConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatabaseConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatabaseConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acl_configuration: Option<::Value<AclConfiguration>> = None;
                    let mut column_configuration: Option<::Value<ColumnConfiguration>> = None;
                    let mut connection_configuration: Option<::Value<ConnectionConfiguration>> = None;
                    let mut database_engine_type: Option<::Value<String>> = None;
                    let mut sql_configuration: Option<::Value<SqlConfiguration>> = None;
                    let mut vpc_configuration: Option<::Value<DataSourceVpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AclConfiguration" => {
                                acl_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnConfiguration" => {
                                column_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectionConfiguration" => {
                                connection_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseEngineType" => {
                                database_engine_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqlConfiguration" => {
                                sql_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfiguration" => {
                                vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatabaseConfiguration {
                        acl_configuration: acl_configuration,
                        column_configuration: column_configuration.ok_or(::serde::de::Error::missing_field("ColumnConfiguration"))?,
                        connection_configuration: connection_configuration.ok_or(::serde::de::Error::missing_field("ConnectionConfiguration"))?,
                        database_engine_type: database_engine_type.ok_or(::serde::de::Error::missing_field("DatabaseEngineType"))?,
                        sql_configuration: sql_configuration,
                        vpc_configuration: vpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.DocumentAttributeCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributecondition.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentAttributeCondition {
        /// Property [`ConditionDocumentAttributeKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributecondition.html#cfn-kendra-datasource-documentattributecondition-conditiondocumentattributekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition_document_attribute_key: ::Value<String>,
        /// Property [`ConditionOnValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributecondition.html#cfn-kendra-datasource-documentattributecondition-conditiononvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition_on_value: Option<::Value<DocumentAttributeValue>>,
        /// Property [`Operator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributecondition.html#cfn-kendra-datasource-documentattributecondition-operator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operator: ::Value<String>,
    }

    impl ::codec::SerializeValue for DocumentAttributeCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionDocumentAttributeKey", &self.condition_document_attribute_key)?;
            if let Some(ref condition_on_value) = self.condition_on_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionOnValue", condition_on_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Operator", &self.operator)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentAttributeCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentAttributeCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentAttributeCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentAttributeCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition_document_attribute_key: Option<::Value<String>> = None;
                    let mut condition_on_value: Option<::Value<DocumentAttributeValue>> = None;
                    let mut operator: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConditionDocumentAttributeKey" => {
                                condition_document_attribute_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConditionOnValue" => {
                                condition_on_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Operator" => {
                                operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentAttributeCondition {
                        condition_document_attribute_key: condition_document_attribute_key.ok_or(::serde::de::Error::missing_field("ConditionDocumentAttributeKey"))?,
                        condition_on_value: condition_on_value,
                        operator: operator.ok_or(::serde::de::Error::missing_field("Operator"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.DocumentAttributeTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributetarget.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentAttributeTarget {
        /// Property [`TargetDocumentAttributeKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributetarget.html#cfn-kendra-datasource-documentattributetarget-targetdocumentattributekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_document_attribute_key: ::Value<String>,
        /// Property [`TargetDocumentAttributeValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributetarget.html#cfn-kendra-datasource-documentattributetarget-targetdocumentattributevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_document_attribute_value: Option<::Value<DocumentAttributeValue>>,
        /// Property [`TargetDocumentAttributeValueDeletion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributetarget.html#cfn-kendra-datasource-documentattributetarget-targetdocumentattributevaluedeletion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_document_attribute_value_deletion: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for DocumentAttributeTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetDocumentAttributeKey", &self.target_document_attribute_key)?;
            if let Some(ref target_document_attribute_value) = self.target_document_attribute_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetDocumentAttributeValue", target_document_attribute_value)?;
            }
            if let Some(ref target_document_attribute_value_deletion) = self.target_document_attribute_value_deletion {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetDocumentAttributeValueDeletion", target_document_attribute_value_deletion)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentAttributeTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentAttributeTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentAttributeTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentAttributeTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_document_attribute_key: Option<::Value<String>> = None;
                    let mut target_document_attribute_value: Option<::Value<DocumentAttributeValue>> = None;
                    let mut target_document_attribute_value_deletion: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetDocumentAttributeKey" => {
                                target_document_attribute_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetDocumentAttributeValue" => {
                                target_document_attribute_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetDocumentAttributeValueDeletion" => {
                                target_document_attribute_value_deletion = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentAttributeTarget {
                        target_document_attribute_key: target_document_attribute_key.ok_or(::serde::de::Error::missing_field("TargetDocumentAttributeKey"))?,
                        target_document_attribute_value: target_document_attribute_value,
                        target_document_attribute_value_deletion: target_document_attribute_value_deletion,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.DocumentAttributeValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributevalue.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentAttributeValue {
        /// Property [`DateValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributevalue.html#cfn-kendra-datasource-documentattributevalue-datevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_value: Option<::Value<String>>,
        /// Property [`LongValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributevalue.html#cfn-kendra-datasource-documentattributevalue-longvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub long_value: Option<::Value<u32>>,
        /// Property [`StringListValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributevalue.html#cfn-kendra-datasource-documentattributevalue-stringlistvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_list_value: Option<::ValueList<String>>,
        /// Property [`StringValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentattributevalue.html#cfn-kendra-datasource-documentattributevalue-stringvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DocumentAttributeValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref date_value) = self.date_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateValue", date_value)?;
            }
            if let Some(ref long_value) = self.long_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LongValue", long_value)?;
            }
            if let Some(ref string_list_value) = self.string_list_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringListValue", string_list_value)?;
            }
            if let Some(ref string_value) = self.string_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", string_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentAttributeValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentAttributeValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentAttributeValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentAttributeValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut date_value: Option<::Value<String>> = None;
                    let mut long_value: Option<::Value<u32>> = None;
                    let mut string_list_value: Option<::ValueList<String>> = None;
                    let mut string_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DateValue" => {
                                date_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LongValue" => {
                                long_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringListValue" => {
                                string_list_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentAttributeValue {
                        date_value: date_value,
                        long_value: long_value,
                        string_list_value: string_list_value,
                        string_value: string_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.DocumentsMetadataConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentsmetadataconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentsMetadataConfiguration {
        /// Property [`S3Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-documentsmetadataconfiguration.html#cfn-kendra-datasource-documentsmetadataconfiguration-s3prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DocumentsMetadataConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_prefix) = self.s3_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Prefix", s3_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentsMetadataConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentsMetadataConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentsMetadataConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentsMetadataConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Prefix" => {
                                s3_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentsMetadataConfiguration {
                        s3_prefix: s3_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.GoogleDriveConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-googledriveconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct GoogleDriveConfiguration {
        /// Property [`ExcludeMimeTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-googledriveconfiguration.html#cfn-kendra-datasource-googledriveconfiguration-excludemimetypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_mime_types: Option<::ValueList<String>>,
        /// Property [`ExcludeSharedDrives`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-googledriveconfiguration.html#cfn-kendra-datasource-googledriveconfiguration-excludeshareddrives).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_shared_drives: Option<::ValueList<String>>,
        /// Property [`ExcludeUserAccounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-googledriveconfiguration.html#cfn-kendra-datasource-googledriveconfiguration-excludeuseraccounts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_user_accounts: Option<::ValueList<String>>,
        /// Property [`ExclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-googledriveconfiguration.html#cfn-kendra-datasource-googledriveconfiguration-exclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclusion_patterns: Option<::ValueList<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-googledriveconfiguration.html#cfn-kendra-datasource-googledriveconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
        /// Property [`InclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-googledriveconfiguration.html#cfn-kendra-datasource-googledriveconfiguration-inclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inclusion_patterns: Option<::ValueList<String>>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-googledriveconfiguration.html#cfn-kendra-datasource-googledriveconfiguration-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for GoogleDriveConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exclude_mime_types) = self.exclude_mime_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeMimeTypes", exclude_mime_types)?;
            }
            if let Some(ref exclude_shared_drives) = self.exclude_shared_drives {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeSharedDrives", exclude_shared_drives)?;
            }
            if let Some(ref exclude_user_accounts) = self.exclude_user_accounts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeUserAccounts", exclude_user_accounts)?;
            }
            if let Some(ref exclusion_patterns) = self.exclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExclusionPatterns", exclusion_patterns)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            if let Some(ref inclusion_patterns) = self.inclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InclusionPatterns", inclusion_patterns)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", &self.secret_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GoogleDriveConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GoogleDriveConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GoogleDriveConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GoogleDriveConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exclude_mime_types: Option<::ValueList<String>> = None;
                    let mut exclude_shared_drives: Option<::ValueList<String>> = None;
                    let mut exclude_user_accounts: Option<::ValueList<String>> = None;
                    let mut exclusion_patterns: Option<::ValueList<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;
                    let mut inclusion_patterns: Option<::ValueList<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExcludeMimeTypes" => {
                                exclude_mime_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeSharedDrives" => {
                                exclude_shared_drives = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeUserAccounts" => {
                                exclude_user_accounts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExclusionPatterns" => {
                                exclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InclusionPatterns" => {
                                inclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GoogleDriveConfiguration {
                        exclude_mime_types: exclude_mime_types,
                        exclude_shared_drives: exclude_shared_drives,
                        exclude_user_accounts: exclude_user_accounts,
                        exclusion_patterns: exclusion_patterns,
                        field_mappings: field_mappings,
                        inclusion_patterns: inclusion_patterns,
                        secret_arn: secret_arn.ok_or(::serde::de::Error::missing_field("SecretArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.HookConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-hookconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct HookConfiguration {
        /// Property [`InvocationCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-hookconfiguration.html#cfn-kendra-datasource-hookconfiguration-invocationcondition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invocation_condition: Option<::Value<DocumentAttributeCondition>>,
        /// Property [`LambdaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-hookconfiguration.html#cfn-kendra-datasource-hookconfiguration-lambdaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_arn: ::Value<String>,
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-hookconfiguration.html#cfn-kendra-datasource-hookconfiguration-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: ::Value<String>,
    }

    impl ::codec::SerializeValue for HookConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invocation_condition) = self.invocation_condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationCondition", invocation_condition)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaArn", &self.lambda_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HookConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HookConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HookConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HookConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invocation_condition: Option<::Value<DocumentAttributeCondition>> = None;
                    let mut lambda_arn: Option<::Value<String>> = None;
                    let mut s3_bucket: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InvocationCondition" => {
                                invocation_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaArn" => {
                                lambda_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HookConfiguration {
                        invocation_condition: invocation_condition,
                        lambda_arn: lambda_arn.ok_or(::serde::de::Error::missing_field("LambdaArn"))?,
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.InlineCustomDocumentEnrichmentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-inlinecustomdocumentenrichmentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct InlineCustomDocumentEnrichmentConfiguration {
        /// Property [`Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-inlinecustomdocumentenrichmentconfiguration.html#cfn-kendra-datasource-inlinecustomdocumentenrichmentconfiguration-condition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition: Option<::Value<DocumentAttributeCondition>>,
        /// Property [`DocumentContentDeletion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-inlinecustomdocumentenrichmentconfiguration.html#cfn-kendra-datasource-inlinecustomdocumentenrichmentconfiguration-documentcontentdeletion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_content_deletion: Option<::Value<bool>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-inlinecustomdocumentenrichmentconfiguration.html#cfn-kendra-datasource-inlinecustomdocumentenrichmentconfiguration-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: Option<::Value<DocumentAttributeTarget>>,
    }

    impl ::codec::SerializeValue for InlineCustomDocumentEnrichmentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref condition) = self.condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Condition", condition)?;
            }
            if let Some(ref document_content_deletion) = self.document_content_deletion {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentContentDeletion", document_content_deletion)?;
            }
            if let Some(ref target) = self.target {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", target)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InlineCustomDocumentEnrichmentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InlineCustomDocumentEnrichmentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InlineCustomDocumentEnrichmentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InlineCustomDocumentEnrichmentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition: Option<::Value<DocumentAttributeCondition>> = None;
                    let mut document_content_deletion: Option<::Value<bool>> = None;
                    let mut target: Option<::Value<DocumentAttributeTarget>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Condition" => {
                                condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentContentDeletion" => {
                                document_content_deletion = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InlineCustomDocumentEnrichmentConfiguration {
                        condition: condition,
                        document_content_deletion: document_content_deletion,
                        target: target,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.OneDriveConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct OneDriveConfiguration {
        /// Property [`DisableLocalGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveconfiguration.html#cfn-kendra-datasource-onedriveconfiguration-disablelocalgroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_local_groups: Option<::Value<bool>>,
        /// Property [`ExclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveconfiguration.html#cfn-kendra-datasource-onedriveconfiguration-exclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclusion_patterns: Option<::ValueList<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveconfiguration.html#cfn-kendra-datasource-onedriveconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
        /// Property [`InclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveconfiguration.html#cfn-kendra-datasource-onedriveconfiguration-inclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inclusion_patterns: Option<::ValueList<String>>,
        /// Property [`OneDriveUsers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveconfiguration.html#cfn-kendra-datasource-onedriveconfiguration-onedriveusers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub one_drive_users: ::Value<OneDriveUsers>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveconfiguration.html#cfn-kendra-datasource-onedriveconfiguration-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: ::Value<String>,
        /// Property [`TenantDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveconfiguration.html#cfn-kendra-datasource-onedriveconfiguration-tenantdomain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tenant_domain: ::Value<String>,
    }

    impl ::codec::SerializeValue for OneDriveConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref disable_local_groups) = self.disable_local_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableLocalGroups", disable_local_groups)?;
            }
            if let Some(ref exclusion_patterns) = self.exclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExclusionPatterns", exclusion_patterns)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            if let Some(ref inclusion_patterns) = self.inclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InclusionPatterns", inclusion_patterns)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OneDriveUsers", &self.one_drive_users)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", &self.secret_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TenantDomain", &self.tenant_domain)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OneDriveConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OneDriveConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OneDriveConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OneDriveConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut disable_local_groups: Option<::Value<bool>> = None;
                    let mut exclusion_patterns: Option<::ValueList<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;
                    let mut inclusion_patterns: Option<::ValueList<String>> = None;
                    let mut one_drive_users: Option<::Value<OneDriveUsers>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;
                    let mut tenant_domain: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DisableLocalGroups" => {
                                disable_local_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExclusionPatterns" => {
                                exclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InclusionPatterns" => {
                                inclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OneDriveUsers" => {
                                one_drive_users = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TenantDomain" => {
                                tenant_domain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OneDriveConfiguration {
                        disable_local_groups: disable_local_groups,
                        exclusion_patterns: exclusion_patterns,
                        field_mappings: field_mappings,
                        inclusion_patterns: inclusion_patterns,
                        one_drive_users: one_drive_users.ok_or(::serde::de::Error::missing_field("OneDriveUsers"))?,
                        secret_arn: secret_arn.ok_or(::serde::de::Error::missing_field("SecretArn"))?,
                        tenant_domain: tenant_domain.ok_or(::serde::de::Error::missing_field("TenantDomain"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.OneDriveUsers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveusers.html) property type.
    #[derive(Debug, Default)]
    pub struct OneDriveUsers {
        /// Property [`OneDriveUserList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveusers.html#cfn-kendra-datasource-onedriveusers-onedriveuserlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub one_drive_user_list: Option<::ValueList<String>>,
        /// Property [`OneDriveUserS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-onedriveusers.html#cfn-kendra-datasource-onedriveusers-onedriveusers3path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub one_drive_user_s3_path: Option<::Value<S3Path>>,
    }

    impl ::codec::SerializeValue for OneDriveUsers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref one_drive_user_list) = self.one_drive_user_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OneDriveUserList", one_drive_user_list)?;
            }
            if let Some(ref one_drive_user_s3_path) = self.one_drive_user_s3_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OneDriveUserS3Path", one_drive_user_s3_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OneDriveUsers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OneDriveUsers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OneDriveUsers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OneDriveUsers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut one_drive_user_list: Option<::ValueList<String>> = None;
                    let mut one_drive_user_s3_path: Option<::Value<S3Path>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OneDriveUserList" => {
                                one_drive_user_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OneDriveUserS3Path" => {
                                one_drive_user_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OneDriveUsers {
                        one_drive_user_list: one_drive_user_list,
                        one_drive_user_s3_path: one_drive_user_s3_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ProxyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-proxyconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ProxyConfiguration {
        /// Property [`Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-proxyconfiguration.html#cfn-kendra-datasource-proxyconfiguration-credentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credentials: Option<::Value<String>>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-proxyconfiguration.html#cfn-kendra-datasource-proxyconfiguration-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-proxyconfiguration.html#cfn-kendra-datasource-proxyconfiguration-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ProxyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref credentials) = self.credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credentials", credentials)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProxyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProxyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProxyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProxyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut credentials: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Credentials" => {
                                credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProxyConfiguration {
                        credentials: credentials,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.S3DataSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3datasourceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct S3DataSourceConfiguration {
        /// Property [`AccessControlListConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3datasourceconfiguration.html#cfn-kendra-datasource-s3datasourceconfiguration-accesscontrollistconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_control_list_configuration: Option<::Value<AccessControlListConfiguration>>,
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3datasourceconfiguration.html#cfn-kendra-datasource-s3datasourceconfiguration-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`DocumentsMetadataConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3datasourceconfiguration.html#cfn-kendra-datasource-s3datasourceconfiguration-documentsmetadataconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub documents_metadata_configuration: Option<::Value<DocumentsMetadataConfiguration>>,
        /// Property [`ExclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3datasourceconfiguration.html#cfn-kendra-datasource-s3datasourceconfiguration-exclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclusion_patterns: Option<::ValueList<String>>,
        /// Property [`InclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3datasourceconfiguration.html#cfn-kendra-datasource-s3datasourceconfiguration-inclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inclusion_patterns: Option<::ValueList<String>>,
        /// Property [`InclusionPrefixes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3datasourceconfiguration.html#cfn-kendra-datasource-s3datasourceconfiguration-inclusionprefixes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inclusion_prefixes: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for S3DataSourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_control_list_configuration) = self.access_control_list_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControlListConfiguration", access_control_list_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref documents_metadata_configuration) = self.documents_metadata_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentsMetadataConfiguration", documents_metadata_configuration)?;
            }
            if let Some(ref exclusion_patterns) = self.exclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExclusionPatterns", exclusion_patterns)?;
            }
            if let Some(ref inclusion_patterns) = self.inclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InclusionPatterns", inclusion_patterns)?;
            }
            if let Some(ref inclusion_prefixes) = self.inclusion_prefixes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InclusionPrefixes", inclusion_prefixes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3DataSourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3DataSourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3DataSourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3DataSourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_control_list_configuration: Option<::Value<AccessControlListConfiguration>> = None;
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut documents_metadata_configuration: Option<::Value<DocumentsMetadataConfiguration>> = None;
                    let mut exclusion_patterns: Option<::ValueList<String>> = None;
                    let mut inclusion_patterns: Option<::ValueList<String>> = None;
                    let mut inclusion_prefixes: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessControlListConfiguration" => {
                                access_control_list_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentsMetadataConfiguration" => {
                                documents_metadata_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExclusionPatterns" => {
                                exclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InclusionPatterns" => {
                                inclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InclusionPrefixes" => {
                                inclusion_prefixes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3DataSourceConfiguration {
                        access_control_list_configuration: access_control_list_configuration,
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        documents_metadata_configuration: documents_metadata_configuration,
                        exclusion_patterns: exclusion_patterns,
                        inclusion_patterns: inclusion_patterns,
                        inclusion_prefixes: inclusion_prefixes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.S3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3path.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Path {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3path.html#cfn-kendra-datasource-s3path-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-s3path.html#cfn-kendra-datasource-s3path-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Path {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Path {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Path, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Path;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Path")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Path {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.SalesforceChatterFeedConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcechatterfeedconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceChatterFeedConfiguration {
        /// Property [`DocumentDataFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcechatterfeedconfiguration.html#cfn-kendra-datasource-salesforcechatterfeedconfiguration-documentdatafieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_data_field_name: ::Value<String>,
        /// Property [`DocumentTitleFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcechatterfeedconfiguration.html#cfn-kendra-datasource-salesforcechatterfeedconfiguration-documenttitlefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_title_field_name: Option<::Value<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcechatterfeedconfiguration.html#cfn-kendra-datasource-salesforcechatterfeedconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
        /// Property [`IncludeFilterTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcechatterfeedconfiguration.html#cfn-kendra-datasource-salesforcechatterfeedconfiguration-includefiltertypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_filter_types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SalesforceChatterFeedConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentDataFieldName", &self.document_data_field_name)?;
            if let Some(ref document_title_field_name) = self.document_title_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentTitleFieldName", document_title_field_name)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            if let Some(ref include_filter_types) = self.include_filter_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeFilterTypes", include_filter_types)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceChatterFeedConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceChatterFeedConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceChatterFeedConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceChatterFeedConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_data_field_name: Option<::Value<String>> = None;
                    let mut document_title_field_name: Option<::Value<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;
                    let mut include_filter_types: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentDataFieldName" => {
                                document_data_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentTitleFieldName" => {
                                document_title_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeFilterTypes" => {
                                include_filter_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceChatterFeedConfiguration {
                        document_data_field_name: document_data_field_name.ok_or(::serde::de::Error::missing_field("DocumentDataFieldName"))?,
                        document_title_field_name: document_title_field_name,
                        field_mappings: field_mappings,
                        include_filter_types: include_filter_types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.SalesforceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceConfiguration {
        /// Property [`ChatterFeedConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html#cfn-kendra-datasource-salesforceconfiguration-chatterfeedconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub chatter_feed_configuration: Option<::Value<SalesforceChatterFeedConfiguration>>,
        /// Property [`CrawlAttachments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html#cfn-kendra-datasource-salesforceconfiguration-crawlattachments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawl_attachments: Option<::Value<bool>>,
        /// Property [`ExcludeAttachmentFilePatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html#cfn-kendra-datasource-salesforceconfiguration-excludeattachmentfilepatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_attachment_file_patterns: Option<::ValueList<String>>,
        /// Property [`IncludeAttachmentFilePatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html#cfn-kendra-datasource-salesforceconfiguration-includeattachmentfilepatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_attachment_file_patterns: Option<::ValueList<String>>,
        /// Property [`KnowledgeArticleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html#cfn-kendra-datasource-salesforceconfiguration-knowledgearticleconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub knowledge_article_configuration: Option<::Value<SalesforceKnowledgeArticleConfiguration>>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html#cfn-kendra-datasource-salesforceconfiguration-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: ::Value<String>,
        /// Property [`ServerUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html#cfn-kendra-datasource-salesforceconfiguration-serverurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_url: ::Value<String>,
        /// Property [`StandardObjectAttachmentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html#cfn-kendra-datasource-salesforceconfiguration-standardobjectattachmentconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub standard_object_attachment_configuration: Option<::Value<SalesforceStandardObjectAttachmentConfiguration>>,
        /// Property [`StandardObjectConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceconfiguration.html#cfn-kendra-datasource-salesforceconfiguration-standardobjectconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub standard_object_configurations: Option<::ValueList<SalesforceStandardObjectConfiguration>>,
    }

    impl ::codec::SerializeValue for SalesforceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref chatter_feed_configuration) = self.chatter_feed_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChatterFeedConfiguration", chatter_feed_configuration)?;
            }
            if let Some(ref crawl_attachments) = self.crawl_attachments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlAttachments", crawl_attachments)?;
            }
            if let Some(ref exclude_attachment_file_patterns) = self.exclude_attachment_file_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeAttachmentFilePatterns", exclude_attachment_file_patterns)?;
            }
            if let Some(ref include_attachment_file_patterns) = self.include_attachment_file_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeAttachmentFilePatterns", include_attachment_file_patterns)?;
            }
            if let Some(ref knowledge_article_configuration) = self.knowledge_article_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KnowledgeArticleConfiguration", knowledge_article_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", &self.secret_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerUrl", &self.server_url)?;
            if let Some(ref standard_object_attachment_configuration) = self.standard_object_attachment_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StandardObjectAttachmentConfiguration", standard_object_attachment_configuration)?;
            }
            if let Some(ref standard_object_configurations) = self.standard_object_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StandardObjectConfigurations", standard_object_configurations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut chatter_feed_configuration: Option<::Value<SalesforceChatterFeedConfiguration>> = None;
                    let mut crawl_attachments: Option<::Value<bool>> = None;
                    let mut exclude_attachment_file_patterns: Option<::ValueList<String>> = None;
                    let mut include_attachment_file_patterns: Option<::ValueList<String>> = None;
                    let mut knowledge_article_configuration: Option<::Value<SalesforceKnowledgeArticleConfiguration>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;
                    let mut server_url: Option<::Value<String>> = None;
                    let mut standard_object_attachment_configuration: Option<::Value<SalesforceStandardObjectAttachmentConfiguration>> = None;
                    let mut standard_object_configurations: Option<::ValueList<SalesforceStandardObjectConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChatterFeedConfiguration" => {
                                chatter_feed_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CrawlAttachments" => {
                                crawl_attachments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeAttachmentFilePatterns" => {
                                exclude_attachment_file_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeAttachmentFilePatterns" => {
                                include_attachment_file_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KnowledgeArticleConfiguration" => {
                                knowledge_article_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerUrl" => {
                                server_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StandardObjectAttachmentConfiguration" => {
                                standard_object_attachment_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StandardObjectConfigurations" => {
                                standard_object_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceConfiguration {
                        chatter_feed_configuration: chatter_feed_configuration,
                        crawl_attachments: crawl_attachments,
                        exclude_attachment_file_patterns: exclude_attachment_file_patterns,
                        include_attachment_file_patterns: include_attachment_file_patterns,
                        knowledge_article_configuration: knowledge_article_configuration,
                        secret_arn: secret_arn.ok_or(::serde::de::Error::missing_field("SecretArn"))?,
                        server_url: server_url.ok_or(::serde::de::Error::missing_field("ServerUrl"))?,
                        standard_object_attachment_configuration: standard_object_attachment_configuration,
                        standard_object_configurations: standard_object_configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.SalesforceCustomKnowledgeArticleTypeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcecustomknowledgearticletypeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceCustomKnowledgeArticleTypeConfiguration {
        /// Property [`DocumentDataFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcecustomknowledgearticletypeconfiguration.html#cfn-kendra-datasource-salesforcecustomknowledgearticletypeconfiguration-documentdatafieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_data_field_name: ::Value<String>,
        /// Property [`DocumentTitleFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcecustomknowledgearticletypeconfiguration.html#cfn-kendra-datasource-salesforcecustomknowledgearticletypeconfiguration-documenttitlefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_title_field_name: Option<::Value<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcecustomknowledgearticletypeconfiguration.html#cfn-kendra-datasource-salesforcecustomknowledgearticletypeconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcecustomknowledgearticletypeconfiguration.html#cfn-kendra-datasource-salesforcecustomknowledgearticletypeconfiguration-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SalesforceCustomKnowledgeArticleTypeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentDataFieldName", &self.document_data_field_name)?;
            if let Some(ref document_title_field_name) = self.document_title_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentTitleFieldName", document_title_field_name)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceCustomKnowledgeArticleTypeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceCustomKnowledgeArticleTypeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceCustomKnowledgeArticleTypeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceCustomKnowledgeArticleTypeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_data_field_name: Option<::Value<String>> = None;
                    let mut document_title_field_name: Option<::Value<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentDataFieldName" => {
                                document_data_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentTitleFieldName" => {
                                document_title_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceCustomKnowledgeArticleTypeConfiguration {
                        document_data_field_name: document_data_field_name.ok_or(::serde::de::Error::missing_field("DocumentDataFieldName"))?,
                        document_title_field_name: document_title_field_name,
                        field_mappings: field_mappings,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.SalesforceKnowledgeArticleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceknowledgearticleconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceKnowledgeArticleConfiguration {
        /// Property [`CustomKnowledgeArticleTypeConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceknowledgearticleconfiguration.html#cfn-kendra-datasource-salesforceknowledgearticleconfiguration-customknowledgearticletypeconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_knowledge_article_type_configurations: Option<::ValueList<SalesforceCustomKnowledgeArticleTypeConfiguration>>,
        /// Property [`IncludedStates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceknowledgearticleconfiguration.html#cfn-kendra-datasource-salesforceknowledgearticleconfiguration-includedstates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub included_states: ::ValueList<String>,
        /// Property [`StandardKnowledgeArticleTypeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforceknowledgearticleconfiguration.html#cfn-kendra-datasource-salesforceknowledgearticleconfiguration-standardknowledgearticletypeconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub standard_knowledge_article_type_configuration: Option<::Value<SalesforceStandardKnowledgeArticleTypeConfiguration>>,
    }

    impl ::codec::SerializeValue for SalesforceKnowledgeArticleConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_knowledge_article_type_configurations) = self.custom_knowledge_article_type_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomKnowledgeArticleTypeConfigurations", custom_knowledge_article_type_configurations)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedStates", &self.included_states)?;
            if let Some(ref standard_knowledge_article_type_configuration) = self.standard_knowledge_article_type_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StandardKnowledgeArticleTypeConfiguration", standard_knowledge_article_type_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceKnowledgeArticleConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceKnowledgeArticleConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceKnowledgeArticleConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceKnowledgeArticleConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_knowledge_article_type_configurations: Option<::ValueList<SalesforceCustomKnowledgeArticleTypeConfiguration>> = None;
                    let mut included_states: Option<::ValueList<String>> = None;
                    let mut standard_knowledge_article_type_configuration: Option<::Value<SalesforceStandardKnowledgeArticleTypeConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomKnowledgeArticleTypeConfigurations" => {
                                custom_knowledge_article_type_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludedStates" => {
                                included_states = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StandardKnowledgeArticleTypeConfiguration" => {
                                standard_knowledge_article_type_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceKnowledgeArticleConfiguration {
                        custom_knowledge_article_type_configurations: custom_knowledge_article_type_configurations,
                        included_states: included_states.ok_or(::serde::de::Error::missing_field("IncludedStates"))?,
                        standard_knowledge_article_type_configuration: standard_knowledge_article_type_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.SalesforceStandardKnowledgeArticleTypeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardknowledgearticletypeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceStandardKnowledgeArticleTypeConfiguration {
        /// Property [`DocumentDataFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardknowledgearticletypeconfiguration.html#cfn-kendra-datasource-salesforcestandardknowledgearticletypeconfiguration-documentdatafieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_data_field_name: ::Value<String>,
        /// Property [`DocumentTitleFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardknowledgearticletypeconfiguration.html#cfn-kendra-datasource-salesforcestandardknowledgearticletypeconfiguration-documenttitlefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_title_field_name: Option<::Value<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardknowledgearticletypeconfiguration.html#cfn-kendra-datasource-salesforcestandardknowledgearticletypeconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
    }

    impl ::codec::SerializeValue for SalesforceStandardKnowledgeArticleTypeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentDataFieldName", &self.document_data_field_name)?;
            if let Some(ref document_title_field_name) = self.document_title_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentTitleFieldName", document_title_field_name)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceStandardKnowledgeArticleTypeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceStandardKnowledgeArticleTypeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceStandardKnowledgeArticleTypeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceStandardKnowledgeArticleTypeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_data_field_name: Option<::Value<String>> = None;
                    let mut document_title_field_name: Option<::Value<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentDataFieldName" => {
                                document_data_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentTitleFieldName" => {
                                document_title_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceStandardKnowledgeArticleTypeConfiguration {
                        document_data_field_name: document_data_field_name.ok_or(::serde::de::Error::missing_field("DocumentDataFieldName"))?,
                        document_title_field_name: document_title_field_name,
                        field_mappings: field_mappings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.SalesforceStandardObjectAttachmentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardobjectattachmentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceStandardObjectAttachmentConfiguration {
        /// Property [`DocumentTitleFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardobjectattachmentconfiguration.html#cfn-kendra-datasource-salesforcestandardobjectattachmentconfiguration-documenttitlefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_title_field_name: Option<::Value<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardobjectattachmentconfiguration.html#cfn-kendra-datasource-salesforcestandardobjectattachmentconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
    }

    impl ::codec::SerializeValue for SalesforceStandardObjectAttachmentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref document_title_field_name) = self.document_title_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentTitleFieldName", document_title_field_name)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceStandardObjectAttachmentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceStandardObjectAttachmentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceStandardObjectAttachmentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceStandardObjectAttachmentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_title_field_name: Option<::Value<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentTitleFieldName" => {
                                document_title_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceStandardObjectAttachmentConfiguration {
                        document_title_field_name: document_title_field_name,
                        field_mappings: field_mappings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.SalesforceStandardObjectConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardobjectconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceStandardObjectConfiguration {
        /// Property [`DocumentDataFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardobjectconfiguration.html#cfn-kendra-datasource-salesforcestandardobjectconfiguration-documentdatafieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_data_field_name: ::Value<String>,
        /// Property [`DocumentTitleFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardobjectconfiguration.html#cfn-kendra-datasource-salesforcestandardobjectconfiguration-documenttitlefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_title_field_name: Option<::Value<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardobjectconfiguration.html#cfn-kendra-datasource-salesforcestandardobjectconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-salesforcestandardobjectconfiguration.html#cfn-kendra-datasource-salesforcestandardobjectconfiguration-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SalesforceStandardObjectConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentDataFieldName", &self.document_data_field_name)?;
            if let Some(ref document_title_field_name) = self.document_title_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentTitleFieldName", document_title_field_name)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceStandardObjectConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceStandardObjectConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceStandardObjectConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceStandardObjectConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_data_field_name: Option<::Value<String>> = None;
                    let mut document_title_field_name: Option<::Value<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentDataFieldName" => {
                                document_data_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentTitleFieldName" => {
                                document_title_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceStandardObjectConfiguration {
                        document_data_field_name: document_data_field_name.ok_or(::serde::de::Error::missing_field("DocumentDataFieldName"))?,
                        document_title_field_name: document_title_field_name,
                        field_mappings: field_mappings,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ServiceNowConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceNowConfiguration {
        /// Property [`AuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowconfiguration.html#cfn-kendra-datasource-servicenowconfiguration-authenticationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authentication_type: Option<::Value<String>>,
        /// Property [`HostUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowconfiguration.html#cfn-kendra-datasource-servicenowconfiguration-hosturl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host_url: ::Value<String>,
        /// Property [`KnowledgeArticleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowconfiguration.html#cfn-kendra-datasource-servicenowconfiguration-knowledgearticleconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub knowledge_article_configuration: Option<::Value<ServiceNowKnowledgeArticleConfiguration>>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowconfiguration.html#cfn-kendra-datasource-servicenowconfiguration-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: ::Value<String>,
        /// Property [`ServiceCatalogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowconfiguration.html#cfn-kendra-datasource-servicenowconfiguration-servicecatalogconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_catalog_configuration: Option<::Value<ServiceNowServiceCatalogConfiguration>>,
        /// Property [`ServiceNowBuildVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowconfiguration.html#cfn-kendra-datasource-servicenowconfiguration-servicenowbuildversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_now_build_version: ::Value<String>,
    }

    impl ::codec::SerializeValue for ServiceNowConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authentication_type) = self.authentication_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationType", authentication_type)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostUrl", &self.host_url)?;
            if let Some(ref knowledge_article_configuration) = self.knowledge_article_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KnowledgeArticleConfiguration", knowledge_article_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", &self.secret_arn)?;
            if let Some(ref service_catalog_configuration) = self.service_catalog_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceCatalogConfiguration", service_catalog_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNowBuildVersion", &self.service_now_build_version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceNowConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceNowConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceNowConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceNowConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authentication_type: Option<::Value<String>> = None;
                    let mut host_url: Option<::Value<String>> = None;
                    let mut knowledge_article_configuration: Option<::Value<ServiceNowKnowledgeArticleConfiguration>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;
                    let mut service_catalog_configuration: Option<::Value<ServiceNowServiceCatalogConfiguration>> = None;
                    let mut service_now_build_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticationType" => {
                                authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostUrl" => {
                                host_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KnowledgeArticleConfiguration" => {
                                knowledge_article_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceCatalogConfiguration" => {
                                service_catalog_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNowBuildVersion" => {
                                service_now_build_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceNowConfiguration {
                        authentication_type: authentication_type,
                        host_url: host_url.ok_or(::serde::de::Error::missing_field("HostUrl"))?,
                        knowledge_article_configuration: knowledge_article_configuration,
                        secret_arn: secret_arn.ok_or(::serde::de::Error::missing_field("SecretArn"))?,
                        service_catalog_configuration: service_catalog_configuration,
                        service_now_build_version: service_now_build_version.ok_or(::serde::de::Error::missing_field("ServiceNowBuildVersion"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ServiceNowKnowledgeArticleConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowknowledgearticleconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceNowKnowledgeArticleConfiguration {
        /// Property [`CrawlAttachments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowknowledgearticleconfiguration.html#cfn-kendra-datasource-servicenowknowledgearticleconfiguration-crawlattachments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawl_attachments: Option<::Value<bool>>,
        /// Property [`DocumentDataFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowknowledgearticleconfiguration.html#cfn-kendra-datasource-servicenowknowledgearticleconfiguration-documentdatafieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_data_field_name: ::Value<String>,
        /// Property [`DocumentTitleFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowknowledgearticleconfiguration.html#cfn-kendra-datasource-servicenowknowledgearticleconfiguration-documenttitlefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_title_field_name: Option<::Value<String>>,
        /// Property [`ExcludeAttachmentFilePatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowknowledgearticleconfiguration.html#cfn-kendra-datasource-servicenowknowledgearticleconfiguration-excludeattachmentfilepatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_attachment_file_patterns: Option<::ValueList<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowknowledgearticleconfiguration.html#cfn-kendra-datasource-servicenowknowledgearticleconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
        /// Property [`FilterQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowknowledgearticleconfiguration.html#cfn-kendra-datasource-servicenowknowledgearticleconfiguration-filterquery).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_query: Option<::Value<String>>,
        /// Property [`IncludeAttachmentFilePatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowknowledgearticleconfiguration.html#cfn-kendra-datasource-servicenowknowledgearticleconfiguration-includeattachmentfilepatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_attachment_file_patterns: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ServiceNowKnowledgeArticleConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref crawl_attachments) = self.crawl_attachments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlAttachments", crawl_attachments)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentDataFieldName", &self.document_data_field_name)?;
            if let Some(ref document_title_field_name) = self.document_title_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentTitleFieldName", document_title_field_name)?;
            }
            if let Some(ref exclude_attachment_file_patterns) = self.exclude_attachment_file_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeAttachmentFilePatterns", exclude_attachment_file_patterns)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            if let Some(ref filter_query) = self.filter_query {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterQuery", filter_query)?;
            }
            if let Some(ref include_attachment_file_patterns) = self.include_attachment_file_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeAttachmentFilePatterns", include_attachment_file_patterns)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceNowKnowledgeArticleConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceNowKnowledgeArticleConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceNowKnowledgeArticleConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceNowKnowledgeArticleConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut crawl_attachments: Option<::Value<bool>> = None;
                    let mut document_data_field_name: Option<::Value<String>> = None;
                    let mut document_title_field_name: Option<::Value<String>> = None;
                    let mut exclude_attachment_file_patterns: Option<::ValueList<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;
                    let mut filter_query: Option<::Value<String>> = None;
                    let mut include_attachment_file_patterns: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CrawlAttachments" => {
                                crawl_attachments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentDataFieldName" => {
                                document_data_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentTitleFieldName" => {
                                document_title_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeAttachmentFilePatterns" => {
                                exclude_attachment_file_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterQuery" => {
                                filter_query = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeAttachmentFilePatterns" => {
                                include_attachment_file_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceNowKnowledgeArticleConfiguration {
                        crawl_attachments: crawl_attachments,
                        document_data_field_name: document_data_field_name.ok_or(::serde::de::Error::missing_field("DocumentDataFieldName"))?,
                        document_title_field_name: document_title_field_name,
                        exclude_attachment_file_patterns: exclude_attachment_file_patterns,
                        field_mappings: field_mappings,
                        filter_query: filter_query,
                        include_attachment_file_patterns: include_attachment_file_patterns,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.ServiceNowServiceCatalogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowservicecatalogconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceNowServiceCatalogConfiguration {
        /// Property [`CrawlAttachments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowservicecatalogconfiguration.html#cfn-kendra-datasource-servicenowservicecatalogconfiguration-crawlattachments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawl_attachments: Option<::Value<bool>>,
        /// Property [`DocumentDataFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowservicecatalogconfiguration.html#cfn-kendra-datasource-servicenowservicecatalogconfiguration-documentdatafieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_data_field_name: ::Value<String>,
        /// Property [`DocumentTitleFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowservicecatalogconfiguration.html#cfn-kendra-datasource-servicenowservicecatalogconfiguration-documenttitlefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_title_field_name: Option<::Value<String>>,
        /// Property [`ExcludeAttachmentFilePatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowservicecatalogconfiguration.html#cfn-kendra-datasource-servicenowservicecatalogconfiguration-excludeattachmentfilepatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_attachment_file_patterns: Option<::ValueList<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowservicecatalogconfiguration.html#cfn-kendra-datasource-servicenowservicecatalogconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
        /// Property [`IncludeAttachmentFilePatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-servicenowservicecatalogconfiguration.html#cfn-kendra-datasource-servicenowservicecatalogconfiguration-includeattachmentfilepatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_attachment_file_patterns: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ServiceNowServiceCatalogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref crawl_attachments) = self.crawl_attachments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlAttachments", crawl_attachments)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentDataFieldName", &self.document_data_field_name)?;
            if let Some(ref document_title_field_name) = self.document_title_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentTitleFieldName", document_title_field_name)?;
            }
            if let Some(ref exclude_attachment_file_patterns) = self.exclude_attachment_file_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeAttachmentFilePatterns", exclude_attachment_file_patterns)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            if let Some(ref include_attachment_file_patterns) = self.include_attachment_file_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeAttachmentFilePatterns", include_attachment_file_patterns)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceNowServiceCatalogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceNowServiceCatalogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceNowServiceCatalogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceNowServiceCatalogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut crawl_attachments: Option<::Value<bool>> = None;
                    let mut document_data_field_name: Option<::Value<String>> = None;
                    let mut document_title_field_name: Option<::Value<String>> = None;
                    let mut exclude_attachment_file_patterns: Option<::ValueList<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;
                    let mut include_attachment_file_patterns: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CrawlAttachments" => {
                                crawl_attachments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentDataFieldName" => {
                                document_data_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentTitleFieldName" => {
                                document_title_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeAttachmentFilePatterns" => {
                                exclude_attachment_file_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeAttachmentFilePatterns" => {
                                include_attachment_file_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceNowServiceCatalogConfiguration {
                        crawl_attachments: crawl_attachments,
                        document_data_field_name: document_data_field_name.ok_or(::serde::de::Error::missing_field("DocumentDataFieldName"))?,
                        document_title_field_name: document_title_field_name,
                        exclude_attachment_file_patterns: exclude_attachment_file_patterns,
                        field_mappings: field_mappings,
                        include_attachment_file_patterns: include_attachment_file_patterns,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.SharePointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SharePointConfiguration {
        /// Property [`CrawlAttachments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-crawlattachments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawl_attachments: Option<::Value<bool>>,
        /// Property [`DisableLocalGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-disablelocalgroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_local_groups: Option<::Value<bool>>,
        /// Property [`DocumentTitleFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-documenttitlefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_title_field_name: Option<::Value<String>>,
        /// Property [`ExclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-exclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclusion_patterns: Option<::ValueList<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
        /// Property [`InclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-inclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inclusion_patterns: Option<::ValueList<String>>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: ::Value<String>,
        /// Property [`SharePointVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-sharepointversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub share_point_version: ::Value<String>,
        /// Property [`SslCertificateS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-sslcertificates3path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_certificate_s3_path: Option<::Value<S3Path>>,
        /// Property [`Urls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-urls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub urls: ::ValueList<String>,
        /// Property [`UseChangeLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-usechangelog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_change_log: Option<::Value<bool>>,
        /// Property [`VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sharepointconfiguration.html#cfn-kendra-datasource-sharepointconfiguration-vpcconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_configuration: Option<::Value<DataSourceVpcConfiguration>>,
    }

    impl ::codec::SerializeValue for SharePointConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref crawl_attachments) = self.crawl_attachments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlAttachments", crawl_attachments)?;
            }
            if let Some(ref disable_local_groups) = self.disable_local_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableLocalGroups", disable_local_groups)?;
            }
            if let Some(ref document_title_field_name) = self.document_title_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentTitleFieldName", document_title_field_name)?;
            }
            if let Some(ref exclusion_patterns) = self.exclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExclusionPatterns", exclusion_patterns)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            if let Some(ref inclusion_patterns) = self.inclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InclusionPatterns", inclusion_patterns)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", &self.secret_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SharePointVersion", &self.share_point_version)?;
            if let Some(ref ssl_certificate_s3_path) = self.ssl_certificate_s3_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslCertificateS3Path", ssl_certificate_s3_path)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Urls", &self.urls)?;
            if let Some(ref use_change_log) = self.use_change_log {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseChangeLog", use_change_log)?;
            }
            if let Some(ref vpc_configuration) = self.vpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfiguration", vpc_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SharePointConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SharePointConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SharePointConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SharePointConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut crawl_attachments: Option<::Value<bool>> = None;
                    let mut disable_local_groups: Option<::Value<bool>> = None;
                    let mut document_title_field_name: Option<::Value<String>> = None;
                    let mut exclusion_patterns: Option<::ValueList<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;
                    let mut inclusion_patterns: Option<::ValueList<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;
                    let mut share_point_version: Option<::Value<String>> = None;
                    let mut ssl_certificate_s3_path: Option<::Value<S3Path>> = None;
                    let mut urls: Option<::ValueList<String>> = None;
                    let mut use_change_log: Option<::Value<bool>> = None;
                    let mut vpc_configuration: Option<::Value<DataSourceVpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CrawlAttachments" => {
                                crawl_attachments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisableLocalGroups" => {
                                disable_local_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentTitleFieldName" => {
                                document_title_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExclusionPatterns" => {
                                exclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InclusionPatterns" => {
                                inclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SharePointVersion" => {
                                share_point_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslCertificateS3Path" => {
                                ssl_certificate_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Urls" => {
                                urls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseChangeLog" => {
                                use_change_log = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfiguration" => {
                                vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SharePointConfiguration {
                        crawl_attachments: crawl_attachments,
                        disable_local_groups: disable_local_groups,
                        document_title_field_name: document_title_field_name,
                        exclusion_patterns: exclusion_patterns,
                        field_mappings: field_mappings,
                        inclusion_patterns: inclusion_patterns,
                        secret_arn: secret_arn.ok_or(::serde::de::Error::missing_field("SecretArn"))?,
                        share_point_version: share_point_version.ok_or(::serde::de::Error::missing_field("SharePointVersion"))?,
                        ssl_certificate_s3_path: ssl_certificate_s3_path,
                        urls: urls.ok_or(::serde::de::Error::missing_field("Urls"))?,
                        use_change_log: use_change_log,
                        vpc_configuration: vpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.SqlConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sqlconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SqlConfiguration {
        /// Property [`QueryIdentifiersEnclosingOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-sqlconfiguration.html#cfn-kendra-datasource-sqlconfiguration-queryidentifiersenclosingoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_identifiers_enclosing_option: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SqlConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref query_identifiers_enclosing_option) = self.query_identifiers_enclosing_option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryIdentifiersEnclosingOption", query_identifiers_enclosing_option)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SqlConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SqlConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SqlConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SqlConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut query_identifiers_enclosing_option: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "QueryIdentifiersEnclosingOption" => {
                                query_identifiers_enclosing_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SqlConfiguration {
                        query_identifiers_enclosing_option: query_identifiers_enclosing_option,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.WebCrawlerAuthenticationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerauthenticationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct WebCrawlerAuthenticationConfiguration {
        /// Property [`BasicAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerauthenticationconfiguration.html#cfn-kendra-datasource-webcrawlerauthenticationconfiguration-basicauthentication).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub basic_authentication: Option<::ValueList<WebCrawlerBasicAuthentication>>,
    }

    impl ::codec::SerializeValue for WebCrawlerAuthenticationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref basic_authentication) = self.basic_authentication {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasicAuthentication", basic_authentication)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WebCrawlerAuthenticationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WebCrawlerAuthenticationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WebCrawlerAuthenticationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WebCrawlerAuthenticationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut basic_authentication: Option<::ValueList<WebCrawlerBasicAuthentication>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BasicAuthentication" => {
                                basic_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WebCrawlerAuthenticationConfiguration {
                        basic_authentication: basic_authentication,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.WebCrawlerBasicAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerbasicauthentication.html) property type.
    #[derive(Debug, Default)]
    pub struct WebCrawlerBasicAuthentication {
        /// Property [`Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerbasicauthentication.html#cfn-kendra-datasource-webcrawlerbasicauthentication-credentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credentials: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerbasicauthentication.html#cfn-kendra-datasource-webcrawlerbasicauthentication-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerbasicauthentication.html#cfn-kendra-datasource-webcrawlerbasicauthentication-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
    }

    impl ::codec::SerializeValue for WebCrawlerBasicAuthentication {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credentials", &self.credentials)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WebCrawlerBasicAuthentication {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WebCrawlerBasicAuthentication, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WebCrawlerBasicAuthentication;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WebCrawlerBasicAuthentication")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut credentials: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Credentials" => {
                                credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WebCrawlerBasicAuthentication {
                        credentials: credentials.ok_or(::serde::de::Error::missing_field("Credentials"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.WebCrawlerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct WebCrawlerConfiguration {
        /// Property [`AuthenticationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html#cfn-kendra-datasource-webcrawlerconfiguration-authenticationconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authentication_configuration: Option<::Value<WebCrawlerAuthenticationConfiguration>>,
        /// Property [`CrawlDepth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html#cfn-kendra-datasource-webcrawlerconfiguration-crawldepth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawl_depth: Option<::Value<u32>>,
        /// Property [`MaxContentSizePerPageInMegaBytes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html#cfn-kendra-datasource-webcrawlerconfiguration-maxcontentsizeperpageinmegabytes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_content_size_per_page_in_mega_bytes: Option<::Value<f64>>,
        /// Property [`MaxLinksPerPage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html#cfn-kendra-datasource-webcrawlerconfiguration-maxlinksperpage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_links_per_page: Option<::Value<u32>>,
        /// Property [`MaxUrlsPerMinuteCrawlRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html#cfn-kendra-datasource-webcrawlerconfiguration-maxurlsperminutecrawlrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_urls_per_minute_crawl_rate: Option<::Value<u32>>,
        /// Property [`ProxyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html#cfn-kendra-datasource-webcrawlerconfiguration-proxyconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub proxy_configuration: Option<::Value<ProxyConfiguration>>,
        /// Property [`UrlExclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html#cfn-kendra-datasource-webcrawlerconfiguration-urlexclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url_exclusion_patterns: Option<::ValueList<String>>,
        /// Property [`UrlInclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html#cfn-kendra-datasource-webcrawlerconfiguration-urlinclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url_inclusion_patterns: Option<::ValueList<String>>,
        /// Property [`Urls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerconfiguration.html#cfn-kendra-datasource-webcrawlerconfiguration-urls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub urls: ::Value<WebCrawlerUrls>,
    }

    impl ::codec::SerializeValue for WebCrawlerConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authentication_configuration) = self.authentication_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationConfiguration", authentication_configuration)?;
            }
            if let Some(ref crawl_depth) = self.crawl_depth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlDepth", crawl_depth)?;
            }
            if let Some(ref max_content_size_per_page_in_mega_bytes) = self.max_content_size_per_page_in_mega_bytes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxContentSizePerPageInMegaBytes", max_content_size_per_page_in_mega_bytes)?;
            }
            if let Some(ref max_links_per_page) = self.max_links_per_page {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxLinksPerPage", max_links_per_page)?;
            }
            if let Some(ref max_urls_per_minute_crawl_rate) = self.max_urls_per_minute_crawl_rate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxUrlsPerMinuteCrawlRate", max_urls_per_minute_crawl_rate)?;
            }
            if let Some(ref proxy_configuration) = self.proxy_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProxyConfiguration", proxy_configuration)?;
            }
            if let Some(ref url_exclusion_patterns) = self.url_exclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UrlExclusionPatterns", url_exclusion_patterns)?;
            }
            if let Some(ref url_inclusion_patterns) = self.url_inclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UrlInclusionPatterns", url_inclusion_patterns)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Urls", &self.urls)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WebCrawlerConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WebCrawlerConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WebCrawlerConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WebCrawlerConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authentication_configuration: Option<::Value<WebCrawlerAuthenticationConfiguration>> = None;
                    let mut crawl_depth: Option<::Value<u32>> = None;
                    let mut max_content_size_per_page_in_mega_bytes: Option<::Value<f64>> = None;
                    let mut max_links_per_page: Option<::Value<u32>> = None;
                    let mut max_urls_per_minute_crawl_rate: Option<::Value<u32>> = None;
                    let mut proxy_configuration: Option<::Value<ProxyConfiguration>> = None;
                    let mut url_exclusion_patterns: Option<::ValueList<String>> = None;
                    let mut url_inclusion_patterns: Option<::ValueList<String>> = None;
                    let mut urls: Option<::Value<WebCrawlerUrls>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticationConfiguration" => {
                                authentication_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CrawlDepth" => {
                                crawl_depth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxContentSizePerPageInMegaBytes" => {
                                max_content_size_per_page_in_mega_bytes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxLinksPerPage" => {
                                max_links_per_page = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxUrlsPerMinuteCrawlRate" => {
                                max_urls_per_minute_crawl_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProxyConfiguration" => {
                                proxy_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UrlExclusionPatterns" => {
                                url_exclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UrlInclusionPatterns" => {
                                url_inclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Urls" => {
                                urls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WebCrawlerConfiguration {
                        authentication_configuration: authentication_configuration,
                        crawl_depth: crawl_depth,
                        max_content_size_per_page_in_mega_bytes: max_content_size_per_page_in_mega_bytes,
                        max_links_per_page: max_links_per_page,
                        max_urls_per_minute_crawl_rate: max_urls_per_minute_crawl_rate,
                        proxy_configuration: proxy_configuration,
                        url_exclusion_patterns: url_exclusion_patterns,
                        url_inclusion_patterns: url_inclusion_patterns,
                        urls: urls.ok_or(::serde::de::Error::missing_field("Urls"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.WebCrawlerSeedUrlConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerseedurlconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct WebCrawlerSeedUrlConfiguration {
        /// Property [`SeedUrls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerseedurlconfiguration.html#cfn-kendra-datasource-webcrawlerseedurlconfiguration-seedurls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub seed_urls: ::ValueList<String>,
        /// Property [`WebCrawlerMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerseedurlconfiguration.html#cfn-kendra-datasource-webcrawlerseedurlconfiguration-webcrawlermode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub web_crawler_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for WebCrawlerSeedUrlConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SeedUrls", &self.seed_urls)?;
            if let Some(ref web_crawler_mode) = self.web_crawler_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebCrawlerMode", web_crawler_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WebCrawlerSeedUrlConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WebCrawlerSeedUrlConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WebCrawlerSeedUrlConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WebCrawlerSeedUrlConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut seed_urls: Option<::ValueList<String>> = None;
                    let mut web_crawler_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SeedUrls" => {
                                seed_urls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WebCrawlerMode" => {
                                web_crawler_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WebCrawlerSeedUrlConfiguration {
                        seed_urls: seed_urls.ok_or(::serde::de::Error::missing_field("SeedUrls"))?,
                        web_crawler_mode: web_crawler_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.WebCrawlerSiteMapsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlersitemapsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct WebCrawlerSiteMapsConfiguration {
        /// Property [`SiteMaps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlersitemapsconfiguration.html#cfn-kendra-datasource-webcrawlersitemapsconfiguration-sitemaps).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub site_maps: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for WebCrawlerSiteMapsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SiteMaps", &self.site_maps)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WebCrawlerSiteMapsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WebCrawlerSiteMapsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WebCrawlerSiteMapsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WebCrawlerSiteMapsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut site_maps: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SiteMaps" => {
                                site_maps = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WebCrawlerSiteMapsConfiguration {
                        site_maps: site_maps.ok_or(::serde::de::Error::missing_field("SiteMaps"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.WebCrawlerUrls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerurls.html) property type.
    #[derive(Debug, Default)]
    pub struct WebCrawlerUrls {
        /// Property [`SeedUrlConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerurls.html#cfn-kendra-datasource-webcrawlerurls-seedurlconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub seed_url_configuration: Option<::Value<WebCrawlerSeedUrlConfiguration>>,
        /// Property [`SiteMapsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-webcrawlerurls.html#cfn-kendra-datasource-webcrawlerurls-sitemapsconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub site_maps_configuration: Option<::Value<WebCrawlerSiteMapsConfiguration>>,
    }

    impl ::codec::SerializeValue for WebCrawlerUrls {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref seed_url_configuration) = self.seed_url_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SeedUrlConfiguration", seed_url_configuration)?;
            }
            if let Some(ref site_maps_configuration) = self.site_maps_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SiteMapsConfiguration", site_maps_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WebCrawlerUrls {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WebCrawlerUrls, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WebCrawlerUrls;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WebCrawlerUrls")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut seed_url_configuration: Option<::Value<WebCrawlerSeedUrlConfiguration>> = None;
                    let mut site_maps_configuration: Option<::Value<WebCrawlerSiteMapsConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SeedUrlConfiguration" => {
                                seed_url_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SiteMapsConfiguration" => {
                                site_maps_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WebCrawlerUrls {
                        seed_url_configuration: seed_url_configuration,
                        site_maps_configuration: site_maps_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::DataSource.WorkDocsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-workdocsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkDocsConfiguration {
        /// Property [`CrawlComments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-workdocsconfiguration.html#cfn-kendra-datasource-workdocsconfiguration-crawlcomments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawl_comments: Option<::Value<bool>>,
        /// Property [`ExclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-workdocsconfiguration.html#cfn-kendra-datasource-workdocsconfiguration-exclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclusion_patterns: Option<::ValueList<String>>,
        /// Property [`FieldMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-workdocsconfiguration.html#cfn-kendra-datasource-workdocsconfiguration-fieldmappings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>>,
        /// Property [`InclusionPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-workdocsconfiguration.html#cfn-kendra-datasource-workdocsconfiguration-inclusionpatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inclusion_patterns: Option<::ValueList<String>>,
        /// Property [`OrganizationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-workdocsconfiguration.html#cfn-kendra-datasource-workdocsconfiguration-organizationid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub organization_id: ::Value<String>,
        /// Property [`UseChangeLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-datasource-workdocsconfiguration.html#cfn-kendra-datasource-workdocsconfiguration-usechangelog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_change_log: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for WorkDocsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref crawl_comments) = self.crawl_comments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlComments", crawl_comments)?;
            }
            if let Some(ref exclusion_patterns) = self.exclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExclusionPatterns", exclusion_patterns)?;
            }
            if let Some(ref field_mappings) = self.field_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldMappings", field_mappings)?;
            }
            if let Some(ref inclusion_patterns) = self.inclusion_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InclusionPatterns", inclusion_patterns)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationId", &self.organization_id)?;
            if let Some(ref use_change_log) = self.use_change_log {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseChangeLog", use_change_log)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkDocsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkDocsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkDocsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkDocsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut crawl_comments: Option<::Value<bool>> = None;
                    let mut exclusion_patterns: Option<::ValueList<String>> = None;
                    let mut field_mappings: Option<::ValueList<DataSourceToIndexFieldMapping>> = None;
                    let mut inclusion_patterns: Option<::ValueList<String>> = None;
                    let mut organization_id: Option<::Value<String>> = None;
                    let mut use_change_log: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CrawlComments" => {
                                crawl_comments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExclusionPatterns" => {
                                exclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldMappings" => {
                                field_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InclusionPatterns" => {
                                inclusion_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationId" => {
                                organization_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseChangeLog" => {
                                use_change_log = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkDocsConfiguration {
                        crawl_comments: crawl_comments,
                        exclusion_patterns: exclusion_patterns,
                        field_mappings: field_mappings,
                        inclusion_patterns: inclusion_patterns,
                        organization_id: organization_id.ok_or(::serde::de::Error::missing_field("OrganizationId"))?,
                        use_change_log: use_change_log,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod faq {
    //! Property types for the `Faq` resource.

    /// The [`AWS::Kendra::Faq.S3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-faq-s3path.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Path {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-faq-s3path.html#cfn-kendra-faq-s3path-bucket).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-faq-s3path.html#cfn-kendra-faq-s3path-key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Path {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Path {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Path, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Path;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Path")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Path {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod index {
    //! Property types for the `Index` resource.

    /// The [`AWS::Kendra::Index.CapacityUnitsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-capacityunitsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CapacityUnitsConfiguration {
        /// Property [`QueryCapacityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-capacityunitsconfiguration.html#cfn-kendra-index-capacityunitsconfiguration-querycapacityunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_capacity_units: ::Value<u32>,
        /// Property [`StorageCapacityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-capacityunitsconfiguration.html#cfn-kendra-index-capacityunitsconfiguration-storagecapacityunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_capacity_units: ::Value<u32>,
    }

    impl ::codec::SerializeValue for CapacityUnitsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryCapacityUnits", &self.query_capacity_units)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageCapacityUnits", &self.storage_capacity_units)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CapacityUnitsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityUnitsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CapacityUnitsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CapacityUnitsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut query_capacity_units: Option<::Value<u32>> = None;
                    let mut storage_capacity_units: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "QueryCapacityUnits" => {
                                query_capacity_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageCapacityUnits" => {
                                storage_capacity_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CapacityUnitsConfiguration {
                        query_capacity_units: query_capacity_units.ok_or(::serde::de::Error::missing_field("QueryCapacityUnits"))?,
                        storage_capacity_units: storage_capacity_units.ok_or(::serde::de::Error::missing_field("StorageCapacityUnits"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::Index.DocumentMetadataConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-documentmetadataconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentMetadataConfiguration {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-documentmetadataconfiguration.html#cfn-kendra-index-documentmetadataconfiguration-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Relevance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-documentmetadataconfiguration.html#cfn-kendra-index-documentmetadataconfiguration-relevance).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relevance: Option<::Value<Relevance>>,
        /// Property [`Search`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-documentmetadataconfiguration.html#cfn-kendra-index-documentmetadataconfiguration-search).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub search: Option<::Value<Search>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-documentmetadataconfiguration.html#cfn-kendra-index-documentmetadataconfiguration-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for DocumentMetadataConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref relevance) = self.relevance {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Relevance", relevance)?;
            }
            if let Some(ref search) = self.search {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Search", search)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentMetadataConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentMetadataConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentMetadataConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentMetadataConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut relevance: Option<::Value<Relevance>> = None;
                    let mut search: Option<::Value<Search>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Relevance" => {
                                relevance = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Search" => {
                                search = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentMetadataConfiguration {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        relevance: relevance,
                        search: search,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::Index.JsonTokenTypeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jsontokentypeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct JsonTokenTypeConfiguration {
        /// Property [`GroupAttributeField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jsontokentypeconfiguration.html#cfn-kendra-index-jsontokentypeconfiguration-groupattributefield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_attribute_field: ::Value<String>,
        /// Property [`UserNameAttributeField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jsontokentypeconfiguration.html#cfn-kendra-index-jsontokentypeconfiguration-usernameattributefield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_name_attribute_field: ::Value<String>,
    }

    impl ::codec::SerializeValue for JsonTokenTypeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupAttributeField", &self.group_attribute_field)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserNameAttributeField", &self.user_name_attribute_field)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JsonTokenTypeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JsonTokenTypeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JsonTokenTypeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JsonTokenTypeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_attribute_field: Option<::Value<String>> = None;
                    let mut user_name_attribute_field: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupAttributeField" => {
                                group_attribute_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserNameAttributeField" => {
                                user_name_attribute_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JsonTokenTypeConfiguration {
                        group_attribute_field: group_attribute_field.ok_or(::serde::de::Error::missing_field("GroupAttributeField"))?,
                        user_name_attribute_field: user_name_attribute_field.ok_or(::serde::de::Error::missing_field("UserNameAttributeField"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::Index.JwtTokenTypeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jwttokentypeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct JwtTokenTypeConfiguration {
        /// Property [`ClaimRegex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jwttokentypeconfiguration.html#cfn-kendra-index-jwttokentypeconfiguration-claimregex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub claim_regex: Option<::Value<String>>,
        /// Property [`GroupAttributeField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jwttokentypeconfiguration.html#cfn-kendra-index-jwttokentypeconfiguration-groupattributefield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_attribute_field: Option<::Value<String>>,
        /// Property [`Issuer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jwttokentypeconfiguration.html#cfn-kendra-index-jwttokentypeconfiguration-issuer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub issuer: Option<::Value<String>>,
        /// Property [`KeyLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jwttokentypeconfiguration.html#cfn-kendra-index-jwttokentypeconfiguration-keylocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_location: ::Value<String>,
        /// Property [`SecretManagerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jwttokentypeconfiguration.html#cfn-kendra-index-jwttokentypeconfiguration-secretmanagerarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_manager_arn: Option<::Value<String>>,
        /// Property [`URL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jwttokentypeconfiguration.html#cfn-kendra-index-jwttokentypeconfiguration-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
        /// Property [`UserNameAttributeField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-jwttokentypeconfiguration.html#cfn-kendra-index-jwttokentypeconfiguration-usernameattributefield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_name_attribute_field: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for JwtTokenTypeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref claim_regex) = self.claim_regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClaimRegex", claim_regex)?;
            }
            if let Some(ref group_attribute_field) = self.group_attribute_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupAttributeField", group_attribute_field)?;
            }
            if let Some(ref issuer) = self.issuer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Issuer", issuer)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyLocation", &self.key_location)?;
            if let Some(ref secret_manager_arn) = self.secret_manager_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretManagerArn", secret_manager_arn)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "URL", url)?;
            }
            if let Some(ref user_name_attribute_field) = self.user_name_attribute_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserNameAttributeField", user_name_attribute_field)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JwtTokenTypeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JwtTokenTypeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JwtTokenTypeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JwtTokenTypeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut claim_regex: Option<::Value<String>> = None;
                    let mut group_attribute_field: Option<::Value<String>> = None;
                    let mut issuer: Option<::Value<String>> = None;
                    let mut key_location: Option<::Value<String>> = None;
                    let mut secret_manager_arn: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;
                    let mut user_name_attribute_field: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClaimRegex" => {
                                claim_regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupAttributeField" => {
                                group_attribute_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Issuer" => {
                                issuer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyLocation" => {
                                key_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretManagerArn" => {
                                secret_manager_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "URL" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserNameAttributeField" => {
                                user_name_attribute_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JwtTokenTypeConfiguration {
                        claim_regex: claim_regex,
                        group_attribute_field: group_attribute_field,
                        issuer: issuer,
                        key_location: key_location.ok_or(::serde::de::Error::missing_field("KeyLocation"))?,
                        secret_manager_arn: secret_manager_arn,
                        url: url,
                        user_name_attribute_field: user_name_attribute_field,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::Index.Relevance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-relevance.html) property type.
    #[derive(Debug, Default)]
    pub struct Relevance {
        /// Property [`Duration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-relevance.html#cfn-kendra-index-relevance-duration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration: Option<::Value<String>>,
        /// Property [`Freshness`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-relevance.html#cfn-kendra-index-relevance-freshness).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub freshness: Option<::Value<bool>>,
        /// Property [`Importance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-relevance.html#cfn-kendra-index-relevance-importance).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub importance: Option<::Value<u32>>,
        /// Property [`RankOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-relevance.html#cfn-kendra-index-relevance-rankorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rank_order: Option<::Value<String>>,
        /// Property [`ValueImportanceItems`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-relevance.html#cfn-kendra-index-relevance-valueimportanceitems).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value_importance_items: Option<::ValueList<ValueImportanceItem>>,
    }

    impl ::codec::SerializeValue for Relevance {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration) = self.duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Duration", duration)?;
            }
            if let Some(ref freshness) = self.freshness {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Freshness", freshness)?;
            }
            if let Some(ref importance) = self.importance {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Importance", importance)?;
            }
            if let Some(ref rank_order) = self.rank_order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RankOrder", rank_order)?;
            }
            if let Some(ref value_importance_items) = self.value_importance_items {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueImportanceItems", value_importance_items)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Relevance {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Relevance, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Relevance;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Relevance")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut duration: Option<::Value<String>> = None;
                    let mut freshness: Option<::Value<bool>> = None;
                    let mut importance: Option<::Value<u32>> = None;
                    let mut rank_order: Option<::Value<String>> = None;
                    let mut value_importance_items: Option<::ValueList<ValueImportanceItem>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Duration" => {
                                duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Freshness" => {
                                freshness = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Importance" => {
                                importance = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RankOrder" => {
                                rank_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueImportanceItems" => {
                                value_importance_items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Relevance {
                        duration: duration,
                        freshness: freshness,
                        importance: importance,
                        rank_order: rank_order,
                        value_importance_items: value_importance_items,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::Index.Search`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-search.html) property type.
    #[derive(Debug, Default)]
    pub struct Search {
        /// Property [`Displayable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-search.html#cfn-kendra-index-search-displayable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub displayable: Option<::Value<bool>>,
        /// Property [`Facetable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-search.html#cfn-kendra-index-search-facetable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub facetable: Option<::Value<bool>>,
        /// Property [`Searchable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-search.html#cfn-kendra-index-search-searchable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub searchable: Option<::Value<bool>>,
        /// Property [`Sortable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-search.html#cfn-kendra-index-search-sortable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sortable: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for Search {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref displayable) = self.displayable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Displayable", displayable)?;
            }
            if let Some(ref facetable) = self.facetable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Facetable", facetable)?;
            }
            if let Some(ref searchable) = self.searchable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Searchable", searchable)?;
            }
            if let Some(ref sortable) = self.sortable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sortable", sortable)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Search {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Search, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Search;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Search")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut displayable: Option<::Value<bool>> = None;
                    let mut facetable: Option<::Value<bool>> = None;
                    let mut searchable: Option<::Value<bool>> = None;
                    let mut sortable: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Displayable" => {
                                displayable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Facetable" => {
                                facetable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Searchable" => {
                                searchable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sortable" => {
                                sortable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Search {
                        displayable: displayable,
                        facetable: facetable,
                        searchable: searchable,
                        sortable: sortable,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::Index.ServerSideEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-serversideencryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerSideEncryptionConfiguration {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-serversideencryptionconfiguration.html#cfn-kendra-index-serversideencryptionconfiguration-kmskeyid).
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

    /// The [`AWS::Kendra::Index.UserTokenConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-usertokenconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct UserTokenConfiguration {
        /// Property [`JsonTokenTypeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-usertokenconfiguration.html#cfn-kendra-index-usertokenconfiguration-jsontokentypeconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_token_type_configuration: Option<::Value<JsonTokenTypeConfiguration>>,
        /// Property [`JwtTokenTypeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-usertokenconfiguration.html#cfn-kendra-index-usertokenconfiguration-jwttokentypeconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub jwt_token_type_configuration: Option<::Value<JwtTokenTypeConfiguration>>,
    }

    impl ::codec::SerializeValue for UserTokenConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref json_token_type_configuration) = self.json_token_type_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonTokenTypeConfiguration", json_token_type_configuration)?;
            }
            if let Some(ref jwt_token_type_configuration) = self.jwt_token_type_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JwtTokenTypeConfiguration", jwt_token_type_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserTokenConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserTokenConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserTokenConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserTokenConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut json_token_type_configuration: Option<::Value<JsonTokenTypeConfiguration>> = None;
                    let mut jwt_token_type_configuration: Option<::Value<JwtTokenTypeConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JsonTokenTypeConfiguration" => {
                                json_token_type_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JwtTokenTypeConfiguration" => {
                                jwt_token_type_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserTokenConfiguration {
                        json_token_type_configuration: json_token_type_configuration,
                        jwt_token_type_configuration: jwt_token_type_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Kendra::Index.ValueImportanceItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-valueimportanceitem.html) property type.
    #[derive(Debug, Default)]
    pub struct ValueImportanceItem {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-valueimportanceitem.html#cfn-kendra-index-valueimportanceitem-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kendra-index-valueimportanceitem.html#cfn-kendra-index-valueimportanceitem-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ValueImportanceItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ValueImportanceItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ValueImportanceItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ValueImportanceItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ValueImportanceItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<u32>> = None;

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

                    Ok(ValueImportanceItem {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
