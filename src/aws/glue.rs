//! Types for the `Glue` service.

/// The [`AWS::Glue::Classifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html) resource type.
#[derive(Debug, Default)]
pub struct Classifier {
    properties: ClassifierProperties
}

/// Properties for the `Classifier` resource.
#[derive(Debug, Default)]
pub struct ClassifierProperties {
    /// Property [`CsvClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html#cfn-glue-classifier-csvclassifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub csv_classifier: Option<::Value<self::classifier::CsvClassifier>>,
    /// Property [`GrokClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html#cfn-glue-classifier-grokclassifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub grok_classifier: Option<::Value<self::classifier::GrokClassifier>>,
    /// Property [`JsonClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html#cfn-glue-classifier-jsonclassifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub json_classifier: Option<::Value<self::classifier::JsonClassifier>>,
    /// Property [`XMLClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html#cfn-glue-classifier-xmlclassifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub xml_classifier: Option<::Value<self::classifier::XMLClassifier>>,
}

impl ::serde::Serialize for ClassifierProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref csv_classifier) = self.csv_classifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvClassifier", csv_classifier)?;
        }
        if let Some(ref grok_classifier) = self.grok_classifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GrokClassifier", grok_classifier)?;
        }
        if let Some(ref json_classifier) = self.json_classifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonClassifier", json_classifier)?;
        }
        if let Some(ref xml_classifier) = self.xml_classifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "XMLClassifier", xml_classifier)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClassifierProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClassifierProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClassifierProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClassifierProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut csv_classifier: Option<::Value<self::classifier::CsvClassifier>> = None;
                let mut grok_classifier: Option<::Value<self::classifier::GrokClassifier>> = None;
                let mut json_classifier: Option<::Value<self::classifier::JsonClassifier>> = None;
                let mut xml_classifier: Option<::Value<self::classifier::XMLClassifier>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CsvClassifier" => {
                            csv_classifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GrokClassifier" => {
                            grok_classifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JsonClassifier" => {
                            json_classifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "XMLClassifier" => {
                            xml_classifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClassifierProperties {
                    csv_classifier: csv_classifier,
                    grok_classifier: grok_classifier,
                    json_classifier: json_classifier,
                    xml_classifier: xml_classifier,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Classifier {
    type Properties = ClassifierProperties;
    const TYPE: &'static str = "AWS::Glue::Classifier";
    fn properties(&self) -> &ClassifierProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClassifierProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Classifier {}

impl From<ClassifierProperties> for Classifier {
    fn from(properties: ClassifierProperties) -> Classifier {
        Classifier { properties }
    }
}

/// The [`AWS::Glue::Connection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-connection.html) resource type.
#[derive(Debug, Default)]
pub struct Connection {
    properties: ConnectionProperties
}

/// Properties for the `Connection` resource.
#[derive(Debug, Default)]
pub struct ConnectionProperties {
    /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-connection.html#cfn-glue-connection-catalogid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub catalog_id: ::Value<String>,
    /// Property [`ConnectionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-connection.html#cfn-glue-connection-connectioninput).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_input: ::Value<self::connection::ConnectionInput>,
}

impl ::serde::Serialize for ConnectionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionInput", &self.connection_input)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut catalog_id: Option<::Value<String>> = None;
                let mut connection_input: Option<::Value<self::connection::ConnectionInput>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CatalogId" => {
                            catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectionInput" => {
                            connection_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectionProperties {
                    catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                    connection_input: connection_input.ok_or(::serde::de::Error::missing_field("ConnectionInput"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Connection {
    type Properties = ConnectionProperties;
    const TYPE: &'static str = "AWS::Glue::Connection";
    fn properties(&self) -> &ConnectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Connection {}

impl From<ConnectionProperties> for Connection {
    fn from(properties: ConnectionProperties) -> Connection {
        Connection { properties }
    }
}

/// The [`AWS::Glue::Crawler`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html) resource type.
#[derive(Debug, Default)]
pub struct Crawler {
    properties: CrawlerProperties
}

/// Properties for the `Crawler` resource.
#[derive(Debug, Default)]
pub struct CrawlerProperties {
    /// Property [`Classifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-classifiers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub classifiers: Option<::ValueList<String>>,
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-configuration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration: Option<::Value<String>>,
    /// Property [`CrawlerSecurityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-crawlersecurityconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub crawler_security_configuration: Option<::Value<String>>,
    /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-databasename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub database_name: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RecrawlPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-recrawlpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub recrawl_policy: Option<::Value<self::crawler::RecrawlPolicy>>,
    /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-role).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role: ::Value<String>,
    /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-schedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule: Option<::Value<self::crawler::Schedule>>,
    /// Property [`SchemaChangePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-schemachangepolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schema_change_policy: Option<::Value<self::crawler::SchemaChangePolicy>>,
    /// Property [`TablePrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-tableprefix).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub table_prefix: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-targets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub targets: ::Value<self::crawler::Targets>,
}

impl ::serde::Serialize for CrawlerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref classifiers) = self.classifiers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classifiers", classifiers)?;
        }
        if let Some(ref configuration) = self.configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", configuration)?;
        }
        if let Some(ref crawler_security_configuration) = self.crawler_security_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlerSecurityConfiguration", crawler_security_configuration)?;
        }
        if let Some(ref database_name) = self.database_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref recrawl_policy) = self.recrawl_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecrawlPolicy", recrawl_policy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        if let Some(ref schedule) = self.schedule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", schedule)?;
        }
        if let Some(ref schema_change_policy) = self.schema_change_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaChangePolicy", schema_change_policy)?;
        }
        if let Some(ref table_prefix) = self.table_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TablePrefix", table_prefix)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", &self.targets)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CrawlerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CrawlerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CrawlerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CrawlerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut classifiers: Option<::ValueList<String>> = None;
                let mut configuration: Option<::Value<String>> = None;
                let mut crawler_security_configuration: Option<::Value<String>> = None;
                let mut database_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut recrawl_policy: Option<::Value<self::crawler::RecrawlPolicy>> = None;
                let mut role: Option<::Value<String>> = None;
                let mut schedule: Option<::Value<self::crawler::Schedule>> = None;
                let mut schema_change_policy: Option<::Value<self::crawler::SchemaChangePolicy>> = None;
                let mut table_prefix: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut targets: Option<::Value<self::crawler::Targets>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Classifiers" => {
                            classifiers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CrawlerSecurityConfiguration" => {
                            crawler_security_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatabaseName" => {
                            database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RecrawlPolicy" => {
                            recrawl_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Role" => {
                            role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedule" => {
                            schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaChangePolicy" => {
                            schema_change_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TablePrefix" => {
                            table_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CrawlerProperties {
                    classifiers: classifiers,
                    configuration: configuration,
                    crawler_security_configuration: crawler_security_configuration,
                    database_name: database_name,
                    description: description,
                    name: name,
                    recrawl_policy: recrawl_policy,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                    schedule: schedule,
                    schema_change_policy: schema_change_policy,
                    table_prefix: table_prefix,
                    tags: tags,
                    targets: targets.ok_or(::serde::de::Error::missing_field("Targets"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Crawler {
    type Properties = CrawlerProperties;
    const TYPE: &'static str = "AWS::Glue::Crawler";
    fn properties(&self) -> &CrawlerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CrawlerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Crawler {}

impl From<CrawlerProperties> for Crawler {
    fn from(properties: CrawlerProperties) -> Crawler {
        Crawler { properties }
    }
}

/// The [`AWS::Glue::CustomEntityType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-customentitytype.html) resource type.
#[derive(Debug, Default)]
pub struct CustomEntityType {
    properties: CustomEntityTypeProperties
}

/// Properties for the `CustomEntityType` resource.
#[derive(Debug, Default)]
pub struct CustomEntityTypeProperties {
    /// Property [`ContextWords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-customentitytype.html#cfn-glue-customentitytype-contextwords).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub context_words: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-customentitytype.html#cfn-glue-customentitytype-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RegexString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-customentitytype.html#cfn-glue-customentitytype-regexstring).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub regex_string: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-customentitytype.html#cfn-glue-customentitytype-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for CustomEntityTypeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref context_words) = self.context_words {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContextWords", context_words)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref regex_string) = self.regex_string {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegexString", regex_string)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CustomEntityTypeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomEntityTypeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomEntityTypeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CustomEntityTypeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut context_words: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut regex_string: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContextWords" => {
                            context_words = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RegexString" => {
                            regex_string = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CustomEntityTypeProperties {
                    context_words: context_words,
                    name: name,
                    regex_string: regex_string,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CustomEntityType {
    type Properties = CustomEntityTypeProperties;
    const TYPE: &'static str = "AWS::Glue::CustomEntityType";
    fn properties(&self) -> &CustomEntityTypeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomEntityTypeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CustomEntityType {}

impl From<CustomEntityTypeProperties> for CustomEntityType {
    fn from(properties: CustomEntityTypeProperties) -> CustomEntityType {
        CustomEntityType { properties }
    }
}

/// The [`AWS::Glue::DataCatalogEncryptionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-datacatalogencryptionsettings.html) resource type.
#[derive(Debug, Default)]
pub struct DataCatalogEncryptionSettings {
    properties: DataCatalogEncryptionSettingsProperties
}

/// Properties for the `DataCatalogEncryptionSettings` resource.
#[derive(Debug, Default)]
pub struct DataCatalogEncryptionSettingsProperties {
    /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-datacatalogencryptionsettings.html#cfn-glue-datacatalogencryptionsettings-catalogid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub catalog_id: ::Value<String>,
    /// Property [`DataCatalogEncryptionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-datacatalogencryptionsettings.html#cfn-glue-datacatalogencryptionsettings-datacatalogencryptionsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_catalog_encryption_settings: ::Value<self::data_catalog_encryption_settings::DataCatalogEncryptionSettings>,
}

impl ::serde::Serialize for DataCatalogEncryptionSettingsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataCatalogEncryptionSettings", &self.data_catalog_encryption_settings)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataCatalogEncryptionSettingsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataCatalogEncryptionSettingsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataCatalogEncryptionSettingsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataCatalogEncryptionSettingsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut catalog_id: Option<::Value<String>> = None;
                let mut data_catalog_encryption_settings: Option<::Value<self::data_catalog_encryption_settings::DataCatalogEncryptionSettings>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CatalogId" => {
                            catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataCatalogEncryptionSettings" => {
                            data_catalog_encryption_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataCatalogEncryptionSettingsProperties {
                    catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                    data_catalog_encryption_settings: data_catalog_encryption_settings.ok_or(::serde::de::Error::missing_field("DataCatalogEncryptionSettings"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataCatalogEncryptionSettings {
    type Properties = DataCatalogEncryptionSettingsProperties;
    const TYPE: &'static str = "AWS::Glue::DataCatalogEncryptionSettings";
    fn properties(&self) -> &DataCatalogEncryptionSettingsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataCatalogEncryptionSettingsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataCatalogEncryptionSettings {}

impl From<DataCatalogEncryptionSettingsProperties> for DataCatalogEncryptionSettings {
    fn from(properties: DataCatalogEncryptionSettingsProperties) -> DataCatalogEncryptionSettings {
        DataCatalogEncryptionSettings { properties }
    }
}

/// The [`AWS::Glue::DataQualityRuleset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-dataqualityruleset.html) resource type.
#[derive(Debug, Default)]
pub struct DataQualityRuleset {
    properties: DataQualityRulesetProperties
}

/// Properties for the `DataQualityRuleset` resource.
#[derive(Debug, Default)]
pub struct DataQualityRulesetProperties {
    /// Property [`ClientToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-dataqualityruleset.html#cfn-glue-dataqualityruleset-clienttoken).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub client_token: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-dataqualityruleset.html#cfn-glue-dataqualityruleset-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-dataqualityruleset.html#cfn-glue-dataqualityruleset-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Ruleset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-dataqualityruleset.html#cfn-glue-dataqualityruleset-ruleset).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ruleset: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-dataqualityruleset.html#cfn-glue-dataqualityruleset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`TargetTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-dataqualityruleset.html#cfn-glue-dataqualityruleset-targettable).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_table: Option<::Value<self::data_quality_ruleset::DataQualityTargetTable>>,
}

impl ::serde::Serialize for DataQualityRulesetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref client_token) = self.client_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientToken", client_token)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref ruleset) = self.ruleset {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ruleset", ruleset)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_table) = self.target_table {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetTable", target_table)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataQualityRulesetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataQualityRulesetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataQualityRulesetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataQualityRulesetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut client_token: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut ruleset: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut target_table: Option<::Value<self::data_quality_ruleset::DataQualityTargetTable>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClientToken" => {
                            client_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Ruleset" => {
                            ruleset = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetTable" => {
                            target_table = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataQualityRulesetProperties {
                    client_token: client_token,
                    description: description,
                    name: name,
                    ruleset: ruleset,
                    tags: tags,
                    target_table: target_table,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataQualityRuleset {
    type Properties = DataQualityRulesetProperties;
    const TYPE: &'static str = "AWS::Glue::DataQualityRuleset";
    fn properties(&self) -> &DataQualityRulesetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataQualityRulesetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataQualityRuleset {}

impl From<DataQualityRulesetProperties> for DataQualityRuleset {
    fn from(properties: DataQualityRulesetProperties) -> DataQualityRuleset {
        DataQualityRuleset { properties }
    }
}

/// The [`AWS::Glue::Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-database.html) resource type.
#[derive(Debug, Default)]
pub struct Database {
    properties: DatabaseProperties
}

/// Properties for the `Database` resource.
#[derive(Debug, Default)]
pub struct DatabaseProperties {
    /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-database.html#cfn-glue-database-catalogid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub catalog_id: ::Value<String>,
    /// Property [`DatabaseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-database.html#cfn-glue-database-databaseinput).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub database_input: ::Value<self::database::DatabaseInput>,
}

impl ::serde::Serialize for DatabaseProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseInput", &self.database_input)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DatabaseProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DatabaseProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DatabaseProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut catalog_id: Option<::Value<String>> = None;
                let mut database_input: Option<::Value<self::database::DatabaseInput>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CatalogId" => {
                            catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatabaseInput" => {
                            database_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DatabaseProperties {
                    catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                    database_input: database_input.ok_or(::serde::de::Error::missing_field("DatabaseInput"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Database {
    type Properties = DatabaseProperties;
    const TYPE: &'static str = "AWS::Glue::Database";
    fn properties(&self) -> &DatabaseProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DatabaseProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Database {}

impl From<DatabaseProperties> for Database {
    fn from(properties: DatabaseProperties) -> Database {
        Database { properties }
    }
}

/// The [`AWS::Glue::DevEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html) resource type.
#[derive(Debug, Default)]
pub struct DevEndpoint {
    properties: DevEndpointProperties
}

/// Properties for the `DevEndpoint` resource.
#[derive(Debug, Default)]
pub struct DevEndpointProperties {
    /// Property [`Arguments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-arguments).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub arguments: Option<::Value<::json::Value>>,
    /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-endpointname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub endpoint_name: Option<::Value<String>>,
    /// Property [`ExtraJarsS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-extrajarss3path).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub extra_jars_s3_path: Option<::Value<String>>,
    /// Property [`ExtraPythonLibsS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-extrapythonlibss3path).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub extra_python_libs_s3_path: Option<::Value<String>>,
    /// Property [`GlueVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-glueversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub glue_version: Option<::Value<String>>,
    /// Property [`NumberOfNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-numberofnodes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub number_of_nodes: Option<::Value<u32>>,
    /// Property [`NumberOfWorkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-numberofworkers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub number_of_workers: Option<::Value<u32>>,
    /// Property [`PublicKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-publickey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub public_key: Option<::Value<String>>,
    /// Property [`PublicKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-publickeys).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub public_keys: Option<::ValueList<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`SecurityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-securityconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_configuration: Option<::Value<String>>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-securitygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-subnetid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`WorkerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-workertype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub worker_type: Option<::Value<String>>,
}

impl ::serde::Serialize for DevEndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref arguments) = self.arguments {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arguments", arguments)?;
        }
        if let Some(ref endpoint_name) = self.endpoint_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", endpoint_name)?;
        }
        if let Some(ref extra_jars_s3_path) = self.extra_jars_s3_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtraJarsS3Path", extra_jars_s3_path)?;
        }
        if let Some(ref extra_python_libs_s3_path) = self.extra_python_libs_s3_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtraPythonLibsS3Path", extra_python_libs_s3_path)?;
        }
        if let Some(ref glue_version) = self.glue_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlueVersion", glue_version)?;
        }
        if let Some(ref number_of_nodes) = self.number_of_nodes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfNodes", number_of_nodes)?;
        }
        if let Some(ref number_of_workers) = self.number_of_workers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfWorkers", number_of_workers)?;
        }
        if let Some(ref public_key) = self.public_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicKey", public_key)?;
        }
        if let Some(ref public_keys) = self.public_keys {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicKeys", public_keys)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref security_configuration) = self.security_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityConfiguration", security_configuration)?;
        }
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref subnet_id) = self.subnet_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", subnet_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref worker_type) = self.worker_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerType", worker_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DevEndpointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DevEndpointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DevEndpointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DevEndpointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut arguments: Option<::Value<::json::Value>> = None;
                let mut endpoint_name: Option<::Value<String>> = None;
                let mut extra_jars_s3_path: Option<::Value<String>> = None;
                let mut extra_python_libs_s3_path: Option<::Value<String>> = None;
                let mut glue_version: Option<::Value<String>> = None;
                let mut number_of_nodes: Option<::Value<u32>> = None;
                let mut number_of_workers: Option<::Value<u32>> = None;
                let mut public_key: Option<::Value<String>> = None;
                let mut public_keys: Option<::ValueList<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut security_configuration: Option<::Value<String>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut subnet_id: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut worker_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Arguments" => {
                            arguments = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointName" => {
                            endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExtraJarsS3Path" => {
                            extra_jars_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExtraPythonLibsS3Path" => {
                            extra_python_libs_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlueVersion" => {
                            glue_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumberOfNodes" => {
                            number_of_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumberOfWorkers" => {
                            number_of_workers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PublicKey" => {
                            public_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PublicKeys" => {
                            public_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityConfiguration" => {
                            security_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetId" => {
                            subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkerType" => {
                            worker_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DevEndpointProperties {
                    arguments: arguments,
                    endpoint_name: endpoint_name,
                    extra_jars_s3_path: extra_jars_s3_path,
                    extra_python_libs_s3_path: extra_python_libs_s3_path,
                    glue_version: glue_version,
                    number_of_nodes: number_of_nodes,
                    number_of_workers: number_of_workers,
                    public_key: public_key,
                    public_keys: public_keys,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    security_configuration: security_configuration,
                    security_group_ids: security_group_ids,
                    subnet_id: subnet_id,
                    tags: tags,
                    worker_type: worker_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DevEndpoint {
    type Properties = DevEndpointProperties;
    const TYPE: &'static str = "AWS::Glue::DevEndpoint";
    fn properties(&self) -> &DevEndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DevEndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DevEndpoint {}

impl From<DevEndpointProperties> for DevEndpoint {
    fn from(properties: DevEndpointProperties) -> DevEndpoint {
        DevEndpoint { properties }
    }
}

/// The [`AWS::Glue::Job`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html) resource type.
#[derive(Debug, Default)]
pub struct Job {
    properties: JobProperties
}

/// Properties for the `Job` resource.
#[derive(Debug, Default)]
pub struct JobProperties {
    /// Property [`AllocatedCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-allocatedcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allocated_capacity: Option<::Value<f64>>,
    /// Property [`Command`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-command).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub command: ::Value<self::job::JobCommand>,
    /// Property [`Connections`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-connections).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connections: Option<::Value<self::job::ConnectionsList>>,
    /// Property [`DefaultArguments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-defaultarguments).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_arguments: Option<::Value<::json::Value>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`ExecutionClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-executionclass).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_class: Option<::Value<String>>,
    /// Property [`ExecutionProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-executionproperty).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_property: Option<::Value<self::job::ExecutionProperty>>,
    /// Property [`GlueVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-glueversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub glue_version: Option<::Value<String>>,
    /// Property [`LogUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-loguri).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_uri: Option<::Value<String>>,
    /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-maxcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_capacity: Option<::Value<f64>>,
    /// Property [`MaxRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-maxretries).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_retries: Option<::Value<f64>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`NonOverridableArguments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-nonoverridablearguments).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub non_overridable_arguments: Option<::Value<::json::Value>>,
    /// Property [`NotificationProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-notificationproperty).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_property: Option<::Value<self::job::NotificationProperty>>,
    /// Property [`NumberOfWorkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-numberofworkers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub number_of_workers: Option<::Value<u32>>,
    /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-role).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role: ::Value<String>,
    /// Property [`SecurityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-securityconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_configuration: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-timeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout: Option<::Value<u32>>,
    /// Property [`WorkerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-workertype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub worker_type: Option<::Value<String>>,
}

impl ::serde::Serialize for JobProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allocated_capacity) = self.allocated_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocatedCapacity", allocated_capacity)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", &self.command)?;
        if let Some(ref connections) = self.connections {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Connections", connections)?;
        }
        if let Some(ref default_arguments) = self.default_arguments {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultArguments", default_arguments)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref execution_class) = self.execution_class {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionClass", execution_class)?;
        }
        if let Some(ref execution_property) = self.execution_property {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionProperty", execution_property)?;
        }
        if let Some(ref glue_version) = self.glue_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlueVersion", glue_version)?;
        }
        if let Some(ref log_uri) = self.log_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogUri", log_uri)?;
        }
        if let Some(ref max_capacity) = self.max_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", max_capacity)?;
        }
        if let Some(ref max_retries) = self.max_retries {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRetries", max_retries)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref non_overridable_arguments) = self.non_overridable_arguments {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NonOverridableArguments", non_overridable_arguments)?;
        }
        if let Some(ref notification_property) = self.notification_property {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationProperty", notification_property)?;
        }
        if let Some(ref number_of_workers) = self.number_of_workers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfWorkers", number_of_workers)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        if let Some(ref security_configuration) = self.security_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityConfiguration", security_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timeout) = self.timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
        }
        if let Some(ref worker_type) = self.worker_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerType", worker_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for JobProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<JobProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JobProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type JobProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allocated_capacity: Option<::Value<f64>> = None;
                let mut command: Option<::Value<self::job::JobCommand>> = None;
                let mut connections: Option<::Value<self::job::ConnectionsList>> = None;
                let mut default_arguments: Option<::Value<::json::Value>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut execution_class: Option<::Value<String>> = None;
                let mut execution_property: Option<::Value<self::job::ExecutionProperty>> = None;
                let mut glue_version: Option<::Value<String>> = None;
                let mut log_uri: Option<::Value<String>> = None;
                let mut max_capacity: Option<::Value<f64>> = None;
                let mut max_retries: Option<::Value<f64>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut non_overridable_arguments: Option<::Value<::json::Value>> = None;
                let mut notification_property: Option<::Value<self::job::NotificationProperty>> = None;
                let mut number_of_workers: Option<::Value<u32>> = None;
                let mut role: Option<::Value<String>> = None;
                let mut security_configuration: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut timeout: Option<::Value<u32>> = None;
                let mut worker_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllocatedCapacity" => {
                            allocated_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Command" => {
                            command = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Connections" => {
                            connections = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultArguments" => {
                            default_arguments = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionClass" => {
                            execution_class = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionProperty" => {
                            execution_property = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlueVersion" => {
                            glue_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogUri" => {
                            log_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxCapacity" => {
                            max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxRetries" => {
                            max_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NonOverridableArguments" => {
                            non_overridable_arguments = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationProperty" => {
                            notification_property = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumberOfWorkers" => {
                            number_of_workers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Role" => {
                            role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityConfiguration" => {
                            security_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Timeout" => {
                            timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkerType" => {
                            worker_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(JobProperties {
                    allocated_capacity: allocated_capacity,
                    command: command.ok_or(::serde::de::Error::missing_field("Command"))?,
                    connections: connections,
                    default_arguments: default_arguments,
                    description: description,
                    execution_class: execution_class,
                    execution_property: execution_property,
                    glue_version: glue_version,
                    log_uri: log_uri,
                    max_capacity: max_capacity,
                    max_retries: max_retries,
                    name: name,
                    non_overridable_arguments: non_overridable_arguments,
                    notification_property: notification_property,
                    number_of_workers: number_of_workers,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                    security_configuration: security_configuration,
                    tags: tags,
                    timeout: timeout,
                    worker_type: worker_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Job {
    type Properties = JobProperties;
    const TYPE: &'static str = "AWS::Glue::Job";
    fn properties(&self) -> &JobProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Job {}

impl From<JobProperties> for Job {
    fn from(properties: JobProperties) -> Job {
        Job { properties }
    }
}

/// The [`AWS::Glue::MLTransform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html) resource type.
#[derive(Debug, Default)]
pub struct MLTransform {
    properties: MLTransformProperties
}

/// Properties for the `MLTransform` resource.
#[derive(Debug, Default)]
pub struct MLTransformProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`GlueVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-glueversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub glue_version: Option<::Value<String>>,
    /// Property [`InputRecordTables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-inputrecordtables).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub input_record_tables: ::Value<self::ml_transform::InputRecordTables>,
    /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-maxcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_capacity: Option<::Value<f64>>,
    /// Property [`MaxRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-maxretries).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_retries: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`NumberOfWorkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-numberofworkers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub number_of_workers: Option<::Value<u32>>,
    /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-role).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-timeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout: Option<::Value<u32>>,
    /// Property [`TransformEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-transformencryption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub transform_encryption: Option<::Value<self::ml_transform::TransformEncryption>>,
    /// Property [`TransformParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-transformparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub transform_parameters: ::Value<self::ml_transform::TransformParameters>,
    /// Property [`WorkerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-mltransform.html#cfn-glue-mltransform-workertype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub worker_type: Option<::Value<String>>,
}

impl ::serde::Serialize for MLTransformProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref glue_version) = self.glue_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlueVersion", glue_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputRecordTables", &self.input_record_tables)?;
        if let Some(ref max_capacity) = self.max_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", max_capacity)?;
        }
        if let Some(ref max_retries) = self.max_retries {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRetries", max_retries)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref number_of_workers) = self.number_of_workers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfWorkers", number_of_workers)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timeout) = self.timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
        }
        if let Some(ref transform_encryption) = self.transform_encryption {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransformEncryption", transform_encryption)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransformParameters", &self.transform_parameters)?;
        if let Some(ref worker_type) = self.worker_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerType", worker_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MLTransformProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MLTransformProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MLTransformProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MLTransformProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut glue_version: Option<::Value<String>> = None;
                let mut input_record_tables: Option<::Value<self::ml_transform::InputRecordTables>> = None;
                let mut max_capacity: Option<::Value<f64>> = None;
                let mut max_retries: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut number_of_workers: Option<::Value<u32>> = None;
                let mut role: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut timeout: Option<::Value<u32>> = None;
                let mut transform_encryption: Option<::Value<self::ml_transform::TransformEncryption>> = None;
                let mut transform_parameters: Option<::Value<self::ml_transform::TransformParameters>> = None;
                let mut worker_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlueVersion" => {
                            glue_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputRecordTables" => {
                            input_record_tables = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxCapacity" => {
                            max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxRetries" => {
                            max_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumberOfWorkers" => {
                            number_of_workers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Role" => {
                            role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Timeout" => {
                            timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransformEncryption" => {
                            transform_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransformParameters" => {
                            transform_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkerType" => {
                            worker_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MLTransformProperties {
                    description: description,
                    glue_version: glue_version,
                    input_record_tables: input_record_tables.ok_or(::serde::de::Error::missing_field("InputRecordTables"))?,
                    max_capacity: max_capacity,
                    max_retries: max_retries,
                    name: name,
                    number_of_workers: number_of_workers,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                    tags: tags,
                    timeout: timeout,
                    transform_encryption: transform_encryption,
                    transform_parameters: transform_parameters.ok_or(::serde::de::Error::missing_field("TransformParameters"))?,
                    worker_type: worker_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MLTransform {
    type Properties = MLTransformProperties;
    const TYPE: &'static str = "AWS::Glue::MLTransform";
    fn properties(&self) -> &MLTransformProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MLTransformProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MLTransform {}

impl From<MLTransformProperties> for MLTransform {
    fn from(properties: MLTransformProperties) -> MLTransform {
        MLTransform { properties }
    }
}

/// The [`AWS::Glue::Partition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-partition.html) resource type.
#[derive(Debug, Default)]
pub struct Partition {
    properties: PartitionProperties
}

/// Properties for the `Partition` resource.
#[derive(Debug, Default)]
pub struct PartitionProperties {
    /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-partition.html#cfn-glue-partition-catalogid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub catalog_id: ::Value<String>,
    /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-partition.html#cfn-glue-partition-databasename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub database_name: ::Value<String>,
    /// Property [`PartitionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-partition.html#cfn-glue-partition-partitioninput).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub partition_input: ::Value<self::partition::PartitionInput>,
    /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-partition.html#cfn-glue-partition-tablename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub table_name: ::Value<String>,
}

impl ::serde::Serialize for PartitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionInput", &self.partition_input)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PartitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PartitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PartitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PartitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut catalog_id: Option<::Value<String>> = None;
                let mut database_name: Option<::Value<String>> = None;
                let mut partition_input: Option<::Value<self::partition::PartitionInput>> = None;
                let mut table_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CatalogId" => {
                            catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatabaseName" => {
                            database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PartitionInput" => {
                            partition_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableName" => {
                            table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PartitionProperties {
                    catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                    database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                    partition_input: partition_input.ok_or(::serde::de::Error::missing_field("PartitionInput"))?,
                    table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Partition {
    type Properties = PartitionProperties;
    const TYPE: &'static str = "AWS::Glue::Partition";
    fn properties(&self) -> &PartitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PartitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Partition {}

impl From<PartitionProperties> for Partition {
    fn from(properties: PartitionProperties) -> Partition {
        Partition { properties }
    }
}

/// The [`AWS::Glue::Registry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-registry.html) resource type.
#[derive(Debug, Default)]
pub struct Registry {
    properties: RegistryProperties
}

/// Properties for the `Registry` resource.
#[derive(Debug, Default)]
pub struct RegistryProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-registry.html#cfn-glue-registry-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-registry.html#cfn-glue-registry-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-registry.html#cfn-glue-registry-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for RegistryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RegistryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RegistryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RegistryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RegistryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RegistryProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Registry {
    type Properties = RegistryProperties;
    const TYPE: &'static str = "AWS::Glue::Registry";
    fn properties(&self) -> &RegistryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RegistryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Registry {}

impl From<RegistryProperties> for Registry {
    fn from(properties: RegistryProperties) -> Registry {
        Registry { properties }
    }
}

/// The [`AWS::Glue::Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schema.html) resource type.
#[derive(Debug, Default)]
pub struct Schema {
    properties: SchemaProperties
}

/// Properties for the `Schema` resource.
#[derive(Debug, Default)]
pub struct SchemaProperties {
    /// Property [`CheckpointVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schema.html#cfn-glue-schema-checkpointversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub checkpoint_version: Option<::Value<self::schema::SchemaVersion>>,
    /// Property [`Compatibility`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schema.html#cfn-glue-schema-compatibility).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub compatibility: ::Value<String>,
    /// Property [`DataFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schema.html#cfn-glue-schema-dataformat).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_format: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schema.html#cfn-glue-schema-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schema.html#cfn-glue-schema-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Registry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schema.html#cfn-glue-schema-registry).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub registry: Option<::Value<self::schema::Registry>>,
    /// Property [`SchemaDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schema.html#cfn-glue-schema-schemadefinition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub schema_definition: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schema.html#cfn-glue-schema-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SchemaProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref checkpoint_version) = self.checkpoint_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CheckpointVersion", checkpoint_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compatibility", &self.compatibility)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataFormat", &self.data_format)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref registry) = self.registry {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Registry", registry)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaDefinition", &self.schema_definition)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SchemaProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SchemaProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SchemaProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut checkpoint_version: Option<::Value<self::schema::SchemaVersion>> = None;
                let mut compatibility: Option<::Value<String>> = None;
                let mut data_format: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut registry: Option<::Value<self::schema::Registry>> = None;
                let mut schema_definition: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CheckpointVersion" => {
                            checkpoint_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Compatibility" => {
                            compatibility = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataFormat" => {
                            data_format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Registry" => {
                            registry = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaDefinition" => {
                            schema_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SchemaProperties {
                    checkpoint_version: checkpoint_version,
                    compatibility: compatibility.ok_or(::serde::de::Error::missing_field("Compatibility"))?,
                    data_format: data_format.ok_or(::serde::de::Error::missing_field("DataFormat"))?,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    registry: registry,
                    schema_definition: schema_definition.ok_or(::serde::de::Error::missing_field("SchemaDefinition"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Schema {
    type Properties = SchemaProperties;
    const TYPE: &'static str = "AWS::Glue::Schema";
    fn properties(&self) -> &SchemaProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SchemaProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Schema {}

impl From<SchemaProperties> for Schema {
    fn from(properties: SchemaProperties) -> Schema {
        Schema { properties }
    }
}

/// The [`AWS::Glue::SchemaVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schemaversion.html) resource type.
#[derive(Debug, Default)]
pub struct SchemaVersion {
    properties: SchemaVersionProperties
}

/// Properties for the `SchemaVersion` resource.
#[derive(Debug, Default)]
pub struct SchemaVersionProperties {
    /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schemaversion.html#cfn-glue-schemaversion-schema).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub schema: ::Value<self::schema_version::Schema>,
    /// Property [`SchemaDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schemaversion.html#cfn-glue-schemaversion-schemadefinition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub schema_definition: ::Value<String>,
}

impl ::serde::Serialize for SchemaVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", &self.schema)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaDefinition", &self.schema_definition)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SchemaVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SchemaVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SchemaVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut schema: Option<::Value<self::schema_version::Schema>> = None;
                let mut schema_definition: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Schema" => {
                            schema = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaDefinition" => {
                            schema_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SchemaVersionProperties {
                    schema: schema.ok_or(::serde::de::Error::missing_field("Schema"))?,
                    schema_definition: schema_definition.ok_or(::serde::de::Error::missing_field("SchemaDefinition"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SchemaVersion {
    type Properties = SchemaVersionProperties;
    const TYPE: &'static str = "AWS::Glue::SchemaVersion";
    fn properties(&self) -> &SchemaVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SchemaVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SchemaVersion {}

impl From<SchemaVersionProperties> for SchemaVersion {
    fn from(properties: SchemaVersionProperties) -> SchemaVersion {
        SchemaVersion { properties }
    }
}

/// The [`AWS::Glue::SchemaVersionMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schemaversionmetadata.html) resource type.
#[derive(Debug, Default)]
pub struct SchemaVersionMetadata {
    properties: SchemaVersionMetadataProperties
}

/// Properties for the `SchemaVersionMetadata` resource.
#[derive(Debug, Default)]
pub struct SchemaVersionMetadataProperties {
    /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schemaversionmetadata.html#cfn-glue-schemaversionmetadata-key).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key: ::Value<String>,
    /// Property [`SchemaVersionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schemaversionmetadata.html#cfn-glue-schemaversionmetadata-schemaversionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub schema_version_id: ::Value<String>,
    /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-schemaversionmetadata.html#cfn-glue-schemaversionmetadata-value).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub value: ::Value<String>,
}

impl ::serde::Serialize for SchemaVersionMetadataProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaVersionId", &self.schema_version_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SchemaVersionMetadataProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaVersionMetadataProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SchemaVersionMetadataProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SchemaVersionMetadataProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut key: Option<::Value<String>> = None;
                let mut schema_version_id: Option<::Value<String>> = None;
                let mut value: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Key" => {
                            key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaVersionId" => {
                            schema_version_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Value" => {
                            value = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SchemaVersionMetadataProperties {
                    key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                    schema_version_id: schema_version_id.ok_or(::serde::de::Error::missing_field("SchemaVersionId"))?,
                    value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SchemaVersionMetadata {
    type Properties = SchemaVersionMetadataProperties;
    const TYPE: &'static str = "AWS::Glue::SchemaVersionMetadata";
    fn properties(&self) -> &SchemaVersionMetadataProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SchemaVersionMetadataProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SchemaVersionMetadata {}

impl From<SchemaVersionMetadataProperties> for SchemaVersionMetadata {
    fn from(properties: SchemaVersionMetadataProperties) -> SchemaVersionMetadata {
        SchemaVersionMetadata { properties }
    }
}

/// The [`AWS::Glue::SecurityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-securityconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct SecurityConfiguration {
    properties: SecurityConfigurationProperties
}

/// Properties for the `SecurityConfiguration` resource.
#[derive(Debug, Default)]
pub struct SecurityConfigurationProperties {
    /// Property [`EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-securityconfiguration.html#cfn-glue-securityconfiguration-encryptionconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_configuration: ::Value<self::security_configuration::EncryptionConfiguration>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-securityconfiguration.html#cfn-glue-securityconfiguration-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for SecurityConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", &self.encryption_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecurityConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecurityConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut encryption_configuration: Option<::Value<self::security_configuration::EncryptionConfiguration>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EncryptionConfiguration" => {
                            encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SecurityConfigurationProperties {
                    encryption_configuration: encryption_configuration.ok_or(::serde::de::Error::missing_field("EncryptionConfiguration"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SecurityConfiguration {
    type Properties = SecurityConfigurationProperties;
    const TYPE: &'static str = "AWS::Glue::SecurityConfiguration";
    fn properties(&self) -> &SecurityConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityConfiguration {}

impl From<SecurityConfigurationProperties> for SecurityConfiguration {
    fn from(properties: SecurityConfigurationProperties) -> SecurityConfiguration {
        SecurityConfiguration { properties }
    }
}

/// The [`AWS::Glue::Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-table.html) resource type.
#[derive(Debug, Default)]
pub struct Table {
    properties: TableProperties
}

/// Properties for the `Table` resource.
#[derive(Debug, Default)]
pub struct TableProperties {
    /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-table.html#cfn-glue-table-catalogid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub catalog_id: ::Value<String>,
    /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-table.html#cfn-glue-table-databasename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub database_name: ::Value<String>,
    /// Property [`OpenTableFormatInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-table.html#cfn-glue-table-opentableformatinput).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub open_table_format_input: Option<::Value<self::table::OpenTableFormatInput>>,
    /// Property [`TableInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-table.html#cfn-glue-table-tableinput).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub table_input: ::Value<self::table::TableInput>,
}

impl ::serde::Serialize for TableProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
        if let Some(ref open_table_format_input) = self.open_table_format_input {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenTableFormatInput", open_table_format_input)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableInput", &self.table_input)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TableProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TableProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TableProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TableProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut catalog_id: Option<::Value<String>> = None;
                let mut database_name: Option<::Value<String>> = None;
                let mut open_table_format_input: Option<::Value<self::table::OpenTableFormatInput>> = None;
                let mut table_input: Option<::Value<self::table::TableInput>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CatalogId" => {
                            catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatabaseName" => {
                            database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OpenTableFormatInput" => {
                            open_table_format_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableInput" => {
                            table_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TableProperties {
                    catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                    database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                    open_table_format_input: open_table_format_input,
                    table_input: table_input.ok_or(::serde::de::Error::missing_field("TableInput"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Table {
    type Properties = TableProperties;
    const TYPE: &'static str = "AWS::Glue::Table";
    fn properties(&self) -> &TableProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TableProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Table {}

impl From<TableProperties> for Table {
    fn from(properties: TableProperties) -> Table {
        Table { properties }
    }
}

/// The [`AWS::Glue::TableOptimizer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-tableoptimizer.html) resource type.
#[derive(Debug, Default)]
pub struct TableOptimizer {
    properties: TableOptimizerProperties
}

/// Properties for the `TableOptimizer` resource.
#[derive(Debug, Default)]
pub struct TableOptimizerProperties {
    /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-tableoptimizer.html#cfn-glue-tableoptimizer-catalogid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub catalog_id: ::Value<String>,
    /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-tableoptimizer.html#cfn-glue-tableoptimizer-databasename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub database_name: ::Value<String>,
    /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-tableoptimizer.html#cfn-glue-tableoptimizer-tablename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub table_name: ::Value<String>,
    /// Property [`TableOptimizerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-tableoptimizer.html#cfn-glue-tableoptimizer-tableoptimizerconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub table_optimizer_configuration: ::Value<self::table_optimizer::TableOptimizerConfiguration>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-tableoptimizer.html#cfn-glue-tableoptimizer-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for TableOptimizerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableOptimizerConfiguration", &self.table_optimizer_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TableOptimizerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TableOptimizerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TableOptimizerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TableOptimizerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut catalog_id: Option<::Value<String>> = None;
                let mut database_name: Option<::Value<String>> = None;
                let mut table_name: Option<::Value<String>> = None;
                let mut table_optimizer_configuration: Option<::Value<self::table_optimizer::TableOptimizerConfiguration>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CatalogId" => {
                            catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatabaseName" => {
                            database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableName" => {
                            table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableOptimizerConfiguration" => {
                            table_optimizer_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TableOptimizerProperties {
                    catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                    database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                    table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    table_optimizer_configuration: table_optimizer_configuration.ok_or(::serde::de::Error::missing_field("TableOptimizerConfiguration"))?,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TableOptimizer {
    type Properties = TableOptimizerProperties;
    const TYPE: &'static str = "AWS::Glue::TableOptimizer";
    fn properties(&self) -> &TableOptimizerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TableOptimizerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TableOptimizer {}

impl From<TableOptimizerProperties> for TableOptimizer {
    fn from(properties: TableOptimizerProperties) -> TableOptimizer {
        TableOptimizer { properties }
    }
}

/// The [`AWS::Glue::Trigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html) resource type.
#[derive(Debug, Default)]
pub struct Trigger {
    properties: TriggerProperties
}

/// Properties for the `Trigger` resource.
#[derive(Debug, Default)]
pub struct TriggerProperties {
    /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-actions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions: ::ValueList<self::trigger::Action>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EventBatchingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-eventbatchingcondition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_batching_condition: Option<::Value<self::trigger::EventBatchingCondition>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-predicate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub predicate: Option<::Value<self::trigger::Predicate>>,
    /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-schedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule: Option<::Value<String>>,
    /// Property [`StartOnCreation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-startoncreation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub start_on_creation: Option<::Value<bool>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
    /// Property [`WorkflowName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-workflowname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workflow_name: Option<::Value<String>>,
}

impl ::serde::Serialize for TriggerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref event_batching_condition) = self.event_batching_condition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventBatchingCondition", event_batching_condition)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref predicate) = self.predicate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Predicate", predicate)?;
        }
        if let Some(ref schedule) = self.schedule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", schedule)?;
        }
        if let Some(ref start_on_creation) = self.start_on_creation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartOnCreation", start_on_creation)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        if let Some(ref workflow_name) = self.workflow_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkflowName", workflow_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TriggerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TriggerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TriggerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TriggerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions: Option<::ValueList<self::trigger::Action>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut event_batching_condition: Option<::Value<self::trigger::EventBatchingCondition>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut predicate: Option<::Value<self::trigger::Predicate>> = None;
                let mut schedule: Option<::Value<String>> = None;
                let mut start_on_creation: Option<::Value<bool>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut r#type: Option<::Value<String>> = None;
                let mut workflow_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Actions" => {
                            actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventBatchingCondition" => {
                            event_batching_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Predicate" => {
                            predicate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedule" => {
                            schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartOnCreation" => {
                            start_on_creation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkflowName" => {
                            workflow_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TriggerProperties {
                    actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                    description: description,
                    event_batching_condition: event_batching_condition,
                    name: name,
                    predicate: predicate,
                    schedule: schedule,
                    start_on_creation: start_on_creation,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    workflow_name: workflow_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Trigger {
    type Properties = TriggerProperties;
    const TYPE: &'static str = "AWS::Glue::Trigger";
    fn properties(&self) -> &TriggerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TriggerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Trigger {}

impl From<TriggerProperties> for Trigger {
    fn from(properties: TriggerProperties) -> Trigger {
        Trigger { properties }
    }
}

/// The [`AWS::Glue::Workflow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-workflow.html) resource type.
#[derive(Debug, Default)]
pub struct Workflow {
    properties: WorkflowProperties
}

/// Properties for the `Workflow` resource.
#[derive(Debug, Default)]
pub struct WorkflowProperties {
    /// Property [`DefaultRunProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-workflow.html#cfn-glue-workflow-defaultrunproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_run_properties: Option<::Value<::json::Value>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-workflow.html#cfn-glue-workflow-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`MaxConcurrentRuns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-workflow.html#cfn-glue-workflow-maxconcurrentruns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_concurrent_runs: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-workflow.html#cfn-glue-workflow-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-workflow.html#cfn-glue-workflow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for WorkflowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref default_run_properties) = self.default_run_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultRunProperties", default_run_properties)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref max_concurrent_runs) = self.max_concurrent_runs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrentRuns", max_concurrent_runs)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
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
                let mut default_run_properties: Option<::Value<::json::Value>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut max_concurrent_runs: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DefaultRunProperties" => {
                            default_run_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxConcurrentRuns" => {
                            max_concurrent_runs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkflowProperties {
                    default_run_properties: default_run_properties,
                    description: description,
                    max_concurrent_runs: max_concurrent_runs,
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Workflow {
    type Properties = WorkflowProperties;
    const TYPE: &'static str = "AWS::Glue::Workflow";
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

pub mod classifier {
    //! Property types for the `Classifier` resource.

    /// The [`AWS::Glue::Classifier.CsvClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html) property type.
    #[derive(Debug, Default)]
    pub struct CsvClassifier {
        /// Property [`AllowSingleColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html#cfn-glue-classifier-csvclassifier-allowsinglecolumn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_single_column: Option<::Value<bool>>,
        /// Property [`ContainsCustomDatatype`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html#cfn-glue-classifier-csvclassifier-containscustomdatatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contains_custom_datatype: Option<::ValueList<String>>,
        /// Property [`ContainsHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html#cfn-glue-classifier-csvclassifier-containsheader).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contains_header: Option<::Value<String>>,
        /// Property [`CustomDatatypeConfigured`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html#cfn-glue-classifier-csvclassifier-customdatatypeconfigured).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_datatype_configured: Option<::Value<bool>>,
        /// Property [`Delimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html#cfn-glue-classifier-csvclassifier-delimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delimiter: Option<::Value<String>>,
        /// Property [`DisableValueTrimming`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html#cfn-glue-classifier-csvclassifier-disablevaluetrimming).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_value_trimming: Option<::Value<bool>>,
        /// Property [`Header`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html#cfn-glue-classifier-csvclassifier-header).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header: Option<::ValueList<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html#cfn-glue-classifier-csvclassifier-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`QuoteSymbol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-csvclassifier.html#cfn-glue-classifier-csvclassifier-quotesymbol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quote_symbol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CsvClassifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_single_column) = self.allow_single_column {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowSingleColumn", allow_single_column)?;
            }
            if let Some(ref contains_custom_datatype) = self.contains_custom_datatype {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainsCustomDatatype", contains_custom_datatype)?;
            }
            if let Some(ref contains_header) = self.contains_header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainsHeader", contains_header)?;
            }
            if let Some(ref custom_datatype_configured) = self.custom_datatype_configured {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomDatatypeConfigured", custom_datatype_configured)?;
            }
            if let Some(ref delimiter) = self.delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Delimiter", delimiter)?;
            }
            if let Some(ref disable_value_trimming) = self.disable_value_trimming {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableValueTrimming", disable_value_trimming)?;
            }
            if let Some(ref header) = self.header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Header", header)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref quote_symbol) = self.quote_symbol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuoteSymbol", quote_symbol)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CsvClassifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CsvClassifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CsvClassifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CsvClassifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_single_column: Option<::Value<bool>> = None;
                    let mut contains_custom_datatype: Option<::ValueList<String>> = None;
                    let mut contains_header: Option<::Value<String>> = None;
                    let mut custom_datatype_configured: Option<::Value<bool>> = None;
                    let mut delimiter: Option<::Value<String>> = None;
                    let mut disable_value_trimming: Option<::Value<bool>> = None;
                    let mut header: Option<::ValueList<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut quote_symbol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowSingleColumn" => {
                                allow_single_column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainsCustomDatatype" => {
                                contains_custom_datatype = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainsHeader" => {
                                contains_header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomDatatypeConfigured" => {
                                custom_datatype_configured = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Delimiter" => {
                                delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisableValueTrimming" => {
                                disable_value_trimming = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Header" => {
                                header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QuoteSymbol" => {
                                quote_symbol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CsvClassifier {
                        allow_single_column: allow_single_column,
                        contains_custom_datatype: contains_custom_datatype,
                        contains_header: contains_header,
                        custom_datatype_configured: custom_datatype_configured,
                        delimiter: delimiter,
                        disable_value_trimming: disable_value_trimming,
                        header: header,
                        name: name,
                        quote_symbol: quote_symbol,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Classifier.GrokClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-grokclassifier.html) property type.
    #[derive(Debug, Default)]
    pub struct GrokClassifier {
        /// Property [`Classification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-grokclassifier.html#cfn-glue-classifier-grokclassifier-classification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub classification: ::Value<String>,
        /// Property [`CustomPatterns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-grokclassifier.html#cfn-glue-classifier-grokclassifier-custompatterns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_patterns: Option<::Value<String>>,
        /// Property [`GrokPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-grokclassifier.html#cfn-glue-classifier-grokclassifier-grokpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub grok_pattern: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-grokclassifier.html#cfn-glue-classifier-grokclassifier-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GrokClassifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classification", &self.classification)?;
            if let Some(ref custom_patterns) = self.custom_patterns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomPatterns", custom_patterns)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GrokPattern", &self.grok_pattern)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrokClassifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrokClassifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrokClassifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrokClassifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut classification: Option<::Value<String>> = None;
                    let mut custom_patterns: Option<::Value<String>> = None;
                    let mut grok_pattern: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Classification" => {
                                classification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomPatterns" => {
                                custom_patterns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GrokPattern" => {
                                grok_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrokClassifier {
                        classification: classification.ok_or(::serde::de::Error::missing_field("Classification"))?,
                        custom_patterns: custom_patterns,
                        grok_pattern: grok_pattern.ok_or(::serde::de::Error::missing_field("GrokPattern"))?,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Classifier.JsonClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-jsonclassifier.html) property type.
    #[derive(Debug, Default)]
    pub struct JsonClassifier {
        /// Property [`JsonPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-jsonclassifier.html#cfn-glue-classifier-jsonclassifier-jsonpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_path: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-jsonclassifier.html#cfn-glue-classifier-jsonclassifier-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for JsonClassifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonPath", &self.json_path)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JsonClassifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JsonClassifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JsonClassifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JsonClassifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut json_path: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JsonPath" => {
                                json_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JsonClassifier {
                        json_path: json_path.ok_or(::serde::de::Error::missing_field("JsonPath"))?,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Classifier.XMLClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-xmlclassifier.html) property type.
    #[derive(Debug, Default)]
    pub struct XMLClassifier {
        /// Property [`Classification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-xmlclassifier.html#cfn-glue-classifier-xmlclassifier-classification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub classification: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-xmlclassifier.html#cfn-glue-classifier-xmlclassifier-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`RowTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-xmlclassifier.html#cfn-glue-classifier-xmlclassifier-rowtag).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub row_tag: ::Value<String>,
    }

    impl ::codec::SerializeValue for XMLClassifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classification", &self.classification)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RowTag", &self.row_tag)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for XMLClassifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<XMLClassifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = XMLClassifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type XMLClassifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut classification: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut row_tag: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Classification" => {
                                classification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RowTag" => {
                                row_tag = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(XMLClassifier {
                        classification: classification.ok_or(::serde::de::Error::missing_field("Classification"))?,
                        name: name,
                        row_tag: row_tag.ok_or(::serde::de::Error::missing_field("RowTag"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod connection {
    //! Property types for the `Connection` resource.

    /// The [`AWS::Glue::Connection.ConnectionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectionInput {
        /// Property [`ConnectionProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html#cfn-glue-connection-connectioninput-connectionproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_properties: Option<::Value<::json::Value>>,
        /// Property [`ConnectionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html#cfn-glue-connection-connectioninput-connectiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_type: ::Value<String>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html#cfn-glue-connection-connectioninput-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`MatchCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html#cfn-glue-connection-connectioninput-matchcriteria).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_criteria: Option<::ValueList<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html#cfn-glue-connection-connectioninput-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`PhysicalConnectionRequirements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html#cfn-glue-connection-connectioninput-physicalconnectionrequirements).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub physical_connection_requirements: Option<::Value<PhysicalConnectionRequirements>>,
    }

    impl ::codec::SerializeValue for ConnectionInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_properties) = self.connection_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionProperties", connection_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionType", &self.connection_type)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref match_criteria) = self.match_criteria {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchCriteria", match_criteria)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref physical_connection_requirements) = self.physical_connection_requirements {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhysicalConnectionRequirements", physical_connection_requirements)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectionInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_properties: Option<::Value<::json::Value>> = None;
                    let mut connection_type: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut match_criteria: Option<::ValueList<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut physical_connection_requirements: Option<::Value<PhysicalConnectionRequirements>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionProperties" => {
                                connection_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectionType" => {
                                connection_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchCriteria" => {
                                match_criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PhysicalConnectionRequirements" => {
                                physical_connection_requirements = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionInput {
                        connection_properties: connection_properties,
                        connection_type: connection_type.ok_or(::serde::de::Error::missing_field("ConnectionType"))?,
                        description: description,
                        match_criteria: match_criteria,
                        name: name,
                        physical_connection_requirements: physical_connection_requirements,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Connection.PhysicalConnectionRequirements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-physicalconnectionrequirements.html) property type.
    #[derive(Debug, Default)]
    pub struct PhysicalConnectionRequirements {
        /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-physicalconnectionrequirements.html#cfn-glue-connection-physicalconnectionrequirements-availabilityzone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_zone: Option<::Value<String>>,
        /// Property [`SecurityGroupIdList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-physicalconnectionrequirements.html#cfn-glue-connection-physicalconnectionrequirements-securitygroupidlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_id_list: Option<::ValueList<String>>,
        /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-physicalconnectionrequirements.html#cfn-glue-connection-physicalconnectionrequirements-subnetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PhysicalConnectionRequirements {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_zone) = self.availability_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
            }
            if let Some(ref security_group_id_list) = self.security_group_id_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIdList", security_group_id_list)?;
            }
            if let Some(ref subnet_id) = self.subnet_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", subnet_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PhysicalConnectionRequirements {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PhysicalConnectionRequirements, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PhysicalConnectionRequirements;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PhysicalConnectionRequirements")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone: Option<::Value<String>> = None;
                    let mut security_group_id_list: Option<::ValueList<String>> = None;
                    let mut subnet_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIdList" => {
                                security_group_id_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetId" => {
                                subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PhysicalConnectionRequirements {
                        availability_zone: availability_zone,
                        security_group_id_list: security_group_id_list,
                        subnet_id: subnet_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod crawler {
    //! Property types for the `Crawler` resource.

    /// The [`AWS::Glue::Crawler.CatalogTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-catalogtarget.html) property type.
    #[derive(Debug, Default)]
    pub struct CatalogTarget {
        /// Property [`ConnectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-catalogtarget.html#cfn-glue-crawler-catalogtarget-connectionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_name: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-catalogtarget.html#cfn-glue-crawler-catalogtarget-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`DlqEventQueueArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-catalogtarget.html#cfn-glue-crawler-catalogtarget-dlqeventqueuearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dlq_event_queue_arn: Option<::Value<String>>,
        /// Property [`EventQueueArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-catalogtarget.html#cfn-glue-crawler-catalogtarget-eventqueuearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_queue_arn: Option<::Value<String>>,
        /// Property [`Tables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-catalogtarget.html#cfn-glue-crawler-catalogtarget-tables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tables: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CatalogTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_name) = self.connection_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionName", connection_name)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref dlq_event_queue_arn) = self.dlq_event_queue_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DlqEventQueueArn", dlq_event_queue_arn)?;
            }
            if let Some(ref event_queue_arn) = self.event_queue_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventQueueArn", event_queue_arn)?;
            }
            if let Some(ref tables) = self.tables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tables", tables)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CatalogTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CatalogTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CatalogTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CatalogTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_name: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut dlq_event_queue_arn: Option<::Value<String>> = None;
                    let mut event_queue_arn: Option<::Value<String>> = None;
                    let mut tables: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionName" => {
                                connection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DlqEventQueueArn" => {
                                dlq_event_queue_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventQueueArn" => {
                                event_queue_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tables" => {
                                tables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CatalogTarget {
                        connection_name: connection_name,
                        database_name: database_name,
                        dlq_event_queue_arn: dlq_event_queue_arn,
                        event_queue_arn: event_queue_arn,
                        tables: tables,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.DeltaTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-deltatarget.html) property type.
    #[derive(Debug, Default)]
    pub struct DeltaTarget {
        /// Property [`ConnectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-deltatarget.html#cfn-glue-crawler-deltatarget-connectionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_name: Option<::Value<String>>,
        /// Property [`CreateNativeDeltaTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-deltatarget.html#cfn-glue-crawler-deltatarget-createnativedeltatable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub create_native_delta_table: Option<::Value<bool>>,
        /// Property [`DeltaTables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-deltatarget.html#cfn-glue-crawler-deltatarget-deltatables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delta_tables: Option<::ValueList<String>>,
        /// Property [`WriteManifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-deltatarget.html#cfn-glue-crawler-deltatarget-writemanifest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_manifest: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for DeltaTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_name) = self.connection_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionName", connection_name)?;
            }
            if let Some(ref create_native_delta_table) = self.create_native_delta_table {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateNativeDeltaTable", create_native_delta_table)?;
            }
            if let Some(ref delta_tables) = self.delta_tables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeltaTables", delta_tables)?;
            }
            if let Some(ref write_manifest) = self.write_manifest {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteManifest", write_manifest)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeltaTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeltaTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeltaTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeltaTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_name: Option<::Value<String>> = None;
                    let mut create_native_delta_table: Option<::Value<bool>> = None;
                    let mut delta_tables: Option<::ValueList<String>> = None;
                    let mut write_manifest: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionName" => {
                                connection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreateNativeDeltaTable" => {
                                create_native_delta_table = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeltaTables" => {
                                delta_tables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WriteManifest" => {
                                write_manifest = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeltaTarget {
                        connection_name: connection_name,
                        create_native_delta_table: create_native_delta_table,
                        delta_tables: delta_tables,
                        write_manifest: write_manifest,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.DynamoDBTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-dynamodbtarget.html) property type.
    #[derive(Debug, Default)]
    pub struct DynamoDBTarget {
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-dynamodbtarget.html#cfn-glue-crawler-dynamodbtarget-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DynamoDBTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynamoDBTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamoDBTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamoDBTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamoDBTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamoDBTarget {
                        path: path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.IcebergTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-icebergtarget.html) property type.
    #[derive(Debug, Default)]
    pub struct IcebergTarget {
        /// Property [`ConnectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-icebergtarget.html#cfn-glue-crawler-icebergtarget-connectionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_name: Option<::Value<String>>,
        /// Property [`Exclusions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-icebergtarget.html#cfn-glue-crawler-icebergtarget-exclusions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclusions: Option<::ValueList<String>>,
        /// Property [`MaximumTraversalDepth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-icebergtarget.html#cfn-glue-crawler-icebergtarget-maximumtraversaldepth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_traversal_depth: Option<::Value<u32>>,
        /// Property [`Paths`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-icebergtarget.html#cfn-glue-crawler-icebergtarget-paths).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub paths: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for IcebergTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_name) = self.connection_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionName", connection_name)?;
            }
            if let Some(ref exclusions) = self.exclusions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exclusions", exclusions)?;
            }
            if let Some(ref maximum_traversal_depth) = self.maximum_traversal_depth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumTraversalDepth", maximum_traversal_depth)?;
            }
            if let Some(ref paths) = self.paths {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Paths", paths)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IcebergTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IcebergTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IcebergTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IcebergTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_name: Option<::Value<String>> = None;
                    let mut exclusions: Option<::ValueList<String>> = None;
                    let mut maximum_traversal_depth: Option<::Value<u32>> = None;
                    let mut paths: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionName" => {
                                connection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Exclusions" => {
                                exclusions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumTraversalDepth" => {
                                maximum_traversal_depth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Paths" => {
                                paths = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IcebergTarget {
                        connection_name: connection_name,
                        exclusions: exclusions,
                        maximum_traversal_depth: maximum_traversal_depth,
                        paths: paths,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.JdbcTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-jdbctarget.html) property type.
    #[derive(Debug, Default)]
    pub struct JdbcTarget {
        /// Property [`ConnectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-jdbctarget.html#cfn-glue-crawler-jdbctarget-connectionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_name: Option<::Value<String>>,
        /// Property [`Exclusions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-jdbctarget.html#cfn-glue-crawler-jdbctarget-exclusions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclusions: Option<::ValueList<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-jdbctarget.html#cfn-glue-crawler-jdbctarget-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for JdbcTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_name) = self.connection_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionName", connection_name)?;
            }
            if let Some(ref exclusions) = self.exclusions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exclusions", exclusions)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JdbcTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JdbcTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JdbcTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JdbcTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_name: Option<::Value<String>> = None;
                    let mut exclusions: Option<::ValueList<String>> = None;
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionName" => {
                                connection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Exclusions" => {
                                exclusions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JdbcTarget {
                        connection_name: connection_name,
                        exclusions: exclusions,
                        path: path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.MongoDBTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-mongodbtarget.html) property type.
    #[derive(Debug, Default)]
    pub struct MongoDBTarget {
        /// Property [`ConnectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-mongodbtarget.html#cfn-glue-crawler-mongodbtarget-connectionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_name: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-mongodbtarget.html#cfn-glue-crawler-mongodbtarget-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MongoDBTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_name) = self.connection_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionName", connection_name)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MongoDBTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MongoDBTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MongoDBTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MongoDBTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_name: Option<::Value<String>> = None;
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionName" => {
                                connection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MongoDBTarget {
                        connection_name: connection_name,
                        path: path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.RecrawlPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-recrawlpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct RecrawlPolicy {
        /// Property [`RecrawlBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-recrawlpolicy.html#cfn-glue-crawler-recrawlpolicy-recrawlbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recrawl_behavior: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RecrawlPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref recrawl_behavior) = self.recrawl_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecrawlBehavior", recrawl_behavior)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecrawlPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecrawlPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecrawlPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecrawlPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut recrawl_behavior: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecrawlBehavior" => {
                                recrawl_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecrawlPolicy {
                        recrawl_behavior: recrawl_behavior,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.S3Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Target {
        /// Property [`ConnectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html#cfn-glue-crawler-s3target-connectionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_name: Option<::Value<String>>,
        /// Property [`DlqEventQueueArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html#cfn-glue-crawler-s3target-dlqeventqueuearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dlq_event_queue_arn: Option<::Value<String>>,
        /// Property [`EventQueueArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html#cfn-glue-crawler-s3target-eventqueuearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_queue_arn: Option<::Value<String>>,
        /// Property [`Exclusions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html#cfn-glue-crawler-s3target-exclusions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclusions: Option<::ValueList<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html#cfn-glue-crawler-s3target-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
        /// Property [`SampleSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html#cfn-glue-crawler-s3target-samplesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sample_size: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for S3Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_name) = self.connection_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionName", connection_name)?;
            }
            if let Some(ref dlq_event_queue_arn) = self.dlq_event_queue_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DlqEventQueueArn", dlq_event_queue_arn)?;
            }
            if let Some(ref event_queue_arn) = self.event_queue_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventQueueArn", event_queue_arn)?;
            }
            if let Some(ref exclusions) = self.exclusions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exclusions", exclusions)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref sample_size) = self.sample_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampleSize", sample_size)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Target {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Target, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Target;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Target")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_name: Option<::Value<String>> = None;
                    let mut dlq_event_queue_arn: Option<::Value<String>> = None;
                    let mut event_queue_arn: Option<::Value<String>> = None;
                    let mut exclusions: Option<::ValueList<String>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut sample_size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionName" => {
                                connection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DlqEventQueueArn" => {
                                dlq_event_queue_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventQueueArn" => {
                                event_queue_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Exclusions" => {
                                exclusions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SampleSize" => {
                                sample_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Target {
                        connection_name: connection_name,
                        dlq_event_queue_arn: dlq_event_queue_arn,
                        event_queue_arn: event_queue_arn,
                        exclusions: exclusions,
                        path: path,
                        sample_size: sample_size,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schedule.html) property type.
    #[derive(Debug, Default)]
    pub struct Schedule {
        /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schedule.html#cfn-glue-crawler-schedule-scheduleexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_expression: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Schedule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref schedule_expression) = self.schedule_expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", schedule_expression)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Schedule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Schedule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Schedule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Schedule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut schedule_expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ScheduleExpression" => {
                                schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Schedule {
                        schedule_expression: schedule_expression,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.SchemaChangePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schemachangepolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaChangePolicy {
        /// Property [`DeleteBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schemachangepolicy.html#cfn-glue-crawler-schemachangepolicy-deletebehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delete_behavior: Option<::Value<String>>,
        /// Property [`UpdateBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schemachangepolicy.html#cfn-glue-crawler-schemachangepolicy-updatebehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_behavior: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SchemaChangePolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delete_behavior) = self.delete_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteBehavior", delete_behavior)?;
            }
            if let Some(ref update_behavior) = self.update_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateBehavior", update_behavior)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaChangePolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaChangePolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaChangePolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaChangePolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_behavior: Option<::Value<String>> = None;
                    let mut update_behavior: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteBehavior" => {
                                delete_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateBehavior" => {
                                update_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaChangePolicy {
                        delete_behavior: delete_behavior,
                        update_behavior: update_behavior,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html) property type.
    #[derive(Debug, Default)]
    pub struct Targets {
        /// Property [`CatalogTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html#cfn-glue-crawler-targets-catalogtargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_targets: Option<::ValueList<CatalogTarget>>,
        /// Property [`DeltaTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html#cfn-glue-crawler-targets-deltatargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delta_targets: Option<::ValueList<DeltaTarget>>,
        /// Property [`DynamoDBTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html#cfn-glue-crawler-targets-dynamodbtargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamo_db_targets: Option<::ValueList<DynamoDBTarget>>,
        /// Property [`IcebergTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html#cfn-glue-crawler-targets-icebergtargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iceberg_targets: Option<::ValueList<IcebergTarget>>,
        /// Property [`JdbcTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html#cfn-glue-crawler-targets-jdbctargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub jdbc_targets: Option<::ValueList<JdbcTarget>>,
        /// Property [`MongoDBTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html#cfn-glue-crawler-targets-mongodbtargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mongo_db_targets: Option<::ValueList<MongoDBTarget>>,
        /// Property [`S3Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html#cfn-glue-crawler-targets-s3targets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_targets: Option<::ValueList<S3Target>>,
    }

    impl ::codec::SerializeValue for Targets {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_targets) = self.catalog_targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogTargets", catalog_targets)?;
            }
            if let Some(ref delta_targets) = self.delta_targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeltaTargets", delta_targets)?;
            }
            if let Some(ref dynamo_db_targets) = self.dynamo_db_targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDBTargets", dynamo_db_targets)?;
            }
            if let Some(ref iceberg_targets) = self.iceberg_targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IcebergTargets", iceberg_targets)?;
            }
            if let Some(ref jdbc_targets) = self.jdbc_targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JdbcTargets", jdbc_targets)?;
            }
            if let Some(ref mongo_db_targets) = self.mongo_db_targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MongoDBTargets", mongo_db_targets)?;
            }
            if let Some(ref s3_targets) = self.s3_targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Targets", s3_targets)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Targets {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Targets, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Targets;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Targets")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_targets: Option<::ValueList<CatalogTarget>> = None;
                    let mut delta_targets: Option<::ValueList<DeltaTarget>> = None;
                    let mut dynamo_db_targets: Option<::ValueList<DynamoDBTarget>> = None;
                    let mut iceberg_targets: Option<::ValueList<IcebergTarget>> = None;
                    let mut jdbc_targets: Option<::ValueList<JdbcTarget>> = None;
                    let mut mongo_db_targets: Option<::ValueList<MongoDBTarget>> = None;
                    let mut s3_targets: Option<::ValueList<S3Target>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogTargets" => {
                                catalog_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeltaTargets" => {
                                delta_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DynamoDBTargets" => {
                                dynamo_db_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IcebergTargets" => {
                                iceberg_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JdbcTargets" => {
                                jdbc_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MongoDBTargets" => {
                                mongo_db_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Targets" => {
                                s3_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Targets {
                        catalog_targets: catalog_targets,
                        delta_targets: delta_targets,
                        dynamo_db_targets: dynamo_db_targets,
                        iceberg_targets: iceberg_targets,
                        jdbc_targets: jdbc_targets,
                        mongo_db_targets: mongo_db_targets,
                        s3_targets: s3_targets,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod data_catalog_encryption_settings {
    //! Property types for the `DataCatalogEncryptionSettings` resource.

    /// The [`AWS::Glue::DataCatalogEncryptionSettings.ConnectionPasswordEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-connectionpasswordencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectionPasswordEncryption {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-connectionpasswordencryption.html#cfn-glue-datacatalogencryptionsettings-connectionpasswordencryption-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`ReturnConnectionPasswordEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-connectionpasswordencryption.html#cfn-glue-datacatalogencryptionsettings-connectionpasswordencryption-returnconnectionpasswordencrypted).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub return_connection_password_encrypted: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ConnectionPasswordEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            if let Some(ref return_connection_password_encrypted) = self.return_connection_password_encrypted {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReturnConnectionPasswordEncrypted", return_connection_password_encrypted)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectionPasswordEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionPasswordEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionPasswordEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionPasswordEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut return_connection_password_encrypted: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReturnConnectionPasswordEncrypted" => {
                                return_connection_password_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionPasswordEncryption {
                        kms_key_id: kms_key_id,
                        return_connection_password_encrypted: return_connection_password_encrypted,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::DataCatalogEncryptionSettings.DataCatalogEncryptionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-datacatalogencryptionsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DataCatalogEncryptionSettings {
        /// Property [`ConnectionPasswordEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-datacatalogencryptionsettings.html#cfn-glue-datacatalogencryptionsettings-datacatalogencryptionsettings-connectionpasswordencryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_password_encryption: Option<::Value<ConnectionPasswordEncryption>>,
        /// Property [`EncryptionAtRest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-datacatalogencryptionsettings.html#cfn-glue-datacatalogencryptionsettings-datacatalogencryptionsettings-encryptionatrest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_at_rest: Option<::Value<EncryptionAtRest>>,
    }

    impl ::codec::SerializeValue for DataCatalogEncryptionSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_password_encryption) = self.connection_password_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionPasswordEncryption", connection_password_encryption)?;
            }
            if let Some(ref encryption_at_rest) = self.encryption_at_rest {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionAtRest", encryption_at_rest)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataCatalogEncryptionSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataCatalogEncryptionSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataCatalogEncryptionSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataCatalogEncryptionSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_password_encryption: Option<::Value<ConnectionPasswordEncryption>> = None;
                    let mut encryption_at_rest: Option<::Value<EncryptionAtRest>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionPasswordEncryption" => {
                                connection_password_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionAtRest" => {
                                encryption_at_rest = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataCatalogEncryptionSettings {
                        connection_password_encryption: connection_password_encryption,
                        encryption_at_rest: encryption_at_rest,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::DataCatalogEncryptionSettings.EncryptionAtRest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-encryptionatrest.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionAtRest {
        /// Property [`CatalogEncryptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-encryptionatrest.html#cfn-glue-datacatalogencryptionsettings-encryptionatrest-catalogencryptionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_encryption_mode: Option<::Value<String>>,
        /// Property [`SseAwsKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-datacatalogencryptionsettings-encryptionatrest.html#cfn-glue-datacatalogencryptionsettings-encryptionatrest-sseawskmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sse_aws_kms_key_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EncryptionAtRest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_encryption_mode) = self.catalog_encryption_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogEncryptionMode", catalog_encryption_mode)?;
            }
            if let Some(ref sse_aws_kms_key_id) = self.sse_aws_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SseAwsKmsKeyId", sse_aws_kms_key_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionAtRest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionAtRest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionAtRest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionAtRest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_encryption_mode: Option<::Value<String>> = None;
                    let mut sse_aws_kms_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogEncryptionMode" => {
                                catalog_encryption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SseAwsKmsKeyId" => {
                                sse_aws_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionAtRest {
                        catalog_encryption_mode: catalog_encryption_mode,
                        sse_aws_kms_key_id: sse_aws_kms_key_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod data_quality_ruleset {
    //! Property types for the `DataQualityRuleset` resource.

    /// The [`AWS::Glue::DataQualityRuleset.DataQualityTargetTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-dataqualityruleset-dataqualitytargettable.html) property type.
    #[derive(Debug, Default)]
    pub struct DataQualityTargetTable {
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-dataqualityruleset-dataqualitytargettable.html#cfn-glue-dataqualityruleset-dataqualitytargettable-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-dataqualityruleset-dataqualitytargettable.html#cfn-glue-dataqualityruleset-dataqualitytargettable-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataQualityTargetTable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref table_name) = self.table_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", table_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataQualityTargetTable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataQualityTargetTable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataQualityTargetTable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataQualityTargetTable")
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

                    Ok(DataQualityTargetTable {
                        database_name: database_name,
                        table_name: table_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod database {
    //! Property types for the `Database` resource.

    /// The [`AWS::Glue::Database.DataLakePrincipal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-datalakeprincipal.html) property type.
    #[derive(Debug, Default)]
    pub struct DataLakePrincipal {
        /// Property [`DataLakePrincipalIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-datalakeprincipal.html#cfn-glue-database-datalakeprincipal-datalakeprincipalidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_lake_principal_identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataLakePrincipal {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_lake_principal_identifier) = self.data_lake_principal_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLakePrincipalIdentifier", data_lake_principal_identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataLakePrincipal {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLakePrincipal, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataLakePrincipal;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataLakePrincipal")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_lake_principal_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataLakePrincipalIdentifier" => {
                                data_lake_principal_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataLakePrincipal {
                        data_lake_principal_identifier: data_lake_principal_identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Database.DatabaseIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseidentifier.html) property type.
    #[derive(Debug, Default)]
    pub struct DatabaseIdentifier {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseidentifier.html#cfn-glue-database-databaseidentifier-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseidentifier.html#cfn-glue-database-databaseidentifier-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseidentifier.html#cfn-glue-database-databaseidentifier-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DatabaseIdentifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref region) = self.region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatabaseIdentifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseIdentifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatabaseIdentifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatabaseIdentifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatabaseIdentifier {
                        catalog_id: catalog_id,
                        database_name: database_name,
                        region: region,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Database.DatabaseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html) property type.
    #[derive(Debug, Default)]
    pub struct DatabaseInput {
        /// Property [`CreateTableDefaultPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html#cfn-glue-database-databaseinput-createtabledefaultpermissions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub create_table_default_permissions: Option<::ValueList<PrincipalPrivileges>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html#cfn-glue-database-databaseinput-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`FederatedDatabase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html#cfn-glue-database-databaseinput-federateddatabase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub federated_database: Option<::Value<FederatedDatabase>>,
        /// Property [`LocationUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html#cfn-glue-database-databaseinput-locationuri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location_uri: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html#cfn-glue-database-databaseinput-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html#cfn-glue-database-databaseinput-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
        /// Property [`TargetDatabase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html#cfn-glue-database-databaseinput-targetdatabase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_database: Option<::Value<DatabaseIdentifier>>,
    }

    impl ::codec::SerializeValue for DatabaseInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref create_table_default_permissions) = self.create_table_default_permissions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateTableDefaultPermissions", create_table_default_permissions)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref federated_database) = self.federated_database {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FederatedDatabase", federated_database)?;
            }
            if let Some(ref location_uri) = self.location_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocationUri", location_uri)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref target_database) = self.target_database {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetDatabase", target_database)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatabaseInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatabaseInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatabaseInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut create_table_default_permissions: Option<::ValueList<PrincipalPrivileges>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut federated_database: Option<::Value<FederatedDatabase>> = None;
                    let mut location_uri: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;
                    let mut target_database: Option<::Value<DatabaseIdentifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CreateTableDefaultPermissions" => {
                                create_table_default_permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FederatedDatabase" => {
                                federated_database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocationUri" => {
                                location_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetDatabase" => {
                                target_database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatabaseInput {
                        create_table_default_permissions: create_table_default_permissions,
                        description: description,
                        federated_database: federated_database,
                        location_uri: location_uri,
                        name: name,
                        parameters: parameters,
                        target_database: target_database,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Database.FederatedDatabase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput-federateddatabase.html) property type.
    #[derive(Debug, Default)]
    pub struct FederatedDatabase {
        /// Property [`ConnectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput-federateddatabase.html#cfn-glue-database-databaseinput-federateddatabase-connectionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_name: Option<::Value<String>>,
        /// Property [`Identifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput-federateddatabase.html#cfn-glue-database-databaseinput-federateddatabase-identifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FederatedDatabase {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_name) = self.connection_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionName", connection_name)?;
            }
            if let Some(ref identifier) = self.identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Identifier", identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FederatedDatabase {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FederatedDatabase, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FederatedDatabase;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FederatedDatabase")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_name: Option<::Value<String>> = None;
                    let mut identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionName" => {
                                connection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Identifier" => {
                                identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FederatedDatabase {
                        connection_name: connection_name,
                        identifier: identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Database.PrincipalPrivileges`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-principalprivileges.html) property type.
    #[derive(Debug, Default)]
    pub struct PrincipalPrivileges {
        /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-principalprivileges.html#cfn-glue-database-principalprivileges-permissions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub permissions: Option<::ValueList<String>>,
        /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-principalprivileges.html#cfn-glue-database-principalprivileges-principal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principal: Option<::Value<DataLakePrincipal>>,
    }

    impl ::codec::SerializeValue for PrincipalPrivileges {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref permissions) = self.permissions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
            }
            if let Some(ref principal) = self.principal {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", principal)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrincipalPrivileges {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrincipalPrivileges, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrincipalPrivileges;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrincipalPrivileges")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut permissions: Option<::ValueList<String>> = None;
                    let mut principal: Option<::Value<DataLakePrincipal>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Permissions" => {
                                permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Principal" => {
                                principal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrincipalPrivileges {
                        permissions: permissions,
                        principal: principal,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod job {
    //! Property types for the `Job` resource.

    /// The [`AWS::Glue::Job.ConnectionsList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-connectionslist.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectionsList {
        /// Property [`Connections`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-connectionslist.html#cfn-glue-job-connectionslist-connections).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connections: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ConnectionsList {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connections) = self.connections {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Connections", connections)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectionsList {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionsList, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionsList;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionsList")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connections: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Connections" => {
                                connections = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionsList {
                        connections: connections,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Job.ExecutionProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-executionproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct ExecutionProperty {
        /// Property [`MaxConcurrentRuns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-executionproperty.html#cfn-glue-job-executionproperty-maxconcurrentruns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_concurrent_runs: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for ExecutionProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_concurrent_runs) = self.max_concurrent_runs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrentRuns", max_concurrent_runs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExecutionProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExecutionProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExecutionProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExecutionProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_concurrent_runs: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxConcurrentRuns" => {
                                max_concurrent_runs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExecutionProperty {
                        max_concurrent_runs: max_concurrent_runs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Job.JobCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html) property type.
    #[derive(Debug, Default)]
    pub struct JobCommand {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html#cfn-glue-job-jobcommand-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`PythonVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html#cfn-glue-job-jobcommand-pythonversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub python_version: Option<::Value<String>>,
        /// Property [`Runtime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html#cfn-glue-job-jobcommand-runtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub runtime: Option<::Value<String>>,
        /// Property [`ScriptLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html#cfn-glue-job-jobcommand-scriptlocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub script_location: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for JobCommand {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref python_version) = self.python_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PythonVersion", python_version)?;
            }
            if let Some(ref runtime) = self.runtime {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Runtime", runtime)?;
            }
            if let Some(ref script_location) = self.script_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScriptLocation", script_location)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JobCommand {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JobCommand, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JobCommand;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JobCommand")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut python_version: Option<::Value<String>> = None;
                    let mut runtime: Option<::Value<String>> = None;
                    let mut script_location: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PythonVersion" => {
                                python_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Runtime" => {
                                runtime = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScriptLocation" => {
                                script_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JobCommand {
                        name: name,
                        python_version: python_version,
                        runtime: runtime,
                        script_location: script_location,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Job.NotificationProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-notificationproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationProperty {
        /// Property [`NotifyDelayAfter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-notificationproperty.html#cfn-glue-job-notificationproperty-notifydelayafter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notify_delay_after: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for NotificationProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref notify_delay_after) = self.notify_delay_after {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotifyDelayAfter", notify_delay_after)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut notify_delay_after: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NotifyDelayAfter" => {
                                notify_delay_after = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationProperty {
                        notify_delay_after: notify_delay_after,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod ml_transform {
    //! Property types for the `MLTransform` resource.

    /// The [`AWS::Glue::MLTransform.FindMatchesParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformparameters-findmatchesparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct FindMatchesParameters {
        /// Property [`AccuracyCostTradeoff`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformparameters-findmatchesparameters.html#cfn-glue-mltransform-transformparameters-findmatchesparameters-accuracycosttradeoff).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub accuracy_cost_tradeoff: Option<::Value<f64>>,
        /// Property [`EnforceProvidedLabels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformparameters-findmatchesparameters.html#cfn-glue-mltransform-transformparameters-findmatchesparameters-enforceprovidedlabels).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enforce_provided_labels: Option<::Value<bool>>,
        /// Property [`PrecisionRecallTradeoff`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformparameters-findmatchesparameters.html#cfn-glue-mltransform-transformparameters-findmatchesparameters-precisionrecalltradeoff).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub precision_recall_tradeoff: Option<::Value<f64>>,
        /// Property [`PrimaryKeyColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformparameters-findmatchesparameters.html#cfn-glue-mltransform-transformparameters-findmatchesparameters-primarykeycolumnname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub primary_key_column_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for FindMatchesParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref accuracy_cost_tradeoff) = self.accuracy_cost_tradeoff {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccuracyCostTradeoff", accuracy_cost_tradeoff)?;
            }
            if let Some(ref enforce_provided_labels) = self.enforce_provided_labels {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnforceProvidedLabels", enforce_provided_labels)?;
            }
            if let Some(ref precision_recall_tradeoff) = self.precision_recall_tradeoff {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrecisionRecallTradeoff", precision_recall_tradeoff)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryKeyColumnName", &self.primary_key_column_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FindMatchesParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FindMatchesParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FindMatchesParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FindMatchesParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut accuracy_cost_tradeoff: Option<::Value<f64>> = None;
                    let mut enforce_provided_labels: Option<::Value<bool>> = None;
                    let mut precision_recall_tradeoff: Option<::Value<f64>> = None;
                    let mut primary_key_column_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccuracyCostTradeoff" => {
                                accuracy_cost_tradeoff = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnforceProvidedLabels" => {
                                enforce_provided_labels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrecisionRecallTradeoff" => {
                                precision_recall_tradeoff = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrimaryKeyColumnName" => {
                                primary_key_column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FindMatchesParameters {
                        accuracy_cost_tradeoff: accuracy_cost_tradeoff,
                        enforce_provided_labels: enforce_provided_labels,
                        precision_recall_tradeoff: precision_recall_tradeoff,
                        primary_key_column_name: primary_key_column_name.ok_or(::serde::de::Error::missing_field("PrimaryKeyColumnName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::MLTransform.GlueTables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-inputrecordtables-gluetables.html) property type.
    #[derive(Debug, Default)]
    pub struct GlueTables {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-inputrecordtables-gluetables.html#cfn-glue-mltransform-inputrecordtables-gluetables-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`ConnectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-inputrecordtables-gluetables.html#cfn-glue-mltransform-inputrecordtables-gluetables-connectionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_name: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-inputrecordtables-gluetables.html#cfn-glue-mltransform-inputrecordtables-gluetables-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-inputrecordtables-gluetables.html#cfn-glue-mltransform-inputrecordtables-gluetables-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for GlueTables {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref connection_name) = self.connection_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionName", connection_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GlueTables {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GlueTables, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GlueTables;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GlueTables")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut connection_name: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectionName" => {
                                connection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GlueTables {
                        catalog_id: catalog_id,
                        connection_name: connection_name,
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::MLTransform.InputRecordTables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-inputrecordtables.html) property type.
    #[derive(Debug, Default)]
    pub struct InputRecordTables {
        /// Property [`GlueTables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-inputrecordtables.html#cfn-glue-mltransform-inputrecordtables-gluetables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub glue_tables: Option<::ValueList<GlueTables>>,
    }

    impl ::codec::SerializeValue for InputRecordTables {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref glue_tables) = self.glue_tables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlueTables", glue_tables)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputRecordTables {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputRecordTables, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputRecordTables;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputRecordTables")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut glue_tables: Option<::ValueList<GlueTables>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GlueTables" => {
                                glue_tables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputRecordTables {
                        glue_tables: glue_tables,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::MLTransform.MLUserDataEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformencryption-mluserdataencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct MLUserDataEncryption {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformencryption-mluserdataencryption.html#cfn-glue-mltransform-transformencryption-mluserdataencryption-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`MLUserDataEncryptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformencryption-mluserdataencryption.html#cfn-glue-mltransform-transformencryption-mluserdataencryption-mluserdataencryptionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ml_user_data_encryption_mode: ::Value<String>,
    }

    impl ::codec::SerializeValue for MLUserDataEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MLUserDataEncryptionMode", &self.ml_user_data_encryption_mode)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MLUserDataEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MLUserDataEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MLUserDataEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MLUserDataEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut ml_user_data_encryption_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MLUserDataEncryptionMode" => {
                                ml_user_data_encryption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MLUserDataEncryption {
                        kms_key_id: kms_key_id,
                        ml_user_data_encryption_mode: ml_user_data_encryption_mode.ok_or(::serde::de::Error::missing_field("MLUserDataEncryptionMode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::MLTransform.TransformEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct TransformEncryption {
        /// Property [`MLUserDataEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformencryption.html#cfn-glue-mltransform-transformencryption-mluserdataencryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ml_user_data_encryption: Option<::Value<MLUserDataEncryption>>,
        /// Property [`TaskRunSecurityConfigurationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformencryption.html#cfn-glue-mltransform-transformencryption-taskrunsecurityconfigurationname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_run_security_configuration_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TransformEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ml_user_data_encryption) = self.ml_user_data_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MLUserDataEncryption", ml_user_data_encryption)?;
            }
            if let Some(ref task_run_security_configuration_name) = self.task_run_security_configuration_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskRunSecurityConfigurationName", task_run_security_configuration_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TransformEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TransformEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TransformEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TransformEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ml_user_data_encryption: Option<::Value<MLUserDataEncryption>> = None;
                    let mut task_run_security_configuration_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MLUserDataEncryption" => {
                                ml_user_data_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskRunSecurityConfigurationName" => {
                                task_run_security_configuration_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TransformEncryption {
                        ml_user_data_encryption: ml_user_data_encryption,
                        task_run_security_configuration_name: task_run_security_configuration_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::MLTransform.TransformParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct TransformParameters {
        /// Property [`FindMatchesParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformparameters.html#cfn-glue-mltransform-transformparameters-findmatchesparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub find_matches_parameters: Option<::Value<FindMatchesParameters>>,
        /// Property [`TransformType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-mltransform-transformparameters.html#cfn-glue-mltransform-transformparameters-transformtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transform_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for TransformParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref find_matches_parameters) = self.find_matches_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FindMatchesParameters", find_matches_parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransformType", &self.transform_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TransformParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TransformParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TransformParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TransformParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut find_matches_parameters: Option<::Value<FindMatchesParameters>> = None;
                    let mut transform_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FindMatchesParameters" => {
                                find_matches_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransformType" => {
                                transform_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TransformParameters {
                        find_matches_parameters: find_matches_parameters,
                        transform_type: transform_type.ok_or(::serde::de::Error::missing_field("TransformType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod partition {
    //! Property types for the `Partition` resource.

    /// The [`AWS::Glue::Partition.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-column.html) property type.
    #[derive(Debug, Default)]
    pub struct Column {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-column.html#cfn-glue-partition-column-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-column.html#cfn-glue-partition-column-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-column.html#cfn-glue-partition-column-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Column {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Column {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Column, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Column;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Column")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(Column {
                        comment: comment,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Partition.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-order.html) property type.
    #[derive(Debug, Default)]
    pub struct Order {
        /// Property [`Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-order.html#cfn-glue-partition-order-column).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column: ::Value<String>,
        /// Property [`SortOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-order.html#cfn-glue-partition-order-sortorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sort_order: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Order {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Column", &self.column)?;
            if let Some(ref sort_order) = self.sort_order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SortOrder", sort_order)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Order {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Order, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Order;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Order")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column: Option<::Value<String>> = None;
                    let mut sort_order: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Column" => {
                                column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SortOrder" => {
                                sort_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Order {
                        column: column.ok_or(::serde::de::Error::missing_field("Column"))?,
                        sort_order: sort_order,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Partition.PartitionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-partitioninput.html) property type.
    #[derive(Debug, Default)]
    pub struct PartitionInput {
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-partitioninput.html#cfn-glue-partition-partitioninput-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
        /// Property [`StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-partitioninput.html#cfn-glue-partition-partitioninput-storagedescriptor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_descriptor: Option<::Value<StorageDescriptor>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-partitioninput.html#cfn-glue-partition-partitioninput-values).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for PartitionInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref storage_descriptor) = self.storage_descriptor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageDescriptor", storage_descriptor)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PartitionInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PartitionInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PartitionInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PartitionInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameters: Option<::Value<::json::Value>> = None;
                    let mut storage_descriptor: Option<::Value<StorageDescriptor>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageDescriptor" => {
                                storage_descriptor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PartitionInput {
                        parameters: parameters,
                        storage_descriptor: storage_descriptor,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Partition.SchemaId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-schemaid.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaId {
        /// Property [`RegistryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-schemaid.html#cfn-glue-partition-schemaid-registryname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub registry_name: Option<::Value<String>>,
        /// Property [`SchemaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-schemaid.html#cfn-glue-partition-schemaid-schemaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_arn: Option<::Value<String>>,
        /// Property [`SchemaName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-schemaid.html#cfn-glue-partition-schemaid-schemaname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SchemaId {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref registry_name) = self.registry_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegistryName", registry_name)?;
            }
            if let Some(ref schema_arn) = self.schema_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaArn", schema_arn)?;
            }
            if let Some(ref schema_name) = self.schema_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaName", schema_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaId {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaId, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaId;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaId")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut registry_name: Option<::Value<String>> = None;
                    let mut schema_arn: Option<::Value<String>> = None;
                    let mut schema_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RegistryName" => {
                                registry_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaArn" => {
                                schema_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaName" => {
                                schema_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaId {
                        registry_name: registry_name,
                        schema_arn: schema_arn,
                        schema_name: schema_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Partition.SchemaReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-schemareference.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaReference {
        /// Property [`SchemaId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-schemareference.html#cfn-glue-partition-schemareference-schemaid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_id: Option<::Value<SchemaId>>,
        /// Property [`SchemaVersionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-schemareference.html#cfn-glue-partition-schemareference-schemaversionid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_version_id: Option<::Value<String>>,
        /// Property [`SchemaVersionNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-schemareference.html#cfn-glue-partition-schemareference-schemaversionnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_version_number: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SchemaReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref schema_id) = self.schema_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaId", schema_id)?;
            }
            if let Some(ref schema_version_id) = self.schema_version_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaVersionId", schema_version_id)?;
            }
            if let Some(ref schema_version_number) = self.schema_version_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaVersionNumber", schema_version_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut schema_id: Option<::Value<SchemaId>> = None;
                    let mut schema_version_id: Option<::Value<String>> = None;
                    let mut schema_version_number: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SchemaId" => {
                                schema_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaVersionId" => {
                                schema_version_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaVersionNumber" => {
                                schema_version_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaReference {
                        schema_id: schema_id,
                        schema_version_id: schema_version_id,
                        schema_version_number: schema_version_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Partition.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-serdeinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct SerdeInfo {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-serdeinfo.html#cfn-glue-partition-serdeinfo-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-serdeinfo.html#cfn-glue-partition-serdeinfo-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
        /// Property [`SerializationLibrary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-serdeinfo.html#cfn-glue-partition-serdeinfo-serializationlibrary).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub serialization_library: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SerdeInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref serialization_library) = self.serialization_library {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SerializationLibrary", serialization_library)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SerdeInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SerdeInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SerdeInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SerdeInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;
                    let mut serialization_library: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SerializationLibrary" => {
                                serialization_library = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SerdeInfo {
                        name: name,
                        parameters: parameters,
                        serialization_library: serialization_library,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Partition.SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-skewedinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct SkewedInfo {
        /// Property [`SkewedColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-skewedinfo.html#cfn-glue-partition-skewedinfo-skewedcolumnnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub skewed_column_names: Option<::ValueList<String>>,
        /// Property [`SkewedColumnValueLocationMaps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-skewedinfo.html#cfn-glue-partition-skewedinfo-skewedcolumnvaluelocationmaps).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub skewed_column_value_location_maps: Option<::Value<::json::Value>>,
        /// Property [`SkewedColumnValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-skewedinfo.html#cfn-glue-partition-skewedinfo-skewedcolumnvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub skewed_column_values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SkewedInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref skewed_column_names) = self.skewed_column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SkewedColumnNames", skewed_column_names)?;
            }
            if let Some(ref skewed_column_value_location_maps) = self.skewed_column_value_location_maps {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SkewedColumnValueLocationMaps", skewed_column_value_location_maps)?;
            }
            if let Some(ref skewed_column_values) = self.skewed_column_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SkewedColumnValues", skewed_column_values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SkewedInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SkewedInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SkewedInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SkewedInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut skewed_column_names: Option<::ValueList<String>> = None;
                    let mut skewed_column_value_location_maps: Option<::Value<::json::Value>> = None;
                    let mut skewed_column_values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SkewedColumnNames" => {
                                skewed_column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SkewedColumnValueLocationMaps" => {
                                skewed_column_value_location_maps = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SkewedColumnValues" => {
                                skewed_column_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SkewedInfo {
                        skewed_column_names: skewed_column_names,
                        skewed_column_value_location_maps: skewed_column_value_location_maps,
                        skewed_column_values: skewed_column_values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Partition.StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html) property type.
    #[derive(Debug, Default)]
    pub struct StorageDescriptor {
        /// Property [`BucketColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-bucketcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_columns: Option<::ValueList<String>>,
        /// Property [`Columns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-columns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub columns: Option<::ValueList<Column>>,
        /// Property [`Compressed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-compressed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compressed: Option<::Value<bool>>,
        /// Property [`InputFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-inputformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_format: Option<::Value<String>>,
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: Option<::Value<String>>,
        /// Property [`NumberOfBuckets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-numberofbuckets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_of_buckets: Option<::Value<u32>>,
        /// Property [`OutputFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-outputformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_format: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
        /// Property [`SchemaReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-schemareference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_reference: Option<::Value<SchemaReference>>,
        /// Property [`SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-serdeinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub serde_info: Option<::Value<SerdeInfo>>,
        /// Property [`SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-skewedinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub skewed_info: Option<::Value<SkewedInfo>>,
        /// Property [`SortColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-sortcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sort_columns: Option<::ValueList<Order>>,
        /// Property [`StoredAsSubDirectories`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html#cfn-glue-partition-storagedescriptor-storedassubdirectories).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stored_as_sub_directories: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for StorageDescriptor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_columns) = self.bucket_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketColumns", bucket_columns)?;
            }
            if let Some(ref columns) = self.columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Columns", columns)?;
            }
            if let Some(ref compressed) = self.compressed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compressed", compressed)?;
            }
            if let Some(ref input_format) = self.input_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputFormat", input_format)?;
            }
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            if let Some(ref number_of_buckets) = self.number_of_buckets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfBuckets", number_of_buckets)?;
            }
            if let Some(ref output_format) = self.output_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputFormat", output_format)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref schema_reference) = self.schema_reference {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaReference", schema_reference)?;
            }
            if let Some(ref serde_info) = self.serde_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SerdeInfo", serde_info)?;
            }
            if let Some(ref skewed_info) = self.skewed_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SkewedInfo", skewed_info)?;
            }
            if let Some(ref sort_columns) = self.sort_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SortColumns", sort_columns)?;
            }
            if let Some(ref stored_as_sub_directories) = self.stored_as_sub_directories {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StoredAsSubDirectories", stored_as_sub_directories)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StorageDescriptor {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StorageDescriptor, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StorageDescriptor;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StorageDescriptor")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_columns: Option<::ValueList<String>> = None;
                    let mut columns: Option<::ValueList<Column>> = None;
                    let mut compressed: Option<::Value<bool>> = None;
                    let mut input_format: Option<::Value<String>> = None;
                    let mut location: Option<::Value<String>> = None;
                    let mut number_of_buckets: Option<::Value<u32>> = None;
                    let mut output_format: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;
                    let mut schema_reference: Option<::Value<SchemaReference>> = None;
                    let mut serde_info: Option<::Value<SerdeInfo>> = None;
                    let mut skewed_info: Option<::Value<SkewedInfo>> = None;
                    let mut sort_columns: Option<::ValueList<Order>> = None;
                    let mut stored_as_sub_directories: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketColumns" => {
                                bucket_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Columns" => {
                                columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Compressed" => {
                                compressed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputFormat" => {
                                input_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberOfBuckets" => {
                                number_of_buckets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputFormat" => {
                                output_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaReference" => {
                                schema_reference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SerdeInfo" => {
                                serde_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SkewedInfo" => {
                                skewed_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SortColumns" => {
                                sort_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StoredAsSubDirectories" => {
                                stored_as_sub_directories = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StorageDescriptor {
                        bucket_columns: bucket_columns,
                        columns: columns,
                        compressed: compressed,
                        input_format: input_format,
                        location: location,
                        number_of_buckets: number_of_buckets,
                        output_format: output_format,
                        parameters: parameters,
                        schema_reference: schema_reference,
                        serde_info: serde_info,
                        skewed_info: skewed_info,
                        sort_columns: sort_columns,
                        stored_as_sub_directories: stored_as_sub_directories,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod schema {
    //! Property types for the `Schema` resource.

    /// The [`AWS::Glue::Schema.Registry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schema-registry.html) property type.
    #[derive(Debug, Default)]
    pub struct Registry {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schema-registry.html#cfn-glue-schema-registry-arn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schema-registry.html#cfn-glue-schema-registry-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Registry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Registry {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Registry, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Registry;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Registry")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Registry {
                        arn: arn,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Schema.SchemaVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schema-schemaversion.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaVersion {
        /// Property [`IsLatest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schema-schemaversion.html#cfn-glue-schema-schemaversion-islatest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_latest: Option<::Value<bool>>,
        /// Property [`VersionNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schema-schemaversion.html#cfn-glue-schema-schemaversion-versionnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version_number: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SchemaVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref is_latest) = self.is_latest {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsLatest", is_latest)?;
            }
            if let Some(ref version_number) = self.version_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionNumber", version_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut is_latest: Option<::Value<bool>> = None;
                    let mut version_number: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IsLatest" => {
                                is_latest = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VersionNumber" => {
                                version_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaVersion {
                        is_latest: is_latest,
                        version_number: version_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod schema_version {
    //! Property types for the `SchemaVersion` resource.

    /// The [`AWS::Glue::SchemaVersion.Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schemaversion-schema.html) property type.
    #[derive(Debug, Default)]
    pub struct Schema {
        /// Property [`RegistryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schemaversion-schema.html#cfn-glue-schemaversion-schema-registryname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub registry_name: Option<::Value<String>>,
        /// Property [`SchemaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schemaversion-schema.html#cfn-glue-schemaversion-schema-schemaarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub schema_arn: Option<::Value<String>>,
        /// Property [`SchemaName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-schemaversion-schema.html#cfn-glue-schemaversion-schema-schemaname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub schema_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Schema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref registry_name) = self.registry_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegistryName", registry_name)?;
            }
            if let Some(ref schema_arn) = self.schema_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaArn", schema_arn)?;
            }
            if let Some(ref schema_name) = self.schema_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaName", schema_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Schema {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Schema, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Schema;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Schema")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut registry_name: Option<::Value<String>> = None;
                    let mut schema_arn: Option<::Value<String>> = None;
                    let mut schema_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RegistryName" => {
                                registry_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaArn" => {
                                schema_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaName" => {
                                schema_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Schema {
                        registry_name: registry_name,
                        schema_arn: schema_arn,
                        schema_name: schema_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod security_configuration {
    //! Property types for the `SecurityConfiguration` resource.

    /// The [`AWS::Glue::SecurityConfiguration.CloudWatchEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-cloudwatchencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchEncryption {
        /// Property [`CloudWatchEncryptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-cloudwatchencryption.html#cfn-glue-securityconfiguration-cloudwatchencryption-cloudwatchencryptionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_encryption_mode: Option<::Value<String>>,
        /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-cloudwatchencryption.html#cfn-glue-securityconfiguration-cloudwatchencryption-kmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudWatchEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_encryption_mode) = self.cloud_watch_encryption_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchEncryptionMode", cloud_watch_encryption_mode)?;
            }
            if let Some(ref kms_key_arn) = self.kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_encryption_mode: Option<::Value<String>> = None;
                    let mut kms_key_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchEncryptionMode" => {
                                cloud_watch_encryption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchEncryption {
                        cloud_watch_encryption_mode: cloud_watch_encryption_mode,
                        kms_key_arn: kms_key_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::SecurityConfiguration.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-encryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionConfiguration {
        /// Property [`CloudWatchEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-encryptionconfiguration.html#cfn-glue-securityconfiguration-encryptionconfiguration-cloudwatchencryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_encryption: Option<::Value<CloudWatchEncryption>>,
        /// Property [`JobBookmarksEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-encryptionconfiguration.html#cfn-glue-securityconfiguration-encryptionconfiguration-jobbookmarksencryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_bookmarks_encryption: Option<::Value<JobBookmarksEncryption>>,
        /// Property [`S3Encryptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-encryptionconfiguration.html#cfn-glue-securityconfiguration-encryptionconfiguration-s3encryptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_encryptions: Option<::Value<S3Encryptions>>,
    }

    impl ::codec::SerializeValue for EncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_encryption) = self.cloud_watch_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchEncryption", cloud_watch_encryption)?;
            }
            if let Some(ref job_bookmarks_encryption) = self.job_bookmarks_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobBookmarksEncryption", job_bookmarks_encryption)?;
            }
            if let Some(ref s3_encryptions) = self.s3_encryptions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Encryptions", s3_encryptions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_encryption: Option<::Value<CloudWatchEncryption>> = None;
                    let mut job_bookmarks_encryption: Option<::Value<JobBookmarksEncryption>> = None;
                    let mut s3_encryptions: Option<::Value<S3Encryptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchEncryption" => {
                                cloud_watch_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobBookmarksEncryption" => {
                                job_bookmarks_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Encryptions" => {
                                s3_encryptions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfiguration {
                        cloud_watch_encryption: cloud_watch_encryption,
                        job_bookmarks_encryption: job_bookmarks_encryption,
                        s3_encryptions: s3_encryptions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::SecurityConfiguration.JobBookmarksEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-jobbookmarksencryption.html) property type.
    #[derive(Debug, Default)]
    pub struct JobBookmarksEncryption {
        /// Property [`JobBookmarksEncryptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-jobbookmarksencryption.html#cfn-glue-securityconfiguration-jobbookmarksencryption-jobbookmarksencryptionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_bookmarks_encryption_mode: Option<::Value<String>>,
        /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-jobbookmarksencryption.html#cfn-glue-securityconfiguration-jobbookmarksencryption-kmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for JobBookmarksEncryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref job_bookmarks_encryption_mode) = self.job_bookmarks_encryption_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobBookmarksEncryptionMode", job_bookmarks_encryption_mode)?;
            }
            if let Some(ref kms_key_arn) = self.kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JobBookmarksEncryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JobBookmarksEncryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JobBookmarksEncryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JobBookmarksEncryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut job_bookmarks_encryption_mode: Option<::Value<String>> = None;
                    let mut kms_key_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JobBookmarksEncryptionMode" => {
                                job_bookmarks_encryption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JobBookmarksEncryption {
                        job_bookmarks_encryption_mode: job_bookmarks_encryption_mode,
                        kms_key_arn: kms_key_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::SecurityConfiguration.S3Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-s3encryption.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Encryption {
        /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-s3encryption.html#cfn-glue-securityconfiguration-s3encryption-kmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_arn: Option<::Value<String>>,
        /// Property [`S3EncryptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-s3encryption.html#cfn-glue-securityconfiguration-s3encryption-s3encryptionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_encryption_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Encryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_arn) = self.kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
            }
            if let Some(ref s3_encryption_mode) = self.s3_encryption_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3EncryptionMode", s3_encryption_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Encryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Encryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Encryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Encryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_arn: Option<::Value<String>> = None;
                    let mut s3_encryption_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3EncryptionMode" => {
                                s3_encryption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Encryption {
                        kms_key_arn: kms_key_arn,
                        s3_encryption_mode: s3_encryption_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::SecurityConfiguration.S3Encryptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-securityconfiguration-s3encryptions.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Encryptions {
    }

    impl ::codec::SerializeValue for S3Encryptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Encryptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Encryptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Encryptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Encryptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(S3Encryptions {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod table {
    //! Property types for the `Table` resource.

    /// The [`AWS::Glue::Table.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-column.html) property type.
    #[derive(Debug, Default)]
    pub struct Column {
        /// Property [`Comment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-column.html#cfn-glue-table-column-comment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comment: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-column.html#cfn-glue-table-column-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-column.html#cfn-glue-table-column-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Column {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Column {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Column, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Column;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Column")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(Column {
                        comment: comment,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.IcebergInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-iceberginput.html) property type.
    #[derive(Debug, Default)]
    pub struct IcebergInput {
        /// Property [`MetadataOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-iceberginput.html#cfn-glue-table-iceberginput-metadataoperation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metadata_operation: Option<::Value<MetadataOperation>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-iceberginput.html#cfn-glue-table-iceberginput-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IcebergInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref metadata_operation) = self.metadata_operation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetadataOperation", metadata_operation)?;
            }
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IcebergInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IcebergInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IcebergInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IcebergInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metadata_operation: Option<::Value<MetadataOperation>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MetadataOperation" => {
                                metadata_operation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IcebergInput {
                        metadata_operation: metadata_operation,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.MetadataOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-metadataoperation.html) property type.
    #[derive(Debug, Default)]
    pub struct MetadataOperation {
    }

    impl ::codec::SerializeValue for MetadataOperation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetadataOperation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetadataOperation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetadataOperation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetadataOperation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(MetadataOperation {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.OpenTableFormatInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-opentableformatinput.html) property type.
    #[derive(Debug, Default)]
    pub struct OpenTableFormatInput {
        /// Property [`IcebergInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-opentableformatinput.html#cfn-glue-table-opentableformatinput-iceberginput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iceberg_input: Option<::Value<IcebergInput>>,
    }

    impl ::codec::SerializeValue for OpenTableFormatInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iceberg_input) = self.iceberg_input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IcebergInput", iceberg_input)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OpenTableFormatInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OpenTableFormatInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OpenTableFormatInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OpenTableFormatInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iceberg_input: Option<::Value<IcebergInput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IcebergInput" => {
                                iceberg_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OpenTableFormatInput {
                        iceberg_input: iceberg_input,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-order.html) property type.
    #[derive(Debug, Default)]
    pub struct Order {
        /// Property [`Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-order.html#cfn-glue-table-order-column).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column: ::Value<String>,
        /// Property [`SortOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-order.html#cfn-glue-table-order-sortorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sort_order: ::Value<u32>,
    }

    impl ::codec::SerializeValue for Order {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Column", &self.column)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SortOrder", &self.sort_order)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Order {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Order, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Order;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Order")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column: Option<::Value<String>> = None;
                    let mut sort_order: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Column" => {
                                column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SortOrder" => {
                                sort_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Order {
                        column: column.ok_or(::serde::de::Error::missing_field("Column"))?,
                        sort_order: sort_order.ok_or(::serde::de::Error::missing_field("SortOrder"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.SchemaId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-schemaid.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaId {
        /// Property [`RegistryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-schemaid.html#cfn-glue-table-schemaid-registryname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub registry_name: Option<::Value<String>>,
        /// Property [`SchemaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-schemaid.html#cfn-glue-table-schemaid-schemaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_arn: Option<::Value<String>>,
        /// Property [`SchemaName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-schemaid.html#cfn-glue-table-schemaid-schemaname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SchemaId {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref registry_name) = self.registry_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegistryName", registry_name)?;
            }
            if let Some(ref schema_arn) = self.schema_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaArn", schema_arn)?;
            }
            if let Some(ref schema_name) = self.schema_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaName", schema_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaId {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaId, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaId;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaId")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut registry_name: Option<::Value<String>> = None;
                    let mut schema_arn: Option<::Value<String>> = None;
                    let mut schema_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RegistryName" => {
                                registry_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaArn" => {
                                schema_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaName" => {
                                schema_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaId {
                        registry_name: registry_name,
                        schema_arn: schema_arn,
                        schema_name: schema_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.SchemaReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-schemareference.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaReference {
        /// Property [`SchemaId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-schemareference.html#cfn-glue-table-schemareference-schemaid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_id: Option<::Value<SchemaId>>,
        /// Property [`SchemaVersionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-schemareference.html#cfn-glue-table-schemareference-schemaversionid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_version_id: Option<::Value<String>>,
        /// Property [`SchemaVersionNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-schemareference.html#cfn-glue-table-schemareference-schemaversionnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_version_number: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SchemaReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref schema_id) = self.schema_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaId", schema_id)?;
            }
            if let Some(ref schema_version_id) = self.schema_version_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaVersionId", schema_version_id)?;
            }
            if let Some(ref schema_version_number) = self.schema_version_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaVersionNumber", schema_version_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut schema_id: Option<::Value<SchemaId>> = None;
                    let mut schema_version_id: Option<::Value<String>> = None;
                    let mut schema_version_number: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SchemaId" => {
                                schema_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaVersionId" => {
                                schema_version_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaVersionNumber" => {
                                schema_version_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaReference {
                        schema_id: schema_id,
                        schema_version_id: schema_version_id,
                        schema_version_number: schema_version_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-serdeinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct SerdeInfo {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-serdeinfo.html#cfn-glue-table-serdeinfo-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-serdeinfo.html#cfn-glue-table-serdeinfo-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
        /// Property [`SerializationLibrary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-serdeinfo.html#cfn-glue-table-serdeinfo-serializationlibrary).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub serialization_library: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SerdeInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref serialization_library) = self.serialization_library {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SerializationLibrary", serialization_library)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SerdeInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SerdeInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SerdeInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SerdeInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;
                    let mut serialization_library: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SerializationLibrary" => {
                                serialization_library = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SerdeInfo {
                        name: name,
                        parameters: parameters,
                        serialization_library: serialization_library,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-skewedinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct SkewedInfo {
        /// Property [`SkewedColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-skewedinfo.html#cfn-glue-table-skewedinfo-skewedcolumnnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub skewed_column_names: Option<::ValueList<String>>,
        /// Property [`SkewedColumnValueLocationMaps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-skewedinfo.html#cfn-glue-table-skewedinfo-skewedcolumnvaluelocationmaps).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub skewed_column_value_location_maps: Option<::Value<::json::Value>>,
        /// Property [`SkewedColumnValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-skewedinfo.html#cfn-glue-table-skewedinfo-skewedcolumnvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub skewed_column_values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SkewedInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref skewed_column_names) = self.skewed_column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SkewedColumnNames", skewed_column_names)?;
            }
            if let Some(ref skewed_column_value_location_maps) = self.skewed_column_value_location_maps {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SkewedColumnValueLocationMaps", skewed_column_value_location_maps)?;
            }
            if let Some(ref skewed_column_values) = self.skewed_column_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SkewedColumnValues", skewed_column_values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SkewedInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SkewedInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SkewedInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SkewedInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut skewed_column_names: Option<::ValueList<String>> = None;
                    let mut skewed_column_value_location_maps: Option<::Value<::json::Value>> = None;
                    let mut skewed_column_values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SkewedColumnNames" => {
                                skewed_column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SkewedColumnValueLocationMaps" => {
                                skewed_column_value_location_maps = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SkewedColumnValues" => {
                                skewed_column_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SkewedInfo {
                        skewed_column_names: skewed_column_names,
                        skewed_column_value_location_maps: skewed_column_value_location_maps,
                        skewed_column_values: skewed_column_values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html) property type.
    #[derive(Debug, Default)]
    pub struct StorageDescriptor {
        /// Property [`BucketColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-bucketcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_columns: Option<::ValueList<String>>,
        /// Property [`Columns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-columns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub columns: Option<::ValueList<Column>>,
        /// Property [`Compressed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-compressed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compressed: Option<::Value<bool>>,
        /// Property [`InputFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-inputformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_format: Option<::Value<String>>,
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: Option<::Value<String>>,
        /// Property [`NumberOfBuckets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-numberofbuckets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_of_buckets: Option<::Value<u32>>,
        /// Property [`OutputFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-outputformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_format: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
        /// Property [`SchemaReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-schemareference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_reference: Option<::Value<SchemaReference>>,
        /// Property [`SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-serdeinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub serde_info: Option<::Value<SerdeInfo>>,
        /// Property [`SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-skewedinfo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub skewed_info: Option<::Value<SkewedInfo>>,
        /// Property [`SortColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-sortcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sort_columns: Option<::ValueList<Order>>,
        /// Property [`StoredAsSubDirectories`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html#cfn-glue-table-storagedescriptor-storedassubdirectories).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stored_as_sub_directories: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for StorageDescriptor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_columns) = self.bucket_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketColumns", bucket_columns)?;
            }
            if let Some(ref columns) = self.columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Columns", columns)?;
            }
            if let Some(ref compressed) = self.compressed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compressed", compressed)?;
            }
            if let Some(ref input_format) = self.input_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputFormat", input_format)?;
            }
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            if let Some(ref number_of_buckets) = self.number_of_buckets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfBuckets", number_of_buckets)?;
            }
            if let Some(ref output_format) = self.output_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputFormat", output_format)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref schema_reference) = self.schema_reference {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaReference", schema_reference)?;
            }
            if let Some(ref serde_info) = self.serde_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SerdeInfo", serde_info)?;
            }
            if let Some(ref skewed_info) = self.skewed_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SkewedInfo", skewed_info)?;
            }
            if let Some(ref sort_columns) = self.sort_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SortColumns", sort_columns)?;
            }
            if let Some(ref stored_as_sub_directories) = self.stored_as_sub_directories {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StoredAsSubDirectories", stored_as_sub_directories)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StorageDescriptor {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StorageDescriptor, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StorageDescriptor;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StorageDescriptor")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_columns: Option<::ValueList<String>> = None;
                    let mut columns: Option<::ValueList<Column>> = None;
                    let mut compressed: Option<::Value<bool>> = None;
                    let mut input_format: Option<::Value<String>> = None;
                    let mut location: Option<::Value<String>> = None;
                    let mut number_of_buckets: Option<::Value<u32>> = None;
                    let mut output_format: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;
                    let mut schema_reference: Option<::Value<SchemaReference>> = None;
                    let mut serde_info: Option<::Value<SerdeInfo>> = None;
                    let mut skewed_info: Option<::Value<SkewedInfo>> = None;
                    let mut sort_columns: Option<::ValueList<Order>> = None;
                    let mut stored_as_sub_directories: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketColumns" => {
                                bucket_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Columns" => {
                                columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Compressed" => {
                                compressed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputFormat" => {
                                input_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberOfBuckets" => {
                                number_of_buckets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputFormat" => {
                                output_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaReference" => {
                                schema_reference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SerdeInfo" => {
                                serde_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SkewedInfo" => {
                                skewed_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SortColumns" => {
                                sort_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StoredAsSubDirectories" => {
                                stored_as_sub_directories = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StorageDescriptor {
                        bucket_columns: bucket_columns,
                        columns: columns,
                        compressed: compressed,
                        input_format: input_format,
                        location: location,
                        number_of_buckets: number_of_buckets,
                        output_format: output_format,
                        parameters: parameters,
                        schema_reference: schema_reference,
                        serde_info: serde_info,
                        skewed_info: skewed_info,
                        sort_columns: sort_columns,
                        stored_as_sub_directories: stored_as_sub_directories,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.TableIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableidentifier.html) property type.
    #[derive(Debug, Default)]
    pub struct TableIdentifier {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableidentifier.html#cfn-glue-table-tableidentifier-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableidentifier.html#cfn-glue-table-tableidentifier-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableidentifier.html#cfn-glue-table-tableidentifier-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableidentifier.html#cfn-glue-table-tableidentifier-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TableIdentifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref region) = self.region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableIdentifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableIdentifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableIdentifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableIdentifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableIdentifier {
                        catalog_id: catalog_id,
                        database_name: database_name,
                        name: name,
                        region: region,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.TableInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html) property type.
    #[derive(Debug, Default)]
    pub struct TableInput {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Owner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-owner).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub owner: Option<::Value<String>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<::json::Value>>,
        /// Property [`PartitionKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-partitionkeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition_keys: Option<::ValueList<Column>>,
        /// Property [`Retention`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-retention).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retention: Option<::Value<u32>>,
        /// Property [`StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-storagedescriptor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_descriptor: Option<::Value<StorageDescriptor>>,
        /// Property [`TableType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-tabletype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_type: Option<::Value<String>>,
        /// Property [`TargetTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-targettable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_table: Option<::Value<TableIdentifier>>,
        /// Property [`ViewExpandedText`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-viewexpandedtext).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub view_expanded_text: Option<::Value<String>>,
        /// Property [`ViewOriginalText`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html#cfn-glue-table-tableinput-vieworiginaltext).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub view_original_text: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TableInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref owner) = self.owner {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Owner", owner)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref partition_keys) = self.partition_keys {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionKeys", partition_keys)?;
            }
            if let Some(ref retention) = self.retention {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Retention", retention)?;
            }
            if let Some(ref storage_descriptor) = self.storage_descriptor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageDescriptor", storage_descriptor)?;
            }
            if let Some(ref table_type) = self.table_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableType", table_type)?;
            }
            if let Some(ref target_table) = self.target_table {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetTable", target_table)?;
            }
            if let Some(ref view_expanded_text) = self.view_expanded_text {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewExpandedText", view_expanded_text)?;
            }
            if let Some(ref view_original_text) = self.view_original_text {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewOriginalText", view_original_text)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut owner: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;
                    let mut partition_keys: Option<::ValueList<Column>> = None;
                    let mut retention: Option<::Value<u32>> = None;
                    let mut storage_descriptor: Option<::Value<StorageDescriptor>> = None;
                    let mut table_type: Option<::Value<String>> = None;
                    let mut target_table: Option<::Value<TableIdentifier>> = None;
                    let mut view_expanded_text: Option<::Value<String>> = None;
                    let mut view_original_text: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Owner" => {
                                owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PartitionKeys" => {
                                partition_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Retention" => {
                                retention = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageDescriptor" => {
                                storage_descriptor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableType" => {
                                table_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetTable" => {
                                target_table = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ViewExpandedText" => {
                                view_expanded_text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ViewOriginalText" => {
                                view_original_text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableInput {
                        description: description,
                        name: name,
                        owner: owner,
                        parameters: parameters,
                        partition_keys: partition_keys,
                        retention: retention,
                        storage_descriptor: storage_descriptor,
                        table_type: table_type,
                        target_table: target_table,
                        view_expanded_text: view_expanded_text,
                        view_original_text: view_original_text,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod table_optimizer {
    //! Property types for the `TableOptimizer` resource.

    /// The [`AWS::Glue::TableOptimizer.TableOptimizerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-tableoptimizer-tableoptimizerconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct TableOptimizerConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-tableoptimizer-tableoptimizerconfiguration.html#cfn-glue-tableoptimizer-tableoptimizerconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-tableoptimizer-tableoptimizerconfiguration.html#cfn-glue-tableoptimizer-tableoptimizerconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TableOptimizerConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableOptimizerConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableOptimizerConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableOptimizerConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableOptimizerConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableOptimizerConfiguration {
                        enabled: enabled,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod trigger {
    //! Property types for the `Trigger` resource.

    /// The [`AWS::Glue::Trigger.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`Arguments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html#cfn-glue-trigger-action-arguments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arguments: Option<::Value<::json::Value>>,
        /// Property [`CrawlerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html#cfn-glue-trigger-action-crawlername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawler_name: Option<::Value<String>>,
        /// Property [`JobName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html#cfn-glue-trigger-action-jobname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_name: Option<::Value<String>>,
        /// Property [`NotificationProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html#cfn-glue-trigger-action-notificationproperty).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_property: Option<::Value<NotificationProperty>>,
        /// Property [`SecurityConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html#cfn-glue-trigger-action-securityconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_configuration: Option<::Value<String>>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html#cfn-glue-trigger-action-timeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arguments) = self.arguments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arguments", arguments)?;
            }
            if let Some(ref crawler_name) = self.crawler_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlerName", crawler_name)?;
            }
            if let Some(ref job_name) = self.job_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobName", job_name)?;
            }
            if let Some(ref notification_property) = self.notification_property {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationProperty", notification_property)?;
            }
            if let Some(ref security_configuration) = self.security_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityConfiguration", security_configuration)?;
            }
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
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
                    let mut arguments: Option<::Value<::json::Value>> = None;
                    let mut crawler_name: Option<::Value<String>> = None;
                    let mut job_name: Option<::Value<String>> = None;
                    let mut notification_property: Option<::Value<NotificationProperty>> = None;
                    let mut security_configuration: Option<::Value<String>> = None;
                    let mut timeout: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arguments" => {
                                arguments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CrawlerName" => {
                                crawler_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobName" => {
                                job_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationProperty" => {
                                notification_property = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityConfiguration" => {
                                security_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        arguments: arguments,
                        crawler_name: crawler_name,
                        job_name: job_name,
                        notification_property: notification_property,
                        security_configuration: security_configuration,
                        timeout: timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Trigger.Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html) property type.
    #[derive(Debug, Default)]
    pub struct Condition {
        /// Property [`CrawlState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html#cfn-glue-trigger-condition-crawlstate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawl_state: Option<::Value<String>>,
        /// Property [`CrawlerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html#cfn-glue-trigger-condition-crawlername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub crawler_name: Option<::Value<String>>,
        /// Property [`JobName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html#cfn-glue-trigger-condition-jobname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_name: Option<::Value<String>>,
        /// Property [`LogicalOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html#cfn-glue-trigger-condition-logicaloperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logical_operator: Option<::Value<String>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html#cfn-glue-trigger-condition-state).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Condition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref crawl_state) = self.crawl_state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlState", crawl_state)?;
            }
            if let Some(ref crawler_name) = self.crawler_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrawlerName", crawler_name)?;
            }
            if let Some(ref job_name) = self.job_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobName", job_name)?;
            }
            if let Some(ref logical_operator) = self.logical_operator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogicalOperator", logical_operator)?;
            }
            if let Some(ref state) = self.state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Condition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Condition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Condition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Condition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut crawl_state: Option<::Value<String>> = None;
                    let mut crawler_name: Option<::Value<String>> = None;
                    let mut job_name: Option<::Value<String>> = None;
                    let mut logical_operator: Option<::Value<String>> = None;
                    let mut state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CrawlState" => {
                                crawl_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CrawlerName" => {
                                crawler_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobName" => {
                                job_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogicalOperator" => {
                                logical_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Condition {
                        crawl_state: crawl_state,
                        crawler_name: crawler_name,
                        job_name: job_name,
                        logical_operator: logical_operator,
                        state: state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Trigger.EventBatchingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-eventbatchingcondition.html) property type.
    #[derive(Debug, Default)]
    pub struct EventBatchingCondition {
        /// Property [`BatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-eventbatchingcondition.html#cfn-glue-trigger-eventbatchingcondition-batchsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_size: ::Value<u32>,
        /// Property [`BatchWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-eventbatchingcondition.html#cfn-glue-trigger-eventbatchingcondition-batchwindow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_window: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EventBatchingCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchSize", &self.batch_size)?;
            if let Some(ref batch_window) = self.batch_window {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchWindow", batch_window)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventBatchingCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventBatchingCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventBatchingCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventBatchingCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_size: Option<::Value<u32>> = None;
                    let mut batch_window: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchSize" => {
                                batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BatchWindow" => {
                                batch_window = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventBatchingCondition {
                        batch_size: batch_size.ok_or(::serde::de::Error::missing_field("BatchSize"))?,
                        batch_window: batch_window,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Trigger.NotificationProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-notificationproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationProperty {
        /// Property [`NotifyDelayAfter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-notificationproperty.html#cfn-glue-trigger-notificationproperty-notifydelayafter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notify_delay_after: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for NotificationProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref notify_delay_after) = self.notify_delay_after {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotifyDelayAfter", notify_delay_after)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut notify_delay_after: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NotifyDelayAfter" => {
                                notify_delay_after = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationProperty {
                        notify_delay_after: notify_delay_after,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Trigger.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-predicate.html) property type.
    #[derive(Debug, Default)]
    pub struct Predicate {
        /// Property [`Conditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-predicate.html#cfn-glue-trigger-predicate-conditions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conditions: Option<::ValueList<Condition>>,
        /// Property [`Logical`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-predicate.html#cfn-glue-trigger-predicate-logical).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logical: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Predicate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref conditions) = self.conditions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Conditions", conditions)?;
            }
            if let Some(ref logical) = self.logical {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logical", logical)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Predicate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Predicate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Predicate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Predicate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut conditions: Option<::ValueList<Condition>> = None;
                    let mut logical: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Conditions" => {
                                conditions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Logical" => {
                                logical = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Predicate {
                        conditions: conditions,
                        logical: logical,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
