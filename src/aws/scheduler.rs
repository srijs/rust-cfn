//! Types for the `Scheduler` service.

/// The [`AWS::Scheduler::Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html) resource type.
#[derive(Debug, Default)]
pub struct Schedule {
    properties: ScheduleProperties
}

/// Properties for the `Schedule` resource.
#[derive(Debug, Default)]
pub struct ScheduleProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EndDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-enddate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub end_date: Option<::Value<String>>,
    /// Property [`FlexibleTimeWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-flexibletimewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub flexible_time_window: ::Value<self::schedule::FlexibleTimeWindow>,
    /// Property [`GroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-groupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub group_name: Option<::Value<String>>,
    /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-kmskeyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_arn: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-scheduleexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule_expression: ::Value<String>,
    /// Property [`ScheduleExpressionTimezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-scheduleexpressiontimezone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule_expression_timezone: Option<::Value<String>>,
    /// Property [`StartDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-startdate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub start_date: Option<::Value<String>>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html#cfn-scheduler-schedule-target).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target: ::Value<self::schedule::Target>,
}

impl ::serde::Serialize for ScheduleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref end_date) = self.end_date {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndDate", end_date)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlexibleTimeWindow", &self.flexible_time_window)?;
        if let Some(ref group_name) = self.group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", group_name)?;
        }
        if let Some(ref kms_key_arn) = self.kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", &self.schedule_expression)?;
        if let Some(ref schedule_expression_timezone) = self.schedule_expression_timezone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpressionTimezone", schedule_expression_timezone)?;
        }
        if let Some(ref start_date) = self.start_date {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartDate", start_date)?;
        }
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScheduleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScheduleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScheduleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut end_date: Option<::Value<String>> = None;
                let mut flexible_time_window: Option<::Value<self::schedule::FlexibleTimeWindow>> = None;
                let mut group_name: Option<::Value<String>> = None;
                let mut kms_key_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut schedule_expression: Option<::Value<String>> = None;
                let mut schedule_expression_timezone: Option<::Value<String>> = None;
                let mut start_date: Option<::Value<String>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut target: Option<::Value<self::schedule::Target>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndDate" => {
                            end_date = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlexibleTimeWindow" => {
                            flexible_time_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GroupName" => {
                            group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyArn" => {
                            kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduleExpression" => {
                            schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduleExpressionTimezone" => {
                            schedule_expression_timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartDate" => {
                            start_date = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Target" => {
                            target = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScheduleProperties {
                    description: description,
                    end_date: end_date,
                    flexible_time_window: flexible_time_window.ok_or(::serde::de::Error::missing_field("FlexibleTimeWindow"))?,
                    group_name: group_name,
                    kms_key_arn: kms_key_arn,
                    name: name,
                    schedule_expression: schedule_expression.ok_or(::serde::de::Error::missing_field("ScheduleExpression"))?,
                    schedule_expression_timezone: schedule_expression_timezone,
                    start_date: start_date,
                    state: state,
                    target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Schedule {
    type Properties = ScheduleProperties;
    const TYPE: &'static str = "AWS::Scheduler::Schedule";
    fn properties(&self) -> &ScheduleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScheduleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Schedule {}

impl From<ScheduleProperties> for Schedule {
    fn from(properties: ScheduleProperties) -> Schedule {
        Schedule { properties }
    }
}

/// The [`AWS::Scheduler::ScheduleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedulegroup.html) resource type.
#[derive(Debug, Default)]
pub struct ScheduleGroup {
    properties: ScheduleGroupProperties
}

/// Properties for the `ScheduleGroup` resource.
#[derive(Debug, Default)]
pub struct ScheduleGroupProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedulegroup.html#cfn-scheduler-schedulegroup-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedulegroup.html#cfn-scheduler-schedulegroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ScheduleGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScheduleGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduleGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScheduleGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScheduleGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScheduleGroupProperties {
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ScheduleGroup {
    type Properties = ScheduleGroupProperties;
    const TYPE: &'static str = "AWS::Scheduler::ScheduleGroup";
    fn properties(&self) -> &ScheduleGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScheduleGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ScheduleGroup {}

impl From<ScheduleGroupProperties> for ScheduleGroup {
    fn from(properties: ScheduleGroupProperties) -> ScheduleGroup {
        ScheduleGroup { properties }
    }
}

pub mod schedule {
    //! Property types for the `Schedule` resource.

    /// The [`AWS::Scheduler::Schedule.AwsVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-awsvpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AwsVpcConfiguration {
        /// Property [`AssignPublicIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-awsvpcconfiguration.html#cfn-scheduler-schedule-awsvpcconfiguration-assignpublicip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assign_public_ip: Option<::Value<String>>,
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-awsvpcconfiguration.html#cfn-scheduler-schedule-awsvpcconfiguration-securitygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_groups: Option<::ValueList<String>>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-awsvpcconfiguration.html#cfn-scheduler-schedule-awsvpcconfiguration-subnets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AwsVpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref assign_public_ip) = self.assign_public_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignPublicIp", assign_public_ip)?;
            }
            if let Some(ref security_groups) = self.security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AwsVpcConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AwsVpcConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AwsVpcConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AwsVpcConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut assign_public_ip: Option<::Value<String>> = None;
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssignPublicIp" => {
                                assign_public_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AwsVpcConfiguration {
                        assign_public_ip: assign_public_ip,
                        security_groups: security_groups,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.CapacityProviderStrategyItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-capacityproviderstrategyitem.html) property type.
    #[derive(Debug, Default)]
    pub struct CapacityProviderStrategyItem {
        /// Property [`Base`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-capacityproviderstrategyitem.html#cfn-scheduler-schedule-capacityproviderstrategyitem-base).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base: Option<::Value<f64>>,
        /// Property [`CapacityProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-capacityproviderstrategyitem.html#cfn-scheduler-schedule-capacityproviderstrategyitem-capacityprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capacity_provider: ::Value<String>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-capacityproviderstrategyitem.html#cfn-scheduler-schedule-capacityproviderstrategyitem-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for CapacityProviderStrategyItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref base) = self.base {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Base", base)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityProvider", &self.capacity_provider)?;
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CapacityProviderStrategyItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityProviderStrategyItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CapacityProviderStrategyItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CapacityProviderStrategyItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base: Option<::Value<f64>> = None;
                    let mut capacity_provider: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Base" => {
                                base = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CapacityProvider" => {
                                capacity_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CapacityProviderStrategyItem {
                        base: base,
                        capacity_provider: capacity_provider.ok_or(::serde::de::Error::missing_field("CapacityProvider"))?,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-deadletterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DeadLetterConfig {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-deadletterconfig.html#cfn-scheduler-schedule-deadletterconfig-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeadLetterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeadLetterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeadLetterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeadLetterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeadLetterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeadLetterConfig {
                        arn: arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.EcsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct EcsParameters {
        /// Property [`CapacityProviderStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-capacityproviderstrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capacity_provider_strategy: Option<::ValueList<CapacityProviderStrategyItem>>,
        /// Property [`EnableECSManagedTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-enableecsmanagedtags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_ecs_managed_tags: Option<::Value<bool>>,
        /// Property [`EnableExecuteCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-enableexecutecommand).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_execute_command: Option<::Value<bool>>,
        /// Property [`Group`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-group).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group: Option<::Value<String>>,
        /// Property [`LaunchType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-launchtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub launch_type: Option<::Value<String>>,
        /// Property [`NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-networkconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_configuration: Option<::Value<NetworkConfiguration>>,
        /// Property [`PlacementConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-placementconstraints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub placement_constraints: Option<::ValueList<PlacementConstraint>>,
        /// Property [`PlacementStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-placementstrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub placement_strategy: Option<::ValueList<PlacementStrategy>>,
        /// Property [`PlatformVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-platformversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub platform_version: Option<::Value<String>>,
        /// Property [`PropagateTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-propagatetags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub propagate_tags: Option<::Value<String>>,
        /// Property [`ReferenceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-referenceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reference_id: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::Value<::json::Value>>,
        /// Property [`TaskCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-taskcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_count: Option<::Value<f64>>,
        /// Property [`TaskDefinitionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html#cfn-scheduler-schedule-ecsparameters-taskdefinitionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_definition_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for EcsParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref capacity_provider_strategy) = self.capacity_provider_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityProviderStrategy", capacity_provider_strategy)?;
            }
            if let Some(ref enable_ecs_managed_tags) = self.enable_ecs_managed_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableECSManagedTags", enable_ecs_managed_tags)?;
            }
            if let Some(ref enable_execute_command) = self.enable_execute_command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableExecuteCommand", enable_execute_command)?;
            }
            if let Some(ref group) = self.group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Group", group)?;
            }
            if let Some(ref launch_type) = self.launch_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchType", launch_type)?;
            }
            if let Some(ref network_configuration) = self.network_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", network_configuration)?;
            }
            if let Some(ref placement_constraints) = self.placement_constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementConstraints", placement_constraints)?;
            }
            if let Some(ref placement_strategy) = self.placement_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementStrategy", placement_strategy)?;
            }
            if let Some(ref platform_version) = self.platform_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformVersion", platform_version)?;
            }
            if let Some(ref propagate_tags) = self.propagate_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropagateTags", propagate_tags)?;
            }
            if let Some(ref reference_id) = self.reference_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferenceId", reference_id)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            if let Some(ref task_count) = self.task_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskCount", task_count)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskDefinitionArn", &self.task_definition_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EcsParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EcsParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EcsParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EcsParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut capacity_provider_strategy: Option<::ValueList<CapacityProviderStrategyItem>> = None;
                    let mut enable_ecs_managed_tags: Option<::Value<bool>> = None;
                    let mut enable_execute_command: Option<::Value<bool>> = None;
                    let mut group: Option<::Value<String>> = None;
                    let mut launch_type: Option<::Value<String>> = None;
                    let mut network_configuration: Option<::Value<NetworkConfiguration>> = None;
                    let mut placement_constraints: Option<::ValueList<PlacementConstraint>> = None;
                    let mut placement_strategy: Option<::ValueList<PlacementStrategy>> = None;
                    let mut platform_version: Option<::Value<String>> = None;
                    let mut propagate_tags: Option<::Value<String>> = None;
                    let mut reference_id: Option<::Value<String>> = None;
                    let mut tags: Option<::Value<::json::Value>> = None;
                    let mut task_count: Option<::Value<f64>> = None;
                    let mut task_definition_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CapacityProviderStrategy" => {
                                capacity_provider_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableECSManagedTags" => {
                                enable_ecs_managed_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableExecuteCommand" => {
                                enable_execute_command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Group" => {
                                group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LaunchType" => {
                                launch_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkConfiguration" => {
                                network_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlacementConstraints" => {
                                placement_constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlacementStrategy" => {
                                placement_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlatformVersion" => {
                                platform_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropagateTags" => {
                                propagate_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReferenceId" => {
                                reference_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskCount" => {
                                task_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskDefinitionArn" => {
                                task_definition_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EcsParameters {
                        capacity_provider_strategy: capacity_provider_strategy,
                        enable_ecs_managed_tags: enable_ecs_managed_tags,
                        enable_execute_command: enable_execute_command,
                        group: group,
                        launch_type: launch_type,
                        network_configuration: network_configuration,
                        placement_constraints: placement_constraints,
                        placement_strategy: placement_strategy,
                        platform_version: platform_version,
                        propagate_tags: propagate_tags,
                        reference_id: reference_id,
                        tags: tags,
                        task_count: task_count,
                        task_definition_arn: task_definition_arn.ok_or(::serde::de::Error::missing_field("TaskDefinitionArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.EventBridgeParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-eventbridgeparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct EventBridgeParameters {
        /// Property [`DetailType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-eventbridgeparameters.html#cfn-scheduler-schedule-eventbridgeparameters-detailtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub detail_type: ::Value<String>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-eventbridgeparameters.html#cfn-scheduler-schedule-eventbridgeparameters-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: ::Value<String>,
    }

    impl ::codec::SerializeValue for EventBridgeParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetailType", &self.detail_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventBridgeParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventBridgeParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventBridgeParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventBridgeParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut detail_type: Option<::Value<String>> = None;
                    let mut source: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DetailType" => {
                                detail_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventBridgeParameters {
                        detail_type: detail_type.ok_or(::serde::de::Error::missing_field("DetailType"))?,
                        source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.FlexibleTimeWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-flexibletimewindow.html) property type.
    #[derive(Debug, Default)]
    pub struct FlexibleTimeWindow {
        /// Property [`MaximumWindowInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-flexibletimewindow.html#cfn-scheduler-schedule-flexibletimewindow-maximumwindowinminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_window_in_minutes: Option<::Value<f64>>,
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-flexibletimewindow.html#cfn-scheduler-schedule-flexibletimewindow-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: ::Value<String>,
    }

    impl ::codec::SerializeValue for FlexibleTimeWindow {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref maximum_window_in_minutes) = self.maximum_window_in_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumWindowInMinutes", maximum_window_in_minutes)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", &self.mode)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FlexibleTimeWindow {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FlexibleTimeWindow, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FlexibleTimeWindow;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FlexibleTimeWindow")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maximum_window_in_minutes: Option<::Value<f64>> = None;
                    let mut mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaximumWindowInMinutes" => {
                                maximum_window_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FlexibleTimeWindow {
                        maximum_window_in_minutes: maximum_window_in_minutes,
                        mode: mode.ok_or(::serde::de::Error::missing_field("Mode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.KinesisParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-kinesisparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisParameters {
        /// Property [`PartitionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-kinesisparameters.html#cfn-scheduler-schedule-kinesisparameters-partitionkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionKey", &self.partition_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut partition_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PartitionKey" => {
                                partition_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisParameters {
                        partition_key: partition_key.ok_or(::serde::de::Error::missing_field("PartitionKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-networkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfiguration {
        /// Property [`AwsvpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-networkconfiguration.html#cfn-scheduler-schedule-networkconfiguration-awsvpcconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub awsvpc_configuration: Option<::Value<AwsVpcConfiguration>>,
    }

    impl ::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref awsvpc_configuration) = self.awsvpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsvpcConfiguration", awsvpc_configuration)?;
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
                    let mut awsvpc_configuration: Option<::Value<AwsVpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsvpcConfiguration" => {
                                awsvpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfiguration {
                        awsvpc_configuration: awsvpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.PlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-placementconstraint.html) property type.
    #[derive(Debug, Default)]
    pub struct PlacementConstraint {
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-placementconstraint.html#cfn-scheduler-schedule-placementconstraint-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-placementconstraint.html#cfn-scheduler-schedule-placementconstraint-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PlacementConstraint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref expression) = self.expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", expression)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PlacementConstraint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PlacementConstraint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PlacementConstraint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PlacementConstraint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expression: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PlacementConstraint {
                        expression: expression,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.PlacementStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-placementstrategy.html) property type.
    #[derive(Debug, Default)]
    pub struct PlacementStrategy {
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-placementstrategy.html#cfn-scheduler-schedule-placementstrategy-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-placementstrategy.html#cfn-scheduler-schedule-placementstrategy-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PlacementStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref field) = self.field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", field)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PlacementStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PlacementStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PlacementStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PlacementStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PlacementStrategy {
                        field: field,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.RetryPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-retrypolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct RetryPolicy {
        /// Property [`MaximumEventAgeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-retrypolicy.html#cfn-scheduler-schedule-retrypolicy-maximumeventageinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_event_age_in_seconds: Option<::Value<f64>>,
        /// Property [`MaximumRetryAttempts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-retrypolicy.html#cfn-scheduler-schedule-retrypolicy-maximumretryattempts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_retry_attempts: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for RetryPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref maximum_event_age_in_seconds) = self.maximum_event_age_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumEventAgeInSeconds", maximum_event_age_in_seconds)?;
            }
            if let Some(ref maximum_retry_attempts) = self.maximum_retry_attempts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumRetryAttempts", maximum_retry_attempts)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RetryPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RetryPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RetryPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RetryPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maximum_event_age_in_seconds: Option<::Value<f64>> = None;
                    let mut maximum_retry_attempts: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaximumEventAgeInSeconds" => {
                                maximum_event_age_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumRetryAttempts" => {
                                maximum_retry_attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RetryPolicy {
                        maximum_event_age_in_seconds: maximum_event_age_in_seconds,
                        maximum_retry_attempts: maximum_retry_attempts,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.SageMakerPipelineParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-sagemakerpipelineparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct SageMakerPipelineParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-sagemakerpipelineparameter.html#cfn-scheduler-schedule-sagemakerpipelineparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-sagemakerpipelineparameter.html#cfn-scheduler-schedule-sagemakerpipelineparameter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for SageMakerPipelineParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SageMakerPipelineParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SageMakerPipelineParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SageMakerPipelineParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SageMakerPipelineParameter")
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

                    Ok(SageMakerPipelineParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.SageMakerPipelineParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-sagemakerpipelineparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct SageMakerPipelineParameters {
        /// Property [`PipelineParameterList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-sagemakerpipelineparameters.html#cfn-scheduler-schedule-sagemakerpipelineparameters-pipelineparameterlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pipeline_parameter_list: Option<::ValueList<SageMakerPipelineParameter>>,
    }

    impl ::codec::SerializeValue for SageMakerPipelineParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref pipeline_parameter_list) = self.pipeline_parameter_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineParameterList", pipeline_parameter_list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SageMakerPipelineParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SageMakerPipelineParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SageMakerPipelineParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SageMakerPipelineParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pipeline_parameter_list: Option<::ValueList<SageMakerPipelineParameter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PipelineParameterList" => {
                                pipeline_parameter_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SageMakerPipelineParameters {
                        pipeline_parameter_list: pipeline_parameter_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.SqsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-sqsparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct SqsParameters {
        /// Property [`MessageGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-sqsparameters.html#cfn-scheduler-schedule-sqsparameters-messagegroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_group_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SqsParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref message_group_id) = self.message_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageGroupId", message_group_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SqsParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SqsParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SqsParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SqsParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message_group_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MessageGroupId" => {
                                message_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SqsParameters {
                        message_group_id: message_group_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Scheduler::Schedule.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html) property type.
    #[derive(Debug, Default)]
    pub struct Target {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html#cfn-scheduler-schedule-target-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`DeadLetterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html#cfn-scheduler-schedule-target-deadletterconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dead_letter_config: Option<::Value<DeadLetterConfig>>,
        /// Property [`EcsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html#cfn-scheduler-schedule-target-ecsparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecs_parameters: Option<::Value<EcsParameters>>,
        /// Property [`EventBridgeParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html#cfn-scheduler-schedule-target-eventbridgeparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_bridge_parameters: Option<::Value<EventBridgeParameters>>,
        /// Property [`Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html#cfn-scheduler-schedule-target-input).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input: Option<::Value<String>>,
        /// Property [`KinesisParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html#cfn-scheduler-schedule-target-kinesisparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_parameters: Option<::Value<KinesisParameters>>,
        /// Property [`RetryPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html#cfn-scheduler-schedule-target-retrypolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retry_policy: Option<::Value<RetryPolicy>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html#cfn-scheduler-schedule-target-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SageMakerPipelineParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html#cfn-scheduler-schedule-target-sagemakerpipelineparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sage_maker_pipeline_parameters: Option<::Value<SageMakerPipelineParameters>>,
        /// Property [`SqsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html#cfn-scheduler-schedule-target-sqsparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sqs_parameters: Option<::Value<SqsParameters>>,
    }

    impl ::codec::SerializeValue for Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            if let Some(ref dead_letter_config) = self.dead_letter_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeadLetterConfig", dead_letter_config)?;
            }
            if let Some(ref ecs_parameters) = self.ecs_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcsParameters", ecs_parameters)?;
            }
            if let Some(ref event_bridge_parameters) = self.event_bridge_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventBridgeParameters", event_bridge_parameters)?;
            }
            if let Some(ref input) = self.input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Input", input)?;
            }
            if let Some(ref kinesis_parameters) = self.kinesis_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisParameters", kinesis_parameters)?;
            }
            if let Some(ref retry_policy) = self.retry_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryPolicy", retry_policy)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref sage_maker_pipeline_parameters) = self.sage_maker_pipeline_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerPipelineParameters", sage_maker_pipeline_parameters)?;
            }
            if let Some(ref sqs_parameters) = self.sqs_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqsParameters", sqs_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Target {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Target, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Target;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Target")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut dead_letter_config: Option<::Value<DeadLetterConfig>> = None;
                    let mut ecs_parameters: Option<::Value<EcsParameters>> = None;
                    let mut event_bridge_parameters: Option<::Value<EventBridgeParameters>> = None;
                    let mut input: Option<::Value<String>> = None;
                    let mut kinesis_parameters: Option<::Value<KinesisParameters>> = None;
                    let mut retry_policy: Option<::Value<RetryPolicy>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut sage_maker_pipeline_parameters: Option<::Value<SageMakerPipelineParameters>> = None;
                    let mut sqs_parameters: Option<::Value<SqsParameters>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeadLetterConfig" => {
                                dead_letter_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcsParameters" => {
                                ecs_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventBridgeParameters" => {
                                event_bridge_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Input" => {
                                input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisParameters" => {
                                kinesis_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetryPolicy" => {
                                retry_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerPipelineParameters" => {
                                sage_maker_pipeline_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqsParameters" => {
                                sqs_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Target {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        dead_letter_config: dead_letter_config,
                        ecs_parameters: ecs_parameters,
                        event_bridge_parameters: event_bridge_parameters,
                        input: input,
                        kinesis_parameters: kinesis_parameters,
                        retry_policy: retry_policy,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        sage_maker_pipeline_parameters: sage_maker_pipeline_parameters,
                        sqs_parameters: sqs_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
