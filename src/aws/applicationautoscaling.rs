//! Types for the `ApplicationAutoScaling` service.

/// The [`AWS::ApplicationAutoScaling::ScalableTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html) resource type.
#[derive(Debug, Default)]
pub struct ScalableTarget {
    properties: ScalableTargetProperties
}

/// Properties for the `ScalableTarget` resource.
#[derive(Debug, Default)]
pub struct ScalableTargetProperties {
    /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html#cfn-applicationautoscaling-scalabletarget-maxcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_capacity: ::Value<u32>,
    /// Property [`MinCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html#cfn-applicationautoscaling-scalabletarget-mincapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_capacity: ::Value<u32>,
    /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html#cfn-applicationautoscaling-scalabletarget-resourceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_id: ::Value<String>,
    /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html#cfn-applicationautoscaling-scalabletarget-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`ScalableDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html#cfn-applicationautoscaling-scalabletarget-scalabledimension).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scalable_dimension: ::Value<String>,
    /// Property [`ScheduledActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html#cfn-applicationautoscaling-scalabletarget-scheduledactions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scheduled_actions: Option<::ValueList<self::scalable_target::ScheduledAction>>,
    /// Property [`ServiceNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html#cfn-applicationautoscaling-scalabletarget-servicenamespace).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_namespace: ::Value<String>,
    /// Property [`SuspendedState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalabletarget.html#cfn-applicationautoscaling-scalabletarget-suspendedstate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub suspended_state: Option<::Value<self::scalable_target::SuspendedState>>,
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
        if let Some(ref suspended_state) = self.suspended_state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuspendedState", suspended_state)?;
        }
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
                let mut max_capacity: Option<::Value<u32>> = None;
                let mut min_capacity: Option<::Value<u32>> = None;
                let mut resource_id: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut scalable_dimension: Option<::Value<String>> = None;
                let mut scheduled_actions: Option<::ValueList<self::scalable_target::ScheduledAction>> = None;
                let mut service_namespace: Option<::Value<String>> = None;
                let mut suspended_state: Option<::Value<self::scalable_target::SuspendedState>> = None;

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
                        "SuspendedState" => {
                            suspended_state = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    suspended_state: suspended_state,
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
#[derive(Debug, Default)]
pub struct ScalingPolicy {
    properties: ScalingPolicyProperties
}

/// Properties for the `ScalingPolicy` resource.
#[derive(Debug, Default)]
pub struct ScalingPolicyProperties {
    /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html#cfn-applicationautoscaling-scalingpolicy-policyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub policy_name: ::Value<String>,
    /// Property [`PolicyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html#cfn-applicationautoscaling-scalingpolicy-policytype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_type: ::Value<String>,
    /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html#cfn-applicationautoscaling-scalingpolicy-resourceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_id: Option<::Value<String>>,
    /// Property [`ScalableDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html#cfn-applicationautoscaling-scalingpolicy-scalabledimension).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scalable_dimension: Option<::Value<String>>,
    /// Property [`ScalingTargetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html#cfn-applicationautoscaling-scalingpolicy-scalingtargetid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scaling_target_id: Option<::Value<String>>,
    /// Property [`ServiceNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html#cfn-applicationautoscaling-scalingpolicy-servicenamespace).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_namespace: Option<::Value<String>>,
    /// Property [`StepScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html#cfn-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub step_scaling_policy_configuration: Option<::Value<self::scaling_policy::StepScalingPolicyConfiguration>>,
    /// Property [`TargetTrackingScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-applicationautoscaling-scalingpolicy.html#cfn-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                let mut policy_name: Option<::Value<String>> = None;
                let mut policy_type: Option<::Value<String>> = None;
                let mut resource_id: Option<::Value<String>> = None;
                let mut scalable_dimension: Option<::Value<String>> = None;
                let mut scaling_target_id: Option<::Value<String>> = None;
                let mut service_namespace: Option<::Value<String>> = None;
                let mut step_scaling_policy_configuration: Option<::Value<self::scaling_policy::StepScalingPolicyConfiguration>> = None;
                let mut target_tracking_scaling_policy_configuration: Option<::Value<self::scaling_policy::TargetTrackingScalingPolicyConfiguration>> = None;

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
    #[derive(Debug, Default)]
    pub struct ScalableTargetAction {
        /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scalabletargetaction.html#cfn-applicationautoscaling-scalabletarget-scalabletargetaction-maxcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_capacity: Option<::Value<u32>>,
        /// Property [`MinCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scalabletargetaction.html#cfn-applicationautoscaling-scalabletarget-scalabletargetaction-mincapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut max_capacity: Option<::Value<u32>> = None;
                    let mut min_capacity: Option<::Value<u32>> = None;

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
    #[derive(Debug, Default)]
    pub struct ScheduledAction {
        /// Property [`EndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html#cfn-applicationautoscaling-scalabletarget-scheduledaction-endtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_time: Option<::Value<String>>,
        /// Property [`ScalableTargetAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html#cfn-applicationautoscaling-scalabletarget-scheduledaction-scalabletargetaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scalable_target_action: Option<::Value<ScalableTargetAction>>,
        /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html#cfn-applicationautoscaling-scalabletarget-scheduledaction-schedule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule: ::Value<String>,
        /// Property [`ScheduledActionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html#cfn-applicationautoscaling-scalabletarget-scheduledaction-scheduledactionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scheduled_action_name: ::Value<String>,
        /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html#cfn-applicationautoscaling-scalabletarget-scheduledaction-starttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time: Option<::Value<String>>,
        /// Property [`Timezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-scheduledaction.html#cfn-applicationautoscaling-scalabletarget-scheduledaction-timezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timezone: Option<::Value<String>>,
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
            if let Some(ref timezone) = self.timezone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timezone", timezone)?;
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
                    let mut end_time: Option<::Value<String>> = None;
                    let mut scalable_target_action: Option<::Value<ScalableTargetAction>> = None;
                    let mut schedule: Option<::Value<String>> = None;
                    let mut scheduled_action_name: Option<::Value<String>> = None;
                    let mut start_time: Option<::Value<String>> = None;
                    let mut timezone: Option<::Value<String>> = None;

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
                            "Timezone" => {
                                timezone = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        timezone: timezone,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ApplicationAutoScaling::ScalableTarget.SuspendedState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-suspendedstate.html) property type.
    #[derive(Debug, Default)]
    pub struct SuspendedState {
        /// Property [`DynamicScalingInSuspended`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-suspendedstate.html#cfn-applicationautoscaling-scalabletarget-suspendedstate-dynamicscalinginsuspended).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamic_scaling_in_suspended: Option<::Value<bool>>,
        /// Property [`DynamicScalingOutSuspended`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-suspendedstate.html#cfn-applicationautoscaling-scalabletarget-suspendedstate-dynamicscalingoutsuspended).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamic_scaling_out_suspended: Option<::Value<bool>>,
        /// Property [`ScheduledScalingSuspended`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalabletarget-suspendedstate.html#cfn-applicationautoscaling-scalabletarget-suspendedstate-scheduledscalingsuspended).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scheduled_scaling_suspended: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SuspendedState {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dynamic_scaling_in_suspended) = self.dynamic_scaling_in_suspended {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamicScalingInSuspended", dynamic_scaling_in_suspended)?;
            }
            if let Some(ref dynamic_scaling_out_suspended) = self.dynamic_scaling_out_suspended {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamicScalingOutSuspended", dynamic_scaling_out_suspended)?;
            }
            if let Some(ref scheduled_scaling_suspended) = self.scheduled_scaling_suspended {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduledScalingSuspended", scheduled_scaling_suspended)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SuspendedState {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SuspendedState, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SuspendedState;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SuspendedState")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dynamic_scaling_in_suspended: Option<::Value<bool>> = None;
                    let mut dynamic_scaling_out_suspended: Option<::Value<bool>> = None;
                    let mut scheduled_scaling_suspended: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DynamicScalingInSuspended" => {
                                dynamic_scaling_in_suspended = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DynamicScalingOutSuspended" => {
                                dynamic_scaling_out_suspended = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduledScalingSuspended" => {
                                scheduled_scaling_suspended = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SuspendedState {
                        dynamic_scaling_in_suspended: dynamic_scaling_in_suspended,
                        dynamic_scaling_out_suspended: dynamic_scaling_out_suspended,
                        scheduled_scaling_suspended: scheduled_scaling_suspended,
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
    #[derive(Debug, Default)]
    pub struct CustomizedMetricSpecification {
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-customizedmetricspecification.html#cfn-applicationautoscaling-scalingpolicy-customizedmetricspecification-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-customizedmetricspecification.html#cfn-applicationautoscaling-scalingpolicy-customizedmetricspecification-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-customizedmetricspecification.html#cfn-applicationautoscaling-scalingpolicy-customizedmetricspecification-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: ::Value<String>,
        /// Property [`Statistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-customizedmetricspecification.html#cfn-applicationautoscaling-scalingpolicy-customizedmetricspecification-statistic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statistic: ::Value<String>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-customizedmetricspecification.html#cfn-applicationautoscaling-scalingpolicy-customizedmetricspecification-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut dimensions: Option<::ValueList<MetricDimension>> = None;
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut namespace: Option<::Value<String>> = None;
                    let mut statistic: Option<::Value<String>> = None;
                    let mut unit: Option<::Value<String>> = None;

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
    #[derive(Debug, Default)]
    pub struct MetricDimension {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-metricdimension.html#cfn-applicationautoscaling-scalingpolicy-metricdimension-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-metricdimension.html#cfn-applicationautoscaling-scalingpolicy-metricdimension-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
    #[derive(Debug, Default)]
    pub struct PredefinedMetricSpecification {
        /// Property [`PredefinedMetricType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predefinedmetricspecification.html#cfn-applicationautoscaling-scalingpolicy-predefinedmetricspecification-predefinedmetrictype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predefined_metric_type: ::Value<String>,
        /// Property [`ResourceLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-predefinedmetricspecification.html#cfn-applicationautoscaling-scalingpolicy-predefinedmetricspecification-resourcelabel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut predefined_metric_type: Option<::Value<String>> = None;
                    let mut resource_label: Option<::Value<String>> = None;

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
    #[derive(Debug, Default)]
    pub struct StepAdjustment {
        /// Property [`MetricIntervalLowerBound`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustment.html#cfn-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustment-metricintervallowerbound).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_interval_lower_bound: Option<::Value<f64>>,
        /// Property [`MetricIntervalUpperBound`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustment.html#cfn-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustment-metricintervalupperbound).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_interval_upper_bound: Option<::Value<f64>>,
        /// Property [`ScalingAdjustment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustment.html#cfn-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustment-scalingadjustment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut metric_interval_lower_bound: Option<::Value<f64>> = None;
                    let mut metric_interval_upper_bound: Option<::Value<f64>> = None;
                    let mut scaling_adjustment: Option<::Value<u32>> = None;

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
    #[derive(Debug, Default)]
    pub struct StepScalingPolicyConfiguration {
        /// Property [`AdjustmentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-adjustmenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub adjustment_type: Option<::Value<String>>,
        /// Property [`Cooldown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-cooldown).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cooldown: Option<::Value<u32>>,
        /// Property [`MetricAggregationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-metricaggregationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_aggregation_type: Option<::Value<String>>,
        /// Property [`MinAdjustmentMagnitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-minadjustmentmagnitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_adjustment_magnitude: Option<::Value<u32>>,
        /// Property [`StepAdjustments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-stepscalingpolicyconfiguration-stepadjustments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut adjustment_type: Option<::Value<String>> = None;
                    let mut cooldown: Option<::Value<u32>> = None;
                    let mut metric_aggregation_type: Option<::Value<String>> = None;
                    let mut min_adjustment_magnitude: Option<::Value<u32>> = None;
                    let mut step_adjustments: Option<::ValueList<StepAdjustment>> = None;

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
    #[derive(Debug, Default)]
    pub struct TargetTrackingScalingPolicyConfiguration {
        /// Property [`CustomizedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration-customizedmetricspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub customized_metric_specification: Option<::Value<CustomizedMetricSpecification>>,
        /// Property [`DisableScaleIn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration-disablescalein).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_scale_in: Option<::Value<bool>>,
        /// Property [`PredefinedMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration-predefinedmetricspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predefined_metric_specification: Option<::Value<PredefinedMetricSpecification>>,
        /// Property [`ScaleInCooldown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration-scaleincooldown).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scale_in_cooldown: Option<::Value<u32>>,
        /// Property [`ScaleOutCooldown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration-scaleoutcooldown).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scale_out_cooldown: Option<::Value<u32>>,
        /// Property [`TargetValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration.html#cfn-applicationautoscaling-scalingpolicy-targettrackingscalingpolicyconfiguration-targetvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut customized_metric_specification: Option<::Value<CustomizedMetricSpecification>> = None;
                    let mut disable_scale_in: Option<::Value<bool>> = None;
                    let mut predefined_metric_specification: Option<::Value<PredefinedMetricSpecification>> = None;
                    let mut scale_in_cooldown: Option<::Value<u32>> = None;
                    let mut scale_out_cooldown: Option<::Value<u32>> = None;
                    let mut target_value: Option<::Value<f64>> = None;

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
