//! Types for the `ApplicationAutoScaling` service.

/// The [`AWS::ApplicationAutoScaling::ScalableTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html) resource type.
#[derive(Debug)]
pub struct ScalableTarget {
    properties: ScalableTargetProperties
}

/// Properties for the `ScalableTarget` resource.
#[derive(Debug)]
pub struct ScalableTargetProperties {
    /// Property `MaxCapacity`.
    pub max_capacity: ::Value<u32>,
    /// Property `MinCapacity`.
    pub min_capacity: ::Value<u32>,
    /// Property `ResourceId`.
    pub resource_id: ::Value<String>,
    /// Property `RoleARN`.
    pub role_arn: ::Value<String>,
    /// Property `ScalableDimension`.
    pub scalable_dimension: ::Value<String>,
    /// Property `ScheduledActions`.
    pub scheduled_actions: Option<::ValueList<self::scalable_target::ScheduledAction>>,
    /// Property `ServiceNamespace`.
    pub service_namespace: ::Value<String>,
}

impl ::serde::Serialize for ScalableTargetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", &self.max_capacity)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinCapacity", &self.min_capacity)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", &self.resource_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalableDimension", &self.scalable_dimension)?;
        if let Some(ref scheduled_actions) = self.scheduled_actions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduledActions", scheduled_actions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNamespace", &self.service_namespace)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScalableTargetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalableTargetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScalableTargetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScalableTargetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut max_capacity = None;
                let mut min_capacity = None;
                let mut resource_id = None;
                let mut role_arn = None;
                let mut scalable_dimension = None;
                let mut scheduled_actions = None;
                let mut service_namespace = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MaxCapacity" => {
                            max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinCapacity" => {
                            min_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceId" => {
                            resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleARN" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScalableDimension" => {
                            scalable_dimension = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduledActions" => {
                            scheduled_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceNamespace" => {
                            service_namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScalableTargetProperties {
                    max_capacity: max_capacity.ok_or(::serde::de::Error::missing_field("MaxCapacity"))?,
                    min_capacity: min_capacity.ok_or(::serde::de::Error::missing_field("MinCapacity"))?,
                    resource_id: resource_id.ok_or(::serde::de::Error::missing_field("ResourceId"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    scalable_dimension: scalable_dimension.ok_or(::serde::de::Error::missing_field("ScalableDimension"))?,
                    scheduled_actions: scheduled_actions,
                    service_namespace: service_namespace.ok_or(::serde::de::Error::missing_field("ServiceNamespace"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ScalableTarget {
    type Properties = ScalableTargetProperties;
    const TYPE: &'static str = "AWS::ApplicationAutoScaling::ScalableTarget";
    fn properties(&self) -> &ScalableTargetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScalableTargetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ScalableTarget {}

impl From<ScalableTargetProperties> for ScalableTarget {
    fn from(properties: ScalableTargetProperties) -> ScalableTarget {
        ScalableTarget { properties }
    }
}

/// The [`AWS::ApplicationAutoScaling::ScalingPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html) resource type.
#[derive(Debug)]
pub struct ScalingPolicy {
    properties: ScalingPolicyProperties
}

/// Properties for the `ScalingPolicy` resource.
#[derive(Debug)]
pub struct ScalingPolicyProperties {
    /// Property `PolicyName`.
    pub policy_name: ::Value<String>,
    /// Property `PolicyType`.
    pub policy_type: ::Value<String>,
    /// Property `ResourceId`.
    pub resource_id: Option<::Value<String>>,
    /// Property `ScalableDimension`.
    pub scalable_dimension: Option<::Value<String>>,
    /// Property `ScalingTargetId`.
    pub scaling_target_id: Option<::Value<String>>,
    /// Property `ServiceNamespace`.
    pub service_namespace: Option<::Value<String>>,
    /// Property `StepScalingPolicyConfiguration`.
    pub step_scaling_policy_configuration: Option<::Value<self::scaling_policy::StepScalingPolicyConfiguration>>,
    /// Property `TargetTrackingScalingPolicyConfiguration`.
    pub target_tracking_scaling_policy_configuration: Option<::Value<self::scaling_policy::TargetTrackingScalingPolicyConfiguration>>,
}

impl ::serde::Serialize for ScalingPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyType", &self.policy_type)?;
        if let Some(ref resource_id) = self.resource_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", resource_id)?;
        }
        if let Some(ref scalable_dimension) = self.scalable_dimension {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalableDimension", scalable_dimension)?;
        }
        if let Some(ref scaling_target_id) = self.scaling_target_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingTargetId", scaling_target_id)?;
        }
        if let Some(ref service_namespace) = self.service_namespace {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNamespace", service_namespace)?;
        }
        if let Some(ref step_scaling_policy_configuration) = self.step_scaling_policy_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StepScalingPolicyConfiguration", step_scaling_policy_configuration)?;
        }
        if let Some(ref target_tracking_scaling_policy_configuration) = self.target_tracking_scaling_policy_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetTrackingScalingPolicyConfiguration", target_tracking_scaling_policy_configuration)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScalingPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScalingPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScalingPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut policy_name = None;
                let mut policy_type = None;
                let mut resource_id = None;
                let mut scalable_dimension = None;
                let mut scaling_target_id = None;
                let mut service_namespace = None;
                let mut step_scaling_policy_configuration = None;
                let mut target_tracking_scaling_policy_configuration = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PolicyName" => {
                            policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyType" => {
                            policy_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceId" => {
                            resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScalableDimension" => {
                            scalable_dimension = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScalingTargetId" => {
                            scaling_target_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceNamespace" => {
                            service_namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StepScalingPolicyConfiguration" => {
                            step_scaling_policy_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetTrackingScalingPolicyConfiguration" => {
                            target_tracking_scaling_policy_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScalingPolicyProperties {
                    policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    policy_type: policy_type.ok_or(::serde::de::Error::missing_field("PolicyType"))?,
                    resource_id: resource_id,
                    scalable_dimension: scalable_dimension,
                    scaling_target_id: scaling_target_id,
                    service_namespace: service_namespace,
                    step_scaling_policy_configuration: step_scaling_policy_configuration,
                    target_tracking_scaling_policy_configuration: target_tracking_scaling_policy_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ScalingPolicy {
    type Properties = ScalingPolicyProperties;
    const TYPE: &'static str = "AWS::ApplicationAutoScaling::ScalingPolicy";
    fn properties(&self) -> &ScalingPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScalingPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ScalingPolicy {}

impl From<ScalingPolicyProperties> for ScalingPolicy {
    fn from(properties: ScalingPolicyProperties) -> ScalingPolicy {
        ScalingPolicy { properties }
    }
}

pub mod scalable_target {
    //! Property types for the `ScalableTarget` resource.

    /// The [`AWS::ApplicationAutoScaling::ScalableTarget.ScalableTargetAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scalabletargetaction.html) property type.
    #[derive(Debug)]
    pub struct ScalableTargetAction {
        /// Property `MaxCapacity`.
        pub max_capacity: Option<::Value<u32>>,
        /// Property `MinCapacity`.
        pub min_capacity: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ScalableTargetAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_capacity) = self.max_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", max_capacity)?;
            }
            if let Some(ref min_capacity) = self.min_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinCapacity", min_capacity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalableTargetAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalableTargetAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalableTargetAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalableTargetAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_capacity = None;
                    let mut min_capacity = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxCapacity" => {
                                max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinCapacity" => {
                                min_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalableTargetAction {
                        max_capacity: max_capacity,
                        min_capacity: min_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationAutoScaling::ScalableTarget.ScheduledAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html) property type.
    #[derive(Debug)]
    pub struct ScheduledAction {
        /// Property `EndTime`.
        pub end_time: Option<::Value<String>>,
        /// Property `ScalableTargetAction`.
        pub scalable_target_action: Option<::Value<ScalableTargetAction>>,
        /// Property `Schedule`.
        pub schedule: ::Value<String>,
        /// Property `ScheduledActionName`.
        pub scheduled_action_name: ::Value<String>,
        /// Property `StartTime`.
        pub start_time: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ScheduledAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref end_time) = self.end_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTime", end_time)?;
            }
            if let Some(ref scalable_target_action) = self.scalable_target_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalableTargetAction", scalable_target_action)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", &self.schedule)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduledActionName", &self.scheduled_action_name)?;
            if let Some(ref start_time) = self.start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", start_time)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScheduledAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduledAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScheduledAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScheduledAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end_time = None;
                    let mut scalable_target_action = None;
                    let mut schedule = None;
                    let mut scheduled_action_name = None;
                    let mut start_time = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndTime" => {
                                end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScalableTargetAction" => {
                                scalable_target_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Schedule" => {
                                schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduledActionName" => {
                                scheduled_action_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTime" => {
                                start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScheduledAction {
                        end_time: end_time,
                        scalable_target_action: scalable_target_action,
                        schedule: schedule.ok_or(::serde::de::Error::missing_field("Schedule"))?,
                        scheduled_action_name: scheduled_action_name.ok_or(::serde::de::Error::missing_field("ScheduledActionName"))?,
                        start_time: start_time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod scaling_policy {
    //! Property types for the `ScalingPolicy` resource.

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-customizedmetricspecification.html) property type.
    #[derive(Debug)]
    pub struct CustomizedMetricSpecification {
        /// Property `Dimensions`.
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property `MetricName`.
        pub metric_name: ::Value<String>,
        /// Property `Namespace`.
        pub namespace: ::Value<String>,
        /// Property `Statistic`.
        pub statistic: ::Value<String>,
        /// Property `Unit`.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CustomizedMetricSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dimensions) = self.dimensions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", dimensions)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", &self.namespace)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", &self.statistic)?;
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomizedMetricSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomizedMetricSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomizedMetricSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomizedMetricSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimensions = None;
                    let mut metric_name = None;
                    let mut namespace = None;
                    let mut statistic = None;
                    let mut unit = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Dimensions" => {
                                dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statistic" => {
                                statistic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomizedMetricSpecification {
                        dimensions: dimensions,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        namespace: namespace.ok_or(::serde::de::Error::missing_field("Namespace"))?,
                        statistic: statistic.ok_or(::serde::de::Error::missing_field("Statistic"))?,
                        unit: unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-metricdimension.html) property type.
    #[derive(Debug)]
    pub struct MetricDimension {
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for MetricDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricDimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricDimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricDimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricDimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut value = None;

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

                    Ok(MetricDimension {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.PredefinedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predefinedmetricspecification.html) property type.
    #[derive(Debug)]
    pub struct PredefinedMetricSpecification {
        /// Property `PredefinedMetricType`.
        pub predefined_metric_type: ::Value<String>,
        /// Property `ResourceLabel`.
        pub resource_label: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PredefinedMetricSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredefinedMetricType", &self.predefined_metric_type)?;
            if let Some(ref resource_label) = self.resource_label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceLabel", resource_label)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PredefinedMetricSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PredefinedMetricSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PredefinedMetricSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PredefinedMetricSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut predefined_metric_type = None;
                    let mut resource_label = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PredefinedMetricType" => {
                                predefined_metric_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceLabel" => {
                                resource_label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PredefinedMetricSpecification {
                        predefined_metric_type: predefined_metric_type.ok_or(::serde::de::Error::missing_field("PredefinedMetricType"))?,
                        resource_label: resource_label,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.StepAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustment.html) property type.
    #[derive(Debug)]
    pub struct StepAdjustment {
        /// Property `MetricIntervalLowerBound`.
        pub metric_interval_lower_bound: Option<::Value<f64>>,
        /// Property `MetricIntervalUpperBound`.
        pub metric_interval_upper_bound: Option<::Value<f64>>,
        /// Property `ScalingAdjustment`.
        pub scaling_adjustment: ::Value<u32>,
    }

    impl ::codec::SerializeValue for StepAdjustment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref metric_interval_lower_bound) = self.metric_interval_lower_bound {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricIntervalLowerBound", metric_interval_lower_bound)?;
            }
            if let Some(ref metric_interval_upper_bound) = self.metric_interval_upper_bound {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricIntervalUpperBound", metric_interval_upper_bound)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingAdjustment", &self.scaling_adjustment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StepAdjustment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StepAdjustment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StepAdjustment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StepAdjustment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metric_interval_lower_bound = None;
                    let mut metric_interval_upper_bound = None;
                    let mut scaling_adjustment = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MetricIntervalLowerBound" => {
                                metric_interval_lower_bound = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricIntervalUpperBound" => {
                                metric_interval_upper_bound = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScalingAdjustment" => {
                                scaling_adjustment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StepAdjustment {
                        metric_interval_lower_bound: metric_interval_lower_bound,
                        metric_interval_upper_bound: metric_interval_upper_bound,
                        scaling_adjustment: scaling_adjustment.ok_or(::serde::de::Error::missing_field("ScalingAdjustment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.StepScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration.html) property type.
    #[derive(Debug)]
    pub struct StepScalingPolicyConfiguration {
        /// Property `AdjustmentType`.
        pub adjustment_type: Option<::Value<String>>,
        /// Property `Cooldown`.
        pub cooldown: Option<::Value<u32>>,
        /// Property `MetricAggregationType`.
        pub metric_aggregation_type: Option<::Value<String>>,
        /// Property `MinAdjustmentMagnitude`.
        pub min_adjustment_magnitude: Option<::Value<u32>>,
        /// Property `StepAdjustments`.
        pub step_adjustments: Option<::ValueList<StepAdjustment>>,
    }

    impl ::codec::SerializeValue for StepScalingPolicyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref adjustment_type) = self.adjustment_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdjustmentType", adjustment_type)?;
            }
            if let Some(ref cooldown) = self.cooldown {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cooldown", cooldown)?;
            }
            if let Some(ref metric_aggregation_type) = self.metric_aggregation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricAggregationType", metric_aggregation_type)?;
            }
            if let Some(ref min_adjustment_magnitude) = self.min_adjustment_magnitude {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinAdjustmentMagnitude", min_adjustment_magnitude)?;
            }
            if let Some(ref step_adjustments) = self.step_adjustments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StepAdjustments", step_adjustments)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StepScalingPolicyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StepScalingPolicyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StepScalingPolicyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StepScalingPolicyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut adjustment_type = None;
                    let mut cooldown = None;
                    let mut metric_aggregation_type = None;
                    let mut min_adjustment_magnitude = None;
                    let mut step_adjustments = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdjustmentType" => {
                                adjustment_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cooldown" => {
                                cooldown = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricAggregationType" => {
                                metric_aggregation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinAdjustmentMagnitude" => {
                                min_adjustment_magnitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StepAdjustments" => {
                                step_adjustments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StepScalingPolicyConfiguration {
                        adjustment_type: adjustment_type,
                        cooldown: cooldown,
                        metric_aggregation_type: metric_aggregation_type,
                        min_adjustment_magnitude: min_adjustment_magnitude,
                        step_adjustments: step_adjustments,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationAutoScaling::ScalingPolicy.TargetTrackingScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html) property type.
    #[derive(Debug)]
    pub struct TargetTrackingScalingPolicyConfiguration {
        /// Property `CustomizedMetricSpecification`.
        pub customized_metric_specification: Option<::Value<CustomizedMetricSpecification>>,
        /// Property `DisableScaleIn`.
        pub disable_scale_in: Option<::Value<bool>>,
        /// Property `PredefinedMetricSpecification`.
        pub predefined_metric_specification: Option<::Value<PredefinedMetricSpecification>>,
        /// Property `ScaleInCooldown`.
        pub scale_in_cooldown: Option<::Value<u32>>,
        /// Property `ScaleOutCooldown`.
        pub scale_out_cooldown: Option<::Value<u32>>,
        /// Property `TargetValue`.
        pub target_value: ::Value<f64>,
    }

    impl ::codec::SerializeValue for TargetTrackingScalingPolicyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref customized_metric_specification) = self.customized_metric_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomizedMetricSpecification", customized_metric_specification)?;
            }
            if let Some(ref disable_scale_in) = self.disable_scale_in {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableScaleIn", disable_scale_in)?;
            }
            if let Some(ref predefined_metric_specification) = self.predefined_metric_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredefinedMetricSpecification", predefined_metric_specification)?;
            }
            if let Some(ref scale_in_cooldown) = self.scale_in_cooldown {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScaleInCooldown", scale_in_cooldown)?;
            }
            if let Some(ref scale_out_cooldown) = self.scale_out_cooldown {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScaleOutCooldown", scale_out_cooldown)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetValue", &self.target_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetTrackingScalingPolicyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetTrackingScalingPolicyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetTrackingScalingPolicyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetTrackingScalingPolicyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut customized_metric_specification = None;
                    let mut disable_scale_in = None;
                    let mut predefined_metric_specification = None;
                    let mut scale_in_cooldown = None;
                    let mut scale_out_cooldown = None;
                    let mut target_value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomizedMetricSpecification" => {
                                customized_metric_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisableScaleIn" => {
                                disable_scale_in = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PredefinedMetricSpecification" => {
                                predefined_metric_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScaleInCooldown" => {
                                scale_in_cooldown = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScaleOutCooldown" => {
                                scale_out_cooldown = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetValue" => {
                                target_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetTrackingScalingPolicyConfiguration {
                        customized_metric_specification: customized_metric_specification,
                        disable_scale_in: disable_scale_in,
                        predefined_metric_specification: predefined_metric_specification,
                        scale_in_cooldown: scale_in_cooldown,
                        scale_out_cooldown: scale_out_cooldown,
                        target_value: target_value.ok_or(::serde::de::Error::missing_field("TargetValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
