//! Types for the `CE` service.

/// The [`AWS::CE::AnomalyMonitor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalymonitor.html) resource type.
#[derive(Debug, Default)]
pub struct AnomalyMonitor {
    properties: AnomalyMonitorProperties
}

/// Properties for the `AnomalyMonitor` resource.
#[derive(Debug, Default)]
pub struct AnomalyMonitorProperties {
    /// Property [`MonitorDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalymonitor.html#cfn-ce-anomalymonitor-monitordimension).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub monitor_dimension: Option<::Value<String>>,
    /// Property [`MonitorName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalymonitor.html#cfn-ce-anomalymonitor-monitorname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub monitor_name: ::Value<String>,
    /// Property [`MonitorSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalymonitor.html#cfn-ce-anomalymonitor-monitorspecification).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub monitor_specification: Option<::Value<String>>,
    /// Property [`MonitorType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalymonitor.html#cfn-ce-anomalymonitor-monitortype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub monitor_type: ::Value<String>,
    /// Property [`ResourceTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalymonitor.html#cfn-ce-anomalymonitor-resourcetags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_tags: Option<::ValueList<self::anomaly_monitor::ResourceTag>>,
}

impl ::serde::Serialize for AnomalyMonitorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref monitor_dimension) = self.monitor_dimension {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitorDimension", monitor_dimension)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitorName", &self.monitor_name)?;
        if let Some(ref monitor_specification) = self.monitor_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitorSpecification", monitor_specification)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitorType", &self.monitor_type)?;
        if let Some(ref resource_tags) = self.resource_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTags", resource_tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AnomalyMonitorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AnomalyMonitorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AnomalyMonitorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AnomalyMonitorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut monitor_dimension: Option<::Value<String>> = None;
                let mut monitor_name: Option<::Value<String>> = None;
                let mut monitor_specification: Option<::Value<String>> = None;
                let mut monitor_type: Option<::Value<String>> = None;
                let mut resource_tags: Option<::ValueList<self::anomaly_monitor::ResourceTag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MonitorDimension" => {
                            monitor_dimension = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitorName" => {
                            monitor_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitorSpecification" => {
                            monitor_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitorType" => {
                            monitor_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceTags" => {
                            resource_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AnomalyMonitorProperties {
                    monitor_dimension: monitor_dimension,
                    monitor_name: monitor_name.ok_or(::serde::de::Error::missing_field("MonitorName"))?,
                    monitor_specification: monitor_specification,
                    monitor_type: monitor_type.ok_or(::serde::de::Error::missing_field("MonitorType"))?,
                    resource_tags: resource_tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AnomalyMonitor {
    type Properties = AnomalyMonitorProperties;
    const TYPE: &'static str = "AWS::CE::AnomalyMonitor";
    fn properties(&self) -> &AnomalyMonitorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AnomalyMonitorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AnomalyMonitor {}

impl From<AnomalyMonitorProperties> for AnomalyMonitor {
    fn from(properties: AnomalyMonitorProperties) -> AnomalyMonitor {
        AnomalyMonitor { properties }
    }
}

/// The [`AWS::CE::AnomalySubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalysubscription.html) resource type.
#[derive(Debug, Default)]
pub struct AnomalySubscription {
    properties: AnomalySubscriptionProperties
}

/// Properties for the `AnomalySubscription` resource.
#[derive(Debug, Default)]
pub struct AnomalySubscriptionProperties {
    /// Property [`Frequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalysubscription.html#cfn-ce-anomalysubscription-frequency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub frequency: ::Value<String>,
    /// Property [`MonitorArnList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalysubscription.html#cfn-ce-anomalysubscription-monitorarnlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub monitor_arn_list: ::ValueList<String>,
    /// Property [`ResourceTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalysubscription.html#cfn-ce-anomalysubscription-resourcetags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_tags: Option<::ValueList<self::anomaly_subscription::ResourceTag>>,
    /// Property [`Subscribers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalysubscription.html#cfn-ce-anomalysubscription-subscribers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subscribers: ::ValueList<self::anomaly_subscription::Subscriber>,
    /// Property [`SubscriptionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalysubscription.html#cfn-ce-anomalysubscription-subscriptionname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subscription_name: ::Value<String>,
    /// Property [`Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalysubscription.html#cfn-ce-anomalysubscription-threshold).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub threshold: Option<::Value<f64>>,
    /// Property [`ThresholdExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-anomalysubscription.html#cfn-ce-anomalysubscription-thresholdexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub threshold_expression: Option<::Value<String>>,
}

impl ::serde::Serialize for AnomalySubscriptionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Frequency", &self.frequency)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitorArnList", &self.monitor_arn_list)?;
        if let Some(ref resource_tags) = self.resource_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTags", resource_tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subscribers", &self.subscribers)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubscriptionName", &self.subscription_name)?;
        if let Some(ref threshold) = self.threshold {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", threshold)?;
        }
        if let Some(ref threshold_expression) = self.threshold_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThresholdExpression", threshold_expression)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AnomalySubscriptionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AnomalySubscriptionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AnomalySubscriptionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AnomalySubscriptionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut frequency: Option<::Value<String>> = None;
                let mut monitor_arn_list: Option<::ValueList<String>> = None;
                let mut resource_tags: Option<::ValueList<self::anomaly_subscription::ResourceTag>> = None;
                let mut subscribers: Option<::ValueList<self::anomaly_subscription::Subscriber>> = None;
                let mut subscription_name: Option<::Value<String>> = None;
                let mut threshold: Option<::Value<f64>> = None;
                let mut threshold_expression: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Frequency" => {
                            frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitorArnList" => {
                            monitor_arn_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceTags" => {
                            resource_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subscribers" => {
                            subscribers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubscriptionName" => {
                            subscription_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Threshold" => {
                            threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThresholdExpression" => {
                            threshold_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AnomalySubscriptionProperties {
                    frequency: frequency.ok_or(::serde::de::Error::missing_field("Frequency"))?,
                    monitor_arn_list: monitor_arn_list.ok_or(::serde::de::Error::missing_field("MonitorArnList"))?,
                    resource_tags: resource_tags,
                    subscribers: subscribers.ok_or(::serde::de::Error::missing_field("Subscribers"))?,
                    subscription_name: subscription_name.ok_or(::serde::de::Error::missing_field("SubscriptionName"))?,
                    threshold: threshold,
                    threshold_expression: threshold_expression,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AnomalySubscription {
    type Properties = AnomalySubscriptionProperties;
    const TYPE: &'static str = "AWS::CE::AnomalySubscription";
    fn properties(&self) -> &AnomalySubscriptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AnomalySubscriptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AnomalySubscription {}

impl From<AnomalySubscriptionProperties> for AnomalySubscription {
    fn from(properties: AnomalySubscriptionProperties) -> AnomalySubscription {
        AnomalySubscription { properties }
    }
}

/// The [`AWS::CE::CostCategory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-costcategory.html) resource type.
#[derive(Debug, Default)]
pub struct CostCategory {
    properties: CostCategoryProperties
}

/// Properties for the `CostCategory` resource.
#[derive(Debug, Default)]
pub struct CostCategoryProperties {
    /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-costcategory.html#cfn-ce-costcategory-defaultvalue).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_value: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-costcategory.html#cfn-ce-costcategory-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RuleVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-costcategory.html#cfn-ce-costcategory-ruleversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_version: ::Value<String>,
    /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-costcategory.html#cfn-ce-costcategory-rules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rules: ::Value<String>,
    /// Property [`SplitChargeRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ce-costcategory.html#cfn-ce-costcategory-splitchargerules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub split_charge_rules: Option<::Value<String>>,
}

impl ::serde::Serialize for CostCategoryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref default_value) = self.default_value {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleVersion", &self.rule_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
        if let Some(ref split_charge_rules) = self.split_charge_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SplitChargeRules", split_charge_rules)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CostCategoryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CostCategoryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CostCategoryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CostCategoryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut default_value: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut rule_version: Option<::Value<String>> = None;
                let mut rules: Option<::Value<String>> = None;
                let mut split_charge_rules: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DefaultValue" => {
                            default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleVersion" => {
                            rule_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Rules" => {
                            rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SplitChargeRules" => {
                            split_charge_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CostCategoryProperties {
                    default_value: default_value,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    rule_version: rule_version.ok_or(::serde::de::Error::missing_field("RuleVersion"))?,
                    rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                    split_charge_rules: split_charge_rules,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CostCategory {
    type Properties = CostCategoryProperties;
    const TYPE: &'static str = "AWS::CE::CostCategory";
    fn properties(&self) -> &CostCategoryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CostCategoryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CostCategory {}

impl From<CostCategoryProperties> for CostCategory {
    fn from(properties: CostCategoryProperties) -> CostCategory {
        CostCategory { properties }
    }
}

pub mod anomaly_monitor {
    //! Property types for the `AnomalyMonitor` resource.

    /// The [`AWS::CE::AnomalyMonitor.ResourceTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalymonitor-resourcetag.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceTag {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalymonitor-resourcetag.html#cfn-ce-anomalymonitor-resourcetag-key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalymonitor-resourcetag.html#cfn-ce-anomalymonitor-resourcetag-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourceTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod anomaly_subscription {
    //! Property types for the `AnomalySubscription` resource.

    /// The [`AWS::CE::AnomalySubscription.ResourceTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalysubscription-resourcetag.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceTag {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalysubscription-resourcetag.html#cfn-ce-anomalysubscription-resourcetag-key).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalysubscription-resourcetag.html#cfn-ce-anomalysubscription-resourcetag-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourceTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::CE::AnomalySubscription.Subscriber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalysubscription-subscriber.html) property type.
    #[derive(Debug, Default)]
    pub struct Subscriber {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalysubscription-subscriber.html#cfn-ce-anomalysubscription-subscriber-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: ::Value<String>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalysubscription-subscriber.html#cfn-ce-anomalysubscription-subscriber-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ce-anomalysubscription-subscriber.html#cfn-ce-anomalysubscription-subscriber-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Subscriber {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", &self.address)?;
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Subscriber {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Subscriber, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Subscriber;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Subscriber")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Subscriber {
                        address: address.ok_or(::serde::de::Error::missing_field("Address"))?,
                        status: status,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
