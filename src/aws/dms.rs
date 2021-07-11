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
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
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
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
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
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
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
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
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
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
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

pub mod endpoint {
    //! Property types for the `Endpoint` resource.

    /// The [`AWS::DMS::Endpoint.DocDbSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-docdbsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DocDbSettings {
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

                    Ok(DocDbSettings {
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

    /// The [`AWS::DMS::Endpoint.IbmDb2Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-ibmdb2settings.html) property type.
    #[derive(Debug, Default)]
    pub struct IbmDb2Settings {
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
    }

    impl ::codec::SerializeValue for IbmDb2Settings {
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

    impl ::codec::DeserializeValue for IbmDb2Settings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IbmDb2Settings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IbmDb2Settings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IbmDb2Settings")
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

                    Ok(IbmDb2Settings {
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
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
                    let mut topic: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Broker" => {
                                broker = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Topic" => {
                                topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KafkaSettings {
                        broker: broker,
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
        /// Property [`MessageFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-kinesissettings.html#cfn-dms-endpoint-kinesissettings-messageformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_format: Option<::Value<String>>,
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
            if let Some(ref message_format) = self.message_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageFormat", message_format)?;
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
                    let mut message_format: Option<::Value<String>> = None;
                    let mut service_access_role_arn: Option<::Value<String>> = None;
                    let mut stream_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MessageFormat" => {
                                message_format = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        message_format: message_format,
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
    }

    impl ::codec::SerializeValue for MicrosoftSqlServerSettings {
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

    impl ::codec::DeserializeValue for MicrosoftSqlServerSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MicrosoftSqlServerSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MicrosoftSqlServerSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MicrosoftSqlServerSettings")
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

                    Ok(MicrosoftSqlServerSettings {
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
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
    }

    impl ::codec::SerializeValue for MySqlSettings {
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

    impl ::codec::DeserializeValue for MySqlSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MySqlSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MySqlSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MySqlSettings")
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

                    Ok(MySqlSettings {
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
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
    }

    impl ::codec::SerializeValue for OracleSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
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
                    let mut secrets_manager_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_oracle_asm_access_role_arn: Option<::Value<String>> = None;
                    let mut secrets_manager_oracle_asm_secret_id: Option<::Value<String>> = None;
                    let mut secrets_manager_secret_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
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
                            _ => {}
                        }
                    }

                    Ok(OracleSettings {
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_oracle_asm_access_role_arn: secrets_manager_oracle_asm_access_role_arn,
                        secrets_manager_oracle_asm_secret_id: secrets_manager_oracle_asm_secret_id,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.PostgreSqlSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-postgresqlsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct PostgreSqlSettings {
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
    }

    impl ::codec::SerializeValue for PostgreSqlSettings {
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

    impl ::codec::DeserializeValue for PostgreSqlSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PostgreSqlSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PostgreSqlSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PostgreSqlSettings")
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

                    Ok(PostgreSqlSettings {
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.RedshiftSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-redshiftsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftSettings {
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
    }

    impl ::codec::SerializeValue for RedshiftSettings {
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

    impl ::codec::DeserializeValue for RedshiftSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftSettings")
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

                    Ok(RedshiftSettings {
                        secrets_manager_access_role_arn: secrets_manager_access_role_arn,
                        secrets_manager_secret_id: secrets_manager_secret_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DMS::Endpoint.S3Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Settings {
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
        /// Property [`CsvRowDelimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-csvrowdelimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub csv_row_delimiter: Option<::Value<String>>,
        /// Property [`ExternalTableDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-externaltabledefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub external_table_definition: Option<::Value<String>>,
        /// Property [`ServiceAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dms-endpoint-s3settings.html#cfn-dms-endpoint-s3settings-serviceaccessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_access_role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Settings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_folder) = self.bucket_folder {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketFolder", bucket_folder)?;
            }
            if let Some(ref bucket_name) = self.bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
            }
            if let Some(ref compression_type) = self.compression_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionType", compression_type)?;
            }
            if let Some(ref csv_delimiter) = self.csv_delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvDelimiter", csv_delimiter)?;
            }
            if let Some(ref csv_row_delimiter) = self.csv_row_delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvRowDelimiter", csv_row_delimiter)?;
            }
            if let Some(ref external_table_definition) = self.external_table_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExternalTableDefinition", external_table_definition)?;
            }
            if let Some(ref service_access_role_arn) = self.service_access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceAccessRoleArn", service_access_role_arn)?;
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
                    let mut bucket_folder: Option<::Value<String>> = None;
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut compression_type: Option<::Value<String>> = None;
                    let mut csv_delimiter: Option<::Value<String>> = None;
                    let mut csv_row_delimiter: Option<::Value<String>> = None;
                    let mut external_table_definition: Option<::Value<String>> = None;
                    let mut service_access_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketFolder" => {
                                bucket_folder = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CompressionType" => {
                                compression_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CsvDelimiter" => {
                                csv_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CsvRowDelimiter" => {
                                csv_row_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExternalTableDefinition" => {
                                external_table_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceAccessRoleArn" => {
                                service_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Settings {
                        bucket_folder: bucket_folder,
                        bucket_name: bucket_name,
                        compression_type: compression_type,
                        csv_delimiter: csv_delimiter,
                        csv_row_delimiter: csv_row_delimiter,
                        external_table_definition: external_table_definition,
                        service_access_role_arn: service_access_role_arn,
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
