//! Types for the `AppRunner` service.

/// The [`AWS::AppRunner::Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-service.html) resource type.
#[derive(Debug, Default)]
pub struct Service {
    properties: ServiceProperties
}

/// Properties for the `Service` resource.
#[derive(Debug, Default)]
pub struct ServiceProperties {
    /// Property [`AutoScalingConfigurationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-service.html#cfn-apprunner-service-autoscalingconfigurationarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_scaling_configuration_arn: Option<::Value<String>>,
    /// Property [`EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-service.html#cfn-apprunner-service-encryptionconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub encryption_configuration: Option<::Value<self::service::EncryptionConfiguration>>,
    /// Property [`HealthCheckConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-service.html#cfn-apprunner-service-healthcheckconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_configuration: Option<::Value<self::service::HealthCheckConfiguration>>,
    /// Property [`InstanceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-service.html#cfn-apprunner-service-instanceconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_configuration: Option<::Value<self::service::InstanceConfiguration>>,
    /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-service.html#cfn-apprunner-service-servicename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_name: Option<::Value<String>>,
    /// Property [`SourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-service.html#cfn-apprunner-service-sourceconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_configuration: ::Value<self::service::SourceConfiguration>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apprunner-service.html#cfn-apprunner-service-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ServiceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_scaling_configuration_arn) = self.auto_scaling_configuration_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingConfigurationArn", auto_scaling_configuration_arn)?;
        }
        if let Some(ref encryption_configuration) = self.encryption_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", encryption_configuration)?;
        }
        if let Some(ref health_check_configuration) = self.health_check_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckConfiguration", health_check_configuration)?;
        }
        if let Some(ref instance_configuration) = self.instance_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceConfiguration", instance_configuration)?;
        }
        if let Some(ref service_name) = self.service_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", service_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceConfiguration", &self.source_configuration)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServiceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServiceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_configuration_arn: Option<::Value<String>> = None;
                let mut encryption_configuration: Option<::Value<self::service::EncryptionConfiguration>> = None;
                let mut health_check_configuration: Option<::Value<self::service::HealthCheckConfiguration>> = None;
                let mut instance_configuration: Option<::Value<self::service::InstanceConfiguration>> = None;
                let mut service_name: Option<::Value<String>> = None;
                let mut source_configuration: Option<::Value<self::service::SourceConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingConfigurationArn" => {
                            auto_scaling_configuration_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionConfiguration" => {
                            encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckConfiguration" => {
                            health_check_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceConfiguration" => {
                            instance_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceName" => {
                            service_name = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ServiceProperties {
                    auto_scaling_configuration_arn: auto_scaling_configuration_arn,
                    encryption_configuration: encryption_configuration,
                    health_check_configuration: health_check_configuration,
                    instance_configuration: instance_configuration,
                    service_name: service_name,
                    source_configuration: source_configuration.ok_or(::serde::de::Error::missing_field("SourceConfiguration"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Service {
    type Properties = ServiceProperties;
    const TYPE: &'static str = "AWS::AppRunner::Service";
    fn properties(&self) -> &ServiceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServiceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Service {}

impl From<ServiceProperties> for Service {
    fn from(properties: ServiceProperties) -> Service {
        Service { properties }
    }
}

pub mod service {
    //! Property types for the `Service` resource.

    /// The [`AWS::AppRunner::Service.AuthenticationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-authenticationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthenticationConfiguration {
        /// Property [`AccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-authenticationconfiguration.html#cfn-apprunner-service-authenticationconfiguration-accessrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_role_arn: Option<::Value<String>>,
        /// Property [`ConnectionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-authenticationconfiguration.html#cfn-apprunner-service-authenticationconfiguration-connectionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AuthenticationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_role_arn) = self.access_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessRoleArn", access_role_arn)?;
            }
            if let Some(ref connection_arn) = self.connection_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionArn", connection_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthenticationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthenticationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthenticationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthenticationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_role_arn: Option<::Value<String>> = None;
                    let mut connection_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessRoleArn" => {
                                access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectionArn" => {
                                connection_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthenticationConfiguration {
                        access_role_arn: access_role_arn,
                        connection_arn: connection_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.CodeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CodeConfiguration {
        /// Property [`CodeConfigurationValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfiguration.html#cfn-apprunner-service-codeconfiguration-codeconfigurationvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_configuration_values: Option<::Value<CodeConfigurationValues>>,
        /// Property [`ConfigurationSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfiguration.html#cfn-apprunner-service-codeconfiguration-configurationsource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configuration_source: ::Value<String>,
    }

    impl ::codec::SerializeValue for CodeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref code_configuration_values) = self.code_configuration_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeConfigurationValues", code_configuration_values)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationSource", &self.configuration_source)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CodeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CodeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CodeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code_configuration_values: Option<::Value<CodeConfigurationValues>> = None;
                    let mut configuration_source: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CodeConfigurationValues" => {
                                code_configuration_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfigurationSource" => {
                                configuration_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CodeConfiguration {
                        code_configuration_values: code_configuration_values,
                        configuration_source: configuration_source.ok_or(::serde::de::Error::missing_field("ConfigurationSource"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.CodeConfigurationValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfigurationvalues.html) property type.
    #[derive(Debug, Default)]
    pub struct CodeConfigurationValues {
        /// Property [`BuildCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfigurationvalues.html#cfn-apprunner-service-codeconfigurationvalues-buildcommand).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub build_command: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfigurationvalues.html#cfn-apprunner-service-codeconfigurationvalues-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<String>>,
        /// Property [`Runtime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfigurationvalues.html#cfn-apprunner-service-codeconfigurationvalues-runtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub runtime: ::Value<String>,
        /// Property [`RuntimeEnvironmentVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfigurationvalues.html#cfn-apprunner-service-codeconfigurationvalues-runtimeenvironmentvariables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub runtime_environment_variables: Option<::ValueList<KeyValuePair>>,
        /// Property [`StartCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-codeconfigurationvalues.html#cfn-apprunner-service-codeconfigurationvalues-startcommand).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_command: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CodeConfigurationValues {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref build_command) = self.build_command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BuildCommand", build_command)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Runtime", &self.runtime)?;
            if let Some(ref runtime_environment_variables) = self.runtime_environment_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimeEnvironmentVariables", runtime_environment_variables)?;
            }
            if let Some(ref start_command) = self.start_command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartCommand", start_command)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CodeConfigurationValues {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeConfigurationValues, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CodeConfigurationValues;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CodeConfigurationValues")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut build_command: Option<::Value<String>> = None;
                    let mut port: Option<::Value<String>> = None;
                    let mut runtime: Option<::Value<String>> = None;
                    let mut runtime_environment_variables: Option<::ValueList<KeyValuePair>> = None;
                    let mut start_command: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BuildCommand" => {
                                build_command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Runtime" => {
                                runtime = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuntimeEnvironmentVariables" => {
                                runtime_environment_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartCommand" => {
                                start_command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CodeConfigurationValues {
                        build_command: build_command,
                        port: port,
                        runtime: runtime.ok_or(::serde::de::Error::missing_field("Runtime"))?,
                        runtime_environment_variables: runtime_environment_variables,
                        start_command: start_command,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.CodeRepository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-coderepository.html) property type.
    #[derive(Debug, Default)]
    pub struct CodeRepository {
        /// Property [`CodeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-coderepository.html#cfn-apprunner-service-coderepository-codeconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_configuration: Option<::Value<CodeConfiguration>>,
        /// Property [`RepositoryUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-coderepository.html#cfn-apprunner-service-coderepository-repositoryurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub repository_url: ::Value<String>,
        /// Property [`SourceCodeVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-coderepository.html#cfn-apprunner-service-coderepository-sourcecodeversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_code_version: ::Value<SourceCodeVersion>,
    }

    impl ::codec::SerializeValue for CodeRepository {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref code_configuration) = self.code_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeConfiguration", code_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryUrl", &self.repository_url)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceCodeVersion", &self.source_code_version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CodeRepository {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeRepository, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CodeRepository;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CodeRepository")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code_configuration: Option<::Value<CodeConfiguration>> = None;
                    let mut repository_url: Option<::Value<String>> = None;
                    let mut source_code_version: Option<::Value<SourceCodeVersion>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CodeConfiguration" => {
                                code_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RepositoryUrl" => {
                                repository_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceCodeVersion" => {
                                source_code_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CodeRepository {
                        code_configuration: code_configuration,
                        repository_url: repository_url.ok_or(::serde::de::Error::missing_field("RepositoryUrl"))?,
                        source_code_version: source_code_version.ok_or(::serde::de::Error::missing_field("SourceCodeVersion"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-encryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionConfiguration {
        /// Property [`KmsKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-encryptionconfiguration.html#cfn-apprunner-service-encryptionconfiguration-kmskey).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for EncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKey", &self.kms_key)?;
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
                    let mut kms_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKey" => {
                                kms_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfiguration {
                        kms_key: kms_key.ok_or(::serde::de::Error::missing_field("KmsKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.HealthCheckConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-healthcheckconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct HealthCheckConfiguration {
        /// Property [`HealthyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-healthcheckconfiguration.html#cfn-apprunner-service-healthcheckconfiguration-healthythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub healthy_threshold: Option<::Value<u32>>,
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-healthcheckconfiguration.html#cfn-apprunner-service-healthcheckconfiguration-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: Option<::Value<u32>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-healthcheckconfiguration.html#cfn-apprunner-service-healthcheckconfiguration-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-healthcheckconfiguration.html#cfn-apprunner-service-healthcheckconfiguration-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: Option<::Value<String>>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-healthcheckconfiguration.html#cfn-apprunner-service-healthcheckconfiguration-timeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout: Option<::Value<u32>>,
        /// Property [`UnhealthyThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-healthcheckconfiguration.html#cfn-apprunner-service-healthcheckconfiguration-unhealthythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unhealthy_threshold: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for HealthCheckConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref healthy_threshold) = self.healthy_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthyThreshold", healthy_threshold)?;
            }
            if let Some(ref interval) = self.interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", interval)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
            }
            if let Some(ref unhealthy_threshold) = self.unhealthy_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnhealthyThreshold", unhealthy_threshold)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthCheckConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheckConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthCheckConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthCheckConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut healthy_threshold: Option<::Value<u32>> = None;
                    let mut interval: Option<::Value<u32>> = None;
                    let mut path: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut timeout: Option<::Value<u32>> = None;
                    let mut unhealthy_threshold: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HealthyThreshold" => {
                                healthy_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnhealthyThreshold" => {
                                unhealthy_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheckConfiguration {
                        healthy_threshold: healthy_threshold,
                        interval: interval,
                        path: path,
                        protocol: protocol,
                        timeout: timeout,
                        unhealthy_threshold: unhealthy_threshold,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.ImageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-imageconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ImageConfiguration {
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-imageconfiguration.html#cfn-apprunner-service-imageconfiguration-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<String>>,
        /// Property [`RuntimeEnvironmentVariables`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-imageconfiguration.html#cfn-apprunner-service-imageconfiguration-runtimeenvironmentvariables).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub runtime_environment_variables: Option<::ValueList<KeyValuePair>>,
        /// Property [`StartCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-imageconfiguration.html#cfn-apprunner-service-imageconfiguration-startcommand).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_command: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ImageConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref runtime_environment_variables) = self.runtime_environment_variables {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimeEnvironmentVariables", runtime_environment_variables)?;
            }
            if let Some(ref start_command) = self.start_command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartCommand", start_command)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImageConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImageConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImageConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut port: Option<::Value<String>> = None;
                    let mut runtime_environment_variables: Option<::ValueList<KeyValuePair>> = None;
                    let mut start_command: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuntimeEnvironmentVariables" => {
                                runtime_environment_variables = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartCommand" => {
                                start_command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImageConfiguration {
                        port: port,
                        runtime_environment_variables: runtime_environment_variables,
                        start_command: start_command,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.ImageRepository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-imagerepository.html) property type.
    #[derive(Debug, Default)]
    pub struct ImageRepository {
        /// Property [`ImageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-imagerepository.html#cfn-apprunner-service-imagerepository-imageconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_configuration: Option<::Value<ImageConfiguration>>,
        /// Property [`ImageIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-imagerepository.html#cfn-apprunner-service-imagerepository-imageidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_identifier: ::Value<String>,
        /// Property [`ImageRepositoryType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-imagerepository.html#cfn-apprunner-service-imagerepository-imagerepositorytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_repository_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for ImageRepository {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref image_configuration) = self.image_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageConfiguration", image_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageIdentifier", &self.image_identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageRepositoryType", &self.image_repository_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImageRepository {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageRepository, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImageRepository;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImageRepository")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut image_configuration: Option<::Value<ImageConfiguration>> = None;
                    let mut image_identifier: Option<::Value<String>> = None;
                    let mut image_repository_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImageConfiguration" => {
                                image_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageIdentifier" => {
                                image_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageRepositoryType" => {
                                image_repository_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImageRepository {
                        image_configuration: image_configuration,
                        image_identifier: image_identifier.ok_or(::serde::de::Error::missing_field("ImageIdentifier"))?,
                        image_repository_type: image_repository_type.ok_or(::serde::de::Error::missing_field("ImageRepositoryType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.InstanceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-instanceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct InstanceConfiguration {
        /// Property [`Cpu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-instanceconfiguration.html#cfn-apprunner-service-instanceconfiguration-cpu).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cpu: Option<::Value<String>>,
        /// Property [`InstanceRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-instanceconfiguration.html#cfn-apprunner-service-instanceconfiguration-instancerolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_role_arn: Option<::Value<String>>,
        /// Property [`Memory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-instanceconfiguration.html#cfn-apprunner-service-instanceconfiguration-memory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub memory: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InstanceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cpu) = self.cpu {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", cpu)?;
            }
            if let Some(ref instance_role_arn) = self.instance_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceRoleArn", instance_role_arn)?;
            }
            if let Some(ref memory) = self.memory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", memory)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu: Option<::Value<String>> = None;
                    let mut instance_role_arn: Option<::Value<String>> = None;
                    let mut memory: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cpu" => {
                                cpu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceRoleArn" => {
                                instance_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Memory" => {
                                memory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceConfiguration {
                        cpu: cpu,
                        instance_role_arn: instance_role_arn,
                        memory: memory,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.KeyValuePair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-keyvaluepair.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyValuePair {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-keyvaluepair.html#cfn-apprunner-service-keyvaluepair-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-keyvaluepair.html#cfn-apprunner-service-keyvaluepair-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KeyValuePair {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyValuePair {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyValuePair, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyValuePair;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyValuePair")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyValuePair {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.SourceCodeVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-sourcecodeversion.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceCodeVersion {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-sourcecodeversion.html#cfn-apprunner-service-sourcecodeversion-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-sourcecodeversion.html#cfn-apprunner-service-sourcecodeversion-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourceCodeVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceCodeVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceCodeVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceCodeVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceCodeVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceCodeVersion {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppRunner::Service.SourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-sourceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceConfiguration {
        /// Property [`AuthenticationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-sourceconfiguration.html#cfn-apprunner-service-sourceconfiguration-authenticationconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authentication_configuration: Option<::Value<AuthenticationConfiguration>>,
        /// Property [`AutoDeploymentsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-sourceconfiguration.html#cfn-apprunner-service-sourceconfiguration-autodeploymentsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_deployments_enabled: Option<::Value<bool>>,
        /// Property [`CodeRepository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-sourceconfiguration.html#cfn-apprunner-service-sourceconfiguration-coderepository).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_repository: Option<::Value<CodeRepository>>,
        /// Property [`ImageRepository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apprunner-service-sourceconfiguration.html#cfn-apprunner-service-sourceconfiguration-imagerepository).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_repository: Option<::Value<ImageRepository>>,
    }

    impl ::codec::SerializeValue for SourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authentication_configuration) = self.authentication_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationConfiguration", authentication_configuration)?;
            }
            if let Some(ref auto_deployments_enabled) = self.auto_deployments_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoDeploymentsEnabled", auto_deployments_enabled)?;
            }
            if let Some(ref code_repository) = self.code_repository {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeRepository", code_repository)?;
            }
            if let Some(ref image_repository) = self.image_repository {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageRepository", image_repository)?;
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
                    let mut authentication_configuration: Option<::Value<AuthenticationConfiguration>> = None;
                    let mut auto_deployments_enabled: Option<::Value<bool>> = None;
                    let mut code_repository: Option<::Value<CodeRepository>> = None;
                    let mut image_repository: Option<::Value<ImageRepository>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticationConfiguration" => {
                                authentication_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AutoDeploymentsEnabled" => {
                                auto_deployments_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CodeRepository" => {
                                code_repository = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageRepository" => {
                                image_repository = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceConfiguration {
                        authentication_configuration: authentication_configuration,
                        auto_deployments_enabled: auto_deployments_enabled,
                        code_repository: code_repository,
                        image_repository: image_repository,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
