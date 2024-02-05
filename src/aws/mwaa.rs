//! Types for the `MWAA` service.

/// The [`AWS::MWAA::Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html) resource type.
#[derive(Debug, Default)]
pub struct Environment {
    properties: EnvironmentProperties
}

/// Properties for the `Environment` resource.
#[derive(Debug, Default)]
pub struct EnvironmentProperties {
    /// Property [`AirflowConfigurationOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-airflowconfigurationoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub airflow_configuration_options: Option<::Value<::json::Value>>,
    /// Property [`AirflowVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-airflowversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub airflow_version: Option<::Value<String>>,
    /// Property [`DagS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-dags3path).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dag_s3_path: Option<::Value<String>>,
    /// Property [`EndpointManagement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-endpointmanagement).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub endpoint_management: Option<::Value<String>>,
    /// Property [`EnvironmentClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-environmentclass).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment_class: Option<::Value<String>>,
    /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-executionrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_role_arn: Option<::Value<String>>,
    /// Property [`KmsKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-kmskey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key: Option<::Value<String>>,
    /// Property [`LoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-loggingconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_configuration: Option<::Value<self::environment::LoggingConfiguration>>,
    /// Property [`MaxWorkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-maxworkers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_workers: Option<::Value<u32>>,
    /// Property [`MinWorkers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-minworkers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_workers: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-networkconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_configuration: Option<::Value<self::environment::NetworkConfiguration>>,
    /// Property [`PluginsS3ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-pluginss3objectversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub plugins_s3_object_version: Option<::Value<String>>,
    /// Property [`PluginsS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-pluginss3path).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub plugins_s3_path: Option<::Value<String>>,
    /// Property [`RequirementsS3ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-requirementss3objectversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub requirements_s3_object_version: Option<::Value<String>>,
    /// Property [`RequirementsS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-requirementss3path).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub requirements_s3_path: Option<::Value<String>>,
    /// Property [`Schedulers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-schedulers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedulers: Option<::Value<u32>>,
    /// Property [`SourceBucketArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-sourcebucketarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_bucket_arn: Option<::Value<String>>,
    /// Property [`StartupScriptS3ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-startupscripts3objectversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub startup_script_s3_object_version: Option<::Value<String>>,
    /// Property [`StartupScriptS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-startupscripts3path).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub startup_script_s3_path: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::Value<::json::Value>>,
    /// Property [`WebserverAccessMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-webserveraccessmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub webserver_access_mode: Option<::Value<String>>,
    /// Property [`WeeklyMaintenanceWindowStart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaa-environment.html#cfn-mwaa-environment-weeklymaintenancewindowstart).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub weekly_maintenance_window_start: Option<::Value<String>>,
}

impl ::serde::Serialize for EnvironmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref airflow_configuration_options) = self.airflow_configuration_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AirflowConfigurationOptions", airflow_configuration_options)?;
        }
        if let Some(ref airflow_version) = self.airflow_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AirflowVersion", airflow_version)?;
        }
        if let Some(ref dag_s3_path) = self.dag_s3_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DagS3Path", dag_s3_path)?;
        }
        if let Some(ref endpoint_management) = self.endpoint_management {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointManagement", endpoint_management)?;
        }
        if let Some(ref environment_class) = self.environment_class {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentClass", environment_class)?;
        }
        if let Some(ref execution_role_arn) = self.execution_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", execution_role_arn)?;
        }
        if let Some(ref kms_key) = self.kms_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKey", kms_key)?;
        }
        if let Some(ref logging_configuration) = self.logging_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingConfiguration", logging_configuration)?;
        }
        if let Some(ref max_workers) = self.max_workers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxWorkers", max_workers)?;
        }
        if let Some(ref min_workers) = self.min_workers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinWorkers", min_workers)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref network_configuration) = self.network_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", network_configuration)?;
        }
        if let Some(ref plugins_s3_object_version) = self.plugins_s3_object_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PluginsS3ObjectVersion", plugins_s3_object_version)?;
        }
        if let Some(ref plugins_s3_path) = self.plugins_s3_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PluginsS3Path", plugins_s3_path)?;
        }
        if let Some(ref requirements_s3_object_version) = self.requirements_s3_object_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequirementsS3ObjectVersion", requirements_s3_object_version)?;
        }
        if let Some(ref requirements_s3_path) = self.requirements_s3_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequirementsS3Path", requirements_s3_path)?;
        }
        if let Some(ref schedulers) = self.schedulers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedulers", schedulers)?;
        }
        if let Some(ref source_bucket_arn) = self.source_bucket_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceBucketArn", source_bucket_arn)?;
        }
        if let Some(ref startup_script_s3_object_version) = self.startup_script_s3_object_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartupScriptS3ObjectVersion", startup_script_s3_object_version)?;
        }
        if let Some(ref startup_script_s3_path) = self.startup_script_s3_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartupScriptS3Path", startup_script_s3_path)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref webserver_access_mode) = self.webserver_access_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebserverAccessMode", webserver_access_mode)?;
        }
        if let Some(ref weekly_maintenance_window_start) = self.weekly_maintenance_window_start {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeeklyMaintenanceWindowStart", weekly_maintenance_window_start)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnvironmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnvironmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnvironmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut airflow_configuration_options: Option<::Value<::json::Value>> = None;
                let mut airflow_version: Option<::Value<String>> = None;
                let mut dag_s3_path: Option<::Value<String>> = None;
                let mut endpoint_management: Option<::Value<String>> = None;
                let mut environment_class: Option<::Value<String>> = None;
                let mut execution_role_arn: Option<::Value<String>> = None;
                let mut kms_key: Option<::Value<String>> = None;
                let mut logging_configuration: Option<::Value<self::environment::LoggingConfiguration>> = None;
                let mut max_workers: Option<::Value<u32>> = None;
                let mut min_workers: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut network_configuration: Option<::Value<self::environment::NetworkConfiguration>> = None;
                let mut plugins_s3_object_version: Option<::Value<String>> = None;
                let mut plugins_s3_path: Option<::Value<String>> = None;
                let mut requirements_s3_object_version: Option<::Value<String>> = None;
                let mut requirements_s3_path: Option<::Value<String>> = None;
                let mut schedulers: Option<::Value<u32>> = None;
                let mut source_bucket_arn: Option<::Value<String>> = None;
                let mut startup_script_s3_object_version: Option<::Value<String>> = None;
                let mut startup_script_s3_path: Option<::Value<String>> = None;
                let mut tags: Option<::Value<::json::Value>> = None;
                let mut webserver_access_mode: Option<::Value<String>> = None;
                let mut weekly_maintenance_window_start: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AirflowConfigurationOptions" => {
                            airflow_configuration_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AirflowVersion" => {
                            airflow_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DagS3Path" => {
                            dag_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointManagement" => {
                            endpoint_management = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnvironmentClass" => {
                            environment_class = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionRoleArn" => {
                            execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKey" => {
                            kms_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingConfiguration" => {
                            logging_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxWorkers" => {
                            max_workers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinWorkers" => {
                            min_workers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkConfiguration" => {
                            network_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PluginsS3ObjectVersion" => {
                            plugins_s3_object_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PluginsS3Path" => {
                            plugins_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequirementsS3ObjectVersion" => {
                            requirements_s3_object_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequirementsS3Path" => {
                            requirements_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedulers" => {
                            schedulers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceBucketArn" => {
                            source_bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartupScriptS3ObjectVersion" => {
                            startup_script_s3_object_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartupScriptS3Path" => {
                            startup_script_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WebserverAccessMode" => {
                            webserver_access_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WeeklyMaintenanceWindowStart" => {
                            weekly_maintenance_window_start = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnvironmentProperties {
                    airflow_configuration_options: airflow_configuration_options,
                    airflow_version: airflow_version,
                    dag_s3_path: dag_s3_path,
                    endpoint_management: endpoint_management,
                    environment_class: environment_class,
                    execution_role_arn: execution_role_arn,
                    kms_key: kms_key,
                    logging_configuration: logging_configuration,
                    max_workers: max_workers,
                    min_workers: min_workers,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    network_configuration: network_configuration,
                    plugins_s3_object_version: plugins_s3_object_version,
                    plugins_s3_path: plugins_s3_path,
                    requirements_s3_object_version: requirements_s3_object_version,
                    requirements_s3_path: requirements_s3_path,
                    schedulers: schedulers,
                    source_bucket_arn: source_bucket_arn,
                    startup_script_s3_object_version: startup_script_s3_object_version,
                    startup_script_s3_path: startup_script_s3_path,
                    tags: tags,
                    webserver_access_mode: webserver_access_mode,
                    weekly_maintenance_window_start: weekly_maintenance_window_start,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Environment {
    type Properties = EnvironmentProperties;
    const TYPE: &'static str = "AWS::MWAA::Environment";
    fn properties(&self) -> &EnvironmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Environment {}

impl From<EnvironmentProperties> for Environment {
    fn from(properties: EnvironmentProperties) -> Environment {
        Environment { properties }
    }
}

pub mod environment {
    //! Property types for the `Environment` resource.

    /// The [`AWS::MWAA::Environment.LoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-loggingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingConfiguration {
        /// Property [`DagProcessingLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-loggingconfiguration.html#cfn-mwaa-environment-loggingconfiguration-dagprocessinglogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dag_processing_logs: Option<::Value<ModuleLoggingConfiguration>>,
        /// Property [`SchedulerLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-loggingconfiguration.html#cfn-mwaa-environment-loggingconfiguration-schedulerlogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scheduler_logs: Option<::Value<ModuleLoggingConfiguration>>,
        /// Property [`TaskLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-loggingconfiguration.html#cfn-mwaa-environment-loggingconfiguration-tasklogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_logs: Option<::Value<ModuleLoggingConfiguration>>,
        /// Property [`WebserverLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-loggingconfiguration.html#cfn-mwaa-environment-loggingconfiguration-webserverlogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub webserver_logs: Option<::Value<ModuleLoggingConfiguration>>,
        /// Property [`WorkerLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-loggingconfiguration.html#cfn-mwaa-environment-loggingconfiguration-workerlogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub worker_logs: Option<::Value<ModuleLoggingConfiguration>>,
    }

    impl ::codec::SerializeValue for LoggingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dag_processing_logs) = self.dag_processing_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DagProcessingLogs", dag_processing_logs)?;
            }
            if let Some(ref scheduler_logs) = self.scheduler_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchedulerLogs", scheduler_logs)?;
            }
            if let Some(ref task_logs) = self.task_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskLogs", task_logs)?;
            }
            if let Some(ref webserver_logs) = self.webserver_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WebserverLogs", webserver_logs)?;
            }
            if let Some(ref worker_logs) = self.worker_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerLogs", worker_logs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dag_processing_logs: Option<::Value<ModuleLoggingConfiguration>> = None;
                    let mut scheduler_logs: Option<::Value<ModuleLoggingConfiguration>> = None;
                    let mut task_logs: Option<::Value<ModuleLoggingConfiguration>> = None;
                    let mut webserver_logs: Option<::Value<ModuleLoggingConfiguration>> = None;
                    let mut worker_logs: Option<::Value<ModuleLoggingConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DagProcessingLogs" => {
                                dag_processing_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchedulerLogs" => {
                                scheduler_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskLogs" => {
                                task_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WebserverLogs" => {
                                webserver_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkerLogs" => {
                                worker_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingConfiguration {
                        dag_processing_logs: dag_processing_logs,
                        scheduler_logs: scheduler_logs,
                        task_logs: task_logs,
                        webserver_logs: webserver_logs,
                        worker_logs: worker_logs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MWAA::Environment.ModuleLoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-moduleloggingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ModuleLoggingConfiguration {
        /// Property [`CloudWatchLogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-moduleloggingconfiguration.html#cfn-mwaa-environment-moduleloggingconfiguration-cloudwatchloggrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_log_group_arn: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-moduleloggingconfiguration.html#cfn-mwaa-environment-moduleloggingconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`LogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-moduleloggingconfiguration.html#cfn-mwaa-environment-moduleloggingconfiguration-loglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ModuleLoggingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_log_group_arn) = self.cloud_watch_log_group_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogGroupArn", cloud_watch_log_group_arn)?;
            }
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref log_level) = self.log_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogLevel", log_level)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModuleLoggingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModuleLoggingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModuleLoggingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModuleLoggingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_log_group_arn: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut log_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogGroupArn" => {
                                cloud_watch_log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogLevel" => {
                                log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModuleLoggingConfiguration {
                        cloud_watch_log_group_arn: cloud_watch_log_group_arn,
                        enabled: enabled,
                        log_level: log_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MWAA::Environment.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-networkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfiguration {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-networkconfiguration.html#cfn-mwaa-environment-networkconfiguration-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaa-environment-networkconfiguration.html#cfn-mwaa-environment-networkconfiguration-subnetids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfiguration")
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

                    Ok(NetworkConfiguration {
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
