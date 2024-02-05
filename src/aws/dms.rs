//! Types for the `DMS` service.

/// The [`AWS::DMS::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-certificate.html) resource type.
#[derive(Debug, Default)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug, Default)]
pub struct CertificateProperties {
    /// Property [`CertificateIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-certificate.html#cfn-dms-certificate-certificateidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_identifier: Option<::Value<String>>,
    /// Property [`CertificatePem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-certificate.html#cfn-dms-certificate-certificatepem).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_pem: Option<::Value<String>>,
    /// Property [`CertificateWallet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-certificate.html#cfn-dms-certificate-certificatewallet).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_wallet: Option<::Value<String>>,
}

impl ::serde::Serialize for CertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref certificate_identifier) = self.certificate_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateIdentifier", certificate_identifier)?;
        }
        if let Some(ref certificate_pem) = self.certificate_pem {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificatePem", certificate_pem)?;
        }
        if let Some(ref certificate_wallet) = self.certificate_wallet {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateWallet", certificate_wallet)?;
        }
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
                let mut certificate_identifier: Option<::Value<String>> = None;
                let mut certificate_pem: Option<::Value<String>> = None;
                let mut certificate_wallet: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateIdentifier" => {
                            certificate_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificatePem" => {
                            certificate_pem = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateWallet" => {
                            certificate_wallet = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CertificateProperties {
                    certificate_identifier: certificate_identifier,
                    certificate_pem: certificate_pem,
                    certificate_wallet: certificate_wallet,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::DMS::Certificate";
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

/// The [`AWS::DMS::DataProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-dataprovider.html) resource type.
#[derive(Debug, Default)]
pub struct DataProvider {
    properties: DataProviderProperties
}

/// Properties for the `DataProvider` resource.
#[derive(Debug, Default)]
pub struct DataProviderProperties {
    /// Property [`DataProviderIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-dataprovider.html#cfn-dms-dataprovider-dataprovideridentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_provider_identifier: Option<::Value<String>>,
    /// Property [`DataProviderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-dataprovider.html#cfn-dms-dataprovider-dataprovidername).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_provider_name: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-dataprovider.html#cfn-dms-dataprovider-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-dataprovider.html#cfn-dms-dataprovider-engine).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine: ::Value<String>,
    /// Property [`ExactSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-dataprovider.html#cfn-dms-dataprovider-exactsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub exact_settings: Option<::Value<bool>>,
    /// Property [`Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-dataprovider.html#cfn-dms-dataprovider-settings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub settings: Option<::Value<self::data_provider::Settings>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-dataprovider.html#cfn-dms-dataprovider-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DataProviderProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref data_provider_identifier) = self.data_provider_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataProviderIdentifier", data_provider_identifier)?;
        }
        if let Some(ref data_provider_name) = self.data_provider_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataProviderName", data_provider_name)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        if let Some(ref exact_settings) = self.exact_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExactSettings", exact_settings)?;
        }
        if let Some(ref settings) = self.settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Settings", settings)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataProviderProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataProviderProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataProviderProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataProviderProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_provider_identifier: Option<::Value<String>> = None;
                let mut data_provider_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut exact_settings: Option<::Value<bool>> = None;
                let mut settings: Option<::Value<self::data_provider::Settings>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataProviderIdentifier" => {
                            data_provider_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataProviderName" => {
                            data_provider_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExactSettings" => {
                            exact_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Settings" => {
                            settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataProviderProperties {
                    data_provider_identifier: data_provider_identifier,
                    data_provider_name: data_provider_name,
                    description: description,
                    engine: engine.ok_or(::serde::de::Error::missing_field("Engine"))?,
                    exact_settings: exact_settings,
                    settings: settings,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataProvider {
    type Properties = DataProviderProperties;
    const TYPE: &'static str = "AWS::DMS::DataProvider";
    fn properties(&self) -> &DataProviderProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataProviderProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataProvider {}

impl From<DataProviderProperties> for DataProvider {
    fn from(properties: DataProviderProperties) -> DataProvider {
        DataProvider { properties }
    }
}

/// The [`AWS::DMS::Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html) resource type.
#[derive(Debug, Default)]
pub struct Endpoint {
    properties: EndpointProperties
}

/// Properties for the `Endpoint` resource.
#[derive(Debug, Default)]
pub struct EndpointProperties {
    /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-certificatearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate_arn: Option<::Value<String>>,
    /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-databasename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub database_name: Option<::Value<String>>,
    /// Property [`DocDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-docdbsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub doc_db_settings: Option<::Value<self::endpoint::DocDbSettings>>,
    /// Property [`DynamoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-dynamodbsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dynamo_db_settings: Option<::Value<self::endpoint::DynamoDbSettings>>,
    /// Property [`ElasticsearchSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-elasticsearchsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub elasticsearch_settings: Option<::Value<self::endpoint::ElasticsearchSettings>>,
    /// Property [`EndpointIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-endpointidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_identifier: Option<::Value<String>>,
    /// Property [`EndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-endpointtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_type: ::Value<String>,
    /// Property [`EngineName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-enginename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_name: ::Value<String>,
    /// Property [`ExtraConnectionAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-extraconnectionattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub extra_connection_attributes: Option<::Value<String>>,
    /// Property [`GcpMySQLSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-gcpmysqlsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub gcp_my_sql_settings: Option<::Value<self::endpoint::GcpMySQLSettings>>,
    /// Property [`IbmDb2Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-ibmdb2settings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ibm_db2_settings: Option<::Value<self::endpoint::IbmDb2Settings>>,
    /// Property [`KafkaSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-kafkasettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kafka_settings: Option<::Value<self::endpoint::KafkaSettings>>,
    /// Property [`KinesisSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-kinesissettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kinesis_settings: Option<::Value<self::endpoint::KinesisSettings>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`MicrosoftSqlServerSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-microsoftsqlserversettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub microsoft_sql_server_settings: Option<::Value<self::endpoint::MicrosoftSqlServerSettings>>,
    /// Property [`MongoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-mongodbsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mongo_db_settings: Option<::Value<self::endpoint::MongoDbSettings>>,
    /// Property [`MySqlSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-mysqlsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub my_sql_settings: Option<::Value<self::endpoint::MySqlSettings>>,
    /// Property [`NeptuneSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-neptunesettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub neptune_settings: Option<::Value<self::endpoint::NeptuneSettings>>,
    /// Property [`OracleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-oraclesettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub oracle_settings: Option<::Value<self::endpoint::OracleSettings>>,
    /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-password).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub password: Option<::Value<String>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-port).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub port: Option<::Value<u32>>,
    /// Property [`PostgreSqlSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-postgresqlsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub postgre_sql_settings: Option<::Value<self::endpoint::PostgreSqlSettings>>,
    /// Property [`RedisSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-redissettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub redis_settings: Option<::Value<self::endpoint::RedisSettings>>,
    /// Property [`RedshiftSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-redshiftsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub redshift_settings: Option<::Value<self::endpoint::RedshiftSettings>>,
    /// Property [`ResourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-resourceidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_identifier: Option<::Value<String>>,
    /// Property [`S3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-s3settings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_settings: Option<::Value<self::endpoint::S3Settings>>,
    /// Property [`ServerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-servername).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub server_name: Option<::Value<String>>,
    /// Property [`SslMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-sslmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ssl_mode: Option<::Value<String>>,
    /// Property [`SybaseSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-sybasesettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sybase_settings: Option<::Value<self::endpoint::SybaseSettings>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-endpoint.html#cfn-dms-endpoint-username).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub username: Option<::Value<String>>,
}

impl ::serde::Serialize for EndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref certificate_arn) = self.certificate_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
        }
        if let Some(ref database_name) = self.database_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
        }
        if let Some(ref doc_db_settings) = self.doc_db_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocDbSettings", doc_db_settings)?;
        }
        if let Some(ref dynamo_db_settings) = self.dynamo_db_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDbSettings", dynamo_db_settings)?;
        }
        if let Some(ref elasticsearch_settings) = self.elasticsearch_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticsearchSettings", elasticsearch_settings)?;
        }
        if let Some(ref endpoint_identifier) = self.endpoint_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointIdentifier", endpoint_identifier)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointType", &self.endpoint_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineName", &self.engine_name)?;
        if let Some(ref extra_connection_attributes) = self.extra_connection_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtraConnectionAttributes", extra_connection_attributes)?;
        }
        if let Some(ref gcp_my_sql_settings) = self.gcp_my_sql_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GcpMySQLSettings", gcp_my_sql_settings)?;
        }
        if let Some(ref ibm_db2_settings) = self.ibm_db2_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IbmDb2Settings", ibm_db2_settings)?;
        }
        if let Some(ref kafka_settings) = self.kafka_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KafkaSettings", kafka_settings)?;
        }
        if let Some(ref kinesis_settings) = self.kinesis_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisSettings", kinesis_settings)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref microsoft_sql_server_settings) = self.microsoft_sql_server_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MicrosoftSqlServerSettings", microsoft_sql_server_settings)?;
        }
        if let Some(ref mongo_db_settings) = self.mongo_db_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MongoDbSettings", mongo_db_settings)?;
        }
        if let Some(ref my_sql_settings) = self.my_sql_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MySqlSettings", my_sql_settings)?;
        }
        if let Some(ref neptune_settings) = self.neptune_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NeptuneSettings", neptune_settings)?;
        }
        if let Some(ref oracle_settings) = self.oracle_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OracleSettings", oracle_settings)?;
        }
        if let Some(ref password) = self.password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref postgre_sql_settings) = self.postgre_sql_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostgreSqlSettings", postgre_sql_settings)?;
        }
        if let Some(ref redis_settings) = self.redis_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedisSettings", redis_settings)?;
        }
        if let Some(ref redshift_settings) = self.redshift_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftSettings", redshift_settings)?;
        }
        if let Some(ref resource_identifier) = self.resource_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceIdentifier", resource_identifier)?;
        }
        if let Some(ref s3_settings) = self.s3_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Settings", s3_settings)?;
        }
        if let Some(ref server_name) = self.server_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", server_name)?;
        }
        if let Some(ref ssl_mode) = self.ssl_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslMode", ssl_mode)?;
        }
        if let Some(ref sybase_settings) = self.sybase_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SybaseSettings", sybase_settings)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref username) = self.username {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EndpointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EndpointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate_arn: Option<::Value<String>> = None;
                let mut database_name: Option<::Value<String>> = None;
                let mut doc_db_settings: Option<::Value<self::endpoint::DocDbSettings>> = None;
                let mut dynamo_db_settings: Option<::Value<self::endpoint::DynamoDbSettings>> = None;
                let mut elasticsearch_settings: Option<::Value<self::endpoint::ElasticsearchSettings>> = None;
                let mut endpoint_identifier: Option<::Value<String>> = None;
                let mut endpoint_type: Option<::Value<String>> = None;
                let mut engine_name: Option<::Value<String>> = None;
                let mut extra_connection_attributes: Option<::Value<String>> = None;
                let mut gcp_my_sql_settings: Option<::Value<self::endpoint::GcpMySQLSettings>> = None;
                let mut ibm_db2_settings: Option<::Value<self::endpoint::IbmDb2Settings>> = None;
                let mut kafka_settings: Option<::Value<self::endpoint::KafkaSettings>> = None;
                let mut kinesis_settings: Option<::Value<self::endpoint::KinesisSettings>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut microsoft_sql_server_settings: Option<::Value<self::endpoint::MicrosoftSqlServerSettings>> = None;
                let mut mongo_db_settings: Option<::Value<self::endpoint::MongoDbSettings>> = None;
                let mut my_sql_settings: Option<::Value<self::endpoint::MySqlSettings>> = None;
                let mut neptune_settings: Option<::Value<self::endpoint::NeptuneSettings>> = None;
                let mut oracle_settings: Option<::Value<self::endpoint::OracleSettings>> = None;
                let mut password: Option<::Value<String>> = None;
                let mut port: Option<::Value<u32>> = None;
                let mut postgre_sql_settings: Option<::Value<self::endpoint::PostgreSqlSettings>> = None;
                let mut redis_settings: Option<::Value<self::endpoint::RedisSettings>> = None;
                let mut redshift_settings: Option<::Value<self::endpoint::RedshiftSettings>> = None;
                let mut resource_identifier: Option<::Value<String>> = None;
                let mut s3_settings: Option<::Value<self::endpoint::S3Settings>> = None;
                let mut server_name: Option<::Value<String>> = None;
                let mut ssl_mode: Option<::Value<String>> = None;
                let mut sybase_settings: Option<::Value<self::endpoint::SybaseSettings>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut username: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateArn" => {
                            certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatabaseName" => {
                            database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DocDbSettings" => {
                            doc_db_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DynamoDbSettings" => {
                            dynamo_db_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ElasticsearchSettings" => {
                            elasticsearch_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointIdentifier" => {
                            endpoint_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointType" => {
                            endpoint_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineName" => {
                            engine_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExtraConnectionAttributes" => {
                            extra_connection_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GcpMySQLSettings" => {
                            gcp_my_sql_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IbmDb2Settings" => {
                            ibm_db2_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KafkaSettings" => {
                            kafka_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisSettings" => {
                            kinesis_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MicrosoftSqlServerSettings" => {
                            microsoft_sql_server_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MongoDbSettings" => {
                            mongo_db_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MySqlSettings" => {
                            my_sql_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NeptuneSettings" => {
                            neptune_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OracleSettings" => {
                            oracle_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Password" => {
                            password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PostgreSqlSettings" => {
                            postgre_sql_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RedisSettings" => {
                            redis_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RedshiftSettings" => {
                            redshift_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceIdentifier" => {
                            resource_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Settings" => {
                            s3_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerName" => {
                            server_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SslMode" => {
                            ssl_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SybaseSettings" => {
                            sybase_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Username" => {
                            username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EndpointProperties {
                    certificate_arn: certificate_arn,
                    database_name: database_name,
                    doc_db_settings: doc_db_settings,
                    dynamo_db_settings: dynamo_db_settings,
                    elasticsearch_settings: elasticsearch_settings,
                    endpoint_identifier: endpoint_identifier,
                    endpoint_type: endpoint_type.ok_or(::serde::de::Error::missing_field("EndpointType"))?,
                    engine_name: engine_name.ok_or(::serde::de::Error::missing_field("EngineName"))?,
                    extra_connection_attributes: extra_connection_attributes,
                    gcp_my_sql_settings: gcp_my_sql_settings,
                    ibm_db2_settings: ibm_db2_settings,
                    kafka_settings: kafka_settings,
                    kinesis_settings: kinesis_settings,
                    kms_key_id: kms_key_id,
                    microsoft_sql_server_settings: microsoft_sql_server_settings,
                    mongo_db_settings: mongo_db_settings,
                    my_sql_settings: my_sql_settings,
                    neptune_settings: neptune_settings,
                    oracle_settings: oracle_settings,
                    password: password,
                    port: port,
                    postgre_sql_settings: postgre_sql_settings,
                    redis_settings: redis_settings,
                    redshift_settings: redshift_settings,
                    resource_identifier: resource_identifier,
                    s3_settings: s3_settings,
                    server_name: server_name,
                    ssl_mode: ssl_mode,
                    sybase_settings: sybase_settings,
                    tags: tags,
                    username: username,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Endpoint {
    type Properties = EndpointProperties;
    const TYPE: &'static str = "AWS::DMS::Endpoint";
    fn properties(&self) -> &EndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Endpoint {}

impl From<EndpointProperties> for Endpoint {
    fn from(properties: EndpointProperties) -> Endpoint {
        Endpoint { properties }
    }
}

/// The [`AWS::DMS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html) resource type.
#[derive(Debug, Default)]
pub struct EventSubscription {
    properties: EventSubscriptionProperties
}

/// Properties for the `EventSubscription` resource.
#[derive(Debug, Default)]
pub struct EventSubscriptionProperties {
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html#cfn-dms-eventsubscription-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`EventCategories`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html#cfn-dms-eventsubscription-eventcategories).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_categories: Option<::ValueList<String>>,
    /// Property [`SnsTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html#cfn-dms-eventsubscription-snstopicarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sns_topic_arn: ::Value<String>,
    /// Property [`SourceIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html#cfn-dms-eventsubscription-sourceids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_ids: Option<::ValueList<String>>,
    /// Property [`SourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html#cfn-dms-eventsubscription-sourcetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_type: Option<::Value<String>>,
    /// Property [`SubscriptionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html#cfn-dms-eventsubscription-subscriptionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subscription_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-eventsubscription.html#cfn-dms-eventsubscription-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EventSubscriptionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref event_categories) = self.event_categories {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventCategories", event_categories)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArn", &self.sns_topic_arn)?;
        if let Some(ref source_ids) = self.source_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIds", source_ids)?;
        }
        if let Some(ref source_type) = self.source_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceType", source_type)?;
        }
        if let Some(ref subscription_name) = self.subscription_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubscriptionName", subscription_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventSubscriptionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSubscriptionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventSubscriptionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventSubscriptionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut enabled: Option<::Value<bool>> = None;
                let mut event_categories: Option<::ValueList<String>> = None;
                let mut sns_topic_arn: Option<::Value<String>> = None;
                let mut source_ids: Option<::ValueList<String>> = None;
                let mut source_type: Option<::Value<String>> = None;
                let mut subscription_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventCategories" => {
                            event_categories = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnsTopicArn" => {
                            sns_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceIds" => {
                            source_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceType" => {
                            source_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubscriptionName" => {
                            subscription_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventSubscriptionProperties {
                    enabled: enabled,
                    event_categories: event_categories,
                    sns_topic_arn: sns_topic_arn.ok_or(::serde::de::Error::missing_field("SnsTopicArn"))?,
                    source_ids: source_ids,
                    source_type: source_type,
                    subscription_name: subscription_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventSubscription {
    type Properties = EventSubscriptionProperties;
    const TYPE: &'static str = "AWS::DMS::EventSubscription";
    fn properties(&self) -> &EventSubscriptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventSubscriptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventSubscription {}

impl From<EventSubscriptionProperties> for EventSubscription {
    fn from(properties: EventSubscriptionProperties) -> EventSubscription {
        EventSubscription { properties }
    }
}

/// The [`AWS::DMS::InstanceProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html) resource type.
#[derive(Debug, Default)]
pub struct InstanceProfile {
    properties: InstanceProfileProperties
}

/// Properties for the `InstanceProfile` resource.
#[derive(Debug, Default)]
pub struct InstanceProfileProperties {
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html#cfn-dms-instanceprofile-availabilityzone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub availability_zone: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html#cfn-dms-instanceprofile-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceProfileIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html#cfn-dms-instanceprofile-instanceprofileidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_profile_identifier: Option<::Value<String>>,
    /// Property [`InstanceProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html#cfn-dms-instanceprofile-instanceprofilename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_profile_name: Option<::Value<String>>,
    /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html#cfn-dms-instanceprofile-kmskeyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_arn: Option<::Value<String>>,
    /// Property [`NetworkType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html#cfn-dms-instanceprofile-networktype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_type: Option<::Value<String>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html#cfn-dms-instanceprofile-publiclyaccessible).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property [`SubnetGroupIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html#cfn-dms-instanceprofile-subnetgroupidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_group_identifier: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html#cfn-dms-instanceprofile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-instanceprofile.html#cfn-dms-instanceprofile-vpcsecuritygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_groups: Option<::ValueList<String>>,
}

impl ::serde::Serialize for InstanceProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref instance_profile_identifier) = self.instance_profile_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProfileIdentifier", instance_profile_identifier)?;
        }
        if let Some(ref instance_profile_name) = self.instance_profile_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProfileName", instance_profile_name)?;
        }
        if let Some(ref kms_key_arn) = self.kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
        }
        if let Some(ref network_type) = self.network_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkType", network_type)?;
        }
        if let Some(ref publicly_accessible) = self.publicly_accessible {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
        }
        if let Some(ref subnet_group_identifier) = self.subnet_group_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetGroupIdentifier", subnet_group_identifier)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_security_groups) = self.vpc_security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroups", vpc_security_groups)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut availability_zone: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_profile_identifier: Option<::Value<String>> = None;
                let mut instance_profile_name: Option<::Value<String>> = None;
                let mut kms_key_arn: Option<::Value<String>> = None;
                let mut network_type: Option<::Value<String>> = None;
                let mut publicly_accessible: Option<::Value<bool>> = None;
                let mut subnet_group_identifier: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_security_groups: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceProfileIdentifier" => {
                            instance_profile_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceProfileName" => {
                            instance_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyArn" => {
                            kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkType" => {
                            network_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetGroupIdentifier" => {
                            subnet_group_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroups" => {
                            vpc_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InstanceProfileProperties {
                    availability_zone: availability_zone,
                    description: description,
                    instance_profile_identifier: instance_profile_identifier,
                    instance_profile_name: instance_profile_name,
                    kms_key_arn: kms_key_arn,
                    network_type: network_type,
                    publicly_accessible: publicly_accessible,
                    subnet_group_identifier: subnet_group_identifier,
                    tags: tags,
                    vpc_security_groups: vpc_security_groups,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InstanceProfile {
    type Properties = InstanceProfileProperties;
    const TYPE: &'static str = "AWS::DMS::InstanceProfile";
    fn properties(&self) -> &InstanceProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InstanceProfile {}

impl From<InstanceProfileProperties> for InstanceProfile {
    fn from(properties: InstanceProfileProperties) -> InstanceProfile {
        InstanceProfile { properties }
    }
}

/// The [`AWS::DMS::MigrationProject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html) resource type.
#[derive(Debug, Default)]
pub struct MigrationProject {
    properties: MigrationProjectProperties
}

/// Properties for the `MigrationProject` resource.
#[derive(Debug, Default)]
pub struct MigrationProjectProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceProfileArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-instanceprofilearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_profile_arn: Option<::Value<String>>,
    /// Property [`InstanceProfileIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-instanceprofileidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_profile_identifier: Option<::Value<String>>,
    /// Property [`InstanceProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-instanceprofilename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_profile_name: Option<::Value<String>>,
    /// Property [`MigrationProjectIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-migrationprojectidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub migration_project_identifier: Option<::Value<String>>,
    /// Property [`MigrationProjectName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-migrationprojectname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub migration_project_name: Option<::Value<String>>,
    /// Property [`SchemaConversionApplicationAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-schemaconversionapplicationattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schema_conversion_application_attributes: Option<::Value<self::migration_project::SchemaConversionApplicationAttributes>>,
    /// Property [`SourceDataProviderDescriptors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-sourcedataproviderdescriptors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_data_provider_descriptors: Option<::ValueList<self::migration_project::DataProviderDescriptor>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetDataProviderDescriptors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-targetdataproviderdescriptors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_data_provider_descriptors: Option<::ValueList<self::migration_project::DataProviderDescriptor>>,
    /// Property [`TransformationRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-migrationproject.html#cfn-dms-migrationproject-transformationrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub transformation_rules: Option<::Value<String>>,
}

impl ::serde::Serialize for MigrationProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref instance_profile_arn) = self.instance_profile_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProfileArn", instance_profile_arn)?;
        }
        if let Some(ref instance_profile_identifier) = self.instance_profile_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProfileIdentifier", instance_profile_identifier)?;
        }
        if let Some(ref instance_profile_name) = self.instance_profile_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProfileName", instance_profile_name)?;
        }
        if let Some(ref migration_project_identifier) = self.migration_project_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MigrationProjectIdentifier", migration_project_identifier)?;
        }
        if let Some(ref migration_project_name) = self.migration_project_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MigrationProjectName", migration_project_name)?;
        }
        if let Some(ref schema_conversion_application_attributes) = self.schema_conversion_application_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaConversionApplicationAttributes", schema_conversion_application_attributes)?;
        }
        if let Some(ref source_data_provider_descriptors) = self.source_data_provider_descriptors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDataProviderDescriptors", source_data_provider_descriptors)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_data_provider_descriptors) = self.target_data_provider_descriptors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetDataProviderDescriptors", target_data_provider_descriptors)?;
        }
        if let Some(ref transformation_rules) = self.transformation_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransformationRules", transformation_rules)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MigrationProjectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MigrationProjectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MigrationProjectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MigrationProjectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut instance_profile_arn: Option<::Value<String>> = None;
                let mut instance_profile_identifier: Option<::Value<String>> = None;
                let mut instance_profile_name: Option<::Value<String>> = None;
                let mut migration_project_identifier: Option<::Value<String>> = None;
                let mut migration_project_name: Option<::Value<String>> = None;
                let mut schema_conversion_application_attributes: Option<::Value<self::migration_project::SchemaConversionApplicationAttributes>> = None;
                let mut source_data_provider_descriptors: Option<::ValueList<self::migration_project::DataProviderDescriptor>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_data_provider_descriptors: Option<::ValueList<self::migration_project::DataProviderDescriptor>> = None;
                let mut transformation_rules: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceProfileArn" => {
                            instance_profile_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceProfileIdentifier" => {
                            instance_profile_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceProfileName" => {
                            instance_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MigrationProjectIdentifier" => {
                            migration_project_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MigrationProjectName" => {
                            migration_project_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaConversionApplicationAttributes" => {
                            schema_conversion_application_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceDataProviderDescriptors" => {
                            source_data_provider_descriptors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetDataProviderDescriptors" => {
                            target_data_provider_descriptors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransformationRules" => {
                            transformation_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MigrationProjectProperties {
                    description: description,
                    instance_profile_arn: instance_profile_arn,
                    instance_profile_identifier: instance_profile_identifier,
                    instance_profile_name: instance_profile_name,
                    migration_project_identifier: migration_project_identifier,
                    migration_project_name: migration_project_name,
                    schema_conversion_application_attributes: schema_conversion_application_attributes,
                    source_data_provider_descriptors: source_data_provider_descriptors,
                    tags: tags,
                    target_data_provider_descriptors: target_data_provider_descriptors,
                    transformation_rules: transformation_rules,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MigrationProject {
    type Properties = MigrationProjectProperties;
    const TYPE: &'static str = "AWS::DMS::MigrationProject";
    fn properties(&self) -> &MigrationProjectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MigrationProjectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MigrationProject {}

impl From<MigrationProjectProperties> for MigrationProject {
    fn from(properties: MigrationProjectProperties) -> MigrationProject {
        MigrationProject { properties }
    }
}

/// The [`AWS::DMS::ReplicationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html) resource type.
#[derive(Debug, Default)]
pub struct ReplicationConfig {
    properties: ReplicationConfigProperties
}

/// Properties for the `ReplicationConfig` resource.
#[derive(Debug, Default)]
pub struct ReplicationConfigProperties {
    /// Property [`ComputeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-computeconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub compute_config: Option<::Value<self::replication_config::ComputeConfig>>,
    /// Property [`ReplicationConfigArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-replicationconfigarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_config_arn: Option<::Value<String>>,
    /// Property [`ReplicationConfigIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-replicationconfigidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_config_identifier: Option<::Value<String>>,
    /// Property [`ReplicationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-replicationsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_settings: Option<::Value<::json::Value>>,
    /// Property [`ReplicationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-replicationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_type: Option<::Value<String>>,
    /// Property [`ResourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-resourceidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_identifier: Option<::Value<String>>,
    /// Property [`SourceEndpointArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-sourceendpointarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_endpoint_arn: Option<::Value<String>>,
    /// Property [`SupplementalSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-supplementalsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub supplemental_settings: Option<::Value<::json::Value>>,
    /// Property [`TableMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-tablemappings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub table_mappings: Option<::Value<::json::Value>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetEndpointArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationconfig.html#cfn-dms-replicationconfig-targetendpointarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_endpoint_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for ReplicationConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref compute_config) = self.compute_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeConfig", compute_config)?;
        }
        if let Some(ref replication_config_arn) = self.replication_config_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationConfigArn", replication_config_arn)?;
        }
        if let Some(ref replication_config_identifier) = self.replication_config_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationConfigIdentifier", replication_config_identifier)?;
        }
        if let Some(ref replication_settings) = self.replication_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSettings", replication_settings)?;
        }
        if let Some(ref replication_type) = self.replication_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationType", replication_type)?;
        }
        if let Some(ref resource_identifier) = self.resource_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceIdentifier", resource_identifier)?;
        }
        if let Some(ref source_endpoint_arn) = self.source_endpoint_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceEndpointArn", source_endpoint_arn)?;
        }
        if let Some(ref supplemental_settings) = self.supplemental_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupplementalSettings", supplemental_settings)?;
        }
        if let Some(ref table_mappings) = self.table_mappings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableMappings", table_mappings)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_endpoint_arn) = self.target_endpoint_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetEndpointArn", target_endpoint_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicationConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicationConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut compute_config: Option<::Value<self::replication_config::ComputeConfig>> = None;
                let mut replication_config_arn: Option<::Value<String>> = None;
                let mut replication_config_identifier: Option<::Value<String>> = None;
                let mut replication_settings: Option<::Value<::json::Value>> = None;
                let mut replication_type: Option<::Value<String>> = None;
                let mut resource_identifier: Option<::Value<String>> = None;
                let mut source_endpoint_arn: Option<::Value<String>> = None;
                let mut supplemental_settings: Option<::Value<::json::Value>> = None;
                let mut table_mappings: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_endpoint_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ComputeConfig" => {
                            compute_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationConfigArn" => {
                            replication_config_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationConfigIdentifier" => {
                            replication_config_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationSettings" => {
                            replication_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationType" => {
                            replication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceIdentifier" => {
                            resource_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceEndpointArn" => {
                            source_endpoint_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SupplementalSettings" => {
                            supplemental_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableMappings" => {
                            table_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetEndpointArn" => {
                            target_endpoint_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReplicationConfigProperties {
                    compute_config: compute_config,
                    replication_config_arn: replication_config_arn,
                    replication_config_identifier: replication_config_identifier,
                    replication_settings: replication_settings,
                    replication_type: replication_type,
                    resource_identifier: resource_identifier,
                    source_endpoint_arn: source_endpoint_arn,
                    supplemental_settings: supplemental_settings,
                    table_mappings: table_mappings,
                    tags: tags,
                    target_endpoint_arn: target_endpoint_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReplicationConfig {
    type Properties = ReplicationConfigProperties;
    const TYPE: &'static str = "AWS::DMS::ReplicationConfig";
    fn properties(&self) -> &ReplicationConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationConfig {}

impl From<ReplicationConfigProperties> for ReplicationConfig {
    fn from(properties: ReplicationConfigProperties) -> ReplicationConfig {
        ReplicationConfig { properties }
    }
}

/// The [`AWS::DMS::ReplicationInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html) resource type.
#[derive(Debug, Default)]
pub struct ReplicationInstance {
    properties: ReplicationInstanceProperties
}

/// Properties for the `ReplicationInstance` resource.
#[derive(Debug, Default)]
pub struct ReplicationInstanceProperties {
    /// Property [`AllocatedStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-allocatedstorage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allocated_storage: Option<::Value<u32>>,
    /// Property [`AllowMajorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-allowmajorversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_major_version_upgrade: Option<::Value<bool>>,
    /// Property [`AutoMinorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-autominorversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-availabilityzone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub availability_zone: Option<::Value<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-engineversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_version: Option<::Value<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`MultiAZ`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-multiaz).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub multi_az: Option<::Value<bool>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-publiclyaccessible).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property [`ReplicationInstanceClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-replicationinstanceclass).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_instance_class: ::Value<String>,
    /// Property [`ReplicationInstanceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-replicationinstanceidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_instance_identifier: Option<::Value<String>>,
    /// Property [`ReplicationSubnetGroupIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-replicationsubnetgroupidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub replication_subnet_group_identifier: Option<::Value<String>>,
    /// Property [`ResourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-resourceidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_identifier: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationinstance.html#cfn-dms-replicationinstance-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for ReplicationInstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allocated_storage) = self.allocated_storage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocatedStorage", allocated_storage)?;
        }
        if let Some(ref allow_major_version_upgrade) = self.allow_major_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowMajorVersionUpgrade", allow_major_version_upgrade)?;
        }
        if let Some(ref auto_minor_version_upgrade) = self.auto_minor_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", auto_minor_version_upgrade)?;
        }
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref multi_az) = self.multi_az {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiAZ", multi_az)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref publicly_accessible) = self.publicly_accessible {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationInstanceClass", &self.replication_instance_class)?;
        if let Some(ref replication_instance_identifier) = self.replication_instance_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationInstanceIdentifier", replication_instance_identifier)?;
        }
        if let Some(ref replication_subnet_group_identifier) = self.replication_subnet_group_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSubnetGroupIdentifier", replication_subnet_group_identifier)?;
        }
        if let Some(ref resource_identifier) = self.resource_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceIdentifier", resource_identifier)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicationInstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationInstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationInstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicationInstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allocated_storage: Option<::Value<u32>> = None;
                let mut allow_major_version_upgrade: Option<::Value<bool>> = None;
                let mut auto_minor_version_upgrade: Option<::Value<bool>> = None;
                let mut availability_zone: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut multi_az: Option<::Value<bool>> = None;
                let mut preferred_maintenance_window: Option<::Value<String>> = None;
                let mut publicly_accessible: Option<::Value<bool>> = None;
                let mut replication_instance_class: Option<::Value<String>> = None;
                let mut replication_instance_identifier: Option<::Value<String>> = None;
                let mut replication_subnet_group_identifier: Option<::Value<String>> = None;
                let mut resource_identifier: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_security_group_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllocatedStorage" => {
                            allocated_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AllowMajorVersionUpgrade" => {
                            allow_major_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MultiAZ" => {
                            multi_az = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationInstanceClass" => {
                            replication_instance_class = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationInstanceIdentifier" => {
                            replication_instance_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationSubnetGroupIdentifier" => {
                            replication_subnet_group_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceIdentifier" => {
                            resource_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReplicationInstanceProperties {
                    allocated_storage: allocated_storage,
                    allow_major_version_upgrade: allow_major_version_upgrade,
                    auto_minor_version_upgrade: auto_minor_version_upgrade,
                    availability_zone: availability_zone,
                    engine_version: engine_version,
                    kms_key_id: kms_key_id,
                    multi_az: multi_az,
                    preferred_maintenance_window: preferred_maintenance_window,
                    publicly_accessible: publicly_accessible,
                    replication_instance_class: replication_instance_class.ok_or(::serde::de::Error::missing_field("ReplicationInstanceClass"))?,
                    replication_instance_identifier: replication_instance_identifier,
                    replication_subnet_group_identifier: replication_subnet_group_identifier,
                    resource_identifier: resource_identifier,
                    tags: tags,
                    vpc_security_group_ids: vpc_security_group_ids,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReplicationInstance {
    type Properties = ReplicationInstanceProperties;
    const TYPE: &'static str = "AWS::DMS::ReplicationInstance";
    fn properties(&self) -> &ReplicationInstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationInstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationInstance {}

impl From<ReplicationInstanceProperties> for ReplicationInstance {
    fn from(properties: ReplicationInstanceProperties) -> ReplicationInstance {
        ReplicationInstance { properties }
    }
}

/// The [`AWS::DMS::ReplicationSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationsubnetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct ReplicationSubnetGroup {
    properties: ReplicationSubnetGroupProperties
}

/// Properties for the `ReplicationSubnetGroup` resource.
#[derive(Debug, Default)]
pub struct ReplicationSubnetGroupProperties {
    /// Property [`ReplicationSubnetGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationsubnetgroup.html#cfn-dms-replicationsubnetgroup-replicationsubnetgroupdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_subnet_group_description: ::Value<String>,
    /// Property [`ReplicationSubnetGroupIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationsubnetgroup.html#cfn-dms-replicationsubnetgroup-replicationsubnetgroupidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub replication_subnet_group_identifier: Option<::Value<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationsubnetgroup.html#cfn-dms-replicationsubnetgroup-subnetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_ids: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationsubnetgroup.html#cfn-dms-replicationsubnetgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ReplicationSubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSubnetGroupDescription", &self.replication_subnet_group_description)?;
        if let Some(ref replication_subnet_group_identifier) = self.replication_subnet_group_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSubnetGroupIdentifier", replication_subnet_group_identifier)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicationSubnetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationSubnetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationSubnetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicationSubnetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut replication_subnet_group_description: Option<::Value<String>> = None;
                let mut replication_subnet_group_identifier: Option<::Value<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ReplicationSubnetGroupDescription" => {
                            replication_subnet_group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationSubnetGroupIdentifier" => {
                            replication_subnet_group_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReplicationSubnetGroupProperties {
                    replication_subnet_group_description: replication_subnet_group_description.ok_or(::serde::de::Error::missing_field("ReplicationSubnetGroupDescription"))?,
                    replication_subnet_group_identifier: replication_subnet_group_identifier,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReplicationSubnetGroup {
    type Properties = ReplicationSubnetGroupProperties;
    const TYPE: &'static str = "AWS::DMS::ReplicationSubnetGroup";
    fn properties(&self) -> &ReplicationSubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationSubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationSubnetGroup {}

impl From<ReplicationSubnetGroupProperties> for ReplicationSubnetGroup {
    fn from(properties: ReplicationSubnetGroupProperties) -> ReplicationSubnetGroup {
        ReplicationSubnetGroup { properties }
    }
}

/// The [`AWS::DMS::ReplicationTask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html) resource type.
#[derive(Debug, Default)]
pub struct ReplicationTask {
    properties: ReplicationTaskProperties
}

/// Properties for the `ReplicationTask` resource.
#[derive(Debug, Default)]
pub struct ReplicationTaskProperties {
    /// Property [`CdcStartPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-cdcstartposition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cdc_start_position: Option<::Value<String>>,
    /// Property [`CdcStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-cdcstarttime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cdc_start_time: Option<::Value<f64>>,
    /// Property [`CdcStopPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-cdcstopposition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cdc_stop_position: Option<::Value<String>>,
    /// Property [`MigrationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-migrationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub migration_type: ::Value<String>,
    /// Property [`ReplicationInstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-replicationinstancearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub replication_instance_arn: ::Value<String>,
    /// Property [`ReplicationTaskIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-replicationtaskidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_task_identifier: Option<::Value<String>>,
    /// Property [`ReplicationTaskSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-replicationtasksettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_task_settings: Option<::Value<String>>,
    /// Property [`ResourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-resourceidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_identifier: Option<::Value<String>>,
    /// Property [`SourceEndpointArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-sourceendpointarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_endpoint_arn: ::Value<String>,
    /// Property [`TableMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-tablemappings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub table_mappings: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetEndpointArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-targetendpointarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_endpoint_arn: ::Value<String>,
    /// Property [`TaskData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dms-replicationtask.html#cfn-dms-replicationtask-taskdata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub task_data: Option<::Value<String>>,
}

impl ::serde::Serialize for ReplicationTaskProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cdc_start_position) = self.cdc_start_position {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdcStartPosition", cdc_start_position)?;
        }
        if let Some(ref cdc_start_time) = self.cdc_start_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdcStartTime", cdc_start_time)?;
        }
        if let Some(ref cdc_stop_position) = self.cdc_stop_position {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdcStopPosition", cdc_stop_position)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MigrationType", &self.migration_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationInstanceArn", &self.replication_instance_arn)?;
        if let Some(ref replication_task_identifier) = self.replication_task_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationTaskIdentifier", replication_task_identifier)?;
        }
        if let Some(ref replication_task_settings) = self.replication_task_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationTaskSettings", replication_task_settings)?;
        }
        if let Some(ref resource_identifier) = self.resource_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceIdentifier", resource_identifier)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceEndpointArn", &self.source_endpoint_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableMappings", &self.table_mappings)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetEndpointArn", &self.target_endpoint_arn)?;
        if let Some(ref task_data) = self.task_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskData", task_data)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReplicationTaskProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationTaskProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReplicationTaskProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReplicationTaskProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cdc_start_position: Option<::Value<String>> = None;
                let mut cdc_start_time: Option<::Value<f64>> = None;
                let mut cdc_stop_position: Option<::Value<String>> = None;
                let mut migration_type: Option<::Value<String>> = None;
                let mut replication_instance_arn: Option<::Value<String>> = None;
                let mut replication_task_identifier: Option<::Value<String>> = None;
                let mut replication_task_settings: Option<::Value<String>> = None;
                let mut resource_identifier: Option<::Value<String>> = None;
                let mut source_endpoint_arn: Option<::Value<String>> = None;
                let mut table_mappings: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_endpoint_arn: Option<::Value<String>> = None;
                let mut task_data: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CdcStartPosition" => {
                            cdc_start_position = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CdcStartTime" => {
                            cdc_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CdcStopPosition" => {
                            cdc_stop_position = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MigrationType" => {
                            migration_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationInstanceArn" => {
                            replication_instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationTaskIdentifier" => {
                            replication_task_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationTaskSettings" => {
                            replication_task_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceIdentifier" => {
                            resource_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceEndpointArn" => {
                            source_endpoint_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableMappings" => {
                            table_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetEndpointArn" => {
                            target_endpoint_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskData" => {
                            task_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReplicationTaskProperties {
                    cdc_start_position: cdc_start_position,
                    cdc_start_time: cdc_start_time,
                    cdc_stop_position: cdc_stop_position,
                    migration_type: migration_type.ok_or(::serde::de::Error::missing_field("MigrationType"))?,
                    replication_instance_arn: replication_instance_arn.ok_or(::serde::de::Error::missing_field("ReplicationInstanceArn"))?,
                    replication_task_identifier: replication_task_identifier,
                    replication_task_settings: replication_task_settings,
                    resource_identifier: resource_identifier,
                    source_endpoint_arn: source_endpoint_arn.ok_or(::serde::de::Error::missing_field("SourceEndpointArn"))?,
                    table_mappings: table_mappings.ok_or(::serde::de::Error::missing_field("TableMappings"))?,
                    tags: tags,
                    target_endpoint_arn: target_endpoint_arn.ok_or(::serde::de::Error::missing_field("TargetEndpointArn"))?,
                    task_data: task_data,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReplicationTask {
    type Properties = ReplicationTaskProperties;
    const TYPE: &'static str = "AWS::DMS::ReplicationTask";
    fn properties(&self) -> &ReplicationTaskProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReplicationTaskProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReplicationTask {}

impl From<ReplicationTaskProperties> for ReplicationTask {
    fn from(properties: ReplicationTaskProperties) -> ReplicationTask {
        ReplicationTask { properties }
    }
}

pub mod data_provider {
    //! Property types for the `DataProvider` resource.

    /// The [`AWS::DMS::DataProvider.MicrosoftSqlServerSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-microsoftsqlserversettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MicrosoftSqlServerSettings {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-microsoftsqlserversettings.html#cfn-dms-dataprovider-microsoftsqlserversettings-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-microsoftsqlserversettings.html#cfn-dms-dataprovider-microsoftsqlserversettings-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-microsoftsqlserversettings.html#cfn-dms-dataprovider-microsoftsqlserversettings-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`ServerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-microsoftsqlserversettings.html#cfn-dms-dataprovider-microsoftsqlserversettings-servername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_name: Option<::Value<String>>,
        /// Property [`SslMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-microsoftsqlserversettings.html#cfn-dms-dataprovider-microsoftsqlserversettings-sslmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MicrosoftSqlServerSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_arn) = self.certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref server_name) = self.server_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", server_name)?;
            }
            if let Some(ref ssl_mode) = self.ssl_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslMode", ssl_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MicrosoftSqlServerSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MicrosoftSqlServerSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MicrosoftSqlServerSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MicrosoftSqlServerSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut server_name: Option<::Value<String>> = None;
                    let mut ssl_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerName" => {
                                server_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslMode" => {
                                ssl_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MicrosoftSqlServerSettings {
                        certificate_arn: certificate_arn,
                        database_name: database_name,
                        port: port,
                        server_name: server_name,
                        ssl_mode: ssl_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::DataProvider.MySqlSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-mysqlsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MySqlSettings {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-mysqlsettings.html#cfn-dms-dataprovider-mysqlsettings-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-mysqlsettings.html#cfn-dms-dataprovider-mysqlsettings-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`ServerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-mysqlsettings.html#cfn-dms-dataprovider-mysqlsettings-servername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_name: Option<::Value<String>>,
        /// Property [`SslMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-mysqlsettings.html#cfn-dms-dataprovider-mysqlsettings-sslmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MySqlSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_arn) = self.certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref server_name) = self.server_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", server_name)?;
            }
            if let Some(ref ssl_mode) = self.ssl_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslMode", ssl_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MySqlSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MySqlSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MySqlSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MySqlSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut server_name: Option<::Value<String>> = None;
                    let mut ssl_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerName" => {
                                server_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslMode" => {
                                ssl_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MySqlSettings {
                        certificate_arn: certificate_arn,
                        port: port,
                        server_name: server_name,
                        ssl_mode: ssl_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::DataProvider.OracleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct OracleSettings {
        /// Property [`AsmServer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html#cfn-dms-dataprovider-oraclesettings-asmserver).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub asm_server: Option<::Value<String>>,
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html#cfn-dms-dataprovider-oraclesettings-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html#cfn-dms-dataprovider-oraclesettings-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html#cfn-dms-dataprovider-oraclesettings-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`SecretsManagerOracleAsmAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html#cfn-dms-dataprovider-oraclesettings-secretsmanageroracleasmaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_oracle_asm_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerOracleAsmSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html#cfn-dms-dataprovider-oraclesettings-secretsmanageroracleasmsecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_oracle_asm_secret_id: Option<::Value<String>>,
        /// Property [`SecretsManagerSecurityDbEncryptionAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html#cfn-dms-dataprovider-oraclesettings-secretsmanagersecuritydbencryptionaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_security_db_encryption_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecurityDbEncryptionSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html#cfn-dms-dataprovider-oraclesettings-secretsmanagersecuritydbencryptionsecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_security_db_encryption_secret_id: Option<::Value<String>>,
        /// Property [`ServerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html#cfn-dms-dataprovider-oraclesettings-servername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_name: Option<::Value<String>>,
        /// Property [`SslMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-oraclesettings.html#cfn-dms-dataprovider-oraclesettings-sslmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OracleSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref asm_server) = self.asm_server {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AsmServer", asm_server)?;
            }
            if let Some(ref certificate_arn) = self.certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref secrets_manager_oracle_asm_access_role_arn) = self.secrets_manager_oracle_asm_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerOracleAsmAccessRoleArn", secrets_manager_oracle_asm_access_role_arn)?;
            }
            if let Some(ref secrets_manager_oracle_asm_secret_id) = self.secrets_manager_oracle_asm_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerOracleAsmSecretId", secrets_manager_oracle_asm_secret_id)?;
            }
            if let Some(ref secrets_manager_security_db_encryption_access_role_arn) = self.secrets_manager_security_db_encryption_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecurityDbEncryptionAccessRoleArn", secrets_manager_security_db_encryption_access_role_arn)?;
            }
            if let Some(ref secrets_manager_security_db_encryption_secret_id) = self.secrets_manager_security_db_encryption_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecurityDbEncryptionSecretId", secrets_manager_security_db_encryption_secret_id)?;
            }
            if let Some(ref server_name) = self.server_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", server_name)?;
            }
            if let Some(ref ssl_mode) = self.ssl_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslMode", ssl_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OracleSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OracleSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OracleSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OracleSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut asm_server: Option<::Value<String>> = None;
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut secrets_manager_oracle_asm_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_oracle_asm_secret_id: Option<::Value<String>> = None;
                    let mut secrets_manager_security_db_encryption_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_security_db_encryption_secret_id: Option<::Value<String>> = None;
                    let mut server_name: Option<::Value<String>> = None;
                    let mut ssl_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AsmServer" => {
                                asm_server = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerOracleAsmAccessRoleArn" => {
                                secrets_manager_oracle_asm_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerOracleAsmSecretId" => {
                                secrets_manager_oracle_asm_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecurityDbEncryptionAccessRoleArn" => {
                                secrets_manager_security_db_encryption_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecurityDbEncryptionSecretId" => {
                                secrets_manager_security_db_encryption_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerName" => {
                                server_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslMode" => {
                                ssl_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OracleSettings {
                        asm_server: asm_server,
                        certificate_arn: certificate_arn,
                        database_name: database_name,
                        port: port,
                        secrets_manager_oracle_asm_access_role_arn: secrets_manager_oracle_asm_access_role_arn,
                        secrets_manager_oracle_asm_secret_id: secrets_manager_oracle_asm_secret_id,
                        secrets_manager_security_db_encryption_access_role_arn: secrets_manager_security_db_encryption_access_role_arn,
                        secrets_manager_security_db_encryption_secret_id: secrets_manager_security_db_encryption_secret_id,
                        server_name: server_name,
                        ssl_mode: ssl_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::DataProvider.PostgreSqlSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-postgresqlsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct PostgreSqlSettings {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-postgresqlsettings.html#cfn-dms-dataprovider-postgresqlsettings-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-postgresqlsettings.html#cfn-dms-dataprovider-postgresqlsettings-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-postgresqlsettings.html#cfn-dms-dataprovider-postgresqlsettings-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`ServerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-postgresqlsettings.html#cfn-dms-dataprovider-postgresqlsettings-servername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_name: Option<::Value<String>>,
        /// Property [`SslMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-postgresqlsettings.html#cfn-dms-dataprovider-postgresqlsettings-sslmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PostgreSqlSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_arn) = self.certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref server_name) = self.server_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", server_name)?;
            }
            if let Some(ref ssl_mode) = self.ssl_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslMode", ssl_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PostgreSqlSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PostgreSqlSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PostgreSqlSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PostgreSqlSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut server_name: Option<::Value<String>> = None;
                    let mut ssl_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerName" => {
                                server_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslMode" => {
                                ssl_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PostgreSqlSettings {
                        certificate_arn: certificate_arn,
                        database_name: database_name,
                        port: port,
                        server_name: server_name,
                        ssl_mode: ssl_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::DataProvider.Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-settings.html) property type.
    #[derive(Debug, Default)]
    pub struct Settings {
        /// Property [`MicrosoftSqlServerSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-settings.html#cfn-dms-dataprovider-settings-microsoftsqlserversettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub microsoft_sql_server_settings: Option<::Value<MicrosoftSqlServerSettings>>,
        /// Property [`MySqlSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-settings.html#cfn-dms-dataprovider-settings-mysqlsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub my_sql_settings: Option<::Value<MySqlSettings>>,
        /// Property [`OracleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-settings.html#cfn-dms-dataprovider-settings-oraclesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oracle_settings: Option<::Value<OracleSettings>>,
        /// Property [`PostgreSqlSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-dataprovider-settings.html#cfn-dms-dataprovider-settings-postgresqlsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub postgre_sql_settings: Option<::Value<PostgreSqlSettings>>,
    }

    impl ::codec::SerializeValue for Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref microsoft_sql_server_settings) = self.microsoft_sql_server_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MicrosoftSqlServerSettings", microsoft_sql_server_settings)?;
            }
            if let Some(ref my_sql_settings) = self.my_sql_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MySqlSettings", my_sql_settings)?;
            }
            if let Some(ref oracle_settings) = self.oracle_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OracleSettings", oracle_settings)?;
            }
            if let Some(ref postgre_sql_settings) = self.postgre_sql_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostgreSqlSettings", postgre_sql_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut microsoft_sql_server_settings: Option<::Value<MicrosoftSqlServerSettings>> = None;
                    let mut my_sql_settings: Option<::Value<MySqlSettings>> = None;
                    let mut oracle_settings: Option<::Value<OracleSettings>> = None;
                    let mut postgre_sql_settings: Option<::Value<PostgreSqlSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MicrosoftSqlServerSettings" => {
                                microsoft_sql_server_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MySqlSettings" => {
                                my_sql_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OracleSettings" => {
                                oracle_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostgreSqlSettings" => {
                                postgre_sql_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Settings {
                        microsoft_sql_server_settings: microsoft_sql_server_settings,
                        my_sql_settings: my_sql_settings,
                        oracle_settings: oracle_settings,
                        postgre_sql_settings: postgre_sql_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod endpoint {
    //! Property types for the `Endpoint` resource.

    /// The [`AWS::DMS::Endpoint.DocDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-docdbsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DocDbSettings {
        /// Property [`DocsToInvestigate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-docdbsettings.html#cfn-dms-endpoint-docdbsettings-docstoinvestigate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub docs_to_investigate: Option<::Value<u32>>,
        /// Property [`ExtractDocId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-docdbsettings.html#cfn-dms-endpoint-docdbsettings-extractdocid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub extract_doc_id: Option<::Value<bool>>,
        /// Property [`NestingLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-docdbsettings.html#cfn-dms-endpoint-docdbsettings-nestinglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nesting_level: Option<::Value<String>>,
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-docdbsettings.html#cfn-dms-endpoint-docdbsettings-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-docdbsettings.html#cfn-dms-endpoint-docdbsettings-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DocDbSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref docs_to_investigate) = self.docs_to_investigate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocsToInvestigate", docs_to_investigate)?;
            }
            if let Some(ref extract_doc_id) = self.extract_doc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtractDocId", extract_doc_id)?;
            }
            if let Some(ref nesting_level) = self.nesting_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NestingLevel", nesting_level)?;
            }
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocDbSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocDbSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocDbSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocDbSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut docs_to_investigate: Option<::Value<u32>> = None;
                    let mut extract_doc_id: Option<::Value<bool>> = None;
                    let mut nesting_level: Option<::Value<String>> = None;
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocsToInvestigate" => {
                                docs_to_investigate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExtractDocId" => {
                                extract_doc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NestingLevel" => {
                                nesting_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocDbSettings {
                        docs_to_investigate: docs_to_investigate,
                        extract_doc_id: extract_doc_id,
                        nesting_level: nesting_level,
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.DynamoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-dynamodbsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DynamoDbSettings {
        /// Property [`ServiceAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-dynamodbsettings.html#cfn-dms-endpoint-dynamodbsettings-serviceaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_access_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DynamoDbSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref service_access_role_arn) = self.service_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessRoleArn", service_access_role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynamoDbSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamoDbSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamoDbSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamoDbSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut service_access_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ServiceAccessRoleArn" => {
                                service_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamoDbSettings {
                        service_access_role_arn: service_access_role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.ElasticsearchSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-elasticsearchsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ElasticsearchSettings {
        /// Property [`EndpointUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-elasticsearchsettings.html#cfn-dms-endpoint-elasticsearchsettings-endpointuri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_uri: Option<::Value<String>>,
        /// Property [`ErrorRetryDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-elasticsearchsettings.html#cfn-dms-endpoint-elasticsearchsettings-errorretryduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_retry_duration: Option<::Value<u32>>,
        /// Property [`FullLoadErrorPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-elasticsearchsettings.html#cfn-dms-endpoint-elasticsearchsettings-fullloaderrorpercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub full_load_error_percentage: Option<::Value<u32>>,
        /// Property [`ServiceAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-elasticsearchsettings.html#cfn-dms-endpoint-elasticsearchsettings-serviceaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_access_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ElasticsearchSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref endpoint_uri) = self.endpoint_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointUri", endpoint_uri)?;
            }
            if let Some(ref error_retry_duration) = self.error_retry_duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorRetryDuration", error_retry_duration)?;
            }
            if let Some(ref full_load_error_percentage) = self.full_load_error_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FullLoadErrorPercentage", full_load_error_percentage)?;
            }
            if let Some(ref service_access_role_arn) = self.service_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessRoleArn", service_access_role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticsearchSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticsearchSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticsearchSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticsearchSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_uri: Option<::Value<String>> = None;
                    let mut error_retry_duration: Option<::Value<u32>> = None;
                    let mut full_load_error_percentage: Option<::Value<u32>> = None;
                    let mut service_access_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointUri" => {
                                endpoint_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ErrorRetryDuration" => {
                                error_retry_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FullLoadErrorPercentage" => {
                                full_load_error_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceAccessRoleArn" => {
                                service_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticsearchSettings {
                        endpoint_uri: endpoint_uri,
                        error_retry_duration: error_retry_duration,
                        full_load_error_percentage: full_load_error_percentage,
                        service_access_role_arn: service_access_role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.GcpMySQLSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct GcpMySQLSettings {
        /// Property [`AfterConnectScript`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-afterconnectscript).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub after_connect_script: Option<::Value<String>>,
        /// Property [`CleanSourceMetadataOnMismatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-cleansourcemetadataonmismatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub clean_source_metadata_on_mismatch: Option<::Value<bool>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`EventsPollInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-eventspollinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub events_poll_interval: Option<::Value<u32>>,
        /// Property [`MaxFileSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-maxfilesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_file_size: Option<::Value<u32>>,
        /// Property [`ParallelLoadThreads`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-parallelloadthreads).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parallel_load_threads: Option<::Value<u32>>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
        /// Property [`ServerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-servername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_name: Option<::Value<String>>,
        /// Property [`ServerTimezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-servertimezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_timezone: Option<::Value<String>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-gcpmysqlsettings.html#cfn-dms-endpoint-gcpmysqlsettings-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GcpMySQLSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref after_connect_script) = self.after_connect_script {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AfterConnectScript", after_connect_script)?;
            }
            if let Some(ref clean_source_metadata_on_mismatch) = self.clean_source_metadata_on_mismatch {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CleanSourceMetadataOnMismatch", clean_source_metadata_on_mismatch)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref events_poll_interval) = self.events_poll_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventsPollInterval", events_poll_interval)?;
            }
            if let Some(ref max_file_size) = self.max_file_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxFileSize", max_file_size)?;
            }
            if let Some(ref parallel_load_threads) = self.parallel_load_threads {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParallelLoadThreads", parallel_load_threads)?;
            }
            if let Some(ref password) = self.password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            if let Some(ref server_name) = self.server_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", server_name)?;
            }
            if let Some(ref server_timezone) = self.server_timezone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerTimezone", server_timezone)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GcpMySQLSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GcpMySQLSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GcpMySQLSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GcpMySQLSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut after_connect_script: Option<::Value<String>> = None;
                    let mut clean_source_metadata_on_mismatch: Option<::Value<bool>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut events_poll_interval: Option<::Value<u32>> = None;
                    let mut max_file_size: Option<::Value<u32>> = None;
                    let mut parallel_load_threads: Option<::Value<u32>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;
                    let mut server_name: Option<::Value<String>> = None;
                    let mut server_timezone: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AfterConnectScript" => {
                                after_connect_script = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CleanSourceMetadataOnMismatch" => {
                                clean_source_metadata_on_mismatch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventsPollInterval" => {
                                events_poll_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxFileSize" => {
                                max_file_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParallelLoadThreads" => {
                                parallel_load_threads = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerName" => {
                                server_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerTimezone" => {
                                server_timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GcpMySQLSettings {
                        after_connect_script: after_connect_script,
                        clean_source_metadata_on_mismatch: clean_source_metadata_on_mismatch,
                        database_name: database_name,
                        events_poll_interval: events_poll_interval,
                        max_file_size: max_file_size,
                        parallel_load_threads: parallel_load_threads,
                        password: password,
                        port: port,
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                        server_name: server_name,
                        server_timezone: server_timezone,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.IbmDb2Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html) property type.
    #[derive(Debug, Default)]
    pub struct IbmDb2Settings {
        /// Property [`CurrentLsn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html#cfn-dms-endpoint-ibmdb2settings-currentlsn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub current_lsn: Option<::Value<String>>,
        /// Property [`KeepCsvFiles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html#cfn-dms-endpoint-ibmdb2settings-keepcsvfiles).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub keep_csv_files: Option<::Value<bool>>,
        /// Property [`LoadTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html#cfn-dms-endpoint-ibmdb2settings-loadtimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub load_timeout: Option<::Value<u32>>,
        /// Property [`MaxFileSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html#cfn-dms-endpoint-ibmdb2settings-maxfilesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_file_size: Option<::Value<u32>>,
        /// Property [`MaxKBytesPerRead`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html#cfn-dms-endpoint-ibmdb2settings-maxkbytesperread).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_k_bytes_per_read: Option<::Value<u32>>,
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html#cfn-dms-endpoint-ibmdb2settings-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html#cfn-dms-endpoint-ibmdb2settings-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
        /// Property [`SetDataCaptureChanges`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html#cfn-dms-endpoint-ibmdb2settings-setdatacapturechanges).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub set_data_capture_changes: Option<::Value<bool>>,
        /// Property [`WriteBufferSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html#cfn-dms-endpoint-ibmdb2settings-writebuffersize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_buffer_size: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for IbmDb2Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref current_lsn) = self.current_lsn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CurrentLsn", current_lsn)?;
            }
            if let Some(ref keep_csv_files) = self.keep_csv_files {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeepCsvFiles", keep_csv_files)?;
            }
            if let Some(ref load_timeout) = self.load_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadTimeout", load_timeout)?;
            }
            if let Some(ref max_file_size) = self.max_file_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxFileSize", max_file_size)?;
            }
            if let Some(ref max_k_bytes_per_read) = self.max_k_bytes_per_read {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxKBytesPerRead", max_k_bytes_per_read)?;
            }
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            if let Some(ref set_data_capture_changes) = self.set_data_capture_changes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SetDataCaptureChanges", set_data_capture_changes)?;
            }
            if let Some(ref write_buffer_size) = self.write_buffer_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteBufferSize", write_buffer_size)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IbmDb2Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IbmDb2Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IbmDb2Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IbmDb2Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut current_lsn: Option<::Value<String>> = None;
                    let mut keep_csv_files: Option<::Value<bool>> = None;
                    let mut load_timeout: Option<::Value<u32>> = None;
                    let mut max_file_size: Option<::Value<u32>> = None;
                    let mut max_k_bytes_per_read: Option<::Value<u32>> = None;
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;
                    let mut set_data_capture_changes: Option<::Value<bool>> = None;
                    let mut write_buffer_size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CurrentLsn" => {
                                current_lsn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeepCsvFiles" => {
                                keep_csv_files = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoadTimeout" => {
                                load_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxFileSize" => {
                                max_file_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxKBytesPerRead" => {
                                max_k_bytes_per_read = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SetDataCaptureChanges" => {
                                set_data_capture_changes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WriteBufferSize" => {
                                write_buffer_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IbmDb2Settings {
                        current_lsn: current_lsn,
                        keep_csv_files: keep_csv_files,
                        load_timeout: load_timeout,
                        max_file_size: max_file_size,
                        max_k_bytes_per_read: max_k_bytes_per_read,
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                        set_data_capture_changes: set_data_capture_changes,
                        write_buffer_size: write_buffer_size,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.KafkaSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html) property type.
    #[derive(Debug, Default)]
    pub struct KafkaSettings {
        /// Property [`Broker`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-broker).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub broker: Option<::Value<String>>,
        /// Property [`IncludeControlDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-includecontroldetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_control_details: Option<::Value<bool>>,
        /// Property [`IncludeNullAndEmpty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-includenullandempty).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_null_and_empty: Option<::Value<bool>>,
        /// Property [`IncludePartitionValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-includepartitionvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_partition_value: Option<::Value<bool>>,
        /// Property [`IncludeTableAlterOperations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-includetablealteroperations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_table_alter_operations: Option<::Value<bool>>,
        /// Property [`IncludeTransactionDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-includetransactiondetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_transaction_details: Option<::Value<bool>>,
        /// Property [`MessageFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-messageformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_format: Option<::Value<String>>,
        /// Property [`MessageMaxBytes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-messagemaxbytes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_max_bytes: Option<::Value<u32>>,
        /// Property [`NoHexPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-nohexprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_hex_prefix: Option<::Value<bool>>,
        /// Property [`PartitionIncludeSchemaTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-partitionincludeschematable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition_include_schema_table: Option<::Value<bool>>,
        /// Property [`SaslPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-saslpassword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sasl_password: Option<::Value<String>>,
        /// Property [`SaslUserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-saslusername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sasl_user_name: Option<::Value<String>>,
        /// Property [`SecurityProtocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-securityprotocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_protocol: Option<::Value<String>>,
        /// Property [`SslCaCertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-sslcacertificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_ca_certificate_arn: Option<::Value<String>>,
        /// Property [`SslClientCertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-sslclientcertificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_client_certificate_arn: Option<::Value<String>>,
        /// Property [`SslClientKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-sslclientkeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_client_key_arn: Option<::Value<String>>,
        /// Property [`SslClientKeyPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-sslclientkeypassword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_client_key_password: Option<::Value<String>>,
        /// Property [`Topic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kafkasettings.html#cfn-dms-endpoint-kafkasettings-topic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KafkaSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref broker) = self.broker {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Broker", broker)?;
            }
            if let Some(ref include_control_details) = self.include_control_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeControlDetails", include_control_details)?;
            }
            if let Some(ref include_null_and_empty) = self.include_null_and_empty {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeNullAndEmpty", include_null_and_empty)?;
            }
            if let Some(ref include_partition_value) = self.include_partition_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludePartitionValue", include_partition_value)?;
            }
            if let Some(ref include_table_alter_operations) = self.include_table_alter_operations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeTableAlterOperations", include_table_alter_operations)?;
            }
            if let Some(ref include_transaction_details) = self.include_transaction_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeTransactionDetails", include_transaction_details)?;
            }
            if let Some(ref message_format) = self.message_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageFormat", message_format)?;
            }
            if let Some(ref message_max_bytes) = self.message_max_bytes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageMaxBytes", message_max_bytes)?;
            }
            if let Some(ref no_hex_prefix) = self.no_hex_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoHexPrefix", no_hex_prefix)?;
            }
            if let Some(ref partition_include_schema_table) = self.partition_include_schema_table {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionIncludeSchemaTable", partition_include_schema_table)?;
            }
            if let Some(ref sasl_password) = self.sasl_password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SaslPassword", sasl_password)?;
            }
            if let Some(ref sasl_user_name) = self.sasl_user_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SaslUserName", sasl_user_name)?;
            }
            if let Some(ref security_protocol) = self.security_protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityProtocol", security_protocol)?;
            }
            if let Some(ref ssl_ca_certificate_arn) = self.ssl_ca_certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslCaCertificateArn", ssl_ca_certificate_arn)?;
            }
            if let Some(ref ssl_client_certificate_arn) = self.ssl_client_certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslClientCertificateArn", ssl_client_certificate_arn)?;
            }
            if let Some(ref ssl_client_key_arn) = self.ssl_client_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslClientKeyArn", ssl_client_key_arn)?;
            }
            if let Some(ref ssl_client_key_password) = self.ssl_client_key_password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslClientKeyPassword", ssl_client_key_password)?;
            }
            if let Some(ref topic) = self.topic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Topic", topic)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KafkaSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KafkaSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KafkaSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KafkaSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut broker: Option<::Value<String>> = None;
                    let mut include_control_details: Option<::Value<bool>> = None;
                    let mut include_null_and_empty: Option<::Value<bool>> = None;
                    let mut include_partition_value: Option<::Value<bool>> = None;
                    let mut include_table_alter_operations: Option<::Value<bool>> = None;
                    let mut include_transaction_details: Option<::Value<bool>> = None;
                    let mut message_format: Option<::Value<String>> = None;
                    let mut message_max_bytes: Option<::Value<u32>> = None;
                    let mut no_hex_prefix: Option<::Value<bool>> = None;
                    let mut partition_include_schema_table: Option<::Value<bool>> = None;
                    let mut sasl_password: Option<::Value<String>> = None;
                    let mut sasl_user_name: Option<::Value<String>> = None;
                    let mut security_protocol: Option<::Value<String>> = None;
                    let mut ssl_ca_certificate_arn: Option<::Value<String>> = None;
                    let mut ssl_client_certificate_arn: Option<::Value<String>> = None;
                    let mut ssl_client_key_arn: Option<::Value<String>> = None;
                    let mut ssl_client_key_password: Option<::Value<String>> = None;
                    let mut topic: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Broker" => {
                                broker = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeControlDetails" => {
                                include_control_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeNullAndEmpty" => {
                                include_null_and_empty = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludePartitionValue" => {
                                include_partition_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeTableAlterOperations" => {
                                include_table_alter_operations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeTransactionDetails" => {
                                include_transaction_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageFormat" => {
                                message_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageMaxBytes" => {
                                message_max_bytes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoHexPrefix" => {
                                no_hex_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PartitionIncludeSchemaTable" => {
                                partition_include_schema_table = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SaslPassword" => {
                                sasl_password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SaslUserName" => {
                                sasl_user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityProtocol" => {
                                security_protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslCaCertificateArn" => {
                                ssl_ca_certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslClientCertificateArn" => {
                                ssl_client_certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslClientKeyArn" => {
                                ssl_client_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslClientKeyPassword" => {
                                ssl_client_key_password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Topic" => {
                                topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KafkaSettings {
                        broker: broker,
                        include_control_details: include_control_details,
                        include_null_and_empty: include_null_and_empty,
                        include_partition_value: include_partition_value,
                        include_table_alter_operations: include_table_alter_operations,
                        include_transaction_details: include_transaction_details,
                        message_format: message_format,
                        message_max_bytes: message_max_bytes,
                        no_hex_prefix: no_hex_prefix,
                        partition_include_schema_table: partition_include_schema_table,
                        sasl_password: sasl_password,
                        sasl_user_name: sasl_user_name,
                        security_protocol: security_protocol,
                        ssl_ca_certificate_arn: ssl_ca_certificate_arn,
                        ssl_client_certificate_arn: ssl_client_certificate_arn,
                        ssl_client_key_arn: ssl_client_key_arn,
                        ssl_client_key_password: ssl_client_key_password,
                        topic: topic,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.KinesisSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisSettings {
        /// Property [`IncludeControlDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-includecontroldetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_control_details: Option<::Value<bool>>,
        /// Property [`IncludeNullAndEmpty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-includenullandempty).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_null_and_empty: Option<::Value<bool>>,
        /// Property [`IncludePartitionValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-includepartitionvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_partition_value: Option<::Value<bool>>,
        /// Property [`IncludeTableAlterOperations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-includetablealteroperations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_table_alter_operations: Option<::Value<bool>>,
        /// Property [`IncludeTransactionDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-includetransactiondetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_transaction_details: Option<::Value<bool>>,
        /// Property [`MessageFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-messageformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_format: Option<::Value<String>>,
        /// Property [`NoHexPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-nohexprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_hex_prefix: Option<::Value<bool>>,
        /// Property [`PartitionIncludeSchemaTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-partitionincludeschematable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition_include_schema_table: Option<::Value<bool>>,
        /// Property [`ServiceAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-serviceaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_access_role_arn: Option<::Value<String>>,
        /// Property [`StreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-streamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KinesisSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref include_control_details) = self.include_control_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeControlDetails", include_control_details)?;
            }
            if let Some(ref include_null_and_empty) = self.include_null_and_empty {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeNullAndEmpty", include_null_and_empty)?;
            }
            if let Some(ref include_partition_value) = self.include_partition_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludePartitionValue", include_partition_value)?;
            }
            if let Some(ref include_table_alter_operations) = self.include_table_alter_operations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeTableAlterOperations", include_table_alter_operations)?;
            }
            if let Some(ref include_transaction_details) = self.include_transaction_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeTransactionDetails", include_transaction_details)?;
            }
            if let Some(ref message_format) = self.message_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageFormat", message_format)?;
            }
            if let Some(ref no_hex_prefix) = self.no_hex_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoHexPrefix", no_hex_prefix)?;
            }
            if let Some(ref partition_include_schema_table) = self.partition_include_schema_table {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionIncludeSchemaTable", partition_include_schema_table)?;
            }
            if let Some(ref service_access_role_arn) = self.service_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessRoleArn", service_access_role_arn)?;
            }
            if let Some(ref stream_arn) = self.stream_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamArn", stream_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut include_control_details: Option<::Value<bool>> = None;
                    let mut include_null_and_empty: Option<::Value<bool>> = None;
                    let mut include_partition_value: Option<::Value<bool>> = None;
                    let mut include_table_alter_operations: Option<::Value<bool>> = None;
                    let mut include_transaction_details: Option<::Value<bool>> = None;
                    let mut message_format: Option<::Value<String>> = None;
                    let mut no_hex_prefix: Option<::Value<bool>> = None;
                    let mut partition_include_schema_table: Option<::Value<bool>> = None;
                    let mut service_access_role_arn: Option<::Value<String>> = None;
                    let mut stream_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IncludeControlDetails" => {
                                include_control_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeNullAndEmpty" => {
                                include_null_and_empty = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludePartitionValue" => {
                                include_partition_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeTableAlterOperations" => {
                                include_table_alter_operations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeTransactionDetails" => {
                                include_transaction_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageFormat" => {
                                message_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoHexPrefix" => {
                                no_hex_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PartitionIncludeSchemaTable" => {
                                partition_include_schema_table = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceAccessRoleArn" => {
                                service_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamArn" => {
                                stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisSettings {
                        include_control_details: include_control_details,
                        include_null_and_empty: include_null_and_empty,
                        include_partition_value: include_partition_value,
                        include_table_alter_operations: include_table_alter_operations,
                        include_transaction_details: include_transaction_details,
                        message_format: message_format,
                        no_hex_prefix: no_hex_prefix,
                        partition_include_schema_table: partition_include_schema_table,
                        service_access_role_arn: service_access_role_arn,
                        stream_arn: stream_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.MicrosoftSqlServerSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MicrosoftSqlServerSettings {
        /// Property [`BcpPacketSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-bcppacketsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bcp_packet_size: Option<::Value<u32>>,
        /// Property [`ControlTablesFileGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-controltablesfilegroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub control_tables_file_group: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`ForceLobLookup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-forceloblookup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub force_lob_lookup: Option<::Value<bool>>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`QuerySingleAlwaysOnNode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-querysinglealwaysonnode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_single_always_on_node: Option<::Value<bool>>,
        /// Property [`ReadBackupOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-readbackuponly).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_backup_only: Option<::Value<bool>>,
        /// Property [`SafeguardPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-safeguardpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub safeguard_policy: Option<::Value<String>>,
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
        /// Property [`ServerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-servername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_name: Option<::Value<String>>,
        /// Property [`TlogAccessMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-tlogaccessmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tlog_access_mode: Option<::Value<String>>,
        /// Property [`TrimSpaceInChar`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-trimspaceinchar).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trim_space_in_char: Option<::Value<bool>>,
        /// Property [`UseBcpFullLoad`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-usebcpfullload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_bcp_full_load: Option<::Value<bool>>,
        /// Property [`UseThirdPartyBackupDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-usethirdpartybackupdevice).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_third_party_backup_device: Option<::Value<bool>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-microsoftsqlserversettings.html#cfn-dms-endpoint-microsoftsqlserversettings-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MicrosoftSqlServerSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bcp_packet_size) = self.bcp_packet_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BcpPacketSize", bcp_packet_size)?;
            }
            if let Some(ref control_tables_file_group) = self.control_tables_file_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlTablesFileGroup", control_tables_file_group)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref force_lob_lookup) = self.force_lob_lookup {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForceLobLookup", force_lob_lookup)?;
            }
            if let Some(ref password) = self.password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref query_single_always_on_node) = self.query_single_always_on_node {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuerySingleAlwaysOnNode", query_single_always_on_node)?;
            }
            if let Some(ref read_backup_only) = self.read_backup_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadBackupOnly", read_backup_only)?;
            }
            if let Some(ref safeguard_policy) = self.safeguard_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SafeguardPolicy", safeguard_policy)?;
            }
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            if let Some(ref server_name) = self.server_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", server_name)?;
            }
            if let Some(ref tlog_access_mode) = self.tlog_access_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TlogAccessMode", tlog_access_mode)?;
            }
            if let Some(ref trim_space_in_char) = self.trim_space_in_char {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrimSpaceInChar", trim_space_in_char)?;
            }
            if let Some(ref use_bcp_full_load) = self.use_bcp_full_load {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseBcpFullLoad", use_bcp_full_load)?;
            }
            if let Some(ref use_third_party_backup_device) = self.use_third_party_backup_device {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseThirdPartyBackupDevice", use_third_party_backup_device)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MicrosoftSqlServerSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MicrosoftSqlServerSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MicrosoftSqlServerSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MicrosoftSqlServerSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bcp_packet_size: Option<::Value<u32>> = None;
                    let mut control_tables_file_group: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut force_lob_lookup: Option<::Value<bool>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut query_single_always_on_node: Option<::Value<bool>> = None;
                    let mut read_backup_only: Option<::Value<bool>> = None;
                    let mut safeguard_policy: Option<::Value<String>> = None;
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;
                    let mut server_name: Option<::Value<String>> = None;
                    let mut tlog_access_mode: Option<::Value<String>> = None;
                    let mut trim_space_in_char: Option<::Value<bool>> = None;
                    let mut use_bcp_full_load: Option<::Value<bool>> = None;
                    let mut use_third_party_backup_device: Option<::Value<bool>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BcpPacketSize" => {
                                bcp_packet_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ControlTablesFileGroup" => {
                                control_tables_file_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ForceLobLookup" => {
                                force_lob_lookup = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QuerySingleAlwaysOnNode" => {
                                query_single_always_on_node = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadBackupOnly" => {
                                read_backup_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SafeguardPolicy" => {
                                safeguard_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerName" => {
                                server_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TlogAccessMode" => {
                                tlog_access_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrimSpaceInChar" => {
                                trim_space_in_char = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseBcpFullLoad" => {
                                use_bcp_full_load = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseThirdPartyBackupDevice" => {
                                use_third_party_backup_device = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MicrosoftSqlServerSettings {
                        bcp_packet_size: bcp_packet_size,
                        control_tables_file_group: control_tables_file_group,
                        database_name: database_name,
                        force_lob_lookup: force_lob_lookup,
                        password: password,
                        port: port,
                        query_single_always_on_node: query_single_always_on_node,
                        read_backup_only: read_backup_only,
                        safeguard_policy: safeguard_policy,
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                        server_name: server_name,
                        tlog_access_mode: tlog_access_mode,
                        trim_space_in_char: trim_space_in_char,
                        use_bcp_full_load: use_bcp_full_load,
                        use_third_party_backup_device: use_third_party_backup_device,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.MongoDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MongoDbSettings {
        /// Property [`AuthMechanism`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-authmechanism).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_mechanism: Option<::Value<String>>,
        /// Property [`AuthSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-authsource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_source: Option<::Value<String>>,
        /// Property [`AuthType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-authtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_type: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`DocsToInvestigate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-docstoinvestigate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub docs_to_investigate: Option<::Value<String>>,
        /// Property [`ExtractDocId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-extractdocid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub extract_doc_id: Option<::Value<String>>,
        /// Property [`NestingLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-nestinglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nesting_level: Option<::Value<String>>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
        /// Property [`ServerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-servername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_name: Option<::Value<String>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mongodbsettings.html#cfn-dms-endpoint-mongodbsettings-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MongoDbSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auth_mechanism) = self.auth_mechanism {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthMechanism", auth_mechanism)?;
            }
            if let Some(ref auth_source) = self.auth_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthSource", auth_source)?;
            }
            if let Some(ref auth_type) = self.auth_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthType", auth_type)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref docs_to_investigate) = self.docs_to_investigate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocsToInvestigate", docs_to_investigate)?;
            }
            if let Some(ref extract_doc_id) = self.extract_doc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtractDocId", extract_doc_id)?;
            }
            if let Some(ref nesting_level) = self.nesting_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NestingLevel", nesting_level)?;
            }
            if let Some(ref password) = self.password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            if let Some(ref server_name) = self.server_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", server_name)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MongoDbSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MongoDbSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MongoDbSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MongoDbSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auth_mechanism: Option<::Value<String>> = None;
                    let mut auth_source: Option<::Value<String>> = None;
                    let mut auth_type: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut docs_to_investigate: Option<::Value<String>> = None;
                    let mut extract_doc_id: Option<::Value<String>> = None;
                    let mut nesting_level: Option<::Value<String>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;
                    let mut server_name: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthMechanism" => {
                                auth_mechanism = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthSource" => {
                                auth_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthType" => {
                                auth_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocsToInvestigate" => {
                                docs_to_investigate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExtractDocId" => {
                                extract_doc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NestingLevel" => {
                                nesting_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerName" => {
                                server_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MongoDbSettings {
                        auth_mechanism: auth_mechanism,
                        auth_source: auth_source,
                        auth_type: auth_type,
                        database_name: database_name,
                        docs_to_investigate: docs_to_investigate,
                        extract_doc_id: extract_doc_id,
                        nesting_level: nesting_level,
                        password: password,
                        port: port,
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                        server_name: server_name,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.MySqlSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct MySqlSettings {
        /// Property [`AfterConnectScript`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html#cfn-dms-endpoint-mysqlsettings-afterconnectscript).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub after_connect_script: Option<::Value<String>>,
        /// Property [`CleanSourceMetadataOnMismatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html#cfn-dms-endpoint-mysqlsettings-cleansourcemetadataonmismatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub clean_source_metadata_on_mismatch: Option<::Value<bool>>,
        /// Property [`EventsPollInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html#cfn-dms-endpoint-mysqlsettings-eventspollinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub events_poll_interval: Option<::Value<u32>>,
        /// Property [`MaxFileSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html#cfn-dms-endpoint-mysqlsettings-maxfilesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_file_size: Option<::Value<u32>>,
        /// Property [`ParallelLoadThreads`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html#cfn-dms-endpoint-mysqlsettings-parallelloadthreads).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parallel_load_threads: Option<::Value<u32>>,
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html#cfn-dms-endpoint-mysqlsettings-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html#cfn-dms-endpoint-mysqlsettings-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
        /// Property [`ServerTimezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html#cfn-dms-endpoint-mysqlsettings-servertimezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_timezone: Option<::Value<String>>,
        /// Property [`TargetDbType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-mysqlsettings.html#cfn-dms-endpoint-mysqlsettings-targetdbtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_db_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MySqlSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref after_connect_script) = self.after_connect_script {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AfterConnectScript", after_connect_script)?;
            }
            if let Some(ref clean_source_metadata_on_mismatch) = self.clean_source_metadata_on_mismatch {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CleanSourceMetadataOnMismatch", clean_source_metadata_on_mismatch)?;
            }
            if let Some(ref events_poll_interval) = self.events_poll_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventsPollInterval", events_poll_interval)?;
            }
            if let Some(ref max_file_size) = self.max_file_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxFileSize", max_file_size)?;
            }
            if let Some(ref parallel_load_threads) = self.parallel_load_threads {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParallelLoadThreads", parallel_load_threads)?;
            }
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            if let Some(ref server_timezone) = self.server_timezone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerTimezone", server_timezone)?;
            }
            if let Some(ref target_db_type) = self.target_db_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetDbType", target_db_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MySqlSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MySqlSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MySqlSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MySqlSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut after_connect_script: Option<::Value<String>> = None;
                    let mut clean_source_metadata_on_mismatch: Option<::Value<bool>> = None;
                    let mut events_poll_interval: Option<::Value<u32>> = None;
                    let mut max_file_size: Option<::Value<u32>> = None;
                    let mut parallel_load_threads: Option<::Value<u32>> = None;
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;
                    let mut server_timezone: Option<::Value<String>> = None;
                    let mut target_db_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AfterConnectScript" => {
                                after_connect_script = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CleanSourceMetadataOnMismatch" => {
                                clean_source_metadata_on_mismatch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventsPollInterval" => {
                                events_poll_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxFileSize" => {
                                max_file_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParallelLoadThreads" => {
                                parallel_load_threads = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerTimezone" => {
                                server_timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetDbType" => {
                                target_db_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MySqlSettings {
                        after_connect_script: after_connect_script,
                        clean_source_metadata_on_mismatch: clean_source_metadata_on_mismatch,
                        events_poll_interval: events_poll_interval,
                        max_file_size: max_file_size,
                        parallel_load_threads: parallel_load_threads,
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                        server_timezone: server_timezone,
                        target_db_type: target_db_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.NeptuneSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-neptunesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct NeptuneSettings {
        /// Property [`ErrorRetryDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-neptunesettings.html#cfn-dms-endpoint-neptunesettings-errorretryduration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_retry_duration: Option<::Value<u32>>,
        /// Property [`IamAuthEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-neptunesettings.html#cfn-dms-endpoint-neptunesettings-iamauthenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_auth_enabled: Option<::Value<bool>>,
        /// Property [`MaxFileSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-neptunesettings.html#cfn-dms-endpoint-neptunesettings-maxfilesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_file_size: Option<::Value<u32>>,
        /// Property [`MaxRetryCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-neptunesettings.html#cfn-dms-endpoint-neptunesettings-maxretrycount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_retry_count: Option<::Value<u32>>,
        /// Property [`S3BucketFolder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-neptunesettings.html#cfn-dms-endpoint-neptunesettings-s3bucketfolder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_folder: Option<::Value<String>>,
        /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-neptunesettings.html#cfn-dms-endpoint-neptunesettings-s3bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_name: Option<::Value<String>>,
        /// Property [`ServiceAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-neptunesettings.html#cfn-dms-endpoint-neptunesettings-serviceaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_access_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NeptuneSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error_retry_duration) = self.error_retry_duration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorRetryDuration", error_retry_duration)?;
            }
            if let Some(ref iam_auth_enabled) = self.iam_auth_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamAuthEnabled", iam_auth_enabled)?;
            }
            if let Some(ref max_file_size) = self.max_file_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxFileSize", max_file_size)?;
            }
            if let Some(ref max_retry_count) = self.max_retry_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRetryCount", max_retry_count)?;
            }
            if let Some(ref s3_bucket_folder) = self.s3_bucket_folder {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketFolder", s3_bucket_folder)?;
            }
            if let Some(ref s3_bucket_name) = self.s3_bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", s3_bucket_name)?;
            }
            if let Some(ref service_access_role_arn) = self.service_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessRoleArn", service_access_role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NeptuneSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NeptuneSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NeptuneSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NeptuneSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error_retry_duration: Option<::Value<u32>> = None;
                    let mut iam_auth_enabled: Option<::Value<bool>> = None;
                    let mut max_file_size: Option<::Value<u32>> = None;
                    let mut max_retry_count: Option<::Value<u32>> = None;
                    let mut s3_bucket_folder: Option<::Value<String>> = None;
                    let mut s3_bucket_name: Option<::Value<String>> = None;
                    let mut service_access_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ErrorRetryDuration" => {
                                error_retry_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IamAuthEnabled" => {
                                iam_auth_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxFileSize" => {
                                max_file_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxRetryCount" => {
                                max_retry_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketFolder" => {
                                s3_bucket_folder = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketName" => {
                                s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceAccessRoleArn" => {
                                service_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NeptuneSettings {
                        error_retry_duration: error_retry_duration,
                        iam_auth_enabled: iam_auth_enabled,
                        max_file_size: max_file_size,
                        max_retry_count: max_retry_count,
                        s3_bucket_folder: s3_bucket_folder,
                        s3_bucket_name: s3_bucket_name,
                        service_access_role_arn: service_access_role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.OracleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct OracleSettings {
        /// Property [`AccessAlternateDirectly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-accessalternatedirectly).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_alternate_directly: Option<::Value<bool>>,
        /// Property [`AddSupplementalLogging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-addsupplementallogging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub add_supplemental_logging: Option<::Value<bool>>,
        /// Property [`AdditionalArchivedLogDestId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-additionalarchivedlogdestid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub additional_archived_log_dest_id: Option<::Value<u32>>,
        /// Property [`AllowSelectNestedTables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-allowselectnestedtables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_select_nested_tables: Option<::Value<bool>>,
        /// Property [`ArchivedLogDestId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-archivedlogdestid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub archived_log_dest_id: Option<::Value<u32>>,
        /// Property [`ArchivedLogsOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-archivedlogsonly).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub archived_logs_only: Option<::Value<bool>>,
        /// Property [`AsmPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-asmpassword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub asm_password: Option<::Value<String>>,
        /// Property [`AsmServer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-asmserver).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub asm_server: Option<::Value<String>>,
        /// Property [`AsmUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-asmuser).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub asm_user: Option<::Value<String>>,
        /// Property [`CharLengthSemantics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-charlengthsemantics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub char_length_semantics: Option<::Value<String>>,
        /// Property [`DirectPathNoLog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-directpathnolog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub direct_path_no_log: Option<::Value<bool>>,
        /// Property [`DirectPathParallelLoad`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-directpathparallelload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub direct_path_parallel_load: Option<::Value<bool>>,
        /// Property [`EnableHomogenousTablespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-enablehomogenoustablespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_homogenous_tablespace: Option<::Value<bool>>,
        /// Property [`ExtraArchivedLogDestIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-extraarchivedlogdestids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub extra_archived_log_dest_ids: Option<::ValueList<u32>>,
        /// Property [`FailTasksOnLobTruncation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-failtasksonlobtruncation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fail_tasks_on_lob_truncation: Option<::Value<bool>>,
        /// Property [`NumberDatatypeScale`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-numberdatatypescale).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_datatype_scale: Option<::Value<u32>>,
        /// Property [`OraclePathPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-oraclepathprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oracle_path_prefix: Option<::Value<String>>,
        /// Property [`ParallelAsmReadThreads`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-parallelasmreadthreads).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parallel_asm_read_threads: Option<::Value<u32>>,
        /// Property [`ReadAheadBlocks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-readaheadblocks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_ahead_blocks: Option<::Value<u32>>,
        /// Property [`ReadTableSpaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-readtablespacename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_table_space_name: Option<::Value<bool>>,
        /// Property [`ReplacePathPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-replacepathprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replace_path_prefix: Option<::Value<bool>>,
        /// Property [`RetryInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-retryinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_interval: Option<::Value<u32>>,
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerOracleAsmAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-secretsmanageroracleasmaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_oracle_asm_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerOracleAsmSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-secretsmanageroracleasmsecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_oracle_asm_secret_id: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
        /// Property [`SecurityDbEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-securitydbencryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_db_encryption: Option<::Value<String>>,
        /// Property [`SecurityDbEncryptionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-securitydbencryptionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_db_encryption_name: Option<::Value<String>>,
        /// Property [`SpatialDataOptionToGeoJsonFunctionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-spatialdataoptiontogeojsonfunctionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub spatial_data_option_to_geo_json_function_name: Option<::Value<String>>,
        /// Property [`StandbyDelayTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-standbydelaytime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub standby_delay_time: Option<::Value<u32>>,
        /// Property [`UseAlternateFolderForOnline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-usealternatefolderforonline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_alternate_folder_for_online: Option<::Value<bool>>,
        /// Property [`UseBFile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-usebfile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_b_file: Option<::Value<bool>>,
        /// Property [`UseDirectPathFullLoad`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-usedirectpathfullload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_direct_path_full_load: Option<::Value<bool>>,
        /// Property [`UseLogminerReader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-uselogminerreader).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_logminer_reader: Option<::Value<bool>>,
        /// Property [`UsePathPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-oraclesettings.html#cfn-dms-endpoint-oraclesettings-usepathprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_path_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OracleSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_alternate_directly) = self.access_alternate_directly {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessAlternateDirectly", access_alternate_directly)?;
            }
            if let Some(ref add_supplemental_logging) = self.add_supplemental_logging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddSupplementalLogging", add_supplemental_logging)?;
            }
            if let Some(ref additional_archived_log_dest_id) = self.additional_archived_log_dest_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalArchivedLogDestId", additional_archived_log_dest_id)?;
            }
            if let Some(ref allow_select_nested_tables) = self.allow_select_nested_tables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowSelectNestedTables", allow_select_nested_tables)?;
            }
            if let Some(ref archived_log_dest_id) = self.archived_log_dest_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArchivedLogDestId", archived_log_dest_id)?;
            }
            if let Some(ref archived_logs_only) = self.archived_logs_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArchivedLogsOnly", archived_logs_only)?;
            }
            if let Some(ref asm_password) = self.asm_password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AsmPassword", asm_password)?;
            }
            if let Some(ref asm_server) = self.asm_server {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AsmServer", asm_server)?;
            }
            if let Some(ref asm_user) = self.asm_user {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AsmUser", asm_user)?;
            }
            if let Some(ref char_length_semantics) = self.char_length_semantics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CharLengthSemantics", char_length_semantics)?;
            }
            if let Some(ref direct_path_no_log) = self.direct_path_no_log {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectPathNoLog", direct_path_no_log)?;
            }
            if let Some(ref direct_path_parallel_load) = self.direct_path_parallel_load {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectPathParallelLoad", direct_path_parallel_load)?;
            }
            if let Some(ref enable_homogenous_tablespace) = self.enable_homogenous_tablespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableHomogenousTablespace", enable_homogenous_tablespace)?;
            }
            if let Some(ref extra_archived_log_dest_ids) = self.extra_archived_log_dest_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtraArchivedLogDestIds", extra_archived_log_dest_ids)?;
            }
            if let Some(ref fail_tasks_on_lob_truncation) = self.fail_tasks_on_lob_truncation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailTasksOnLobTruncation", fail_tasks_on_lob_truncation)?;
            }
            if let Some(ref number_datatype_scale) = self.number_datatype_scale {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberDatatypeScale", number_datatype_scale)?;
            }
            if let Some(ref oracle_path_prefix) = self.oracle_path_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OraclePathPrefix", oracle_path_prefix)?;
            }
            if let Some(ref parallel_asm_read_threads) = self.parallel_asm_read_threads {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParallelAsmReadThreads", parallel_asm_read_threads)?;
            }
            if let Some(ref read_ahead_blocks) = self.read_ahead_blocks {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadAheadBlocks", read_ahead_blocks)?;
            }
            if let Some(ref read_table_space_name) = self.read_table_space_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadTableSpaceName", read_table_space_name)?;
            }
            if let Some(ref replace_path_prefix) = self.replace_path_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplacePathPrefix", replace_path_prefix)?;
            }
            if let Some(ref retry_interval) = self.retry_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryInterval", retry_interval)?;
            }
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_oracle_asm_access_role_arn) = self.secrets_manager_oracle_asm_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerOracleAsmAccessRoleArn", secrets_manager_oracle_asm_access_role_arn)?;
            }
            if let Some(ref secrets_manager_oracle_asm_secret_id) = self.secrets_manager_oracle_asm_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerOracleAsmSecretId", secrets_manager_oracle_asm_secret_id)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            if let Some(ref security_db_encryption) = self.security_db_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityDbEncryption", security_db_encryption)?;
            }
            if let Some(ref security_db_encryption_name) = self.security_db_encryption_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityDbEncryptionName", security_db_encryption_name)?;
            }
            if let Some(ref spatial_data_option_to_geo_json_function_name) = self.spatial_data_option_to_geo_json_function_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpatialDataOptionToGeoJsonFunctionName", spatial_data_option_to_geo_json_function_name)?;
            }
            if let Some(ref standby_delay_time) = self.standby_delay_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StandbyDelayTime", standby_delay_time)?;
            }
            if let Some(ref use_alternate_folder_for_online) = self.use_alternate_folder_for_online {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseAlternateFolderForOnline", use_alternate_folder_for_online)?;
            }
            if let Some(ref use_b_file) = self.use_b_file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseBFile", use_b_file)?;
            }
            if let Some(ref use_direct_path_full_load) = self.use_direct_path_full_load {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseDirectPathFullLoad", use_direct_path_full_load)?;
            }
            if let Some(ref use_logminer_reader) = self.use_logminer_reader {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseLogminerReader", use_logminer_reader)?;
            }
            if let Some(ref use_path_prefix) = self.use_path_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsePathPrefix", use_path_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OracleSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OracleSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OracleSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OracleSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_alternate_directly: Option<::Value<bool>> = None;
                    let mut add_supplemental_logging: Option<::Value<bool>> = None;
                    let mut additional_archived_log_dest_id: Option<::Value<u32>> = None;
                    let mut allow_select_nested_tables: Option<::Value<bool>> = None;
                    let mut archived_log_dest_id: Option<::Value<u32>> = None;
                    let mut archived_logs_only: Option<::Value<bool>> = None;
                    let mut asm_password: Option<::Value<String>> = None;
                    let mut asm_server: Option<::Value<String>> = None;
                    let mut asm_user: Option<::Value<String>> = None;
                    let mut char_length_semantics: Option<::Value<String>> = None;
                    let mut direct_path_no_log: Option<::Value<bool>> = None;
                    let mut direct_path_parallel_load: Option<::Value<bool>> = None;
                    let mut enable_homogenous_tablespace: Option<::Value<bool>> = None;
                    let mut extra_archived_log_dest_ids: Option<::ValueList<u32>> = None;
                    let mut fail_tasks_on_lob_truncation: Option<::Value<bool>> = None;
                    let mut number_datatype_scale: Option<::Value<u32>> = None;
                    let mut oracle_path_prefix: Option<::Value<String>> = None;
                    let mut parallel_asm_read_threads: Option<::Value<u32>> = None;
                    let mut read_ahead_blocks: Option<::Value<u32>> = None;
                    let mut read_table_space_name: Option<::Value<bool>> = None;
                    let mut replace_path_prefix: Option<::Value<bool>> = None;
                    let mut retry_interval: Option<::Value<u32>> = None;
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_oracle_asm_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_oracle_asm_secret_id: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;
                    let mut security_db_encryption: Option<::Value<String>> = None;
                    let mut security_db_encryption_name: Option<::Value<String>> = None;
                    let mut spatial_data_option_to_geo_json_function_name: Option<::Value<String>> = None;
                    let mut standby_delay_time: Option<::Value<u32>> = None;
                    let mut use_alternate_folder_for_online: Option<::Value<bool>> = None;
                    let mut use_b_file: Option<::Value<bool>> = None;
                    let mut use_direct_path_full_load: Option<::Value<bool>> = None;
                    let mut use_logminer_reader: Option<::Value<bool>> = None;
                    let mut use_path_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessAlternateDirectly" => {
                                access_alternate_directly = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AddSupplementalLogging" => {
                                add_supplemental_logging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AdditionalArchivedLogDestId" => {
                                additional_archived_log_dest_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowSelectNestedTables" => {
                                allow_select_nested_tables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ArchivedLogDestId" => {
                                archived_log_dest_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ArchivedLogsOnly" => {
                                archived_logs_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AsmPassword" => {
                                asm_password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AsmServer" => {
                                asm_server = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AsmUser" => {
                                asm_user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CharLengthSemantics" => {
                                char_length_semantics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DirectPathNoLog" => {
                                direct_path_no_log = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DirectPathParallelLoad" => {
                                direct_path_parallel_load = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableHomogenousTablespace" => {
                                enable_homogenous_tablespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExtraArchivedLogDestIds" => {
                                extra_archived_log_dest_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailTasksOnLobTruncation" => {
                                fail_tasks_on_lob_truncation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberDatatypeScale" => {
                                number_datatype_scale = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OraclePathPrefix" => {
                                oracle_path_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParallelAsmReadThreads" => {
                                parallel_asm_read_threads = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadAheadBlocks" => {
                                read_ahead_blocks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadTableSpaceName" => {
                                read_table_space_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplacePathPrefix" => {
                                replace_path_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryInterval" => {
                                retry_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerOracleAsmAccessRoleArn" => {
                                secrets_manager_oracle_asm_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerOracleAsmSecretId" => {
                                secrets_manager_oracle_asm_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityDbEncryption" => {
                                security_db_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityDbEncryptionName" => {
                                security_db_encryption_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpatialDataOptionToGeoJsonFunctionName" => {
                                spatial_data_option_to_geo_json_function_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StandbyDelayTime" => {
                                standby_delay_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseAlternateFolderForOnline" => {
                                use_alternate_folder_for_online = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseBFile" => {
                                use_b_file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseDirectPathFullLoad" => {
                                use_direct_path_full_load = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseLogminerReader" => {
                                use_logminer_reader = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UsePathPrefix" => {
                                use_path_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OracleSettings {
                        access_alternate_directly: access_alternate_directly,
                        add_supplemental_logging: add_supplemental_logging,
                        additional_archived_log_dest_id: additional_archived_log_dest_id,
                        allow_select_nested_tables: allow_select_nested_tables,
                        archived_log_dest_id: archived_log_dest_id,
                        archived_logs_only: archived_logs_only,
                        asm_password: asm_password,
                        asm_server: asm_server,
                        asm_user: asm_user,
                        char_length_semantics: char_length_semantics,
                        direct_path_no_log: direct_path_no_log,
                        direct_path_parallel_load: direct_path_parallel_load,
                        enable_homogenous_tablespace: enable_homogenous_tablespace,
                        extra_archived_log_dest_ids: extra_archived_log_dest_ids,
                        fail_tasks_on_lob_truncation: fail_tasks_on_lob_truncation,
                        number_datatype_scale: number_datatype_scale,
                        oracle_path_prefix: oracle_path_prefix,
                        parallel_asm_read_threads: parallel_asm_read_threads,
                        read_ahead_blocks: read_ahead_blocks,
                        read_table_space_name: read_table_space_name,
                        replace_path_prefix: replace_path_prefix,
                        retry_interval: retry_interval,
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_oracle_asm_access_role_arn: secrets_manager_oracle_asm_access_role_arn,
                        secrets_manager_oracle_asm_secret_id: secrets_manager_oracle_asm_secret_id,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                        security_db_encryption: security_db_encryption,
                        security_db_encryption_name: security_db_encryption_name,
                        spatial_data_option_to_geo_json_function_name: spatial_data_option_to_geo_json_function_name,
                        standby_delay_time: standby_delay_time,
                        use_alternate_folder_for_online: use_alternate_folder_for_online,
                        use_b_file: use_b_file,
                        use_direct_path_full_load: use_direct_path_full_load,
                        use_logminer_reader: use_logminer_reader,
                        use_path_prefix: use_path_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.PostgreSqlSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct PostgreSqlSettings {
        /// Property [`AfterConnectScript`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-afterconnectscript).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub after_connect_script: Option<::Value<String>>,
        /// Property [`BabelfishDatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-babelfishdatabasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub babelfish_database_name: Option<::Value<String>>,
        /// Property [`CaptureDdls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-captureddls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capture_ddls: Option<::Value<bool>>,
        /// Property [`DatabaseMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-databasemode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_mode: Option<::Value<String>>,
        /// Property [`DdlArtifactsSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-ddlartifactsschema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ddl_artifacts_schema: Option<::Value<String>>,
        /// Property [`ExecuteTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-executetimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execute_timeout: Option<::Value<u32>>,
        /// Property [`FailTasksOnLobTruncation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-failtasksonlobtruncation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fail_tasks_on_lob_truncation: Option<::Value<bool>>,
        /// Property [`HeartbeatEnable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-heartbeatenable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub heartbeat_enable: Option<::Value<bool>>,
        /// Property [`HeartbeatFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-heartbeatfrequency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub heartbeat_frequency: Option<::Value<u32>>,
        /// Property [`HeartbeatSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-heartbeatschema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub heartbeat_schema: Option<::Value<String>>,
        /// Property [`MapBooleanAsBoolean`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-mapbooleanasboolean).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub map_boolean_as_boolean: Option<::Value<bool>>,
        /// Property [`MaxFileSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-maxfilesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_file_size: Option<::Value<u32>>,
        /// Property [`PluginName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-pluginname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub plugin_name: Option<::Value<String>>,
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
        /// Property [`SlotName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html#cfn-dms-endpoint-postgresqlsettings-slotname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PostgreSqlSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref after_connect_script) = self.after_connect_script {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AfterConnectScript", after_connect_script)?;
            }
            if let Some(ref babelfish_database_name) = self.babelfish_database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BabelfishDatabaseName", babelfish_database_name)?;
            }
            if let Some(ref capture_ddls) = self.capture_ddls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptureDdls", capture_ddls)?;
            }
            if let Some(ref database_mode) = self.database_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseMode", database_mode)?;
            }
            if let Some(ref ddl_artifacts_schema) = self.ddl_artifacts_schema {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DdlArtifactsSchema", ddl_artifacts_schema)?;
            }
            if let Some(ref execute_timeout) = self.execute_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecuteTimeout", execute_timeout)?;
            }
            if let Some(ref fail_tasks_on_lob_truncation) = self.fail_tasks_on_lob_truncation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailTasksOnLobTruncation", fail_tasks_on_lob_truncation)?;
            }
            if let Some(ref heartbeat_enable) = self.heartbeat_enable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeartbeatEnable", heartbeat_enable)?;
            }
            if let Some(ref heartbeat_frequency) = self.heartbeat_frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeartbeatFrequency", heartbeat_frequency)?;
            }
            if let Some(ref heartbeat_schema) = self.heartbeat_schema {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeartbeatSchema", heartbeat_schema)?;
            }
            if let Some(ref map_boolean_as_boolean) = self.map_boolean_as_boolean {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MapBooleanAsBoolean", map_boolean_as_boolean)?;
            }
            if let Some(ref max_file_size) = self.max_file_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxFileSize", max_file_size)?;
            }
            if let Some(ref plugin_name) = self.plugin_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PluginName", plugin_name)?;
            }
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            if let Some(ref slot_name) = self.slot_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotName", slot_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PostgreSqlSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PostgreSqlSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PostgreSqlSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PostgreSqlSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut after_connect_script: Option<::Value<String>> = None;
                    let mut babelfish_database_name: Option<::Value<String>> = None;
                    let mut capture_ddls: Option<::Value<bool>> = None;
                    let mut database_mode: Option<::Value<String>> = None;
                    let mut ddl_artifacts_schema: Option<::Value<String>> = None;
                    let mut execute_timeout: Option<::Value<u32>> = None;
                    let mut fail_tasks_on_lob_truncation: Option<::Value<bool>> = None;
                    let mut heartbeat_enable: Option<::Value<bool>> = None;
                    let mut heartbeat_frequency: Option<::Value<u32>> = None;
                    let mut heartbeat_schema: Option<::Value<String>> = None;
                    let mut map_boolean_as_boolean: Option<::Value<bool>> = None;
                    let mut max_file_size: Option<::Value<u32>> = None;
                    let mut plugin_name: Option<::Value<String>> = None;
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;
                    let mut slot_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AfterConnectScript" => {
                                after_connect_script = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BabelfishDatabaseName" => {
                                babelfish_database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptureDdls" => {
                                capture_ddls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseMode" => {
                                database_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DdlArtifactsSchema" => {
                                ddl_artifacts_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExecuteTimeout" => {
                                execute_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailTasksOnLobTruncation" => {
                                fail_tasks_on_lob_truncation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeartbeatEnable" => {
                                heartbeat_enable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeartbeatFrequency" => {
                                heartbeat_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeartbeatSchema" => {
                                heartbeat_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MapBooleanAsBoolean" => {
                                map_boolean_as_boolean = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxFileSize" => {
                                max_file_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PluginName" => {
                                plugin_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SlotName" => {
                                slot_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PostgreSqlSettings {
                        after_connect_script: after_connect_script,
                        babelfish_database_name: babelfish_database_name,
                        capture_ddls: capture_ddls,
                        database_mode: database_mode,
                        ddl_artifacts_schema: ddl_artifacts_schema,
                        execute_timeout: execute_timeout,
                        fail_tasks_on_lob_truncation: fail_tasks_on_lob_truncation,
                        heartbeat_enable: heartbeat_enable,
                        heartbeat_frequency: heartbeat_frequency,
                        heartbeat_schema: heartbeat_schema,
                        map_boolean_as_boolean: map_boolean_as_boolean,
                        max_file_size: max_file_size,
                        plugin_name: plugin_name,
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                        slot_name: slot_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.RedisSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redissettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RedisSettings {
        /// Property [`AuthPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redissettings.html#cfn-dms-endpoint-redissettings-authpassword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_password: Option<::Value<String>>,
        /// Property [`AuthType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redissettings.html#cfn-dms-endpoint-redissettings-authtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_type: Option<::Value<String>>,
        /// Property [`AuthUserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redissettings.html#cfn-dms-endpoint-redissettings-authusername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_user_name: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redissettings.html#cfn-dms-endpoint-redissettings-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<f64>>,
        /// Property [`ServerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redissettings.html#cfn-dms-endpoint-redissettings-servername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_name: Option<::Value<String>>,
        /// Property [`SslCaCertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redissettings.html#cfn-dms-endpoint-redissettings-sslcacertificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_ca_certificate_arn: Option<::Value<String>>,
        /// Property [`SslSecurityProtocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redissettings.html#cfn-dms-endpoint-redissettings-sslsecurityprotocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssl_security_protocol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RedisSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auth_password) = self.auth_password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthPassword", auth_password)?;
            }
            if let Some(ref auth_type) = self.auth_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthType", auth_type)?;
            }
            if let Some(ref auth_user_name) = self.auth_user_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthUserName", auth_user_name)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref server_name) = self.server_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerName", server_name)?;
            }
            if let Some(ref ssl_ca_certificate_arn) = self.ssl_ca_certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslCaCertificateArn", ssl_ca_certificate_arn)?;
            }
            if let Some(ref ssl_security_protocol) = self.ssl_security_protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslSecurityProtocol", ssl_security_protocol)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedisSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedisSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedisSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedisSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auth_password: Option<::Value<String>> = None;
                    let mut auth_type: Option<::Value<String>> = None;
                    let mut auth_user_name: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;
                    let mut server_name: Option<::Value<String>> = None;
                    let mut ssl_ca_certificate_arn: Option<::Value<String>> = None;
                    let mut ssl_security_protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthPassword" => {
                                auth_password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthType" => {
                                auth_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthUserName" => {
                                auth_user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerName" => {
                                server_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslCaCertificateArn" => {
                                ssl_ca_certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SslSecurityProtocol" => {
                                ssl_security_protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedisSettings {
                        auth_password: auth_password,
                        auth_type: auth_type,
                        auth_user_name: auth_user_name,
                        port: port,
                        server_name: server_name,
                        ssl_ca_certificate_arn: ssl_ca_certificate_arn,
                        ssl_security_protocol: ssl_security_protocol,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.RedshiftSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftSettings {
        /// Property [`AcceptAnyDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-acceptanydate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub accept_any_date: Option<::Value<bool>>,
        /// Property [`AfterConnectScript`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-afterconnectscript).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub after_connect_script: Option<::Value<String>>,
        /// Property [`BucketFolder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-bucketfolder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_folder: Option<::Value<String>>,
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: Option<::Value<String>>,
        /// Property [`CaseSensitiveNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-casesensitivenames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub case_sensitive_names: Option<::Value<bool>>,
        /// Property [`CompUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-compupdate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comp_update: Option<::Value<bool>>,
        /// Property [`ConnectionTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-connectiontimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_timeout: Option<::Value<u32>>,
        /// Property [`DateFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-dateformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_format: Option<::Value<String>>,
        /// Property [`EmptyAsNull`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-emptyasnull).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub empty_as_null: Option<::Value<bool>>,
        /// Property [`EncryptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-encryptionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_mode: Option<::Value<String>>,
        /// Property [`ExplicitIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-explicitids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub explicit_ids: Option<::Value<bool>>,
        /// Property [`FileTransferUploadStreams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-filetransferuploadstreams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_transfer_upload_streams: Option<::Value<u32>>,
        /// Property [`LoadTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-loadtimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub load_timeout: Option<::Value<u32>>,
        /// Property [`MapBooleanAsBoolean`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-mapbooleanasboolean).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub map_boolean_as_boolean: Option<::Value<bool>>,
        /// Property [`MaxFileSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-maxfilesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_file_size: Option<::Value<u32>>,
        /// Property [`RemoveQuotes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-removequotes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_quotes: Option<::Value<bool>>,
        /// Property [`ReplaceChars`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-replacechars).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replace_chars: Option<::Value<String>>,
        /// Property [`ReplaceInvalidChars`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-replaceinvalidchars).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replace_invalid_chars: Option<::Value<String>>,
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
        /// Property [`ServerSideEncryptionKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-serversideencryptionkmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_side_encryption_kms_key_id: Option<::Value<String>>,
        /// Property [`ServiceAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-serviceaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_access_role_arn: Option<::Value<String>>,
        /// Property [`TimeFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-timeformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_format: Option<::Value<String>>,
        /// Property [`TrimBlanks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-trimblanks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trim_blanks: Option<::Value<bool>>,
        /// Property [`TruncateColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-truncatecolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub truncate_columns: Option<::Value<bool>>,
        /// Property [`WriteBufferSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html#cfn-dms-endpoint-redshiftsettings-writebuffersize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_buffer_size: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RedshiftSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref accept_any_date) = self.accept_any_date {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptAnyDate", accept_any_date)?;
            }
            if let Some(ref after_connect_script) = self.after_connect_script {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AfterConnectScript", after_connect_script)?;
            }
            if let Some(ref bucket_folder) = self.bucket_folder {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketFolder", bucket_folder)?;
            }
            if let Some(ref bucket_name) = self.bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
            }
            if let Some(ref case_sensitive_names) = self.case_sensitive_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaseSensitiveNames", case_sensitive_names)?;
            }
            if let Some(ref comp_update) = self.comp_update {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompUpdate", comp_update)?;
            }
            if let Some(ref connection_timeout) = self.connection_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionTimeout", connection_timeout)?;
            }
            if let Some(ref date_format) = self.date_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateFormat", date_format)?;
            }
            if let Some(ref empty_as_null) = self.empty_as_null {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmptyAsNull", empty_as_null)?;
            }
            if let Some(ref encryption_mode) = self.encryption_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionMode", encryption_mode)?;
            }
            if let Some(ref explicit_ids) = self.explicit_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExplicitIds", explicit_ids)?;
            }
            if let Some(ref file_transfer_upload_streams) = self.file_transfer_upload_streams {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileTransferUploadStreams", file_transfer_upload_streams)?;
            }
            if let Some(ref load_timeout) = self.load_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadTimeout", load_timeout)?;
            }
            if let Some(ref map_boolean_as_boolean) = self.map_boolean_as_boolean {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MapBooleanAsBoolean", map_boolean_as_boolean)?;
            }
            if let Some(ref max_file_size) = self.max_file_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxFileSize", max_file_size)?;
            }
            if let Some(ref remove_quotes) = self.remove_quotes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveQuotes", remove_quotes)?;
            }
            if let Some(ref replace_chars) = self.replace_chars {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplaceChars", replace_chars)?;
            }
            if let Some(ref replace_invalid_chars) = self.replace_invalid_chars {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplaceInvalidChars", replace_invalid_chars)?;
            }
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            if let Some(ref server_side_encryption_kms_key_id) = self.server_side_encryption_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerSideEncryptionKmsKeyId", server_side_encryption_kms_key_id)?;
            }
            if let Some(ref service_access_role_arn) = self.service_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessRoleArn", service_access_role_arn)?;
            }
            if let Some(ref time_format) = self.time_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeFormat", time_format)?;
            }
            if let Some(ref trim_blanks) = self.trim_blanks {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrimBlanks", trim_blanks)?;
            }
            if let Some(ref truncate_columns) = self.truncate_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TruncateColumns", truncate_columns)?;
            }
            if let Some(ref write_buffer_size) = self.write_buffer_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteBufferSize", write_buffer_size)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut accept_any_date: Option<::Value<bool>> = None;
                    let mut after_connect_script: Option<::Value<String>> = None;
                    let mut bucket_folder: Option<::Value<String>> = None;
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut case_sensitive_names: Option<::Value<bool>> = None;
                    let mut comp_update: Option<::Value<bool>> = None;
                    let mut connection_timeout: Option<::Value<u32>> = None;
                    let mut date_format: Option<::Value<String>> = None;
                    let mut empty_as_null: Option<::Value<bool>> = None;
                    let mut encryption_mode: Option<::Value<String>> = None;
                    let mut explicit_ids: Option<::Value<bool>> = None;
                    let mut file_transfer_upload_streams: Option<::Value<u32>> = None;
                    let mut load_timeout: Option<::Value<u32>> = None;
                    let mut map_boolean_as_boolean: Option<::Value<bool>> = None;
                    let mut max_file_size: Option<::Value<u32>> = None;
                    let mut remove_quotes: Option<::Value<bool>> = None;
                    let mut replace_chars: Option<::Value<String>> = None;
                    let mut replace_invalid_chars: Option<::Value<String>> = None;
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;
                    let mut server_side_encryption_kms_key_id: Option<::Value<String>> = None;
                    let mut service_access_role_arn: Option<::Value<String>> = None;
                    let mut time_format: Option<::Value<String>> = None;
                    let mut trim_blanks: Option<::Value<bool>> = None;
                    let mut truncate_columns: Option<::Value<bool>> = None;
                    let mut write_buffer_size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AcceptAnyDate" => {
                                accept_any_date = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AfterConnectScript" => {
                                after_connect_script = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketFolder" => {
                                bucket_folder = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaseSensitiveNames" => {
                                case_sensitive_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CompUpdate" => {
                                comp_update = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectionTimeout" => {
                                connection_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DateFormat" => {
                                date_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmptyAsNull" => {
                                empty_as_null = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionMode" => {
                                encryption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExplicitIds" => {
                                explicit_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileTransferUploadStreams" => {
                                file_transfer_upload_streams = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoadTimeout" => {
                                load_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MapBooleanAsBoolean" => {
                                map_boolean_as_boolean = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxFileSize" => {
                                max_file_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveQuotes" => {
                                remove_quotes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplaceChars" => {
                                replace_chars = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplaceInvalidChars" => {
                                replace_invalid_chars = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerSideEncryptionKmsKeyId" => {
                                server_side_encryption_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceAccessRoleArn" => {
                                service_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeFormat" => {
                                time_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrimBlanks" => {
                                trim_blanks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TruncateColumns" => {
                                truncate_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WriteBufferSize" => {
                                write_buffer_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftSettings {
                        accept_any_date: accept_any_date,
                        after_connect_script: after_connect_script,
                        bucket_folder: bucket_folder,
                        bucket_name: bucket_name,
                        case_sensitive_names: case_sensitive_names,
                        comp_update: comp_update,
                        connection_timeout: connection_timeout,
                        date_format: date_format,
                        empty_as_null: empty_as_null,
                        encryption_mode: encryption_mode,
                        explicit_ids: explicit_ids,
                        file_transfer_upload_streams: file_transfer_upload_streams,
                        load_timeout: load_timeout,
                        map_boolean_as_boolean: map_boolean_as_boolean,
                        max_file_size: max_file_size,
                        remove_quotes: remove_quotes,
                        replace_chars: replace_chars,
                        replace_invalid_chars: replace_invalid_chars,
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                        server_side_encryption_kms_key_id: server_side_encryption_kms_key_id,
                        service_access_role_arn: service_access_role_arn,
                        time_format: time_format,
                        trim_blanks: trim_blanks,
                        truncate_columns: truncate_columns,
                        write_buffer_size: write_buffer_size,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.S3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Settings {
        /// Property [`AddColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-addcolumnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub add_column_name: Option<::Value<bool>>,
        /// Property [`AddTrailingPaddingCharacter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-addtrailingpaddingcharacter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub add_trailing_padding_character: Option<::Value<bool>>,
        /// Property [`BucketFolder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-bucketfolder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_folder: Option<::Value<String>>,
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: Option<::Value<String>>,
        /// Property [`CannedAclForObjects`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-cannedaclforobjects).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub canned_acl_for_objects: Option<::Value<String>>,
        /// Property [`CdcInsertsAndUpdates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-cdcinsertsandupdates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cdc_inserts_and_updates: Option<::Value<bool>>,
        /// Property [`CdcInsertsOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-cdcinsertsonly).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cdc_inserts_only: Option<::Value<bool>>,
        /// Property [`CdcMaxBatchInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-cdcmaxbatchinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cdc_max_batch_interval: Option<::Value<u32>>,
        /// Property [`CdcMinFileSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-cdcminfilesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cdc_min_file_size: Option<::Value<u32>>,
        /// Property [`CdcPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-cdcpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cdc_path: Option<::Value<String>>,
        /// Property [`CompressionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-compressiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compression_type: Option<::Value<String>>,
        /// Property [`CsvDelimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-csvdelimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub csv_delimiter: Option<::Value<String>>,
        /// Property [`CsvNoSupValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-csvnosupvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub csv_no_sup_value: Option<::Value<String>>,
        /// Property [`CsvNullValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-csvnullvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub csv_null_value: Option<::Value<String>>,
        /// Property [`CsvRowDelimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-csvrowdelimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub csv_row_delimiter: Option<::Value<String>>,
        /// Property [`DataFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-dataformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_format: Option<::Value<String>>,
        /// Property [`DataPageSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-datapagesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_page_size: Option<::Value<u32>>,
        /// Property [`DatePartitionDelimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-datepartitiondelimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_partition_delimiter: Option<::Value<String>>,
        /// Property [`DatePartitionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-datepartitionenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_partition_enabled: Option<::Value<bool>>,
        /// Property [`DatePartitionSequence`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-datepartitionsequence).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_partition_sequence: Option<::Value<String>>,
        /// Property [`DatePartitionTimezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-datepartitiontimezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_partition_timezone: Option<::Value<String>>,
        /// Property [`DictPageSizeLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-dictpagesizelimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dict_page_size_limit: Option<::Value<u32>>,
        /// Property [`EnableStatistics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-enablestatistics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_statistics: Option<::Value<bool>>,
        /// Property [`EncodingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-encodingtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encoding_type: Option<::Value<String>>,
        /// Property [`EncryptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-encryptionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_mode: Option<::Value<String>>,
        /// Property [`ExpectedBucketOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-expectedbucketowner).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expected_bucket_owner: Option<::Value<String>>,
        /// Property [`ExternalTableDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-externaltabledefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub external_table_definition: Option<::Value<String>>,
        /// Property [`GlueCatalogGeneration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-gluecataloggeneration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub glue_catalog_generation: Option<::Value<bool>>,
        /// Property [`IgnoreHeaderRows`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-ignoreheaderrows).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ignore_header_rows: Option<::Value<u32>>,
        /// Property [`IncludeOpForFullLoad`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-includeopforfullload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_op_for_full_load: Option<::Value<bool>>,
        /// Property [`MaxFileSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-maxfilesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_file_size: Option<::Value<u32>>,
        /// Property [`ParquetTimestampInMillisecond`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-parquettimestampinmillisecond).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parquet_timestamp_in_millisecond: Option<::Value<bool>>,
        /// Property [`ParquetVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-parquetversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parquet_version: Option<::Value<String>>,
        /// Property [`PreserveTransactions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-preservetransactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub preserve_transactions: Option<::Value<bool>>,
        /// Property [`Rfc4180`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-rfc4180).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rfc4180: Option<::Value<bool>>,
        /// Property [`RowGroupLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-rowgrouplength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub row_group_length: Option<::Value<u32>>,
        /// Property [`ServerSideEncryptionKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-serversideencryptionkmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_side_encryption_kms_key_id: Option<::Value<String>>,
        /// Property [`ServiceAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-serviceaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_access_role_arn: Option<::Value<String>>,
        /// Property [`TimestampColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-timestampcolumnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp_column_name: Option<::Value<String>>,
        /// Property [`UseCsvNoSupValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-usecsvnosupvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_csv_no_sup_value: Option<::Value<bool>>,
        /// Property [`UseTaskStartTimeForFullLoadTimestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-usetaskstarttimeforfullloadtimestamp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_task_start_time_for_full_load_timestamp: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for S3Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref add_column_name) = self.add_column_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddColumnName", add_column_name)?;
            }
            if let Some(ref add_trailing_padding_character) = self.add_trailing_padding_character {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddTrailingPaddingCharacter", add_trailing_padding_character)?;
            }
            if let Some(ref bucket_folder) = self.bucket_folder {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketFolder", bucket_folder)?;
            }
            if let Some(ref bucket_name) = self.bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
            }
            if let Some(ref canned_acl_for_objects) = self.canned_acl_for_objects {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CannedAclForObjects", canned_acl_for_objects)?;
            }
            if let Some(ref cdc_inserts_and_updates) = self.cdc_inserts_and_updates {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdcInsertsAndUpdates", cdc_inserts_and_updates)?;
            }
            if let Some(ref cdc_inserts_only) = self.cdc_inserts_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdcInsertsOnly", cdc_inserts_only)?;
            }
            if let Some(ref cdc_max_batch_interval) = self.cdc_max_batch_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdcMaxBatchInterval", cdc_max_batch_interval)?;
            }
            if let Some(ref cdc_min_file_size) = self.cdc_min_file_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdcMinFileSize", cdc_min_file_size)?;
            }
            if let Some(ref cdc_path) = self.cdc_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CdcPath", cdc_path)?;
            }
            if let Some(ref compression_type) = self.compression_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionType", compression_type)?;
            }
            if let Some(ref csv_delimiter) = self.csv_delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvDelimiter", csv_delimiter)?;
            }
            if let Some(ref csv_no_sup_value) = self.csv_no_sup_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvNoSupValue", csv_no_sup_value)?;
            }
            if let Some(ref csv_null_value) = self.csv_null_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvNullValue", csv_null_value)?;
            }
            if let Some(ref csv_row_delimiter) = self.csv_row_delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvRowDelimiter", csv_row_delimiter)?;
            }
            if let Some(ref data_format) = self.data_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataFormat", data_format)?;
            }
            if let Some(ref data_page_size) = self.data_page_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataPageSize", data_page_size)?;
            }
            if let Some(ref date_partition_delimiter) = self.date_partition_delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatePartitionDelimiter", date_partition_delimiter)?;
            }
            if let Some(ref date_partition_enabled) = self.date_partition_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatePartitionEnabled", date_partition_enabled)?;
            }
            if let Some(ref date_partition_sequence) = self.date_partition_sequence {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatePartitionSequence", date_partition_sequence)?;
            }
            if let Some(ref date_partition_timezone) = self.date_partition_timezone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatePartitionTimezone", date_partition_timezone)?;
            }
            if let Some(ref dict_page_size_limit) = self.dict_page_size_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DictPageSizeLimit", dict_page_size_limit)?;
            }
            if let Some(ref enable_statistics) = self.enable_statistics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableStatistics", enable_statistics)?;
            }
            if let Some(ref encoding_type) = self.encoding_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncodingType", encoding_type)?;
            }
            if let Some(ref encryption_mode) = self.encryption_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionMode", encryption_mode)?;
            }
            if let Some(ref expected_bucket_owner) = self.expected_bucket_owner {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpectedBucketOwner", expected_bucket_owner)?;
            }
            if let Some(ref external_table_definition) = self.external_table_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExternalTableDefinition", external_table_definition)?;
            }
            if let Some(ref glue_catalog_generation) = self.glue_catalog_generation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlueCatalogGeneration", glue_catalog_generation)?;
            }
            if let Some(ref ignore_header_rows) = self.ignore_header_rows {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IgnoreHeaderRows", ignore_header_rows)?;
            }
            if let Some(ref include_op_for_full_load) = self.include_op_for_full_load {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeOpForFullLoad", include_op_for_full_load)?;
            }
            if let Some(ref max_file_size) = self.max_file_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxFileSize", max_file_size)?;
            }
            if let Some(ref parquet_timestamp_in_millisecond) = self.parquet_timestamp_in_millisecond {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParquetTimestampInMillisecond", parquet_timestamp_in_millisecond)?;
            }
            if let Some(ref parquet_version) = self.parquet_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParquetVersion", parquet_version)?;
            }
            if let Some(ref preserve_transactions) = self.preserve_transactions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreserveTransactions", preserve_transactions)?;
            }
            if let Some(ref rfc4180) = self.rfc4180 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rfc4180", rfc4180)?;
            }
            if let Some(ref row_group_length) = self.row_group_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RowGroupLength", row_group_length)?;
            }
            if let Some(ref server_side_encryption_kms_key_id) = self.server_side_encryption_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerSideEncryptionKmsKeyId", server_side_encryption_kms_key_id)?;
            }
            if let Some(ref service_access_role_arn) = self.service_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessRoleArn", service_access_role_arn)?;
            }
            if let Some(ref timestamp_column_name) = self.timestamp_column_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestampColumnName", timestamp_column_name)?;
            }
            if let Some(ref use_csv_no_sup_value) = self.use_csv_no_sup_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseCsvNoSupValue", use_csv_no_sup_value)?;
            }
            if let Some(ref use_task_start_time_for_full_load_timestamp) = self.use_task_start_time_for_full_load_timestamp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseTaskStartTimeForFullLoadTimestamp", use_task_start_time_for_full_load_timestamp)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Settings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut add_column_name: Option<::Value<bool>> = None;
                    let mut add_trailing_padding_character: Option<::Value<bool>> = None;
                    let mut bucket_folder: Option<::Value<String>> = None;
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut canned_acl_for_objects: Option<::Value<String>> = None;
                    let mut cdc_inserts_and_updates: Option<::Value<bool>> = None;
                    let mut cdc_inserts_only: Option<::Value<bool>> = None;
                    let mut cdc_max_batch_interval: Option<::Value<u32>> = None;
                    let mut cdc_min_file_size: Option<::Value<u32>> = None;
                    let mut cdc_path: Option<::Value<String>> = None;
                    let mut compression_type: Option<::Value<String>> = None;
                    let mut csv_delimiter: Option<::Value<String>> = None;
                    let mut csv_no_sup_value: Option<::Value<String>> = None;
                    let mut csv_null_value: Option<::Value<String>> = None;
                    let mut csv_row_delimiter: Option<::Value<String>> = None;
                    let mut data_format: Option<::Value<String>> = None;
                    let mut data_page_size: Option<::Value<u32>> = None;
                    let mut date_partition_delimiter: Option<::Value<String>> = None;
                    let mut date_partition_enabled: Option<::Value<bool>> = None;
                    let mut date_partition_sequence: Option<::Value<String>> = None;
                    let mut date_partition_timezone: Option<::Value<String>> = None;
                    let mut dict_page_size_limit: Option<::Value<u32>> = None;
                    let mut enable_statistics: Option<::Value<bool>> = None;
                    let mut encoding_type: Option<::Value<String>> = None;
                    let mut encryption_mode: Option<::Value<String>> = None;
                    let mut expected_bucket_owner: Option<::Value<String>> = None;
                    let mut external_table_definition: Option<::Value<String>> = None;
                    let mut glue_catalog_generation: Option<::Value<bool>> = None;
                    let mut ignore_header_rows: Option<::Value<u32>> = None;
                    let mut include_op_for_full_load: Option<::Value<bool>> = None;
                    let mut max_file_size: Option<::Value<u32>> = None;
                    let mut parquet_timestamp_in_millisecond: Option<::Value<bool>> = None;
                    let mut parquet_version: Option<::Value<String>> = None;
                    let mut preserve_transactions: Option<::Value<bool>> = None;
                    let mut rfc4180: Option<::Value<bool>> = None;
                    let mut row_group_length: Option<::Value<u32>> = None;
                    let mut server_side_encryption_kms_key_id: Option<::Value<String>> = None;
                    let mut service_access_role_arn: Option<::Value<String>> = None;
                    let mut timestamp_column_name: Option<::Value<String>> = None;
                    let mut use_csv_no_sup_value: Option<::Value<bool>> = None;
                    let mut use_task_start_time_for_full_load_timestamp: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddColumnName" => {
                                add_column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AddTrailingPaddingCharacter" => {
                                add_trailing_padding_character = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketFolder" => {
                                bucket_folder = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CannedAclForObjects" => {
                                canned_acl_for_objects = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CdcInsertsAndUpdates" => {
                                cdc_inserts_and_updates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CdcInsertsOnly" => {
                                cdc_inserts_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CdcMaxBatchInterval" => {
                                cdc_max_batch_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CdcMinFileSize" => {
                                cdc_min_file_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CdcPath" => {
                                cdc_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CompressionType" => {
                                compression_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CsvDelimiter" => {
                                csv_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CsvNoSupValue" => {
                                csv_no_sup_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CsvNullValue" => {
                                csv_null_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CsvRowDelimiter" => {
                                csv_row_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataFormat" => {
                                data_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataPageSize" => {
                                data_page_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatePartitionDelimiter" => {
                                date_partition_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatePartitionEnabled" => {
                                date_partition_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatePartitionSequence" => {
                                date_partition_sequence = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatePartitionTimezone" => {
                                date_partition_timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DictPageSizeLimit" => {
                                dict_page_size_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableStatistics" => {
                                enable_statistics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncodingType" => {
                                encoding_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionMode" => {
                                encryption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExpectedBucketOwner" => {
                                expected_bucket_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExternalTableDefinition" => {
                                external_table_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GlueCatalogGeneration" => {
                                glue_catalog_generation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IgnoreHeaderRows" => {
                                ignore_header_rows = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeOpForFullLoad" => {
                                include_op_for_full_load = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxFileSize" => {
                                max_file_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParquetTimestampInMillisecond" => {
                                parquet_timestamp_in_millisecond = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParquetVersion" => {
                                parquet_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreserveTransactions" => {
                                preserve_transactions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rfc4180" => {
                                rfc4180 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RowGroupLength" => {
                                row_group_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerSideEncryptionKmsKeyId" => {
                                server_side_encryption_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceAccessRoleArn" => {
                                service_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimestampColumnName" => {
                                timestamp_column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseCsvNoSupValue" => {
                                use_csv_no_sup_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseTaskStartTimeForFullLoadTimestamp" => {
                                use_task_start_time_for_full_load_timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Settings {
                        add_column_name: add_column_name,
                        add_trailing_padding_character: add_trailing_padding_character,
                        bucket_folder: bucket_folder,
                        bucket_name: bucket_name,
                        canned_acl_for_objects: canned_acl_for_objects,
                        cdc_inserts_and_updates: cdc_inserts_and_updates,
                        cdc_inserts_only: cdc_inserts_only,
                        cdc_max_batch_interval: cdc_max_batch_interval,
                        cdc_min_file_size: cdc_min_file_size,
                        cdc_path: cdc_path,
                        compression_type: compression_type,
                        csv_delimiter: csv_delimiter,
                        csv_no_sup_value: csv_no_sup_value,
                        csv_null_value: csv_null_value,
                        csv_row_delimiter: csv_row_delimiter,
                        data_format: data_format,
                        data_page_size: data_page_size,
                        date_partition_delimiter: date_partition_delimiter,
                        date_partition_enabled: date_partition_enabled,
                        date_partition_sequence: date_partition_sequence,
                        date_partition_timezone: date_partition_timezone,
                        dict_page_size_limit: dict_page_size_limit,
                        enable_statistics: enable_statistics,
                        encoding_type: encoding_type,
                        encryption_mode: encryption_mode,
                        expected_bucket_owner: expected_bucket_owner,
                        external_table_definition: external_table_definition,
                        glue_catalog_generation: glue_catalog_generation,
                        ignore_header_rows: ignore_header_rows,
                        include_op_for_full_load: include_op_for_full_load,
                        max_file_size: max_file_size,
                        parquet_timestamp_in_millisecond: parquet_timestamp_in_millisecond,
                        parquet_version: parquet_version,
                        preserve_transactions: preserve_transactions,
                        rfc4180: rfc4180,
                        row_group_length: row_group_length,
                        server_side_encryption_kms_key_id: server_side_encryption_kms_key_id,
                        service_access_role_arn: service_access_role_arn,
                        timestamp_column_name: timestamp_column_name,
                        use_csv_no_sup_value: use_csv_no_sup_value,
                        use_task_start_time_for_full_load_timestamp: use_task_start_time_for_full_load_timestamp,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.SybaseSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-sybasesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct SybaseSettings {
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-sybasesettings.html#cfn-dms-endpoint-sybasesettings-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-sybasesettings.html#cfn-dms-endpoint-sybasesettings-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SybaseSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SybaseSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SybaseSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SybaseSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SybaseSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SybaseSettings {
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod migration_project {
    //! Property types for the `MigrationProject` resource.

    /// The [`AWS::DMS::MigrationProject.DataProviderDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-dataproviderdescriptor.html) property type.
    #[derive(Debug, Default)]
    pub struct DataProviderDescriptor {
        /// Property [`DataProviderArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-dataproviderdescriptor.html#cfn-dms-migrationproject-dataproviderdescriptor-dataproviderarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_provider_arn: Option<::Value<String>>,
        /// Property [`DataProviderIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-dataproviderdescriptor.html#cfn-dms-migrationproject-dataproviderdescriptor-dataprovideridentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_provider_identifier: Option<::Value<String>>,
        /// Property [`DataProviderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-dataproviderdescriptor.html#cfn-dms-migrationproject-dataproviderdescriptor-dataprovidername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_provider_name: Option<::Value<String>>,
        /// Property [`SecretsManagerAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-dataproviderdescriptor.html#cfn-dms-migrationproject-dataproviderdescriptor-secretsmanageraccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_access_role_arn: Option<::Value<String>>,
        /// Property [`SecretsManagerSecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-dataproviderdescriptor.html#cfn-dms-migrationproject-dataproviderdescriptor-secretsmanagersecretid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secrets_manager_secret_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataProviderDescriptor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_provider_arn) = self.data_provider_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataProviderArn", data_provider_arn)?;
            }
            if let Some(ref data_provider_identifier) = self.data_provider_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataProviderIdentifier", data_provider_identifier)?;
            }
            if let Some(ref data_provider_name) = self.data_provider_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataProviderName", data_provider_name)?;
            }
            if let Some(ref secrets_manager_access_role_arn) = self.secrets_manager_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerAccessRoleArn", secrets_manager_access_role_arn)?;
            }
            if let Some(ref secrets_manager_secret_id) = self.secrets_manager_secret_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretsManagerSecretId", secrets_manager_secret_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataProviderDescriptor {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataProviderDescriptor, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataProviderDescriptor;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataProviderDescriptor")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_provider_arn: Option<::Value<String>> = None;
                    let mut data_provider_identifier: Option<::Value<String>> = None;
                    let mut data_provider_name: Option<::Value<String>> = None;
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataProviderArn" => {
                                data_provider_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataProviderIdentifier" => {
                                data_provider_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataProviderName" => {
                                data_provider_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerAccessRoleArn" => {
                                secrets_manager_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretsManagerSecretId" => {
                                secrets_manager_secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataProviderDescriptor {
                        data_provider_arn: data_provider_arn,
                        data_provider_identifier: data_provider_identifier,
                        data_provider_name: data_provider_name,
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::MigrationProject.SchemaConversionApplicationAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-schemaconversionapplicationattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaConversionApplicationAttributes {
        /// Property [`S3BucketPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-schemaconversionapplicationattributes.html#cfn-dms-migrationproject-schemaconversionapplicationattributes-s3bucketpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_path: Option<::Value<String>>,
        /// Property [`S3BucketRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-migrationproject-schemaconversionapplicationattributes.html#cfn-dms-migrationproject-schemaconversionapplicationattributes-s3bucketrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SchemaConversionApplicationAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_bucket_path) = self.s3_bucket_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketPath", s3_bucket_path)?;
            }
            if let Some(ref s3_bucket_role_arn) = self.s3_bucket_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketRoleArn", s3_bucket_role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaConversionApplicationAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaConversionApplicationAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaConversionApplicationAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaConversionApplicationAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_bucket_path: Option<::Value<String>> = None;
                    let mut s3_bucket_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3BucketPath" => {
                                s3_bucket_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketRoleArn" => {
                                s3_bucket_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaConversionApplicationAttributes {
                        s3_bucket_path: s3_bucket_path,
                        s3_bucket_role_arn: s3_bucket_role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod replication_config {
    //! Property types for the `ReplicationConfig` resource.

    /// The [`AWS::DMS::ReplicationConfig.ComputeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ComputeConfig {
        /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html#cfn-dms-replicationconfig-computeconfig-availabilityzone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_zone: Option<::Value<String>>,
        /// Property [`DnsNameServers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html#cfn-dms-replicationconfig-computeconfig-dnsnameservers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_name_servers: Option<::Value<String>>,
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html#cfn-dms-replicationconfig-computeconfig-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`MaxCapacityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html#cfn-dms-replicationconfig-computeconfig-maxcapacityunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_capacity_units: ::Value<u32>,
        /// Property [`MinCapacityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html#cfn-dms-replicationconfig-computeconfig-mincapacityunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_capacity_units: Option<::Value<u32>>,
        /// Property [`MultiAZ`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html#cfn-dms-replicationconfig-computeconfig-multiaz).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multi_az: Option<::Value<bool>>,
        /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html#cfn-dms-replicationconfig-computeconfig-preferredmaintenancewindow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub preferred_maintenance_window: Option<::Value<String>>,
        /// Property [`ReplicationSubnetGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html#cfn-dms-replicationconfig-computeconfig-replicationsubnetgroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replication_subnet_group_id: Option<::Value<String>>,
        /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-replicationconfig-computeconfig.html#cfn-dms-replicationconfig-computeconfig-vpcsecuritygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_security_group_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ComputeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_zone) = self.availability_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
            }
            if let Some(ref dns_name_servers) = self.dns_name_servers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsNameServers", dns_name_servers)?;
            }
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacityUnits", &self.max_capacity_units)?;
            if let Some(ref min_capacity_units) = self.min_capacity_units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinCapacityUnits", min_capacity_units)?;
            }
            if let Some(ref multi_az) = self.multi_az {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiAZ", multi_az)?;
            }
            if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
            }
            if let Some(ref replication_subnet_group_id) = self.replication_subnet_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSubnetGroupId", replication_subnet_group_id)?;
            }
            if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComputeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComputeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComputeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComputeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone: Option<::Value<String>> = None;
                    let mut dns_name_servers: Option<::Value<String>> = None;
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut max_capacity_units: Option<::Value<u32>> = None;
                    let mut min_capacity_units: Option<::Value<u32>> = None;
                    let mut multi_az: Option<::Value<bool>> = None;
                    let mut preferred_maintenance_window: Option<::Value<String>> = None;
                    let mut replication_subnet_group_id: Option<::Value<String>> = None;
                    let mut vpc_security_group_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DnsNameServers" => {
                                dns_name_servers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxCapacityUnits" => {
                                max_capacity_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinCapacityUnits" => {
                                min_capacity_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MultiAZ" => {
                                multi_az = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreferredMaintenanceWindow" => {
                                preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplicationSubnetGroupId" => {
                                replication_subnet_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcSecurityGroupIds" => {
                                vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComputeConfig {
                        availability_zone: availability_zone,
                        dns_name_servers: dns_name_servers,
                        kms_key_id: kms_key_id,
                        max_capacity_units: max_capacity_units.ok_or(::serde::de::Error::missing_field("MaxCapacityUnits"))?,
                        min_capacity_units: min_capacity_units,
                        multi_az: multi_az,
                        preferred_maintenance_window: preferred_maintenance_window,
                        replication_subnet_group_id: replication_subnet_group_id,
                        vpc_security_group_ids: vpc_security_group_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
