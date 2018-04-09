//! Types for the `CodeDeploy` service.

/// The [`AWS::CodeDeploy::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-application.html) resource type.
#[derive(Debug)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug)]
pub struct ApplicationProperties {
    /// Property `ApplicationName`.
    pub application_name: Option<::Value<String>>,
    /// Property `ComputePlatform`.
    pub compute_platform: Option<::Value<String>>,
}

impl ::serde::Serialize for ApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputePlatform", &self.compute_platform)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_name = None;
                let mut compute_platform = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationName" => {
                            application_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ComputePlatform" => {
                            compute_platform = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationProperties {
                    application_name: application_name,
                    compute_platform: compute_platform,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::CodeDeploy::Application";
    fn properties(&self) -> &ApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Application {}

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

/// The [`AWS::CodeDeploy::DeploymentConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-deploymentconfig.html) resource type.
#[derive(Debug)]
pub struct DeploymentConfig {
    properties: DeploymentConfigProperties
}

/// Properties for the `DeploymentConfig` resource.
#[derive(Debug)]
pub struct DeploymentConfigProperties {
    /// Property `DeploymentConfigName`.
    pub deployment_config_name: Option<::Value<String>>,
    /// Property `MinimumHealthyHosts`.
    pub minimum_healthy_hosts: Option<::Value<self::deployment_config::MinimumHealthyHosts>>,
}

impl ::serde::Serialize for DeploymentConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentConfigName", &self.deployment_config_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumHealthyHosts", &self.minimum_healthy_hosts)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeploymentConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeploymentConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeploymentConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut deployment_config_name = None;
                let mut minimum_healthy_hosts = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeploymentConfigName" => {
                            deployment_config_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MinimumHealthyHosts" => {
                            minimum_healthy_hosts = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DeploymentConfigProperties {
                    deployment_config_name: deployment_config_name,
                    minimum_healthy_hosts: minimum_healthy_hosts,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for DeploymentConfig {
    type Properties = DeploymentConfigProperties;
    const TYPE: &'static str = "AWS::CodeDeploy::DeploymentConfig";
    fn properties(&self) -> &DeploymentConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeploymentConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DeploymentConfig {}

impl From<DeploymentConfigProperties> for DeploymentConfig {
    fn from(properties: DeploymentConfigProperties) -> DeploymentConfig {
        DeploymentConfig { properties }
    }
}

/// The [`AWS::CodeDeploy::DeploymentGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-deploymentgroup.html) resource type.
#[derive(Debug)]
pub struct DeploymentGroup {
    properties: DeploymentGroupProperties
}

/// Properties for the `DeploymentGroup` resource.
#[derive(Debug)]
pub struct DeploymentGroupProperties {
    /// Property `AlarmConfiguration`.
    pub alarm_configuration: Option<::Value<self::deployment_group::AlarmConfiguration>>,
    /// Property `ApplicationName`.
    pub application_name: ::Value<String>,
    /// Property `AutoRollbackConfiguration`.
    pub auto_rollback_configuration: Option<::Value<self::deployment_group::AutoRollbackConfiguration>>,
    /// Property `AutoScalingGroups`.
    pub auto_scaling_groups: Option<::ValueList<String>>,
    /// Property `Deployment`.
    pub deployment: Option<::Value<self::deployment_group::Deployment>>,
    /// Property `DeploymentConfigName`.
    pub deployment_config_name: Option<::Value<String>>,
    /// Property `DeploymentGroupName`.
    pub deployment_group_name: Option<::Value<String>>,
    /// Property `DeploymentStyle`.
    pub deployment_style: Option<::Value<self::deployment_group::DeploymentStyle>>,
    /// Property `Ec2TagFilters`.
    pub ec2_tag_filters: Option<::ValueList<self::deployment_group::EC2TagFilter>>,
    /// Property `LoadBalancerInfo`.
    pub load_balancer_info: Option<::Value<self::deployment_group::LoadBalancerInfo>>,
    /// Property `OnPremisesInstanceTagFilters`.
    pub on_premises_instance_tag_filters: Option<::ValueList<self::deployment_group::TagFilter>>,
    /// Property `ServiceRoleArn`.
    pub service_role_arn: ::Value<String>,
    /// Property `TriggerConfigurations`.
    pub trigger_configurations: Option<::ValueList<self::deployment_group::TriggerConfig>>,
}

impl ::serde::Serialize for DeploymentGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmConfiguration", &self.alarm_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoRollbackConfiguration", &self.auto_rollback_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroups", &self.auto_scaling_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Deployment", &self.deployment)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentConfigName", &self.deployment_config_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentGroupName", &self.deployment_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentStyle", &self.deployment_style)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2TagFilters", &self.ec2_tag_filters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerInfo", &self.load_balancer_info)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnPremisesInstanceTagFilters", &self.on_premises_instance_tag_filters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRoleArn", &self.service_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerConfigurations", &self.trigger_configurations)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeploymentGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeploymentGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeploymentGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeploymentGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut alarm_configuration = None;
                let mut application_name = None;
                let mut auto_rollback_configuration = None;
                let mut auto_scaling_groups = None;
                let mut deployment = None;
                let mut deployment_config_name = None;
                let mut deployment_group_name = None;
                let mut deployment_style = None;
                let mut ec2_tag_filters = None;
                let mut load_balancer_info = None;
                let mut on_premises_instance_tag_filters = None;
                let mut service_role_arn = None;
                let mut trigger_configurations = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AlarmConfiguration" => {
                            alarm_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ApplicationName" => {
                            application_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoRollbackConfiguration" => {
                            auto_rollback_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoScalingGroups" => {
                            auto_scaling_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Deployment" => {
                            deployment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DeploymentConfigName" => {
                            deployment_config_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DeploymentGroupName" => {
                            deployment_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DeploymentStyle" => {
                            deployment_style = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Ec2TagFilters" => {
                            ec2_tag_filters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LoadBalancerInfo" => {
                            load_balancer_info = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OnPremisesInstanceTagFilters" => {
                            on_premises_instance_tag_filters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ServiceRoleArn" => {
                            service_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TriggerConfigurations" => {
                            trigger_configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DeploymentGroupProperties {
                    alarm_configuration: alarm_configuration,
                    application_name: application_name.ok_or(::serde::de::Error::missing_field("ApplicationName"))?,
                    auto_rollback_configuration: auto_rollback_configuration,
                    auto_scaling_groups: auto_scaling_groups,
                    deployment: deployment,
                    deployment_config_name: deployment_config_name,
                    deployment_group_name: deployment_group_name,
                    deployment_style: deployment_style,
                    ec2_tag_filters: ec2_tag_filters,
                    load_balancer_info: load_balancer_info,
                    on_premises_instance_tag_filters: on_premises_instance_tag_filters,
                    service_role_arn: service_role_arn.ok_or(::serde::de::Error::missing_field("ServiceRoleArn"))?,
                    trigger_configurations: trigger_configurations,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for DeploymentGroup {
    type Properties = DeploymentGroupProperties;
    const TYPE: &'static str = "AWS::CodeDeploy::DeploymentGroup";
    fn properties(&self) -> &DeploymentGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeploymentGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DeploymentGroup {}

impl From<DeploymentGroupProperties> for DeploymentGroup {
    fn from(properties: DeploymentGroupProperties) -> DeploymentGroup {
        DeploymentGroup { properties }
    }
}

pub mod deployment_config {
    //! Property types for the `DeploymentConfig` resource.

    /// The [`AWS::CodeDeploy::DeploymentConfig.MinimumHealthyHosts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentconfig-minimumhealthyhosts.html) property type.
    #[derive(Debug)]
    pub struct MinimumHealthyHosts {
        /// Property `Type`.
        pub type_: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<u32>,
    }

    impl ::codec::SerializeValue for MinimumHealthyHosts {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MinimumHealthyHosts {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MinimumHealthyHosts, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MinimumHealthyHosts;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MinimumHealthyHosts")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut type_ = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MinimumHealthyHosts {
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod deployment_group {
    //! Property types for the `DeploymentGroup` resource.

    /// The [`AWS::CodeDeploy::DeploymentGroup.Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-alarm.html) property type.
    #[derive(Debug)]
    pub struct Alarm {
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Alarm {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Alarm {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Alarm, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Alarm;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Alarm")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Alarm {
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.AlarmConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-alarmconfiguration.html) property type.
    #[derive(Debug)]
    pub struct AlarmConfiguration {
        /// Property `Alarms`.
        pub alarms: Option<::ValueList<Alarm>>,
        /// Property `Enabled`.
        pub enabled: Option<::Value<bool>>,
        /// Property `IgnorePollAlarmFailure`.
        pub ignore_poll_alarm_failure: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for AlarmConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alarms", &self.alarms)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IgnorePollAlarmFailure", &self.ignore_poll_alarm_failure)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AlarmConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AlarmConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AlarmConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AlarmConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarms = None;
                    let mut enabled = None;
                    let mut ignore_poll_alarm_failure = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alarms" => {
                                alarms = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IgnorePollAlarmFailure" => {
                                ignore_poll_alarm_failure = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AlarmConfiguration {
                        alarms: alarms,
                        enabled: enabled,
                        ignore_poll_alarm_failure: ignore_poll_alarm_failure,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.AutoRollbackConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-autorollbackconfiguration.html) property type.
    #[derive(Debug)]
    pub struct AutoRollbackConfiguration {
        /// Property `Enabled`.
        pub enabled: Option<::Value<bool>>,
        /// Property `Events`.
        pub events: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AutoRollbackConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Events", &self.events)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoRollbackConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoRollbackConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoRollbackConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoRollbackConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled = None;
                    let mut events = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Events" => {
                                events = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoRollbackConfiguration {
                        enabled: enabled,
                        events: events,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.Deployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment.html) property type.
    #[derive(Debug)]
    pub struct Deployment {
        /// Property `Description`.
        pub description: Option<::Value<String>>,
        /// Property `IgnoreApplicationStopFailures`.
        pub ignore_application_stop_failures: Option<::Value<bool>>,
        /// Property `Revision`.
        pub revision: ::Value<RevisionLocation>,
    }

    impl ::codec::SerializeValue for Deployment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IgnoreApplicationStopFailures", &self.ignore_application_stop_failures)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", &self.revision)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Deployment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Deployment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Deployment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Deployment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description = None;
                    let mut ignore_application_stop_failures = None;
                    let mut revision = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IgnoreApplicationStopFailures" => {
                                ignore_application_stop_failures = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Revision" => {
                                revision = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Deployment {
                        description: description,
                        ignore_application_stop_failures: ignore_application_stop_failures,
                        revision: revision.ok_or(::serde::de::Error::missing_field("Revision"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.DeploymentStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deploymentstyle.html) property type.
    #[derive(Debug)]
    pub struct DeploymentStyle {
        /// Property `DeploymentOption`.
        pub deployment_option: Option<::Value<String>>,
        /// Property `DeploymentType`.
        pub deployment_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeploymentStyle {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentOption", &self.deployment_option)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentType", &self.deployment_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeploymentStyle {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeploymentStyle, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeploymentStyle;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeploymentStyle")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut deployment_option = None;
                    let mut deployment_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeploymentOption" => {
                                deployment_option = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DeploymentType" => {
                                deployment_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(DeploymentStyle {
                        deployment_option: deployment_option,
                        deployment_type: deployment_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.EC2TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-ec2tagfilters.html) property type.
    #[derive(Debug)]
    pub struct EC2TagFilter {
        /// Property `Key`.
        pub key: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
        /// Property `Value`.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EC2TagFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EC2TagFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EC2TagFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EC2TagFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EC2TagFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut type_ = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EC2TagFilter {
                        key: key,
                        type_: type_,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.ELBInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-elbinfo.html) property type.
    #[derive(Debug)]
    pub struct ELBInfo {
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ELBInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ELBInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ELBInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ELBInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ELBInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ELBInfo {
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.GitHubLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision-githublocation.html) property type.
    #[derive(Debug)]
    pub struct GitHubLocation {
        /// Property `CommitId`.
        pub commit_id: ::Value<String>,
        /// Property `Repository`.
        pub repository: ::Value<String>,
    }

    impl ::codec::SerializeValue for GitHubLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CommitId", &self.commit_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Repository", &self.repository)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GitHubLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GitHubLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GitHubLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GitHubLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut commit_id = None;
                    let mut repository = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CommitId" => {
                                commit_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Repository" => {
                                repository = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(GitHubLocation {
                        commit_id: commit_id.ok_or(::serde::de::Error::missing_field("CommitId"))?,
                        repository: repository.ok_or(::serde::de::Error::missing_field("Repository"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.LoadBalancerInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-loadbalancerinfo.html) property type.
    #[derive(Debug)]
    pub struct LoadBalancerInfo {
        /// Property `ElbInfoList`.
        pub elb_info_list: Option<::ValueList<ELBInfo>>,
        /// Property `TargetGroupInfoList`.
        pub target_group_info_list: Option<::ValueList<TargetGroupInfo>>,
    }

    impl ::codec::SerializeValue for LoadBalancerInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElbInfoList", &self.elb_info_list)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupInfoList", &self.target_group_info_list)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoadBalancerInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoadBalancerInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoadBalancerInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoadBalancerInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut elb_info_list = None;
                    let mut target_group_info_list = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ElbInfoList" => {
                                elb_info_list = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TargetGroupInfoList" => {
                                target_group_info_list = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LoadBalancerInfo {
                        elb_info_list: elb_info_list,
                        target_group_info_list: target_group_info_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.RevisionLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision.html) property type.
    #[derive(Debug)]
    pub struct RevisionLocation {
        /// Property `GitHubLocation`.
        pub git_hub_location: Option<::Value<GitHubLocation>>,
        /// Property `RevisionType`.
        pub revision_type: Option<::Value<String>>,
        /// Property `S3Location`.
        pub s3_location: Option<::Value<S3Location>>,
    }

    impl ::codec::SerializeValue for RevisionLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GitHubLocation", &self.git_hub_location)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RevisionType", &self.revision_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Location", &self.s3_location)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RevisionLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RevisionLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RevisionLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RevisionLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut git_hub_location = None;
                    let mut revision_type = None;
                    let mut s3_location = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GitHubLocation" => {
                                git_hub_location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RevisionType" => {
                                revision_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3Location" => {
                                s3_location = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(RevisionLocation {
                        git_hub_location: git_hub_location,
                        revision_type: revision_type,
                        s3_location: s3_location,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision-s3location.html) property type.
    #[derive(Debug)]
    pub struct S3Location {
        /// Property `Bucket`.
        pub bucket: ::Value<String>,
        /// Property `BundleType`.
        pub bundle_type: Option<::Value<String>>,
        /// Property `ETag`.
        pub e_tag: Option<::Value<String>>,
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Version`.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BundleType", &self.bundle_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ETag", &self.e_tag)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket = None;
                    let mut bundle_type = None;
                    let mut e_tag = None;
                    let mut key = None;
                    let mut version = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "BundleType" => {
                                bundle_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ETag" => {
                                e_tag = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Version" => {
                                version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        bundle_type: bundle_type,
                        e_tag: e_tag,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-onpremisesinstancetagfilters.html) property type.
    #[derive(Debug)]
    pub struct TagFilter {
        /// Property `Key`.
        pub key: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
        /// Property `Value`.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TagFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TagFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut type_ = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(TagFilter {
                        key: key,
                        type_: type_,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.TargetGroupInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-targetgroupinfo.html) property type.
    #[derive(Debug)]
    pub struct TargetGroupInfo {
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TargetGroupInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetGroupInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetGroupInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetGroupInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetGroupInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetGroupInfo {
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CodeDeploy::DeploymentGroup.TriggerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-triggerconfig.html) property type.
    #[derive(Debug)]
    pub struct TriggerConfig {
        /// Property `TriggerEvents`.
        pub trigger_events: Option<::ValueList<String>>,
        /// Property `TriggerName`.
        pub trigger_name: Option<::Value<String>>,
        /// Property `TriggerTargetArn`.
        pub trigger_target_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TriggerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerEvents", &self.trigger_events)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerName", &self.trigger_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerTargetArn", &self.trigger_target_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TriggerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TriggerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TriggerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TriggerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut trigger_events = None;
                    let mut trigger_name = None;
                    let mut trigger_target_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TriggerEvents" => {
                                trigger_events = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TriggerName" => {
                                trigger_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TriggerTargetArn" => {
                                trigger_target_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(TriggerConfig {
                        trigger_events: trigger_events,
                        trigger_name: trigger_name,
                        trigger_target_arn: trigger_target_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
