//! Types for the `AppSync` service.

/// The [`AWS::AppSync::ApiCache`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apicache.html) resource type.
#[derive(Debug, Default)]
pub struct ApiCache {
    properties: ApiCacheProperties
}

/// Properties for the `ApiCache` resource.
#[derive(Debug, Default)]
pub struct ApiCacheProperties {
    /// Property [`ApiCachingBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apicache.html#cfn-appsync-apicache-apicachingbehavior).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_caching_behavior: ::Value<String>,
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apicache.html#cfn-appsync-apicache-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`AtRestEncryptionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apicache.html#cfn-appsync-apicache-atrestencryptionenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub at_rest_encryption_enabled: Option<::Value<bool>>,
    /// Property [`TransitEncryptionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apicache.html#cfn-appsync-apicache-transitencryptionenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub transit_encryption_enabled: Option<::Value<bool>>,
    /// Property [`Ttl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apicache.html#cfn-appsync-apicache-ttl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ttl: ::Value<f64>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apicache.html#cfn-appsync-apicache-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for ApiCacheProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiCachingBehavior", &self.api_caching_behavior)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref at_rest_encryption_enabled) = self.at_rest_encryption_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AtRestEncryptionEnabled", at_rest_encryption_enabled)?;
        }
        if let Some(ref transit_encryption_enabled) = self.transit_encryption_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitEncryptionEnabled", transit_encryption_enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ttl", &self.ttl)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApiCacheProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiCacheProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApiCacheProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApiCacheProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_caching_behavior: Option<::Value<String>> = None;
                let mut api_id: Option<::Value<String>> = None;
                let mut at_rest_encryption_enabled: Option<::Value<bool>> = None;
                let mut transit_encryption_enabled: Option<::Value<bool>> = None;
                let mut ttl: Option<::Value<f64>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiCachingBehavior" => {
                            api_caching_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AtRestEncryptionEnabled" => {
                            at_rest_encryption_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransitEncryptionEnabled" => {
                            transit_encryption_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Ttl" => {
                            ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApiCacheProperties {
                    api_caching_behavior: api_caching_behavior.ok_or(::serde::de::Error::missing_field("ApiCachingBehavior"))?,
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    at_rest_encryption_enabled: at_rest_encryption_enabled,
                    transit_encryption_enabled: transit_encryption_enabled,
                    ttl: ttl.ok_or(::serde::de::Error::missing_field("Ttl"))?,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApiCache {
    type Properties = ApiCacheProperties;
    const TYPE: &'static str = "AWS::AppSync::ApiCache";
    fn properties(&self) -> &ApiCacheProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApiCacheProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApiCache {}

impl From<ApiCacheProperties> for ApiCache {
    fn from(properties: ApiCacheProperties) -> ApiCache {
        ApiCache { properties }
    }
}

/// The [`AWS::AppSync::ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apikey.html) resource type.
#[derive(Debug, Default)]
pub struct ApiKey {
    properties: ApiKeyProperties
}

/// Properties for the `ApiKey` resource.
#[derive(Debug, Default)]
pub struct ApiKeyProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apikey.html#cfn-appsync-apikey-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`ApiKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apikey.html#cfn-appsync-apikey-apikeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_key_id: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apikey.html#cfn-appsync-apikey-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Expires`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apikey.html#cfn-appsync-apikey-expires).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub expires: Option<::Value<f64>>,
}

impl ::serde::Serialize for ApiKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref api_key_id) = self.api_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKeyId", api_key_id)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref expires) = self.expires {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expires", expires)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApiKeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiKeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApiKeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApiKeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut api_key_id: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut expires: Option<::Value<f64>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApiKeyId" => {
                            api_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Expires" => {
                            expires = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApiKeyProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    api_key_id: api_key_id,
                    description: description,
                    expires: expires,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApiKey {
    type Properties = ApiKeyProperties;
    const TYPE: &'static str = "AWS::AppSync::ApiKey";
    fn properties(&self) -> &ApiKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApiKeyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApiKey {}

impl From<ApiKeyProperties> for ApiKey {
    fn from(properties: ApiKeyProperties) -> ApiKey {
        ApiKey { properties }
    }
}

/// The [`AWS::AppSync::DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html) resource type.
#[derive(Debug, Default)]
pub struct DataSource {
    properties: DataSourceProperties
}

/// Properties for the `DataSource` resource.
#[derive(Debug, Default)]
pub struct DataSourceProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DynamoDBConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-dynamodbconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dynamo_db_config: Option<::Value<self::data_source::DynamoDBConfig>>,
    /// Property [`ElasticsearchConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-elasticsearchconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub elasticsearch_config: Option<::Value<self::data_source::ElasticsearchConfig>>,
    /// Property [`HttpConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-httpconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub http_config: Option<::Value<self::data_source::HttpConfig>>,
    /// Property [`LambdaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-lambdaconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lambda_config: Option<::Value<self::data_source::LambdaConfig>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OpenSearchServiceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-opensearchserviceconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub open_search_service_config: Option<::Value<self::data_source::OpenSearchServiceConfig>>,
    /// Property [`RelationalDatabaseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-relationaldatabaseconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub relational_database_config: Option<::Value<self::data_source::RelationalDatabaseConfig>>,
    /// Property [`ServiceRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-servicerolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_role_arn: Option<::Value<String>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html#cfn-appsync-datasource-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for DataSourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref dynamo_db_config) = self.dynamo_db_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDBConfig", dynamo_db_config)?;
        }
        if let Some(ref elasticsearch_config) = self.elasticsearch_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticsearchConfig", elasticsearch_config)?;
        }
        if let Some(ref http_config) = self.http_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpConfig", http_config)?;
        }
        if let Some(ref lambda_config) = self.lambda_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaConfig", lambda_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref open_search_service_config) = self.open_search_service_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenSearchServiceConfig", open_search_service_config)?;
        }
        if let Some(ref relational_database_config) = self.relational_database_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationalDatabaseConfig", relational_database_config)?;
        }
        if let Some(ref service_role_arn) = self.service_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRoleArn", service_role_arn)?;
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
                let mut api_id: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut dynamo_db_config: Option<::Value<self::data_source::DynamoDBConfig>> = None;
                let mut elasticsearch_config: Option<::Value<self::data_source::ElasticsearchConfig>> = None;
                let mut http_config: Option<::Value<self::data_source::HttpConfig>> = None;
                let mut lambda_config: Option<::Value<self::data_source::LambdaConfig>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut open_search_service_config: Option<::Value<self::data_source::OpenSearchServiceConfig>> = None;
                let mut relational_database_config: Option<::Value<self::data_source::RelationalDatabaseConfig>> = None;
                let mut service_role_arn: Option<::Value<String>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DynamoDBConfig" => {
                            dynamo_db_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ElasticsearchConfig" => {
                            elasticsearch_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HttpConfig" => {
                            http_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LambdaConfig" => {
                            lambda_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OpenSearchServiceConfig" => {
                            open_search_service_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RelationalDatabaseConfig" => {
                            relational_database_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRoleArn" => {
                            service_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataSourceProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    description: description,
                    dynamo_db_config: dynamo_db_config,
                    elasticsearch_config: elasticsearch_config,
                    http_config: http_config,
                    lambda_config: lambda_config,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    open_search_service_config: open_search_service_config,
                    relational_database_config: relational_database_config,
                    service_role_arn: service_role_arn,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataSource {
    type Properties = DataSourceProperties;
    const TYPE: &'static str = "AWS::AppSync::DataSource";
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

/// The [`AWS::AppSync::DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-domainname.html) resource type.
#[derive(Debug, Default)]
pub struct DomainName {
    properties: DomainNameProperties
}

/// Properties for the `DomainName` resource.
#[derive(Debug, Default)]
pub struct DomainNameProperties {
    /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-domainname.html#cfn-appsync-domainname-certificatearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_arn: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-domainname.html#cfn-appsync-domainname-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-domainname.html#cfn-appsync-domainname-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
}

impl ::serde::Serialize for DomainNameProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", &self.certificate_arn)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DomainNameProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainNameProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DomainNameProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DomainNameProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate_arn: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateArn" => {
                            certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DomainNameProperties {
                    certificate_arn: certificate_arn.ok_or(::serde::de::Error::missing_field("CertificateArn"))?,
                    description: description,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DomainName {
    type Properties = DomainNameProperties;
    const TYPE: &'static str = "AWS::AppSync::DomainName";
    fn properties(&self) -> &DomainNameProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainNameProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DomainName {}

impl From<DomainNameProperties> for DomainName {
    fn from(properties: DomainNameProperties) -> DomainName {
        DomainName { properties }
    }
}

/// The [`AWS::AppSync::DomainNameApiAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-domainnameapiassociation.html) resource type.
#[derive(Debug, Default)]
pub struct DomainNameApiAssociation {
    properties: DomainNameApiAssociationProperties
}

/// Properties for the `DomainNameApiAssociation` resource.
#[derive(Debug, Default)]
pub struct DomainNameApiAssociationProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-domainnameapiassociation.html#cfn-appsync-domainnameapiassociation-apiid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-domainnameapiassociation.html#cfn-appsync-domainnameapiassociation-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
}

impl ::serde::Serialize for DomainNameApiAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DomainNameApiAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainNameApiAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DomainNameApiAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DomainNameApiAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DomainNameApiAssociationProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DomainNameApiAssociation {
    type Properties = DomainNameApiAssociationProperties;
    const TYPE: &'static str = "AWS::AppSync::DomainNameApiAssociation";
    fn properties(&self) -> &DomainNameApiAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainNameApiAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DomainNameApiAssociation {}

impl From<DomainNameApiAssociationProperties> for DomainNameApiAssociation {
    fn from(properties: DomainNameApiAssociationProperties) -> DomainNameApiAssociation {
        DomainNameApiAssociation { properties }
    }
}

/// The [`AWS::AppSync::FunctionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct FunctionConfiguration {
    properties: FunctionConfigurationProperties
}

/// Properties for the `FunctionConfiguration` resource.
#[derive(Debug, Default)]
pub struct FunctionConfigurationProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`DataSourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-datasourcename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_source_name: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FunctionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-functionversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function_version: ::Value<String>,
    /// Property [`MaxBatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-maxbatchsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_batch_size: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RequestMappingTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-requestmappingtemplate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_mapping_template: Option<::Value<String>>,
    /// Property [`RequestMappingTemplateS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-requestmappingtemplates3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_mapping_template_s3_location: Option<::Value<String>>,
    /// Property [`ResponseMappingTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-responsemappingtemplate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_mapping_template: Option<::Value<String>>,
    /// Property [`ResponseMappingTemplateS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-responsemappingtemplates3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_mapping_template_s3_location: Option<::Value<String>>,
    /// Property [`SyncConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html#cfn-appsync-functionconfiguration-syncconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sync_config: Option<::Value<self::function_configuration::SyncConfig>>,
}

impl ::serde::Serialize for FunctionConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceName", &self.data_source_name)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionVersion", &self.function_version)?;
        if let Some(ref max_batch_size) = self.max_batch_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxBatchSize", max_batch_size)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref request_mapping_template) = self.request_mapping_template {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestMappingTemplate", request_mapping_template)?;
        }
        if let Some(ref request_mapping_template_s3_location) = self.request_mapping_template_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestMappingTemplateS3Location", request_mapping_template_s3_location)?;
        }
        if let Some(ref response_mapping_template) = self.response_mapping_template {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseMappingTemplate", response_mapping_template)?;
        }
        if let Some(ref response_mapping_template_s3_location) = self.response_mapping_template_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseMappingTemplateS3Location", response_mapping_template_s3_location)?;
        }
        if let Some(ref sync_config) = self.sync_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncConfig", sync_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FunctionConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FunctionConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FunctionConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FunctionConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut data_source_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut function_version: Option<::Value<String>> = None;
                let mut max_batch_size: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut request_mapping_template: Option<::Value<String>> = None;
                let mut request_mapping_template_s3_location: Option<::Value<String>> = None;
                let mut response_mapping_template: Option<::Value<String>> = None;
                let mut response_mapping_template_s3_location: Option<::Value<String>> = None;
                let mut sync_config: Option<::Value<self::function_configuration::SyncConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSourceName" => {
                            data_source_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FunctionVersion" => {
                            function_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxBatchSize" => {
                            max_batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequestMappingTemplate" => {
                            request_mapping_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequestMappingTemplateS3Location" => {
                            request_mapping_template_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseMappingTemplate" => {
                            response_mapping_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseMappingTemplateS3Location" => {
                            response_mapping_template_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SyncConfig" => {
                            sync_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FunctionConfigurationProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    data_source_name: data_source_name.ok_or(::serde::de::Error::missing_field("DataSourceName"))?,
                    description: description,
                    function_version: function_version.ok_or(::serde::de::Error::missing_field("FunctionVersion"))?,
                    max_batch_size: max_batch_size,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    request_mapping_template: request_mapping_template,
                    request_mapping_template_s3_location: request_mapping_template_s3_location,
                    response_mapping_template: response_mapping_template,
                    response_mapping_template_s3_location: response_mapping_template_s3_location,
                    sync_config: sync_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FunctionConfiguration {
    type Properties = FunctionConfigurationProperties;
    const TYPE: &'static str = "AWS::AppSync::FunctionConfiguration";
    fn properties(&self) -> &FunctionConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FunctionConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FunctionConfiguration {}

impl From<FunctionConfigurationProperties> for FunctionConfiguration {
    fn from(properties: FunctionConfigurationProperties) -> FunctionConfiguration {
        FunctionConfiguration { properties }
    }
}

/// The [`AWS::AppSync::GraphQLApi`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html) resource type.
#[derive(Debug, Default)]
pub struct GraphQLApi {
    properties: GraphQLApiProperties
}

/// Properties for the `GraphQLApi` resource.
#[derive(Debug, Default)]
pub struct GraphQLApiProperties {
    /// Property [`AdditionalAuthenticationProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-additionalauthenticationproviders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub additional_authentication_providers: Option<::Value<self::graph_ql_api::AdditionalAuthenticationProviders>>,
    /// Property [`AuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-authenticationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authentication_type: ::Value<String>,
    /// Property [`LambdaAuthorizerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-lambdaauthorizerconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lambda_authorizer_config: Option<::Value<self::graph_ql_api::LambdaAuthorizerConfig>>,
    /// Property [`LogConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-logconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_config: Option<::Value<self::graph_ql_api::LogConfig>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OpenIDConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-openidconnectconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub open_id_connect_config: Option<::Value<self::graph_ql_api::OpenIDConnectConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<self::graph_ql_api::Tags>>,
    /// Property [`UserPoolConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-userpoolconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_pool_config: Option<::Value<self::graph_ql_api::UserPoolConfig>>,
    /// Property [`XrayEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html#cfn-appsync-graphqlapi-xrayenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub xray_enabled: Option<::Value<bool>>,
}

impl ::serde::Serialize for GraphQLApiProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_authentication_providers) = self.additional_authentication_providers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalAuthenticationProviders", additional_authentication_providers)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationType", &self.authentication_type)?;
        if let Some(ref lambda_authorizer_config) = self.lambda_authorizer_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaAuthorizerConfig", lambda_authorizer_config)?;
        }
        if let Some(ref log_config) = self.log_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogConfig", log_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref open_id_connect_config) = self.open_id_connect_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenIDConnectConfig", open_id_connect_config)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref user_pool_config) = self.user_pool_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolConfig", user_pool_config)?;
        }
        if let Some(ref xray_enabled) = self.xray_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "XrayEnabled", xray_enabled)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GraphQLApiProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GraphQLApiProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GraphQLApiProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GraphQLApiProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_authentication_providers: Option<::Value<self::graph_ql_api::AdditionalAuthenticationProviders>> = None;
                let mut authentication_type: Option<::Value<String>> = None;
                let mut lambda_authorizer_config: Option<::Value<self::graph_ql_api::LambdaAuthorizerConfig>> = None;
                let mut log_config: Option<::Value<self::graph_ql_api::LogConfig>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut open_id_connect_config: Option<::Value<self::graph_ql_api::OpenIDConnectConfig>> = None;
                let mut tags: Option<::Value<self::graph_ql_api::Tags>> = None;
                let mut user_pool_config: Option<::Value<self::graph_ql_api::UserPoolConfig>> = None;
                let mut xray_enabled: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalAuthenticationProviders" => {
                            additional_authentication_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthenticationType" => {
                            authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LambdaAuthorizerConfig" => {
                            lambda_authorizer_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogConfig" => {
                            log_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OpenIDConnectConfig" => {
                            open_id_connect_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolConfig" => {
                            user_pool_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "XrayEnabled" => {
                            xray_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GraphQLApiProperties {
                    additional_authentication_providers: additional_authentication_providers,
                    authentication_type: authentication_type.ok_or(::serde::de::Error::missing_field("AuthenticationType"))?,
                    lambda_authorizer_config: lambda_authorizer_config,
                    log_config: log_config,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    open_id_connect_config: open_id_connect_config,
                    tags: tags,
                    user_pool_config: user_pool_config,
                    xray_enabled: xray_enabled,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GraphQLApi {
    type Properties = GraphQLApiProperties;
    const TYPE: &'static str = "AWS::AppSync::GraphQLApi";
    fn properties(&self) -> &GraphQLApiProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GraphQLApiProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GraphQLApi {}

impl From<GraphQLApiProperties> for GraphQLApi {
    fn from(properties: GraphQLApiProperties) -> GraphQLApi {
        GraphQLApi { properties }
    }
}

/// The [`AWS::AppSync::GraphQLSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlschema.html) resource type.
#[derive(Debug, Default)]
pub struct GraphQLSchema {
    properties: GraphQLSchemaProperties
}

/// Properties for the `GraphQLSchema` resource.
#[derive(Debug, Default)]
pub struct GraphQLSchemaProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlschema.html#cfn-appsync-graphqlschema-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlschema.html#cfn-appsync-graphqlschema-definition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub definition: Option<::Value<String>>,
    /// Property [`DefinitionS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlschema.html#cfn-appsync-graphqlschema-definitions3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub definition_s3_location: Option<::Value<String>>,
}

impl ::serde::Serialize for GraphQLSchemaProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref definition) = self.definition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Definition", definition)?;
        }
        if let Some(ref definition_s3_location) = self.definition_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefinitionS3Location", definition_s3_location)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GraphQLSchemaProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GraphQLSchemaProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GraphQLSchemaProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GraphQLSchemaProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut definition: Option<::Value<String>> = None;
                let mut definition_s3_location: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Definition" => {
                            definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefinitionS3Location" => {
                            definition_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GraphQLSchemaProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    definition: definition,
                    definition_s3_location: definition_s3_location,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GraphQLSchema {
    type Properties = GraphQLSchemaProperties;
    const TYPE: &'static str = "AWS::AppSync::GraphQLSchema";
    fn properties(&self) -> &GraphQLSchemaProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GraphQLSchemaProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GraphQLSchema {}

impl From<GraphQLSchemaProperties> for GraphQLSchema {
    fn from(properties: GraphQLSchemaProperties) -> GraphQLSchema {
        GraphQLSchema { properties }
    }
}

/// The [`AWS::AppSync::Resolver`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html) resource type.
#[derive(Debug, Default)]
pub struct Resolver {
    properties: ResolverProperties
}

/// Properties for the `Resolver` resource.
#[derive(Debug, Default)]
pub struct ResolverProperties {
    /// Property [`ApiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-apiid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub api_id: ::Value<String>,
    /// Property [`CachingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-cachingconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub caching_config: Option<::Value<self::resolver::CachingConfig>>,
    /// Property [`DataSourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-datasourcename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_source_name: Option<::Value<String>>,
    /// Property [`FieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-fieldname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub field_name: ::Value<String>,
    /// Property [`Kind`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-kind).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kind: Option<::Value<String>>,
    /// Property [`MaxBatchSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-maxbatchsize).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_batch_size: Option<::Value<u32>>,
    /// Property [`PipelineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-pipelineconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pipeline_config: Option<::Value<self::resolver::PipelineConfig>>,
    /// Property [`RequestMappingTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-requestmappingtemplate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_mapping_template: Option<::Value<String>>,
    /// Property [`RequestMappingTemplateS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-requestmappingtemplates3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub request_mapping_template_s3_location: Option<::Value<String>>,
    /// Property [`ResponseMappingTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-responsemappingtemplate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_mapping_template: Option<::Value<String>>,
    /// Property [`ResponseMappingTemplateS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-responsemappingtemplates3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub response_mapping_template_s3_location: Option<::Value<String>>,
    /// Property [`SyncConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-syncconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sync_config: Option<::Value<self::resolver::SyncConfig>>,
    /// Property [`TypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html#cfn-appsync-resolver-typename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub type_name: ::Value<String>,
}

impl ::serde::Serialize for ResolverProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiId", &self.api_id)?;
        if let Some(ref caching_config) = self.caching_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachingConfig", caching_config)?;
        }
        if let Some(ref data_source_name) = self.data_source_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceName", data_source_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldName", &self.field_name)?;
        if let Some(ref kind) = self.kind {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Kind", kind)?;
        }
        if let Some(ref max_batch_size) = self.max_batch_size {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxBatchSize", max_batch_size)?;
        }
        if let Some(ref pipeline_config) = self.pipeline_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineConfig", pipeline_config)?;
        }
        if let Some(ref request_mapping_template) = self.request_mapping_template {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestMappingTemplate", request_mapping_template)?;
        }
        if let Some(ref request_mapping_template_s3_location) = self.request_mapping_template_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestMappingTemplateS3Location", request_mapping_template_s3_location)?;
        }
        if let Some(ref response_mapping_template) = self.response_mapping_template {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseMappingTemplate", response_mapping_template)?;
        }
        if let Some(ref response_mapping_template_s3_location) = self.response_mapping_template_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseMappingTemplateS3Location", response_mapping_template_s3_location)?;
        }
        if let Some(ref sync_config) = self.sync_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SyncConfig", sync_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TypeName", &self.type_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResolverProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResolverProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResolverProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResolverProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut api_id: Option<::Value<String>> = None;
                let mut caching_config: Option<::Value<self::resolver::CachingConfig>> = None;
                let mut data_source_name: Option<::Value<String>> = None;
                let mut field_name: Option<::Value<String>> = None;
                let mut kind: Option<::Value<String>> = None;
                let mut max_batch_size: Option<::Value<u32>> = None;
                let mut pipeline_config: Option<::Value<self::resolver::PipelineConfig>> = None;
                let mut request_mapping_template: Option<::Value<String>> = None;
                let mut request_mapping_template_s3_location: Option<::Value<String>> = None;
                let mut response_mapping_template: Option<::Value<String>> = None;
                let mut response_mapping_template_s3_location: Option<::Value<String>> = None;
                let mut sync_config: Option<::Value<self::resolver::SyncConfig>> = None;
                let mut type_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApiId" => {
                            api_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CachingConfig" => {
                            caching_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSourceName" => {
                            data_source_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FieldName" => {
                            field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Kind" => {
                            kind = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxBatchSize" => {
                            max_batch_size = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineConfig" => {
                            pipeline_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequestMappingTemplate" => {
                            request_mapping_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequestMappingTemplateS3Location" => {
                            request_mapping_template_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseMappingTemplate" => {
                            response_mapping_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResponseMappingTemplateS3Location" => {
                            response_mapping_template_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SyncConfig" => {
                            sync_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TypeName" => {
                            type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResolverProperties {
                    api_id: api_id.ok_or(::serde::de::Error::missing_field("ApiId"))?,
                    caching_config: caching_config,
                    data_source_name: data_source_name,
                    field_name: field_name.ok_or(::serde::de::Error::missing_field("FieldName"))?,
                    kind: kind,
                    max_batch_size: max_batch_size,
                    pipeline_config: pipeline_config,
                    request_mapping_template: request_mapping_template,
                    request_mapping_template_s3_location: request_mapping_template_s3_location,
                    response_mapping_template: response_mapping_template,
                    response_mapping_template_s3_location: response_mapping_template_s3_location,
                    sync_config: sync_config,
                    type_name: type_name.ok_or(::serde::de::Error::missing_field("TypeName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Resolver {
    type Properties = ResolverProperties;
    const TYPE: &'static str = "AWS::AppSync::Resolver";
    fn properties(&self) -> &ResolverProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResolverProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Resolver {}

impl From<ResolverProperties> for Resolver {
    fn from(properties: ResolverProperties) -> Resolver {
        Resolver { properties }
    }
}

pub mod data_source {
    //! Property types for the `DataSource` resource.

    /// The [`AWS::AppSync::DataSource.AuthorizationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-authorizationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthorizationConfig {
        /// Property [`AuthorizationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-authorizationconfig.html#cfn-appsync-datasource-authorizationconfig-authorizationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authorization_type: ::Value<String>,
        /// Property [`AwsIamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-authorizationconfig.html#cfn-appsync-datasource-authorizationconfig-awsiamconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_iam_config: Option<::Value<AwsIamConfig>>,
    }

    impl ::codec::SerializeValue for AuthorizationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationType", &self.authorization_type)?;
            if let Some(ref aws_iam_config) = self.aws_iam_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsIamConfig", aws_iam_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthorizationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthorizationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthorizationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthorizationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authorization_type: Option<::Value<String>> = None;
                    let mut aws_iam_config: Option<::Value<AwsIamConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthorizationType" => {
                                authorization_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AwsIamConfig" => {
                                aws_iam_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthorizationConfig {
                        authorization_type: authorization_type.ok_or(::serde::de::Error::missing_field("AuthorizationType"))?,
                        aws_iam_config: aws_iam_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::DataSource.AwsIamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-awsiamconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AwsIamConfig {
        /// Property [`SigningRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-awsiamconfig.html#cfn-appsync-datasource-awsiamconfig-signingregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub signing_region: Option<::Value<String>>,
        /// Property [`SigningServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-awsiamconfig.html#cfn-appsync-datasource-awsiamconfig-signingservicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub signing_service_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AwsIamConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref signing_region) = self.signing_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigningRegion", signing_region)?;
            }
            if let Some(ref signing_service_name) = self.signing_service_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigningServiceName", signing_service_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AwsIamConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AwsIamConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AwsIamConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AwsIamConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut signing_region: Option<::Value<String>> = None;
                    let mut signing_service_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SigningRegion" => {
                                signing_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SigningServiceName" => {
                                signing_service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AwsIamConfig {
                        signing_region: signing_region,
                        signing_service_name: signing_service_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::DataSource.DeltaSyncConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-deltasyncconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DeltaSyncConfig {
        /// Property [`BaseTableTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-deltasyncconfig.html#cfn-appsync-datasource-deltasyncconfig-basetablettl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_table_ttl: ::Value<String>,
        /// Property [`DeltaSyncTableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-deltasyncconfig.html#cfn-appsync-datasource-deltasyncconfig-deltasynctablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delta_sync_table_name: ::Value<String>,
        /// Property [`DeltaSyncTableTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-deltasyncconfig.html#cfn-appsync-datasource-deltasyncconfig-deltasynctablettl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delta_sync_table_ttl: ::Value<String>,
    }

    impl ::codec::SerializeValue for DeltaSyncConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseTableTTL", &self.base_table_ttl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeltaSyncTableName", &self.delta_sync_table_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeltaSyncTableTTL", &self.delta_sync_table_ttl)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeltaSyncConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeltaSyncConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeltaSyncConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeltaSyncConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base_table_ttl: Option<::Value<String>> = None;
                    let mut delta_sync_table_name: Option<::Value<String>> = None;
                    let mut delta_sync_table_ttl: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseTableTTL" => {
                                base_table_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeltaSyncTableName" => {
                                delta_sync_table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeltaSyncTableTTL" => {
                                delta_sync_table_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeltaSyncConfig {
                        base_table_ttl: base_table_ttl.ok_or(::serde::de::Error::missing_field("BaseTableTTL"))?,
                        delta_sync_table_name: delta_sync_table_name.ok_or(::serde::de::Error::missing_field("DeltaSyncTableName"))?,
                        delta_sync_table_ttl: delta_sync_table_ttl.ok_or(::serde::de::Error::missing_field("DeltaSyncTableTTL"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::DataSource.DynamoDBConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-dynamodbconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DynamoDBConfig {
        /// Property [`AwsRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-dynamodbconfig.html#cfn-appsync-datasource-dynamodbconfig-awsregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_region: ::Value<String>,
        /// Property [`DeltaSyncConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-dynamodbconfig.html#cfn-appsync-datasource-dynamodbconfig-deltasyncconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delta_sync_config: Option<::Value<DeltaSyncConfig>>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-dynamodbconfig.html#cfn-appsync-datasource-dynamodbconfig-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
        /// Property [`UseCallerCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-dynamodbconfig.html#cfn-appsync-datasource-dynamodbconfig-usecallercredentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_caller_credentials: Option<::Value<bool>>,
        /// Property [`Versioned`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-dynamodbconfig.html#cfn-appsync-datasource-dynamodbconfig-versioned).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub versioned: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for DynamoDBConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsRegion", &self.aws_region)?;
            if let Some(ref delta_sync_config) = self.delta_sync_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeltaSyncConfig", delta_sync_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            if let Some(ref use_caller_credentials) = self.use_caller_credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseCallerCredentials", use_caller_credentials)?;
            }
            if let Some(ref versioned) = self.versioned {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Versioned", versioned)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynamoDBConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamoDBConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamoDBConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamoDBConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_region: Option<::Value<String>> = None;
                    let mut delta_sync_config: Option<::Value<DeltaSyncConfig>> = None;
                    let mut table_name: Option<::Value<String>> = None;
                    let mut use_caller_credentials: Option<::Value<bool>> = None;
                    let mut versioned: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsRegion" => {
                                aws_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeltaSyncConfig" => {
                                delta_sync_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseCallerCredentials" => {
                                use_caller_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Versioned" => {
                                versioned = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamoDBConfig {
                        aws_region: aws_region.ok_or(::serde::de::Error::missing_field("AwsRegion"))?,
                        delta_sync_config: delta_sync_config,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                        use_caller_credentials: use_caller_credentials,
                        versioned: versioned,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::DataSource.ElasticsearchConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-elasticsearchconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ElasticsearchConfig {
        /// Property [`AwsRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-elasticsearchconfig.html#cfn-appsync-datasource-elasticsearchconfig-awsregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_region: ::Value<String>,
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-elasticsearchconfig.html#cfn-appsync-datasource-elasticsearchconfig-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: ::Value<String>,
    }

    impl ::codec::SerializeValue for ElasticsearchConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsRegion", &self.aws_region)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", &self.endpoint)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticsearchConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticsearchConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticsearchConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticsearchConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_region: Option<::Value<String>> = None;
                    let mut endpoint: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsRegion" => {
                                aws_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticsearchConfig {
                        aws_region: aws_region.ok_or(::serde::de::Error::missing_field("AwsRegion"))?,
                        endpoint: endpoint.ok_or(::serde::de::Error::missing_field("Endpoint"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::DataSource.HttpConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-httpconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpConfig {
        /// Property [`AuthorizationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-httpconfig.html#cfn-appsync-datasource-httpconfig-authorizationconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authorization_config: Option<::Value<AuthorizationConfig>>,
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-httpconfig.html#cfn-appsync-datasource-httpconfig-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: ::Value<String>,
    }

    impl ::codec::SerializeValue for HttpConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authorization_config) = self.authorization_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationConfig", authorization_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", &self.endpoint)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authorization_config: Option<::Value<AuthorizationConfig>> = None;
                    let mut endpoint: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthorizationConfig" => {
                                authorization_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpConfig {
                        authorization_config: authorization_config,
                        endpoint: endpoint.ok_or(::serde::de::Error::missing_field("Endpoint"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::DataSource.LambdaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-lambdaconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaConfig {
        /// Property [`LambdaFunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-lambdaconfig.html#cfn-appsync-datasource-lambdaconfig-lambdafunctionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_function_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for LambdaConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionArn", &self.lambda_function_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_function_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaFunctionArn" => {
                                lambda_function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaConfig {
                        lambda_function_arn: lambda_function_arn.ok_or(::serde::de::Error::missing_field("LambdaFunctionArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::DataSource.OpenSearchServiceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-opensearchserviceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct OpenSearchServiceConfig {
        /// Property [`AwsRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-opensearchserviceconfig.html#cfn-appsync-datasource-opensearchserviceconfig-awsregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_region: ::Value<String>,
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-opensearchserviceconfig.html#cfn-appsync-datasource-opensearchserviceconfig-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: ::Value<String>,
    }

    impl ::codec::SerializeValue for OpenSearchServiceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsRegion", &self.aws_region)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", &self.endpoint)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OpenSearchServiceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OpenSearchServiceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OpenSearchServiceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OpenSearchServiceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_region: Option<::Value<String>> = None;
                    let mut endpoint: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsRegion" => {
                                aws_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OpenSearchServiceConfig {
                        aws_region: aws_region.ok_or(::serde::de::Error::missing_field("AwsRegion"))?,
                        endpoint: endpoint.ok_or(::serde::de::Error::missing_field("Endpoint"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::DataSource.RdsHttpEndpointConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-rdshttpendpointconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RdsHttpEndpointConfig {
        /// Property [`AwsRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-rdshttpendpointconfig.html#cfn-appsync-datasource-rdshttpendpointconfig-awsregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_region: ::Value<String>,
        /// Property [`AwsSecretStoreArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-rdshttpendpointconfig.html#cfn-appsync-datasource-rdshttpendpointconfig-awssecretstorearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_secret_store_arn: ::Value<String>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-rdshttpendpointconfig.html#cfn-appsync-datasource-rdshttpendpointconfig-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`DbClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-rdshttpendpointconfig.html#cfn-appsync-datasource-rdshttpendpointconfig-dbclusteridentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub db_cluster_identifier: ::Value<String>,
        /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-rdshttpendpointconfig.html#cfn-appsync-datasource-rdshttpendpointconfig-schema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RdsHttpEndpointConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsRegion", &self.aws_region)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsSecretStoreArn", &self.aws_secret_store_arn)?;
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DbClusterIdentifier", &self.db_cluster_identifier)?;
            if let Some(ref schema) = self.schema {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", schema)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RdsHttpEndpointConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RdsHttpEndpointConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RdsHttpEndpointConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RdsHttpEndpointConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_region: Option<::Value<String>> = None;
                    let mut aws_secret_store_arn: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut db_cluster_identifier: Option<::Value<String>> = None;
                    let mut schema: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsRegion" => {
                                aws_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AwsSecretStoreArn" => {
                                aws_secret_store_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DbClusterIdentifier" => {
                                db_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Schema" => {
                                schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RdsHttpEndpointConfig {
                        aws_region: aws_region.ok_or(::serde::de::Error::missing_field("AwsRegion"))?,
                        aws_secret_store_arn: aws_secret_store_arn.ok_or(::serde::de::Error::missing_field("AwsSecretStoreArn"))?,
                        database_name: database_name,
                        db_cluster_identifier: db_cluster_identifier.ok_or(::serde::de::Error::missing_field("DbClusterIdentifier"))?,
                        schema: schema,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::DataSource.RelationalDatabaseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-relationaldatabaseconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RelationalDatabaseConfig {
        /// Property [`RdsHttpEndpointConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-relationaldatabaseconfig.html#cfn-appsync-datasource-relationaldatabaseconfig-rdshttpendpointconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rds_http_endpoint_config: Option<::Value<RdsHttpEndpointConfig>>,
        /// Property [`RelationalDatabaseSourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-relationaldatabaseconfig.html#cfn-appsync-datasource-relationaldatabaseconfig-relationaldatabasesourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relational_database_source_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RelationalDatabaseConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref rds_http_endpoint_config) = self.rds_http_endpoint_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RdsHttpEndpointConfig", rds_http_endpoint_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationalDatabaseSourceType", &self.relational_database_source_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RelationalDatabaseConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RelationalDatabaseConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RelationalDatabaseConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RelationalDatabaseConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rds_http_endpoint_config: Option<::Value<RdsHttpEndpointConfig>> = None;
                    let mut relational_database_source_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RdsHttpEndpointConfig" => {
                                rds_http_endpoint_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RelationalDatabaseSourceType" => {
                                relational_database_source_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RelationalDatabaseConfig {
                        rds_http_endpoint_config: rds_http_endpoint_config,
                        relational_database_source_type: relational_database_source_type.ok_or(::serde::de::Error::missing_field("RelationalDatabaseSourceType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod function_configuration {
    //! Property types for the `FunctionConfiguration` resource.

    /// The [`AWS::AppSync::FunctionConfiguration.LambdaConflictHandlerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-functionconfiguration-lambdaconflicthandlerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaConflictHandlerConfig {
        /// Property [`LambdaConflictHandlerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-functionconfiguration-lambdaconflicthandlerconfig.html#cfn-appsync-functionconfiguration-lambdaconflicthandlerconfig-lambdaconflicthandlerarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_conflict_handler_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaConflictHandlerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lambda_conflict_handler_arn) = self.lambda_conflict_handler_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaConflictHandlerArn", lambda_conflict_handler_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaConflictHandlerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaConflictHandlerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaConflictHandlerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaConflictHandlerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_conflict_handler_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaConflictHandlerArn" => {
                                lambda_conflict_handler_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaConflictHandlerConfig {
                        lambda_conflict_handler_arn: lambda_conflict_handler_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::FunctionConfiguration.SyncConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-functionconfiguration-syncconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SyncConfig {
        /// Property [`ConflictDetection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-functionconfiguration-syncconfig.html#cfn-appsync-functionconfiguration-syncconfig-conflictdetection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conflict_detection: ::Value<String>,
        /// Property [`ConflictHandler`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-functionconfiguration-syncconfig.html#cfn-appsync-functionconfiguration-syncconfig-conflicthandler).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conflict_handler: Option<::Value<String>>,
        /// Property [`LambdaConflictHandlerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-functionconfiguration-syncconfig.html#cfn-appsync-functionconfiguration-syncconfig-lambdaconflicthandlerconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_conflict_handler_config: Option<::Value<LambdaConflictHandlerConfig>>,
    }

    impl ::codec::SerializeValue for SyncConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConflictDetection", &self.conflict_detection)?;
            if let Some(ref conflict_handler) = self.conflict_handler {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConflictHandler", conflict_handler)?;
            }
            if let Some(ref lambda_conflict_handler_config) = self.lambda_conflict_handler_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaConflictHandlerConfig", lambda_conflict_handler_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SyncConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SyncConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SyncConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SyncConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut conflict_detection: Option<::Value<String>> = None;
                    let mut conflict_handler: Option<::Value<String>> = None;
                    let mut lambda_conflict_handler_config: Option<::Value<LambdaConflictHandlerConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConflictDetection" => {
                                conflict_detection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConflictHandler" => {
                                conflict_handler = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaConflictHandlerConfig" => {
                                lambda_conflict_handler_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SyncConfig {
                        conflict_detection: conflict_detection.ok_or(::serde::de::Error::missing_field("ConflictDetection"))?,
                        conflict_handler: conflict_handler,
                        lambda_conflict_handler_config: lambda_conflict_handler_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod graph_ql_api {
    //! Property types for the `GraphQLApi` resource.

    /// The [`AWS::AppSync::GraphQLApi.AdditionalAuthenticationProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-additionalauthenticationprovider.html) property type.
    #[derive(Debug, Default)]
    pub struct AdditionalAuthenticationProvider {
        /// Property [`AuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-additionalauthenticationprovider.html#cfn-appsync-graphqlapi-additionalauthenticationprovider-authenticationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authentication_type: ::Value<String>,
        /// Property [`LambdaAuthorizerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-additionalauthenticationprovider.html#cfn-appsync-graphqlapi-additionalauthenticationprovider-lambdaauthorizerconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_authorizer_config: Option<::Value<LambdaAuthorizerConfig>>,
        /// Property [`OpenIDConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-additionalauthenticationprovider.html#cfn-appsync-graphqlapi-additionalauthenticationprovider-openidconnectconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub open_id_connect_config: Option<::Value<OpenIDConnectConfig>>,
        /// Property [`UserPoolConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-additionalauthenticationprovider.html#cfn-appsync-graphqlapi-additionalauthenticationprovider-userpoolconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_config: Option<::Value<CognitoUserPoolConfig>>,
    }

    impl ::codec::SerializeValue for AdditionalAuthenticationProvider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationType", &self.authentication_type)?;
            if let Some(ref lambda_authorizer_config) = self.lambda_authorizer_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaAuthorizerConfig", lambda_authorizer_config)?;
            }
            if let Some(ref open_id_connect_config) = self.open_id_connect_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenIDConnectConfig", open_id_connect_config)?;
            }
            if let Some(ref user_pool_config) = self.user_pool_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolConfig", user_pool_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdditionalAuthenticationProvider {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdditionalAuthenticationProvider, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdditionalAuthenticationProvider;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdditionalAuthenticationProvider")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authentication_type: Option<::Value<String>> = None;
                    let mut lambda_authorizer_config: Option<::Value<LambdaAuthorizerConfig>> = None;
                    let mut open_id_connect_config: Option<::Value<OpenIDConnectConfig>> = None;
                    let mut user_pool_config: Option<::Value<CognitoUserPoolConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticationType" => {
                                authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaAuthorizerConfig" => {
                                lambda_authorizer_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OpenIDConnectConfig" => {
                                open_id_connect_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolConfig" => {
                                user_pool_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdditionalAuthenticationProvider {
                        authentication_type: authentication_type.ok_or(::serde::de::Error::missing_field("AuthenticationType"))?,
                        lambda_authorizer_config: lambda_authorizer_config,
                        open_id_connect_config: open_id_connect_config,
                        user_pool_config: user_pool_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::GraphQLApi.AdditionalAuthenticationProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-additionalauthenticationproviders.html) property type.
    #[derive(Debug, Default)]
    pub struct AdditionalAuthenticationProviders {
    }

    impl ::codec::SerializeValue for AdditionalAuthenticationProviders {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdditionalAuthenticationProviders {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdditionalAuthenticationProviders, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdditionalAuthenticationProviders;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdditionalAuthenticationProviders")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(AdditionalAuthenticationProviders {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::GraphQLApi.CognitoUserPoolConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-cognitouserpoolconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CognitoUserPoolConfig {
        /// Property [`AppIdClientRegex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-cognitouserpoolconfig.html#cfn-appsync-graphqlapi-cognitouserpoolconfig-appidclientregex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_id_client_regex: Option<::Value<String>>,
        /// Property [`AwsRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-cognitouserpoolconfig.html#cfn-appsync-graphqlapi-cognitouserpoolconfig-awsregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_region: Option<::Value<String>>,
        /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-cognitouserpoolconfig.html#cfn-appsync-graphqlapi-cognitouserpoolconfig-userpoolid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CognitoUserPoolConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref app_id_client_regex) = self.app_id_client_regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppIdClientRegex", app_id_client_regex)?;
            }
            if let Some(ref aws_region) = self.aws_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsRegion", aws_region)?;
            }
            if let Some(ref user_pool_id) = self.user_pool_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", user_pool_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CognitoUserPoolConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CognitoUserPoolConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CognitoUserPoolConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CognitoUserPoolConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_id_client_regex: Option<::Value<String>> = None;
                    let mut aws_region: Option<::Value<String>> = None;
                    let mut user_pool_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppIdClientRegex" => {
                                app_id_client_regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AwsRegion" => {
                                aws_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolId" => {
                                user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CognitoUserPoolConfig {
                        app_id_client_regex: app_id_client_regex,
                        aws_region: aws_region,
                        user_pool_id: user_pool_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::GraphQLApi.LambdaAuthorizerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-lambdaauthorizerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaAuthorizerConfig {
        /// Property [`AuthorizerResultTtlInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-lambdaauthorizerconfig.html#cfn-appsync-graphqlapi-lambdaauthorizerconfig-authorizerresultttlinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authorizer_result_ttl_in_seconds: Option<::Value<f64>>,
        /// Property [`AuthorizerUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-lambdaauthorizerconfig.html#cfn-appsync-graphqlapi-lambdaauthorizerconfig-authorizeruri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authorizer_uri: Option<::Value<String>>,
        /// Property [`IdentityValidationExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-lambdaauthorizerconfig.html#cfn-appsync-graphqlapi-lambdaauthorizerconfig-identityvalidationexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identity_validation_expression: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaAuthorizerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authorizer_result_ttl_in_seconds) = self.authorizer_result_ttl_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerResultTtlInSeconds", authorizer_result_ttl_in_seconds)?;
            }
            if let Some(ref authorizer_uri) = self.authorizer_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerUri", authorizer_uri)?;
            }
            if let Some(ref identity_validation_expression) = self.identity_validation_expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityValidationExpression", identity_validation_expression)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaAuthorizerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaAuthorizerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaAuthorizerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaAuthorizerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authorizer_result_ttl_in_seconds: Option<::Value<f64>> = None;
                    let mut authorizer_uri: Option<::Value<String>> = None;
                    let mut identity_validation_expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthorizerResultTtlInSeconds" => {
                                authorizer_result_ttl_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthorizerUri" => {
                                authorizer_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdentityValidationExpression" => {
                                identity_validation_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaAuthorizerConfig {
                        authorizer_result_ttl_in_seconds: authorizer_result_ttl_in_seconds,
                        authorizer_uri: authorizer_uri,
                        identity_validation_expression: identity_validation_expression,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::GraphQLApi.LogConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-logconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LogConfig {
        /// Property [`CloudWatchLogsRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-logconfig.html#cfn-appsync-graphqlapi-logconfig-cloudwatchlogsrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_logs_role_arn: Option<::Value<String>>,
        /// Property [`ExcludeVerboseContent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-logconfig.html#cfn-appsync-graphqlapi-logconfig-excludeverbosecontent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_verbose_content: Option<::Value<bool>>,
        /// Property [`FieldLogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-logconfig.html#cfn-appsync-graphqlapi-logconfig-fieldloglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_log_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LogConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_logs_role_arn) = self.cloud_watch_logs_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogsRoleArn", cloud_watch_logs_role_arn)?;
            }
            if let Some(ref exclude_verbose_content) = self.exclude_verbose_content {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeVerboseContent", exclude_verbose_content)?;
            }
            if let Some(ref field_log_level) = self.field_log_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldLogLevel", field_log_level)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logs_role_arn: Option<::Value<String>> = None;
                    let mut exclude_verbose_content: Option<::Value<bool>> = None;
                    let mut field_log_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogsRoleArn" => {
                                cloud_watch_logs_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeVerboseContent" => {
                                exclude_verbose_content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FieldLogLevel" => {
                                field_log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogConfig {
                        cloud_watch_logs_role_arn: cloud_watch_logs_role_arn,
                        exclude_verbose_content: exclude_verbose_content,
                        field_log_level: field_log_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::GraphQLApi.OpenIDConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-openidconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct OpenIDConnectConfig {
        /// Property [`AuthTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-openidconnectconfig.html#cfn-appsync-graphqlapi-openidconnectconfig-authttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_ttl: Option<::Value<f64>>,
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-openidconnectconfig.html#cfn-appsync-graphqlapi-openidconnectconfig-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: Option<::Value<String>>,
        /// Property [`IatTTL`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-openidconnectconfig.html#cfn-appsync-graphqlapi-openidconnectconfig-iatttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iat_ttl: Option<::Value<f64>>,
        /// Property [`Issuer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-openidconnectconfig.html#cfn-appsync-graphqlapi-openidconnectconfig-issuer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub issuer: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OpenIDConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auth_ttl) = self.auth_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthTTL", auth_ttl)?;
            }
            if let Some(ref client_id) = self.client_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", client_id)?;
            }
            if let Some(ref iat_ttl) = self.iat_ttl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IatTTL", iat_ttl)?;
            }
            if let Some(ref issuer) = self.issuer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Issuer", issuer)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OpenIDConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OpenIDConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OpenIDConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OpenIDConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auth_ttl: Option<::Value<f64>> = None;
                    let mut client_id: Option<::Value<String>> = None;
                    let mut iat_ttl: Option<::Value<f64>> = None;
                    let mut issuer: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthTTL" => {
                                auth_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IatTTL" => {
                                iat_ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Issuer" => {
                                issuer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OpenIDConnectConfig {
                        auth_ttl: auth_ttl,
                        client_id: client_id,
                        iat_ttl: iat_ttl,
                        issuer: issuer,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::GraphQLApi.Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-tags.html) property type.
    #[derive(Debug, Default)]
    pub struct Tags {
    }

    impl ::codec::SerializeValue for Tags {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Tags {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Tags, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Tags;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Tags")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Tags {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::GraphQLApi.UserPoolConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-userpoolconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UserPoolConfig {
        /// Property [`AppIdClientRegex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-userpoolconfig.html#cfn-appsync-graphqlapi-userpoolconfig-appidclientregex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_id_client_regex: Option<::Value<String>>,
        /// Property [`AwsRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-userpoolconfig.html#cfn-appsync-graphqlapi-userpoolconfig-awsregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_region: Option<::Value<String>>,
        /// Property [`DefaultAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-userpoolconfig.html#cfn-appsync-graphqlapi-userpoolconfig-defaultaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_action: Option<::Value<String>>,
        /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-userpoolconfig.html#cfn-appsync-graphqlapi-userpoolconfig-userpoolid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UserPoolConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref app_id_client_regex) = self.app_id_client_regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppIdClientRegex", app_id_client_regex)?;
            }
            if let Some(ref aws_region) = self.aws_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsRegion", aws_region)?;
            }
            if let Some(ref default_action) = self.default_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAction", default_action)?;
            }
            if let Some(ref user_pool_id) = self.user_pool_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", user_pool_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserPoolConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserPoolConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserPoolConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_id_client_regex: Option<::Value<String>> = None;
                    let mut aws_region: Option<::Value<String>> = None;
                    let mut default_action: Option<::Value<String>> = None;
                    let mut user_pool_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppIdClientRegex" => {
                                app_id_client_regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AwsRegion" => {
                                aws_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultAction" => {
                                default_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolId" => {
                                user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserPoolConfig {
                        app_id_client_regex: app_id_client_regex,
                        aws_region: aws_region,
                        default_action: default_action,
                        user_pool_id: user_pool_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod resolver {
    //! Property types for the `Resolver` resource.

    /// The [`AWS::AppSync::Resolver.CachingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-cachingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct CachingConfig {
        /// Property [`CachingKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-cachingconfig.html#cfn-appsync-resolver-cachingconfig-cachingkeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub caching_keys: Option<::ValueList<String>>,
        /// Property [`Ttl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-cachingconfig.html#cfn-appsync-resolver-cachingconfig-ttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ttl: ::Value<f64>,
    }

    impl ::codec::SerializeValue for CachingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref caching_keys) = self.caching_keys {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CachingKeys", caching_keys)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ttl", &self.ttl)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CachingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CachingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CachingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CachingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut caching_keys: Option<::ValueList<String>> = None;
                    let mut ttl: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CachingKeys" => {
                                caching_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ttl" => {
                                ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CachingConfig {
                        caching_keys: caching_keys,
                        ttl: ttl.ok_or(::serde::de::Error::missing_field("Ttl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::Resolver.LambdaConflictHandlerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-lambdaconflicthandlerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaConflictHandlerConfig {
        /// Property [`LambdaConflictHandlerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-lambdaconflicthandlerconfig.html#cfn-appsync-resolver-lambdaconflicthandlerconfig-lambdaconflicthandlerarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_conflict_handler_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaConflictHandlerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lambda_conflict_handler_arn) = self.lambda_conflict_handler_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaConflictHandlerArn", lambda_conflict_handler_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaConflictHandlerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaConflictHandlerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaConflictHandlerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaConflictHandlerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_conflict_handler_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaConflictHandlerArn" => {
                                lambda_conflict_handler_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaConflictHandlerConfig {
                        lambda_conflict_handler_arn: lambda_conflict_handler_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::Resolver.PipelineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-pipelineconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PipelineConfig {
        /// Property [`Functions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-pipelineconfig.html#cfn-appsync-resolver-pipelineconfig-functions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub functions: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PipelineConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref functions) = self.functions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Functions", functions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipelineConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipelineConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipelineConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipelineConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut functions: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Functions" => {
                                functions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipelineConfig {
                        functions: functions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppSync::Resolver.SyncConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-syncconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SyncConfig {
        /// Property [`ConflictDetection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-syncconfig.html#cfn-appsync-resolver-syncconfig-conflictdetection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conflict_detection: ::Value<String>,
        /// Property [`ConflictHandler`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-syncconfig.html#cfn-appsync-resolver-syncconfig-conflicthandler).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conflict_handler: Option<::Value<String>>,
        /// Property [`LambdaConflictHandlerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-syncconfig.html#cfn-appsync-resolver-syncconfig-lambdaconflicthandlerconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_conflict_handler_config: Option<::Value<LambdaConflictHandlerConfig>>,
    }

    impl ::codec::SerializeValue for SyncConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConflictDetection", &self.conflict_detection)?;
            if let Some(ref conflict_handler) = self.conflict_handler {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConflictHandler", conflict_handler)?;
            }
            if let Some(ref lambda_conflict_handler_config) = self.lambda_conflict_handler_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaConflictHandlerConfig", lambda_conflict_handler_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SyncConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SyncConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SyncConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SyncConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut conflict_detection: Option<::Value<String>> = None;
                    let mut conflict_handler: Option<::Value<String>> = None;
                    let mut lambda_conflict_handler_config: Option<::Value<LambdaConflictHandlerConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConflictDetection" => {
                                conflict_detection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConflictHandler" => {
                                conflict_handler = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaConflictHandlerConfig" => {
                                lambda_conflict_handler_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SyncConfig {
                        conflict_detection: conflict_detection.ok_or(::serde::de::Error::missing_field("ConflictDetection"))?,
                        conflict_handler: conflict_handler,
                        lambda_conflict_handler_config: lambda_conflict_handler_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
