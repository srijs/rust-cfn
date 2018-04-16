//! Types for the `Glue` service.

/// The [`AWS::Glue::Classifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html) resource type.
#[derive(Debug)]
pub struct Classifier {
    properties: ClassifierProperties
}

/// Properties for the `Classifier` resource.
#[derive(Debug)]
pub struct ClassifierProperties {
    /// Property [`GrokClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html#cfn-glue-classifier-grokclassifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub grok_classifier: Option<::Value<self::classifier::GrokClassifier>>,
}

impl ::serde::Serialize for ClassifierProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref grok_classifier) = self.grok_classifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GrokClassifier", grok_classifier)?;
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
                let mut grok_classifier: Option<::Value<self::classifier::GrokClassifier>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GrokClassifier" => {
                            grok_classifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClassifierProperties {
                    grok_classifier: grok_classifier,
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
#[derive(Debug)]
pub struct Connection {
    properties: ConnectionProperties
}

/// Properties for the `Connection` resource.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Crawler {
    properties: CrawlerProperties
}

/// Properties for the `Crawler` resource.
#[derive(Debug)]
pub struct CrawlerProperties {
    /// Property [`Classifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-classifiers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub classifiers: Option<::ValueList<String>>,
    /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html#cfn-glue-crawler-databasename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub database_name: ::Value<String>,
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
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
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
                let mut database_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role: Option<::Value<String>> = None;
                let mut schedule: Option<::Value<self::crawler::Schedule>> = None;
                let mut schema_change_policy: Option<::Value<self::crawler::SchemaChangePolicy>> = None;
                let mut table_prefix: Option<::Value<String>> = None;
                let mut targets: Option<::Value<self::crawler::Targets>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Classifiers" => {
                            classifiers = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "Targets" => {
                            targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CrawlerProperties {
                    classifiers: classifiers,
                    database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                    description: description,
                    name: name,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                    schedule: schedule,
                    schema_change_policy: schema_change_policy,
                    table_prefix: table_prefix,
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

/// The [`AWS::Glue::Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-database.html) resource type.
#[derive(Debug)]
pub struct Database {
    properties: DatabaseProperties
}

/// Properties for the `Database` resource.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct DevEndpoint {
    properties: DevEndpointProperties
}

/// Properties for the `DevEndpoint` resource.
#[derive(Debug)]
pub struct DevEndpointProperties {
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
    /// Property [`NumberOfNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-numberofnodes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub number_of_nodes: Option<::Value<u32>>,
    /// Property [`PublicKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-publickey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub public_key: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html#cfn-glue-devendpoint-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
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
}

impl ::serde::Serialize for DevEndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref endpoint_name) = self.endpoint_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", endpoint_name)?;
        }
        if let Some(ref extra_jars_s3_path) = self.extra_jars_s3_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtraJarsS3Path", extra_jars_s3_path)?;
        }
        if let Some(ref extra_python_libs_s3_path) = self.extra_python_libs_s3_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtraPythonLibsS3Path", extra_python_libs_s3_path)?;
        }
        if let Some(ref number_of_nodes) = self.number_of_nodes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfNodes", number_of_nodes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublicKey", &self.public_key)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref subnet_id) = self.subnet_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", subnet_id)?;
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
                let mut endpoint_name: Option<::Value<String>> = None;
                let mut extra_jars_s3_path: Option<::Value<String>> = None;
                let mut extra_python_libs_s3_path: Option<::Value<String>> = None;
                let mut number_of_nodes: Option<::Value<u32>> = None;
                let mut public_key: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut subnet_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EndpointName" => {
                            endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExtraJarsS3Path" => {
                            extra_jars_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExtraPythonLibsS3Path" => {
                            extra_python_libs_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumberOfNodes" => {
                            number_of_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PublicKey" => {
                            public_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetId" => {
                            subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DevEndpointProperties {
                    endpoint_name: endpoint_name,
                    extra_jars_s3_path: extra_jars_s3_path,
                    extra_python_libs_s3_path: extra_python_libs_s3_path,
                    number_of_nodes: number_of_nodes,
                    public_key: public_key.ok_or(::serde::de::Error::missing_field("PublicKey"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    security_group_ids: security_group_ids,
                    subnet_id: subnet_id,
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
#[derive(Debug)]
pub struct Job {
    properties: JobProperties
}

/// Properties for the `Job` resource.
#[derive(Debug)]
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
    /// Property [`ExecutionProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-executionproperty).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_property: Option<::Value<self::job::ExecutionProperty>>,
    /// Property [`LogUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-loguri).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_uri: Option<::Value<String>>,
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
    /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html#cfn-glue-job-role).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role: ::Value<String>,
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
        if let Some(ref execution_property) = self.execution_property {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionProperty", execution_property)?;
        }
        if let Some(ref log_uri) = self.log_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogUri", log_uri)?;
        }
        if let Some(ref max_retries) = self.max_retries {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRetries", max_retries)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
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
                let mut execution_property: Option<::Value<self::job::ExecutionProperty>> = None;
                let mut log_uri: Option<::Value<String>> = None;
                let mut max_retries: Option<::Value<f64>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role: Option<::Value<String>> = None;

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
                        "ExecutionProperty" => {
                            execution_property = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogUri" => {
                            log_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxRetries" => {
                            max_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Role" => {
                            role = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    execution_property: execution_property,
                    log_uri: log_uri,
                    max_retries: max_retries,
                    name: name,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
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

/// The [`AWS::Glue::Partition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-partition.html) resource type.
#[derive(Debug)]
pub struct Partition {
    properties: PartitionProperties
}

/// Properties for the `Partition` resource.
#[derive(Debug)]
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

/// The [`AWS::Glue::Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-table.html) resource type.
#[derive(Debug)]
pub struct Table {
    properties: TableProperties
}

/// Properties for the `Table` resource.
#[derive(Debug)]
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
                let mut table_input: Option<::Value<self::table::TableInput>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CatalogId" => {
                            catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatabaseName" => {
                            database_name = ::serde::de::MapAccess::next_value(&mut map)?;
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

/// The [`AWS::Glue::Trigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html) resource type.
#[derive(Debug)]
pub struct Trigger {
    properties: TriggerProperties
}

/// Properties for the `Trigger` resource.
#[derive(Debug)]
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
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html#cfn-glue-trigger-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub type_: ::Value<String>,
}

impl ::serde::Serialize for TriggerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
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
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
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
                let mut name: Option<::Value<String>> = None;
                let mut predicate: Option<::Value<self::trigger::Predicate>> = None;
                let mut schedule: Option<::Value<String>> = None;
                let mut type_: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Actions" => {
                            actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "Type" => {
                            type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TriggerProperties {
                    actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                    description: description,
                    name: name,
                    predicate: predicate,
                    schedule: schedule,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
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

pub mod classifier {
    //! Property types for the `Classifier` resource.

    /// The [`AWS::Glue::Classifier.GrokClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-grokclassifier.html) property type.
    #[derive(Debug)]
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
}

pub mod connection {
    //! Property types for the `Connection` resource.

    /// The [`AWS::Glue::Connection.ConnectionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html) property type.
    #[derive(Debug)]
    pub struct ConnectionInput {
        /// Property [`ConnectionProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html#cfn-glue-connection-connectioninput-connectionproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_properties: ::Value<::json::Value>,
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
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionProperties", &self.connection_properties)?;
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
                        connection_properties: connection_properties.ok_or(::serde::de::Error::missing_field("ConnectionProperties"))?,
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
    #[derive(Debug)]
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

    /// The [`AWS::Glue::Crawler.JdbcTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-jdbctarget.html) property type.
    #[derive(Debug)]
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

    /// The [`AWS::Glue::Crawler.S3Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html) property type.
    #[derive(Debug)]
    pub struct S3Target {
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
    }

    impl ::codec::SerializeValue for S3Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exclusions) = self.exclusions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exclusions", exclusions)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
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
                    let mut exclusions: Option<::ValueList<String>> = None;
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Exclusions" => {
                                exclusions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Target {
                        exclusions: exclusions,
                        path: path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Crawler.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schedule.html) property type.
    #[derive(Debug)]
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
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub struct Targets {
        /// Property [`JdbcTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html#cfn-glue-crawler-targets-jdbctargets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub jdbc_targets: Option<::ValueList<JdbcTarget>>,
        /// Property [`S3Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html#cfn-glue-crawler-targets-s3targets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_targets: Option<::ValueList<S3Target>>,
    }

    impl ::codec::SerializeValue for Targets {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref jdbc_targets) = self.jdbc_targets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JdbcTargets", jdbc_targets)?;
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
                    let mut jdbc_targets: Option<::ValueList<JdbcTarget>> = None;
                    let mut s3_targets: Option<::ValueList<S3Target>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JdbcTargets" => {
                                jdbc_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Targets" => {
                                s3_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Targets {
                        jdbc_targets: jdbc_targets,
                        s3_targets: s3_targets,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod database {
    //! Property types for the `Database` resource.

    /// The [`AWS::Glue::Database.DatabaseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html) property type.
    #[derive(Debug)]
    pub struct DatabaseInput {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html#cfn-glue-database-databaseinput-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
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
    }

    impl ::codec::SerializeValue for DatabaseInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
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
                    let mut description: Option<::Value<String>> = None;
                    let mut location_uri: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            _ => {}
                        }
                    }

                    Ok(DatabaseInput {
                        description: description,
                        location_uri: location_uri,
                        name: name,
                        parameters: parameters,
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
    #[derive(Debug)]
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
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub struct JobCommand {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html#cfn-glue-job-jobcommand-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
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
                    let mut script_location: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScriptLocation" => {
                                script_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JobCommand {
                        name: name,
                        script_location: script_location,
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
    #[derive(Debug)]
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
        pub type_: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Column {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref type_) = self.type_ {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", type_)?;
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
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Column {
                        comment: comment,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        type_: type_,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Partition.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-order.html) property type.
    #[derive(Debug)]
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
    #[derive(Debug)]
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

    /// The [`AWS::Glue::Partition.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-serdeinfo.html) property type.
    #[derive(Debug)]
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
    #[derive(Debug)]
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
    #[derive(Debug)]
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

pub mod table {
    //! Property types for the `Table` resource.

    /// The [`AWS::Glue::Table.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-column.html) property type.
    #[derive(Debug)]
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
        pub type_: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Column {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comment) = self.comment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", comment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref type_) = self.type_ {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", type_)?;
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
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Column {
                        comment: comment,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        type_: type_,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Table.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-order.html) property type.
    #[derive(Debug)]
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

    /// The [`AWS::Glue::Table.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-serdeinfo.html) property type.
    #[derive(Debug)]
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
    #[derive(Debug)]
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
    #[derive(Debug)]
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

    /// The [`AWS::Glue::Table.TableInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html) property type.
    #[derive(Debug)]
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
                        view_expanded_text: view_expanded_text,
                        view_original_text: view_original_text,
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
    #[derive(Debug)]
    pub struct Action {
        /// Property [`Arguments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html#cfn-glue-trigger-action-arguments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arguments: Option<::Value<::json::Value>>,
        /// Property [`JobName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html#cfn-glue-trigger-action-jobname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arguments) = self.arguments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arguments", arguments)?;
            }
            if let Some(ref job_name) = self.job_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobName", job_name)?;
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
                    let mut job_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arguments" => {
                                arguments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobName" => {
                                job_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        arguments: arguments,
                        job_name: job_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Trigger.Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html) property type.
    #[derive(Debug)]
    pub struct Condition {
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
                    let mut job_name: Option<::Value<String>> = None;
                    let mut logical_operator: Option<::Value<String>> = None;
                    let mut state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
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
                        job_name: job_name,
                        logical_operator: logical_operator,
                        state: state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Glue::Trigger.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-predicate.html) property type.
    #[derive(Debug)]
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
