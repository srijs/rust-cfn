//! Types for the `DLM` service.

/// The [`AWS::DLM::LifecyclePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dlm-lifecyclepolicy.html) resource type.
#[derive(Debug, Default)]
pub struct LifecyclePolicy {
    properties: LifecyclePolicyProperties
}

/// Properties for the `LifecyclePolicy` resource.
#[derive(Debug, Default)]
pub struct LifecyclePolicyProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dlm-lifecyclepolicy.html#cfn-dlm-lifecyclepolicy-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dlm-lifecyclepolicy.html#cfn-dlm-lifecyclepolicy-executionrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_role_arn: Option<::Value<String>>,
    /// Property [`PolicyDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dlm-lifecyclepolicy.html#cfn-dlm-lifecyclepolicy-policydetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_details: Option<::Value<self::lifecycle_policy::PolicyDetails>>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dlm-lifecyclepolicy.html#cfn-dlm-lifecyclepolicy-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dlm-lifecyclepolicy.html#cfn-dlm-lifecyclepolicy-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LifecyclePolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref execution_role_arn) = self.execution_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", execution_role_arn)?;
        }
        if let Some(ref policy_details) = self.policy_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDetails", policy_details)?;
        }
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LifecyclePolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LifecyclePolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LifecyclePolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LifecyclePolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut execution_role_arn: Option<::Value<String>> = None;
                let mut policy_details: Option<::Value<self::lifecycle_policy::PolicyDetails>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionRoleArn" => {
                            execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyDetails" => {
                            policy_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LifecyclePolicyProperties {
                    description: description,
                    execution_role_arn: execution_role_arn,
                    policy_details: policy_details,
                    state: state,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LifecyclePolicy {
    type Properties = LifecyclePolicyProperties;
    const TYPE: &'static str = "AWS::DLM::LifecyclePolicy";
    fn properties(&self) -> &LifecyclePolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LifecyclePolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LifecyclePolicy {}

impl From<LifecyclePolicyProperties> for LifecyclePolicy {
    fn from(properties: LifecyclePolicyProperties) -> LifecyclePolicy {
        LifecyclePolicy { properties }
    }
}

pub mod lifecycle_policy {
    //! Property types for the `LifecyclePolicy` resource.

    /// The [`AWS::DLM::LifecyclePolicy.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`CrossRegionCopy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-action.html#cfn-dlm-lifecyclepolicy-action-crossregioncopy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cross_region_copy: ::ValueList<CrossRegionCopyAction>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-action.html#cfn-dlm-lifecyclepolicy-action-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrossRegionCopy", &self.cross_region_copy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cross_region_copy: Option<::ValueList<CrossRegionCopyAction>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CrossRegionCopy" => {
                                cross_region_copy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        cross_region_copy: cross_region_copy.ok_or(::serde::de::Error::missing_field("CrossRegionCopy"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.CreateRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-createrule.html) property type.
    #[derive(Debug, Default)]
    pub struct CreateRule {
        /// Property [`CronExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-createrule.html#cfn-dlm-lifecyclepolicy-createrule-cronexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cron_expression: Option<::Value<String>>,
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-createrule.html#cfn-dlm-lifecyclepolicy-createrule-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: Option<::Value<u32>>,
        /// Property [`IntervalUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-createrule.html#cfn-dlm-lifecyclepolicy-createrule-intervalunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_unit: Option<::Value<String>>,
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-createrule.html#cfn-dlm-lifecyclepolicy-createrule-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: Option<::Value<String>>,
        /// Property [`Times`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-createrule.html#cfn-dlm-lifecyclepolicy-createrule-times).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub times: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CreateRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cron_expression) = self.cron_expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CronExpression", cron_expression)?;
            }
            if let Some(ref interval) = self.interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", interval)?;
            }
            if let Some(ref interval_unit) = self.interval_unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalUnit", interval_unit)?;
            }
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            if let Some(ref times) = self.times {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Times", times)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CreateRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CreateRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CreateRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CreateRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cron_expression: Option<::Value<String>> = None;
                    let mut interval: Option<::Value<u32>> = None;
                    let mut interval_unit: Option<::Value<String>> = None;
                    let mut location: Option<::Value<String>> = None;
                    let mut times: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CronExpression" => {
                                cron_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntervalUnit" => {
                                interval_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Times" => {
                                times = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CreateRule {
                        cron_expression: cron_expression,
                        interval: interval,
                        interval_unit: interval_unit,
                        location: location,
                        times: times,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.CrossRegionCopyAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CrossRegionCopyAction {
        /// Property [`EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyaction.html#cfn-dlm-lifecyclepolicy-crossregioncopyaction-encryptionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_configuration: ::Value<EncryptionConfiguration>,
        /// Property [`RetainRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyaction.html#cfn-dlm-lifecyclepolicy-crossregioncopyaction-retainrule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retain_rule: Option<::Value<CrossRegionCopyRetainRule>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyaction.html#cfn-dlm-lifecyclepolicy-crossregioncopyaction-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: ::Value<String>,
    }

    impl ::codec::SerializeValue for CrossRegionCopyAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", &self.encryption_configuration)?;
            if let Some(ref retain_rule) = self.retain_rule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetainRule", retain_rule)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CrossRegionCopyAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CrossRegionCopyAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CrossRegionCopyAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CrossRegionCopyAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_configuration: Option<::Value<EncryptionConfiguration>> = None;
                    let mut retain_rule: Option<::Value<CrossRegionCopyRetainRule>> = None;
                    let mut target: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionConfiguration" => {
                                encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetainRule" => {
                                retain_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CrossRegionCopyAction {
                        encryption_configuration: encryption_configuration.ok_or(::serde::de::Error::missing_field("EncryptionConfiguration"))?,
                        retain_rule: retain_rule,
                        target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.CrossRegionCopyDeprecateRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopydeprecaterule.html) property type.
    #[derive(Debug, Default)]
    pub struct CrossRegionCopyDeprecateRule {
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopydeprecaterule.html#cfn-dlm-lifecyclepolicy-crossregioncopydeprecaterule-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: ::Value<u32>,
        /// Property [`IntervalUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopydeprecaterule.html#cfn-dlm-lifecyclepolicy-crossregioncopydeprecaterule-intervalunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_unit: ::Value<String>,
    }

    impl ::codec::SerializeValue for CrossRegionCopyDeprecateRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", &self.interval)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalUnit", &self.interval_unit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CrossRegionCopyDeprecateRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CrossRegionCopyDeprecateRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CrossRegionCopyDeprecateRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CrossRegionCopyDeprecateRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut interval: Option<::Value<u32>> = None;
                    let mut interval_unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntervalUnit" => {
                                interval_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CrossRegionCopyDeprecateRule {
                        interval: interval.ok_or(::serde::de::Error::missing_field("Interval"))?,
                        interval_unit: interval_unit.ok_or(::serde::de::Error::missing_field("IntervalUnit"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.CrossRegionCopyRetainRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyretainrule.html) property type.
    #[derive(Debug, Default)]
    pub struct CrossRegionCopyRetainRule {
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyretainrule.html#cfn-dlm-lifecyclepolicy-crossregioncopyretainrule-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: ::Value<u32>,
        /// Property [`IntervalUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyretainrule.html#cfn-dlm-lifecyclepolicy-crossregioncopyretainrule-intervalunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_unit: ::Value<String>,
    }

    impl ::codec::SerializeValue for CrossRegionCopyRetainRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", &self.interval)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalUnit", &self.interval_unit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CrossRegionCopyRetainRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CrossRegionCopyRetainRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CrossRegionCopyRetainRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CrossRegionCopyRetainRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut interval: Option<::Value<u32>> = None;
                    let mut interval_unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntervalUnit" => {
                                interval_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CrossRegionCopyRetainRule {
                        interval: interval.ok_or(::serde::de::Error::missing_field("Interval"))?,
                        interval_unit: interval_unit.ok_or(::serde::de::Error::missing_field("IntervalUnit"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.CrossRegionCopyRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyrule.html) property type.
    #[derive(Debug, Default)]
    pub struct CrossRegionCopyRule {
        /// Property [`CmkArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyrule.html#cfn-dlm-lifecyclepolicy-crossregioncopyrule-cmkarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cmk_arn: Option<::Value<String>>,
        /// Property [`CopyTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyrule.html#cfn-dlm-lifecyclepolicy-crossregioncopyrule-copytags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_tags: Option<::Value<bool>>,
        /// Property [`DeprecateRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyrule.html#cfn-dlm-lifecyclepolicy-crossregioncopyrule-deprecaterule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub deprecate_rule: Option<::Value<CrossRegionCopyDeprecateRule>>,
        /// Property [`Encrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyrule.html#cfn-dlm-lifecyclepolicy-crossregioncopyrule-encrypted).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encrypted: ::Value<bool>,
        /// Property [`RetainRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyrule.html#cfn-dlm-lifecyclepolicy-crossregioncopyrule-retainrule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retain_rule: Option<::Value<CrossRegionCopyRetainRule>>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyrule.html#cfn-dlm-lifecyclepolicy-crossregioncopyrule-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: Option<::Value<String>>,
        /// Property [`TargetRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-crossregioncopyrule.html#cfn-dlm-lifecyclepolicy-crossregioncopyrule-targetregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_region: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CrossRegionCopyRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cmk_arn) = self.cmk_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CmkArn", cmk_arn)?;
            }
            if let Some(ref copy_tags) = self.copy_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTags", copy_tags)?;
            }
            if let Some(ref deprecate_rule) = self.deprecate_rule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeprecateRule", deprecate_rule)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", &self.encrypted)?;
            if let Some(ref retain_rule) = self.retain_rule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetainRule", retain_rule)?;
            }
            if let Some(ref target) = self.target {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", target)?;
            }
            if let Some(ref target_region) = self.target_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetRegion", target_region)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CrossRegionCopyRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CrossRegionCopyRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CrossRegionCopyRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CrossRegionCopyRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cmk_arn: Option<::Value<String>> = None;
                    let mut copy_tags: Option<::Value<bool>> = None;
                    let mut deprecate_rule: Option<::Value<CrossRegionCopyDeprecateRule>> = None;
                    let mut encrypted: Option<::Value<bool>> = None;
                    let mut retain_rule: Option<::Value<CrossRegionCopyRetainRule>> = None;
                    let mut target: Option<::Value<String>> = None;
                    let mut target_region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CmkArn" => {
                                cmk_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CopyTags" => {
                                copy_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeprecateRule" => {
                                deprecate_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Encrypted" => {
                                encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetainRule" => {
                                retain_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetRegion" => {
                                target_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CrossRegionCopyRule {
                        cmk_arn: cmk_arn,
                        copy_tags: copy_tags,
                        deprecate_rule: deprecate_rule,
                        encrypted: encrypted.ok_or(::serde::de::Error::missing_field("Encrypted"))?,
                        retain_rule: retain_rule,
                        target: target,
                        target_region: target_region,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.DeprecateRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-deprecaterule.html) property type.
    #[derive(Debug, Default)]
    pub struct DeprecateRule {
        /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-deprecaterule.html#cfn-dlm-lifecyclepolicy-deprecaterule-count).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub count: Option<::Value<u32>>,
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-deprecaterule.html#cfn-dlm-lifecyclepolicy-deprecaterule-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: Option<::Value<u32>>,
        /// Property [`IntervalUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-deprecaterule.html#cfn-dlm-lifecyclepolicy-deprecaterule-intervalunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeprecateRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref count) = self.count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
            }
            if let Some(ref interval) = self.interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", interval)?;
            }
            if let Some(ref interval_unit) = self.interval_unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalUnit", interval_unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeprecateRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeprecateRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeprecateRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeprecateRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut count: Option<::Value<u32>> = None;
                    let mut interval: Option<::Value<u32>> = None;
                    let mut interval_unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Count" => {
                                count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntervalUnit" => {
                                interval_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeprecateRule {
                        count: count,
                        interval: interval,
                        interval_unit: interval_unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-encryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionConfiguration {
        /// Property [`CmkArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-encryptionconfiguration.html#cfn-dlm-lifecyclepolicy-encryptionconfiguration-cmkarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cmk_arn: Option<::Value<String>>,
        /// Property [`Encrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-encryptionconfiguration.html#cfn-dlm-lifecyclepolicy-encryptionconfiguration-encrypted).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encrypted: ::Value<bool>,
    }

    impl ::codec::SerializeValue for EncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cmk_arn) = self.cmk_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CmkArn", cmk_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", &self.encrypted)?;
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
                    let mut cmk_arn: Option<::Value<String>> = None;
                    let mut encrypted: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CmkArn" => {
                                cmk_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Encrypted" => {
                                encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfiguration {
                        cmk_arn: cmk_arn,
                        encrypted: encrypted.ok_or(::serde::de::Error::missing_field("Encrypted"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.EventParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-eventparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct EventParameters {
        /// Property [`DescriptionRegex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-eventparameters.html#cfn-dlm-lifecyclepolicy-eventparameters-descriptionregex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description_regex: Option<::Value<String>>,
        /// Property [`EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-eventparameters.html#cfn-dlm-lifecyclepolicy-eventparameters-eventtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_type: ::Value<String>,
        /// Property [`SnapshotOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-eventparameters.html#cfn-dlm-lifecyclepolicy-eventparameters-snapshotowner).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snapshot_owner: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for EventParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description_regex) = self.description_regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DescriptionRegex", description_regex)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventType", &self.event_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotOwner", &self.snapshot_owner)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description_regex: Option<::Value<String>> = None;
                    let mut event_type: Option<::Value<String>> = None;
                    let mut snapshot_owner: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DescriptionRegex" => {
                                description_regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventType" => {
                                event_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnapshotOwner" => {
                                snapshot_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventParameters {
                        description_regex: description_regex,
                        event_type: event_type.ok_or(::serde::de::Error::missing_field("EventType"))?,
                        snapshot_owner: snapshot_owner.ok_or(::serde::de::Error::missing_field("SnapshotOwner"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.EventSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-eventsource.html) property type.
    #[derive(Debug, Default)]
    pub struct EventSource {
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-eventsource.html#cfn-dlm-lifecyclepolicy-eventsource-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<EventParameters>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-eventsource.html#cfn-dlm-lifecyclepolicy-eventsource-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for EventSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameters: Option<::Value<EventParameters>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventSource {
                        parameters: parameters,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.FastRestoreRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-fastrestorerule.html) property type.
    #[derive(Debug, Default)]
    pub struct FastRestoreRule {
        /// Property [`AvailabilityZones`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-fastrestorerule.html#cfn-dlm-lifecyclepolicy-fastrestorerule-availabilityzones).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_zones: Option<::ValueList<String>>,
        /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-fastrestorerule.html#cfn-dlm-lifecyclepolicy-fastrestorerule-count).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub count: Option<::Value<u32>>,
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-fastrestorerule.html#cfn-dlm-lifecyclepolicy-fastrestorerule-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: Option<::Value<u32>>,
        /// Property [`IntervalUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-fastrestorerule.html#cfn-dlm-lifecyclepolicy-fastrestorerule-intervalunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FastRestoreRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_zones) = self.availability_zones {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZones", availability_zones)?;
            }
            if let Some(ref count) = self.count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
            }
            if let Some(ref interval) = self.interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", interval)?;
            }
            if let Some(ref interval_unit) = self.interval_unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalUnit", interval_unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FastRestoreRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FastRestoreRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FastRestoreRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FastRestoreRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zones: Option<::ValueList<String>> = None;
                    let mut count: Option<::Value<u32>> = None;
                    let mut interval: Option<::Value<u32>> = None;
                    let mut interval_unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZones" => {
                                availability_zones = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Count" => {
                                count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntervalUnit" => {
                                interval_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FastRestoreRule {
                        availability_zones: availability_zones,
                        count: count,
                        interval: interval,
                        interval_unit: interval_unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-parameters.html) property type.
    #[derive(Debug, Default)]
    pub struct Parameters {
        /// Property [`ExcludeBootVolume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-parameters.html#cfn-dlm-lifecyclepolicy-parameters-excludebootvolume).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_boot_volume: Option<::Value<bool>>,
        /// Property [`NoReboot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-parameters.html#cfn-dlm-lifecyclepolicy-parameters-noreboot).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_reboot: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for Parameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exclude_boot_volume) = self.exclude_boot_volume {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeBootVolume", exclude_boot_volume)?;
            }
            if let Some(ref no_reboot) = self.no_reboot {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoReboot", no_reboot)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Parameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Parameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Parameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Parameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exclude_boot_volume: Option<::Value<bool>> = None;
                    let mut no_reboot: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExcludeBootVolume" => {
                                exclude_boot_volume = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoReboot" => {
                                no_reboot = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Parameters {
                        exclude_boot_volume: exclude_boot_volume,
                        no_reboot: no_reboot,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.PolicyDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-policydetails.html) property type.
    #[derive(Debug, Default)]
    pub struct PolicyDetails {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-policydetails.html#cfn-dlm-lifecyclepolicy-policydetails-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: Option<::ValueList<Action>>,
        /// Property [`EventSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-policydetails.html#cfn-dlm-lifecyclepolicy-policydetails-eventsource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_source: Option<::Value<EventSource>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-policydetails.html#cfn-dlm-lifecyclepolicy-policydetails-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<Parameters>>,
        /// Property [`PolicyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-policydetails.html#cfn-dlm-lifecyclepolicy-policydetails-policytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_type: Option<::Value<String>>,
        /// Property [`ResourceLocations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-policydetails.html#cfn-dlm-lifecyclepolicy-policydetails-resourcelocations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_locations: Option<::ValueList<String>>,
        /// Property [`ResourceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-policydetails.html#cfn-dlm-lifecyclepolicy-policydetails-resourcetypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_types: Option<::ValueList<String>>,
        /// Property [`Schedules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-policydetails.html#cfn-dlm-lifecyclepolicy-policydetails-schedules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedules: Option<::ValueList<Schedule>>,
        /// Property [`TargetTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-policydetails.html#cfn-dlm-lifecyclepolicy-policydetails-targettags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for PolicyDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref actions) = self.actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", actions)?;
            }
            if let Some(ref event_source) = self.event_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSource", event_source)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            if let Some(ref policy_type) = self.policy_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyType", policy_type)?;
            }
            if let Some(ref resource_locations) = self.resource_locations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceLocations", resource_locations)?;
            }
            if let Some(ref resource_types) = self.resource_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTypes", resource_types)?;
            }
            if let Some(ref schedules) = self.schedules {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedules", schedules)?;
            }
            if let Some(ref target_tags) = self.target_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetTags", target_tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PolicyDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PolicyDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PolicyDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<Action>> = None;
                    let mut event_source: Option<::Value<EventSource>> = None;
                    let mut parameters: Option<::Value<Parameters>> = None;
                    let mut policy_type: Option<::Value<String>> = None;
                    let mut resource_locations: Option<::ValueList<String>> = None;
                    let mut resource_types: Option<::ValueList<String>> = None;
                    let mut schedules: Option<::ValueList<Schedule>> = None;
                    let mut target_tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventSource" => {
                                event_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyType" => {
                                policy_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceLocations" => {
                                resource_locations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceTypes" => {
                                resource_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Schedules" => {
                                schedules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetTags" => {
                                target_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PolicyDetails {
                        actions: actions,
                        event_source: event_source,
                        parameters: parameters,
                        policy_type: policy_type,
                        resource_locations: resource_locations,
                        resource_types: resource_types,
                        schedules: schedules,
                        target_tags: target_tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.RetainRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-retainrule.html) property type.
    #[derive(Debug, Default)]
    pub struct RetainRule {
        /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-retainrule.html#cfn-dlm-lifecyclepolicy-retainrule-count).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub count: Option<::Value<u32>>,
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-retainrule.html#cfn-dlm-lifecyclepolicy-retainrule-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: Option<::Value<u32>>,
        /// Property [`IntervalUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-retainrule.html#cfn-dlm-lifecyclepolicy-retainrule-intervalunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval_unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RetainRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref count) = self.count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
            }
            if let Some(ref interval) = self.interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", interval)?;
            }
            if let Some(ref interval_unit) = self.interval_unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntervalUnit", interval_unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RetainRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RetainRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RetainRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RetainRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut count: Option<::Value<u32>> = None;
                    let mut interval: Option<::Value<u32>> = None;
                    let mut interval_unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Count" => {
                                count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntervalUnit" => {
                                interval_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RetainRule {
                        count: count,
                        interval: interval,
                        interval_unit: interval_unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html) property type.
    #[derive(Debug, Default)]
    pub struct Schedule {
        /// Property [`CopyTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html#cfn-dlm-lifecyclepolicy-schedule-copytags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_tags: Option<::Value<bool>>,
        /// Property [`CreateRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html#cfn-dlm-lifecyclepolicy-schedule-createrule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub create_rule: Option<::Value<CreateRule>>,
        /// Property [`CrossRegionCopyRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html#cfn-dlm-lifecyclepolicy-schedule-crossregioncopyrules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cross_region_copy_rules: Option<::ValueList<CrossRegionCopyRule>>,
        /// Property [`DeprecateRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html#cfn-dlm-lifecyclepolicy-schedule-deprecaterule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub deprecate_rule: Option<::Value<DeprecateRule>>,
        /// Property [`FastRestoreRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html#cfn-dlm-lifecyclepolicy-schedule-fastrestorerule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fast_restore_rule: Option<::Value<FastRestoreRule>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html#cfn-dlm-lifecyclepolicy-schedule-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`RetainRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html#cfn-dlm-lifecyclepolicy-schedule-retainrule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retain_rule: Option<::Value<RetainRule>>,
        /// Property [`ShareRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html#cfn-dlm-lifecyclepolicy-schedule-sharerules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub share_rules: Option<::ValueList<ShareRule>>,
        /// Property [`TagsToAdd`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html#cfn-dlm-lifecyclepolicy-schedule-tagstoadd).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags_to_add: Option<::ValueList<::Tag>>,
        /// Property [`VariableTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-schedule.html#cfn-dlm-lifecyclepolicy-schedule-variabletags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variable_tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for Schedule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref copy_tags) = self.copy_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTags", copy_tags)?;
            }
            if let Some(ref create_rule) = self.create_rule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateRule", create_rule)?;
            }
            if let Some(ref cross_region_copy_rules) = self.cross_region_copy_rules {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrossRegionCopyRules", cross_region_copy_rules)?;
            }
            if let Some(ref deprecate_rule) = self.deprecate_rule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeprecateRule", deprecate_rule)?;
            }
            if let Some(ref fast_restore_rule) = self.fast_restore_rule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FastRestoreRule", fast_restore_rule)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref retain_rule) = self.retain_rule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetainRule", retain_rule)?;
            }
            if let Some(ref share_rules) = self.share_rules {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShareRules", share_rules)?;
            }
            if let Some(ref tags_to_add) = self.tags_to_add {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagsToAdd", tags_to_add)?;
            }
            if let Some(ref variable_tags) = self.variable_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariableTags", variable_tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Schedule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Schedule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Schedule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Schedule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut copy_tags: Option<::Value<bool>> = None;
                    let mut create_rule: Option<::Value<CreateRule>> = None;
                    let mut cross_region_copy_rules: Option<::ValueList<CrossRegionCopyRule>> = None;
                    let mut deprecate_rule: Option<::Value<DeprecateRule>> = None;
                    let mut fast_restore_rule: Option<::Value<FastRestoreRule>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut retain_rule: Option<::Value<RetainRule>> = None;
                    let mut share_rules: Option<::ValueList<ShareRule>> = None;
                    let mut tags_to_add: Option<::ValueList<::Tag>> = None;
                    let mut variable_tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopyTags" => {
                                copy_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreateRule" => {
                                create_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CrossRegionCopyRules" => {
                                cross_region_copy_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeprecateRule" => {
                                deprecate_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FastRestoreRule" => {
                                fast_restore_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetainRule" => {
                                retain_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShareRules" => {
                                share_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagsToAdd" => {
                                tags_to_add = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VariableTags" => {
                                variable_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Schedule {
                        copy_tags: copy_tags,
                        create_rule: create_rule,
                        cross_region_copy_rules: cross_region_copy_rules,
                        deprecate_rule: deprecate_rule,
                        fast_restore_rule: fast_restore_rule,
                        name: name,
                        retain_rule: retain_rule,
                        share_rules: share_rules,
                        tags_to_add: tags_to_add,
                        variable_tags: variable_tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DLM::LifecyclePolicy.ShareRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-sharerule.html) property type.
    #[derive(Debug, Default)]
    pub struct ShareRule {
        /// Property [`TargetAccounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-sharerule.html#cfn-dlm-lifecyclepolicy-sharerule-targetaccounts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_accounts: Option<::ValueList<String>>,
        /// Property [`UnshareInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-sharerule.html#cfn-dlm-lifecyclepolicy-sharerule-unshareinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unshare_interval: Option<::Value<u32>>,
        /// Property [`UnshareIntervalUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dlm-lifecyclepolicy-sharerule.html#cfn-dlm-lifecyclepolicy-sharerule-unshareintervalunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unshare_interval_unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ShareRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref target_accounts) = self.target_accounts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetAccounts", target_accounts)?;
            }
            if let Some(ref unshare_interval) = self.unshare_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnshareInterval", unshare_interval)?;
            }
            if let Some(ref unshare_interval_unit) = self.unshare_interval_unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnshareIntervalUnit", unshare_interval_unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ShareRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ShareRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ShareRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ShareRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut target_accounts: Option<::ValueList<String>> = None;
                    let mut unshare_interval: Option<::Value<u32>> = None;
                    let mut unshare_interval_unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TargetAccounts" => {
                                target_accounts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnshareInterval" => {
                                unshare_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnshareIntervalUnit" => {
                                unshare_interval_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ShareRule {
                        target_accounts: target_accounts,
                        unshare_interval: unshare_interval,
                        unshare_interval_unit: unshare_interval_unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
