//! Types for the `KafkaConnect` service.

/// The [`AWS::KafkaConnect::Connector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html) resource type.
#[derive(Debug, Default)]
pub struct Connector {
    properties: ConnectorProperties
}

/// Properties for the `Connector` resource.
#[derive(Debug, Default)]
pub struct ConnectorProperties {
    /// Property [`Capacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-capacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub capacity: ::Value<self::connector::Capacity>,
    /// Property [`ConnectorConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-connectorconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_configuration: ::ValueMap<String>,
    /// Property [`ConnectorDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-connectordescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_description: Option<::Value<String>>,
    /// Property [`ConnectorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-connectorname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_name: ::Value<String>,
    /// Property [`KafkaCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-kafkacluster).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kafka_cluster: ::Value<self::connector::KafkaCluster>,
    /// Property [`KafkaClusterClientAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-kafkaclusterclientauthentication).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kafka_cluster_client_authentication: ::Value<self::connector::KafkaClusterClientAuthentication>,
    /// Property [`KafkaClusterEncryptionInTransit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-kafkaclusterencryptionintransit).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kafka_cluster_encryption_in_transit: ::Value<self::connector::KafkaClusterEncryptionInTransit>,
    /// Property [`KafkaConnectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-kafkaconnectversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kafka_connect_version: ::Value<String>,
    /// Property [`LogDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-logdelivery).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub log_delivery: Option<::Value<self::connector::LogDelivery>>,
    /// Property [`Plugins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-plugins).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub plugins: ::ValueList<self::connector::Plugin>,
    /// Property [`ServiceExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-serviceexecutionrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_execution_role_arn: ::Value<String>,
    /// Property [`WorkerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html#cfn-kafkaconnect-connector-workerconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub worker_configuration: Option<::Value<self::connector::WorkerConfiguration>>,
}

impl ::serde::Serialize for ConnectorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Capacity", &self.capacity)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorConfiguration", &self.connector_configuration)?;
        if let Some(ref connector_description) = self.connector_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorDescription", connector_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorName", &self.connector_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KafkaCluster", &self.kafka_cluster)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KafkaClusterClientAuthentication", &self.kafka_cluster_client_authentication)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KafkaClusterEncryptionInTransit", &self.kafka_cluster_encryption_in_transit)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KafkaConnectVersion", &self.kafka_connect_version)?;
        if let Some(ref log_delivery) = self.log_delivery {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDelivery", log_delivery)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Plugins", &self.plugins)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceExecutionRoleArn", &self.service_execution_role_arn)?;
        if let Some(ref worker_configuration) = self.worker_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerConfiguration", worker_configuration)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut capacity: Option<::Value<self::connector::Capacity>> = None;
                let mut connector_configuration: Option<::ValueMap<String>> = None;
                let mut connector_description: Option<::Value<String>> = None;
                let mut connector_name: Option<::Value<String>> = None;
                let mut kafka_cluster: Option<::Value<self::connector::KafkaCluster>> = None;
                let mut kafka_cluster_client_authentication: Option<::Value<self::connector::KafkaClusterClientAuthentication>> = None;
                let mut kafka_cluster_encryption_in_transit: Option<::Value<self::connector::KafkaClusterEncryptionInTransit>> = None;
                let mut kafka_connect_version: Option<::Value<String>> = None;
                let mut log_delivery: Option<::Value<self::connector::LogDelivery>> = None;
                let mut plugins: Option<::ValueList<self::connector::Plugin>> = None;
                let mut service_execution_role_arn: Option<::Value<String>> = None;
                let mut worker_configuration: Option<::Value<self::connector::WorkerConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Capacity" => {
                            capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectorConfiguration" => {
                            connector_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectorDescription" => {
                            connector_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectorName" => {
                            connector_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KafkaCluster" => {
                            kafka_cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KafkaClusterClientAuthentication" => {
                            kafka_cluster_client_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KafkaClusterEncryptionInTransit" => {
                            kafka_cluster_encryption_in_transit = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KafkaConnectVersion" => {
                            kafka_connect_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogDelivery" => {
                            log_delivery = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Plugins" => {
                            plugins = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceExecutionRoleArn" => {
                            service_execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkerConfiguration" => {
                            worker_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectorProperties {
                    capacity: capacity.ok_or(::serde::de::Error::missing_field("Capacity"))?,
                    connector_configuration: connector_configuration.ok_or(::serde::de::Error::missing_field("ConnectorConfiguration"))?,
                    connector_description: connector_description,
                    connector_name: connector_name.ok_or(::serde::de::Error::missing_field("ConnectorName"))?,
                    kafka_cluster: kafka_cluster.ok_or(::serde::de::Error::missing_field("KafkaCluster"))?,
                    kafka_cluster_client_authentication: kafka_cluster_client_authentication.ok_or(::serde::de::Error::missing_field("KafkaClusterClientAuthentication"))?,
                    kafka_cluster_encryption_in_transit: kafka_cluster_encryption_in_transit.ok_or(::serde::de::Error::missing_field("KafkaClusterEncryptionInTransit"))?,
                    kafka_connect_version: kafka_connect_version.ok_or(::serde::de::Error::missing_field("KafkaConnectVersion"))?,
                    log_delivery: log_delivery,
                    plugins: plugins.ok_or(::serde::de::Error::missing_field("Plugins"))?,
                    service_execution_role_arn: service_execution_role_arn.ok_or(::serde::de::Error::missing_field("ServiceExecutionRoleArn"))?,
                    worker_configuration: worker_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Connector {
    type Properties = ConnectorProperties;
    const TYPE: &'static str = "AWS::KafkaConnect::Connector";
    fn properties(&self) -> &ConnectorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Connector {}

impl From<ConnectorProperties> for Connector {
    fn from(properties: ConnectorProperties) -> Connector {
        Connector { properties }
    }
}

pub mod connector {
    //! Property types for the `Connector` resource.

    /// The [`AWS::KafkaConnect::Connector.ApacheKafkaCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-apachekafkacluster.html) property type.
    #[derive(Debug, Default)]
    pub struct ApacheKafkaCluster {
        /// Property [`BootstrapServers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-apachekafkacluster.html#cfn-kafkaconnect-connector-apachekafkacluster-bootstrapservers).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bootstrap_servers: ::Value<String>,
        /// Property [`Vpc`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-apachekafkacluster.html#cfn-kafkaconnect-connector-apachekafkacluster-vpc).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vpc: ::Value<Vpc>,
    }

    impl ::codec::SerializeValue for ApacheKafkaCluster {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BootstrapServers", &self.bootstrap_servers)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Vpc", &self.vpc)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApacheKafkaCluster {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApacheKafkaCluster, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApacheKafkaCluster;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApacheKafkaCluster")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bootstrap_servers: Option<::Value<String>> = None;
                    let mut vpc: Option<::Value<Vpc>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BootstrapServers" => {
                                bootstrap_servers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Vpc" => {
                                vpc = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApacheKafkaCluster {
                        bootstrap_servers: bootstrap_servers.ok_or(::serde::de::Error::missing_field("BootstrapServers"))?,
                        vpc: vpc.ok_or(::serde::de::Error::missing_field("Vpc"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.AutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-autoscaling.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoScaling {
        /// Property [`MaxWorkerCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-autoscaling.html#cfn-kafkaconnect-connector-autoscaling-maxworkercount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_worker_count: ::Value<u32>,
        /// Property [`McuCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-autoscaling.html#cfn-kafkaconnect-connector-autoscaling-mcucount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mcu_count: ::Value<u32>,
        /// Property [`MinWorkerCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-autoscaling.html#cfn-kafkaconnect-connector-autoscaling-minworkercount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_worker_count: ::Value<u32>,
        /// Property [`ScaleInPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-autoscaling.html#cfn-kafkaconnect-connector-autoscaling-scaleinpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scale_in_policy: ::Value<ScaleInPolicy>,
        /// Property [`ScaleOutPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-autoscaling.html#cfn-kafkaconnect-connector-autoscaling-scaleoutpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scale_out_policy: ::Value<ScaleOutPolicy>,
    }

    impl ::codec::SerializeValue for AutoScaling {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxWorkerCount", &self.max_worker_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "McuCount", &self.mcu_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinWorkerCount", &self.min_worker_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScaleInPolicy", &self.scale_in_policy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScaleOutPolicy", &self.scale_out_policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoScaling {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoScaling, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoScaling;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoScaling")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_worker_count: Option<::Value<u32>> = None;
                    let mut mcu_count: Option<::Value<u32>> = None;
                    let mut min_worker_count: Option<::Value<u32>> = None;
                    let mut scale_in_policy: Option<::Value<ScaleInPolicy>> = None;
                    let mut scale_out_policy: Option<::Value<ScaleOutPolicy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxWorkerCount" => {
                                max_worker_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "McuCount" => {
                                mcu_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinWorkerCount" => {
                                min_worker_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScaleInPolicy" => {
                                scale_in_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScaleOutPolicy" => {
                                scale_out_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoScaling {
                        max_worker_count: max_worker_count.ok_or(::serde::de::Error::missing_field("MaxWorkerCount"))?,
                        mcu_count: mcu_count.ok_or(::serde::de::Error::missing_field("McuCount"))?,
                        min_worker_count: min_worker_count.ok_or(::serde::de::Error::missing_field("MinWorkerCount"))?,
                        scale_in_policy: scale_in_policy.ok_or(::serde::de::Error::missing_field("ScaleInPolicy"))?,
                        scale_out_policy: scale_out_policy.ok_or(::serde::de::Error::missing_field("ScaleOutPolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.Capacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-capacity.html) property type.
    #[derive(Debug, Default)]
    pub struct Capacity {
        /// Property [`AutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-capacity.html#cfn-kafkaconnect-connector-capacity-autoscaling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_scaling: Option<::Value<AutoScaling>>,
        /// Property [`ProvisionedCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-capacity.html#cfn-kafkaconnect-connector-capacity-provisionedcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provisioned_capacity: Option<::Value<ProvisionedCapacity>>,
    }

    impl ::codec::SerializeValue for Capacity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_scaling) = self.auto_scaling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScaling", auto_scaling)?;
            }
            if let Some(ref provisioned_capacity) = self.provisioned_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedCapacity", provisioned_capacity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Capacity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Capacity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Capacity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Capacity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_scaling: Option<::Value<AutoScaling>> = None;
                    let mut provisioned_capacity: Option<::Value<ProvisionedCapacity>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoScaling" => {
                                auto_scaling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProvisionedCapacity" => {
                                provisioned_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Capacity {
                        auto_scaling: auto_scaling,
                        provisioned_capacity: provisioned_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.CloudWatchLogsLogDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-cloudwatchlogslogdelivery.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLogsLogDelivery {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-cloudwatchlogslogdelivery.html#cfn-kafkaconnect-connector-cloudwatchlogslogdelivery-enabled).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`LogGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-cloudwatchlogslogdelivery.html#cfn-kafkaconnect-connector-cloudwatchlogslogdelivery-loggroup).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub log_group: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CloudWatchLogsLogDelivery {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref log_group) = self.log_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroup", log_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLogsLogDelivery {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLogsLogDelivery, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLogsLogDelivery;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLogsLogDelivery")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut log_group: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogGroup" => {
                                log_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLogsLogDelivery {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        log_group: log_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.CustomPlugin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-customplugin.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomPlugin {
        /// Property [`CustomPluginArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-customplugin.html#cfn-kafkaconnect-connector-customplugin-custompluginarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub custom_plugin_arn: ::Value<String>,
        /// Property [`Revision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-customplugin.html#cfn-kafkaconnect-connector-customplugin-revision).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub revision: ::Value<u32>,
    }

    impl ::codec::SerializeValue for CustomPlugin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomPluginArn", &self.custom_plugin_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", &self.revision)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomPlugin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomPlugin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomPlugin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomPlugin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_plugin_arn: Option<::Value<String>> = None;
                    let mut revision: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomPluginArn" => {
                                custom_plugin_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Revision" => {
                                revision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomPlugin {
                        custom_plugin_arn: custom_plugin_arn.ok_or(::serde::de::Error::missing_field("CustomPluginArn"))?,
                        revision: revision.ok_or(::serde::de::Error::missing_field("Revision"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.FirehoseLogDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-firehoselogdelivery.html) property type.
    #[derive(Debug, Default)]
    pub struct FirehoseLogDelivery {
        /// Property [`DeliveryStream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-firehoselogdelivery.html#cfn-kafkaconnect-connector-firehoselogdelivery-deliverystream).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub delivery_stream: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-firehoselogdelivery.html#cfn-kafkaconnect-connector-firehoselogdelivery-enabled).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for FirehoseLogDelivery {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delivery_stream) = self.delivery_stream {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStream", delivery_stream)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FirehoseLogDelivery {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FirehoseLogDelivery, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FirehoseLogDelivery;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FirehoseLogDelivery")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_stream: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryStream" => {
                                delivery_stream = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FirehoseLogDelivery {
                        delivery_stream: delivery_stream,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.KafkaCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-kafkacluster.html) property type.
    #[derive(Debug, Default)]
    pub struct KafkaCluster {
        /// Property [`ApacheKafkaCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-kafkacluster.html#cfn-kafkaconnect-connector-kafkacluster-apachekafkacluster).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub apache_kafka_cluster: ::Value<ApacheKafkaCluster>,
    }

    impl ::codec::SerializeValue for KafkaCluster {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApacheKafkaCluster", &self.apache_kafka_cluster)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KafkaCluster {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KafkaCluster, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KafkaCluster;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KafkaCluster")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut apache_kafka_cluster: Option<::Value<ApacheKafkaCluster>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApacheKafkaCluster" => {
                                apache_kafka_cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KafkaCluster {
                        apache_kafka_cluster: apache_kafka_cluster.ok_or(::serde::de::Error::missing_field("ApacheKafkaCluster"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.KafkaClusterClientAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-kafkaclusterclientauthentication.html) property type.
    #[derive(Debug, Default)]
    pub struct KafkaClusterClientAuthentication {
        /// Property [`AuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-kafkaclusterclientauthentication.html#cfn-kafkaconnect-connector-kafkaclusterclientauthentication-authenticationtype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub authentication_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for KafkaClusterClientAuthentication {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationType", &self.authentication_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KafkaClusterClientAuthentication {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KafkaClusterClientAuthentication, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KafkaClusterClientAuthentication;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KafkaClusterClientAuthentication")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authentication_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticationType" => {
                                authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KafkaClusterClientAuthentication {
                        authentication_type: authentication_type.ok_or(::serde::de::Error::missing_field("AuthenticationType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.KafkaClusterEncryptionInTransit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-kafkaclusterencryptionintransit.html) property type.
    #[derive(Debug, Default)]
    pub struct KafkaClusterEncryptionInTransit {
        /// Property [`EncryptionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-kafkaclusterencryptionintransit.html#cfn-kafkaconnect-connector-kafkaclusterencryptionintransit-encryptiontype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub encryption_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for KafkaClusterEncryptionInTransit {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionType", &self.encryption_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KafkaClusterEncryptionInTransit {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KafkaClusterEncryptionInTransit, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KafkaClusterEncryptionInTransit;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KafkaClusterEncryptionInTransit")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionType" => {
                                encryption_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KafkaClusterEncryptionInTransit {
                        encryption_type: encryption_type.ok_or(::serde::de::Error::missing_field("EncryptionType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.LogDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-logdelivery.html) property type.
    #[derive(Debug, Default)]
    pub struct LogDelivery {
        /// Property [`WorkerLogDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-logdelivery.html#cfn-kafkaconnect-connector-logdelivery-workerlogdelivery).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub worker_log_delivery: ::Value<WorkerLogDelivery>,
    }

    impl ::codec::SerializeValue for LogDelivery {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerLogDelivery", &self.worker_log_delivery)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogDelivery {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogDelivery, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogDelivery;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogDelivery")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut worker_log_delivery: Option<::Value<WorkerLogDelivery>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WorkerLogDelivery" => {
                                worker_log_delivery = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogDelivery {
                        worker_log_delivery: worker_log_delivery.ok_or(::serde::de::Error::missing_field("WorkerLogDelivery"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.Plugin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-plugin.html) property type.
    #[derive(Debug, Default)]
    pub struct Plugin {
        /// Property [`CustomPlugin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-plugin.html#cfn-kafkaconnect-connector-plugin-customplugin).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub custom_plugin: ::Value<CustomPlugin>,
    }

    impl ::codec::SerializeValue for Plugin {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomPlugin", &self.custom_plugin)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Plugin {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Plugin, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Plugin;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Plugin")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_plugin: Option<::Value<CustomPlugin>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomPlugin" => {
                                custom_plugin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Plugin {
                        custom_plugin: custom_plugin.ok_or(::serde::de::Error::missing_field("CustomPlugin"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.ProvisionedCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-provisionedcapacity.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisionedCapacity {
        /// Property [`McuCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-provisionedcapacity.html#cfn-kafkaconnect-connector-provisionedcapacity-mcucount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mcu_count: Option<::Value<u32>>,
        /// Property [`WorkerCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-provisionedcapacity.html#cfn-kafkaconnect-connector-provisionedcapacity-workercount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub worker_count: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ProvisionedCapacity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mcu_count) = self.mcu_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "McuCount", mcu_count)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerCount", &self.worker_count)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProvisionedCapacity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisionedCapacity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisionedCapacity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisionedCapacity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mcu_count: Option<::Value<u32>> = None;
                    let mut worker_count: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "McuCount" => {
                                mcu_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkerCount" => {
                                worker_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisionedCapacity {
                        mcu_count: mcu_count,
                        worker_count: worker_count.ok_or(::serde::de::Error::missing_field("WorkerCount"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.S3LogDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-s3logdelivery.html) property type.
    #[derive(Debug, Default)]
    pub struct S3LogDelivery {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-s3logdelivery.html#cfn-kafkaconnect-connector-s3logdelivery-bucket).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bucket: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-s3logdelivery.html#cfn-kafkaconnect-connector-s3logdelivery-enabled).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-s3logdelivery.html#cfn-kafkaconnect-connector-s3logdelivery-prefix).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3LogDelivery {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket) = self.bucket {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", bucket)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3LogDelivery {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3LogDelivery, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3LogDelivery;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3LogDelivery")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3LogDelivery {
                        bucket: bucket,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.ScaleInPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-scaleinpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ScaleInPolicy {
        /// Property [`CpuUtilizationPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-scaleinpolicy.html#cfn-kafkaconnect-connector-scaleinpolicy-cpuutilizationpercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cpu_utilization_percentage: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ScaleInPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CpuUtilizationPercentage", &self.cpu_utilization_percentage)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScaleInPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScaleInPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScaleInPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScaleInPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu_utilization_percentage: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CpuUtilizationPercentage" => {
                                cpu_utilization_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScaleInPolicy {
                        cpu_utilization_percentage: cpu_utilization_percentage.ok_or(::serde::de::Error::missing_field("CpuUtilizationPercentage"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.ScaleOutPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-scaleoutpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct ScaleOutPolicy {
        /// Property [`CpuUtilizationPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-scaleoutpolicy.html#cfn-kafkaconnect-connector-scaleoutpolicy-cpuutilizationpercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cpu_utilization_percentage: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ScaleOutPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CpuUtilizationPercentage", &self.cpu_utilization_percentage)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScaleOutPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScaleOutPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScaleOutPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScaleOutPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu_utilization_percentage: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CpuUtilizationPercentage" => {
                                cpu_utilization_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScaleOutPolicy {
                        cpu_utilization_percentage: cpu_utilization_percentage.ok_or(::serde::de::Error::missing_field("CpuUtilizationPercentage"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.Vpc`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-vpc.html) property type.
    #[derive(Debug, Default)]
    pub struct Vpc {
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-vpc.html#cfn-kafkaconnect-connector-vpc-securitygroups).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_groups: ::ValueList<String>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-vpc.html#cfn-kafkaconnect-connector-vpc-subnets).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for Vpc {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", &self.security_groups)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Vpc {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Vpc, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Vpc;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Vpc")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Vpc {
                        security_groups: security_groups.ok_or(::serde::de::Error::missing_field("SecurityGroups"))?,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.WorkerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-workerconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkerConfiguration {
        /// Property [`Revision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-workerconfiguration.html#cfn-kafkaconnect-connector-workerconfiguration-revision).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub revision: ::Value<u32>,
        /// Property [`WorkerConfigurationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-workerconfiguration.html#cfn-kafkaconnect-connector-workerconfiguration-workerconfigurationarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub worker_configuration_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for WorkerConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", &self.revision)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerConfigurationArn", &self.worker_configuration_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkerConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkerConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkerConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkerConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut revision: Option<::Value<u32>> = None;
                    let mut worker_configuration_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Revision" => {
                                revision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkerConfigurationArn" => {
                                worker_configuration_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkerConfiguration {
                        revision: revision.ok_or(::serde::de::Error::missing_field("Revision"))?,
                        worker_configuration_arn: worker_configuration_arn.ok_or(::serde::de::Error::missing_field("WorkerConfigurationArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KafkaConnect::Connector.WorkerLogDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-workerlogdelivery.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkerLogDelivery {
        /// Property [`CloudWatchLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-workerlogdelivery.html#cfn-kafkaconnect-connector-workerlogdelivery-cloudwatchlogs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cloud_watch_logs: Option<::Value<CloudWatchLogsLogDelivery>>,
        /// Property [`Firehose`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-workerlogdelivery.html#cfn-kafkaconnect-connector-workerlogdelivery-firehose).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub firehose: Option<::Value<FirehoseLogDelivery>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-workerlogdelivery.html#cfn-kafkaconnect-connector-workerlogdelivery-s3).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3: Option<::Value<S3LogDelivery>>,
    }

    impl ::codec::SerializeValue for WorkerLogDelivery {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_logs) = self.cloud_watch_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogs", cloud_watch_logs)?;
            }
            if let Some(ref firehose) = self.firehose {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Firehose", firehose)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkerLogDelivery {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkerLogDelivery, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkerLogDelivery;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkerLogDelivery")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logs: Option<::Value<CloudWatchLogsLogDelivery>> = None;
                    let mut firehose: Option<::Value<FirehoseLogDelivery>> = None;
                    let mut s3: Option<::Value<S3LogDelivery>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogs" => {
                                cloud_watch_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Firehose" => {
                                firehose = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkerLogDelivery {
                        cloud_watch_logs: cloud_watch_logs,
                        firehose: firehose,
                        s3: s3,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
