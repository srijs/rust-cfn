//! Types for the `XRay` service.

/// The [`AWS::XRay::Group`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-group.html) resource type.
#[derive(Debug, Default)]
pub struct Group {
    properties: GroupProperties
}

/// Properties for the `Group` resource.
#[derive(Debug, Default)]
pub struct GroupProperties {
    /// Property [`FilterExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-group.html#cfn-xray-group-filterexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub filter_expression: Option<::Value<String>>,
    /// Property [`GroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-group.html#cfn-xray-group-groupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub group_name: Option<::Value<String>>,
    /// Property [`InsightsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-group.html#cfn-xray-group-insightsconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub insights_configuration: Option<::Value<self::group::InsightsConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-group.html#cfn-xray-group-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::json::Value>>,
}

impl ::serde::Serialize for GroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref filter_expression) = self.filter_expression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterExpression", filter_expression)?;
        }
        if let Some(ref group_name) = self.group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", group_name)?;
        }
        if let Some(ref insights_configuration) = self.insights_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsightsConfiguration", insights_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut filter_expression: Option<::Value<String>> = None;
                let mut group_name: Option<::Value<String>> = None;
                let mut insights_configuration: Option<::Value<self::group::InsightsConfiguration>> = None;
                let mut tags: Option<::ValueList<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FilterExpression" => {
                            filter_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GroupName" => {
                            group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InsightsConfiguration" => {
                            insights_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GroupProperties {
                    filter_expression: filter_expression,
                    group_name: group_name,
                    insights_configuration: insights_configuration,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Group {
    type Properties = GroupProperties;
    const TYPE: &'static str = "AWS::XRay::Group";
    fn properties(&self) -> &GroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Group {}

impl From<GroupProperties> for Group {
    fn from(properties: GroupProperties) -> Group {
        Group { properties }
    }
}

/// The [`AWS::XRay::SamplingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-samplingrule.html) resource type.
#[derive(Debug, Default)]
pub struct SamplingRule {
    properties: SamplingRuleProperties
}

/// Properties for the `SamplingRule` resource.
#[derive(Debug, Default)]
pub struct SamplingRuleProperties {
    /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-samplingrule.html#cfn-xray-samplingrule-rulename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule_name: Option<::Value<String>>,
    /// Property [`SamplingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-samplingrule.html#cfn-xray-samplingrule-samplingrule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sampling_rule: Option<::Value<self::sampling_rule::SamplingRule>>,
    /// Property [`SamplingRuleRecord`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-samplingrule.html#cfn-xray-samplingrule-samplingrulerecord).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sampling_rule_record: Option<::Value<self::sampling_rule::SamplingRuleRecord>>,
    /// Property [`SamplingRuleUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-samplingrule.html#cfn-xray-samplingrule-samplingruleupdate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sampling_rule_update: Option<::Value<self::sampling_rule::SamplingRuleUpdate>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-samplingrule.html#cfn-xray-samplingrule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::json::Value>>,
}

impl ::serde::Serialize for SamplingRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref rule_name) = self.rule_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", rule_name)?;
        }
        if let Some(ref sampling_rule) = self.sampling_rule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamplingRule", sampling_rule)?;
        }
        if let Some(ref sampling_rule_record) = self.sampling_rule_record {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamplingRuleRecord", sampling_rule_record)?;
        }
        if let Some(ref sampling_rule_update) = self.sampling_rule_update {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamplingRuleUpdate", sampling_rule_update)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SamplingRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SamplingRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SamplingRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SamplingRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut rule_name: Option<::Value<String>> = None;
                let mut sampling_rule: Option<::Value<self::sampling_rule::SamplingRule>> = None;
                let mut sampling_rule_record: Option<::Value<self::sampling_rule::SamplingRuleRecord>> = None;
                let mut sampling_rule_update: Option<::Value<self::sampling_rule::SamplingRuleUpdate>> = None;
                let mut tags: Option<::ValueList<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RuleName" => {
                            rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SamplingRule" => {
                            sampling_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SamplingRuleRecord" => {
                            sampling_rule_record = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SamplingRuleUpdate" => {
                            sampling_rule_update = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SamplingRuleProperties {
                    rule_name: rule_name,
                    sampling_rule: sampling_rule,
                    sampling_rule_record: sampling_rule_record,
                    sampling_rule_update: sampling_rule_update,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SamplingRule {
    type Properties = SamplingRuleProperties;
    const TYPE: &'static str = "AWS::XRay::SamplingRule";
    fn properties(&self) -> &SamplingRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SamplingRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SamplingRule {}

impl From<SamplingRuleProperties> for SamplingRule {
    fn from(properties: SamplingRuleProperties) -> SamplingRule {
        SamplingRule { properties }
    }
}

pub mod group {
    //! Property types for the `Group` resource.

    /// The [`AWS::XRay::Group.InsightsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-group-insightsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct InsightsConfiguration {
        /// Property [`InsightsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-group-insightsconfiguration.html#cfn-xray-group-insightsconfiguration-insightsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub insights_enabled: Option<::Value<bool>>,
        /// Property [`NotificationsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-group-insightsconfiguration.html#cfn-xray-group-insightsconfiguration-notificationsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notifications_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for InsightsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref insights_enabled) = self.insights_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsightsEnabled", insights_enabled)?;
            }
            if let Some(ref notifications_enabled) = self.notifications_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationsEnabled", notifications_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InsightsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InsightsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InsightsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InsightsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut insights_enabled: Option<::Value<bool>> = None;
                    let mut notifications_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InsightsEnabled" => {
                                insights_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationsEnabled" => {
                                notifications_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InsightsConfiguration {
                        insights_enabled: insights_enabled,
                        notifications_enabled: notifications_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod sampling_rule {
    //! Property types for the `SamplingRule` resource.

    /// The [`AWS::XRay::SamplingRule.SamplingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html) property type.
    #[derive(Debug, Default)]
    pub struct SamplingRule {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::ValueMap<String>>,
        /// Property [`FixedRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-fixedrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fixed_rate: Option<::Value<f64>>,
        /// Property [`HTTPMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-httpmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_method: Option<::Value<String>>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: Option<::Value<String>>,
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: Option<::Value<u32>>,
        /// Property [`ReservoirSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-reservoirsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reservoir_size: Option<::Value<u32>>,
        /// Property [`ResourceARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: Option<::Value<String>>,
        /// Property [`RuleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-rulearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_arn: Option<::Value<String>>,
        /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-rulename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_name: Option<::Value<String>>,
        /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-servicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_name: Option<::Value<String>>,
        /// Property [`ServiceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-servicetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_type: Option<::Value<String>>,
        /// Property [`URLPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-urlpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url_path: Option<::Value<String>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html#cfn-xray-samplingrule-samplingrule-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SamplingRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            if let Some(ref fixed_rate) = self.fixed_rate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FixedRate", fixed_rate)?;
            }
            if let Some(ref http_method) = self.http_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPMethod", http_method)?;
            }
            if let Some(ref host) = self.host {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", host)?;
            }
            if let Some(ref priority) = self.priority {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", priority)?;
            }
            if let Some(ref reservoir_size) = self.reservoir_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReservoirSize", reservoir_size)?;
            }
            if let Some(ref resource_arn) = self.resource_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", resource_arn)?;
            }
            if let Some(ref rule_arn) = self.rule_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleARN", rule_arn)?;
            }
            if let Some(ref rule_name) = self.rule_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", rule_name)?;
            }
            if let Some(ref service_name) = self.service_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", service_name)?;
            }
            if let Some(ref service_type) = self.service_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceType", service_type)?;
            }
            if let Some(ref url_path) = self.url_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "URLPath", url_path)?;
            }
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SamplingRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SamplingRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SamplingRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SamplingRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueMap<String>> = None;
                    let mut fixed_rate: Option<::Value<f64>> = None;
                    let mut http_method: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut priority: Option<::Value<u32>> = None;
                    let mut reservoir_size: Option<::Value<u32>> = None;
                    let mut resource_arn: Option<::Value<String>> = None;
                    let mut rule_arn: Option<::Value<String>> = None;
                    let mut rule_name: Option<::Value<String>> = None;
                    let mut service_name: Option<::Value<String>> = None;
                    let mut service_type: Option<::Value<String>> = None;
                    let mut url_path: Option<::Value<String>> = None;
                    let mut version: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FixedRate" => {
                                fixed_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTPMethod" => {
                                http_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReservoirSize" => {
                                reservoir_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceARN" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleARN" => {
                                rule_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleName" => {
                                rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceName" => {
                                service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceType" => {
                                service_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "URLPath" => {
                                url_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SamplingRule {
                        attributes: attributes,
                        fixed_rate: fixed_rate,
                        http_method: http_method,
                        host: host,
                        priority: priority,
                        reservoir_size: reservoir_size,
                        resource_arn: resource_arn,
                        rule_arn: rule_arn,
                        rule_name: rule_name,
                        service_name: service_name,
                        service_type: service_type,
                        url_path: url_path,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::XRay::SamplingRule.SamplingRuleRecord`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrulerecord.html) property type.
    #[derive(Debug, Default)]
    pub struct SamplingRuleRecord {
        /// Property [`CreatedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrulerecord.html#cfn-xray-samplingrule-samplingrulerecord-createdat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub created_at: Option<::Value<String>>,
        /// Property [`ModifiedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrulerecord.html#cfn-xray-samplingrule-samplingrulerecord-modifiedat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub modified_at: Option<::Value<String>>,
        /// Property [`SamplingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrulerecord.html#cfn-xray-samplingrule-samplingrulerecord-samplingrule).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sampling_rule: Option<::Value<SamplingRule>>,
    }

    impl ::codec::SerializeValue for SamplingRuleRecord {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref created_at) = self.created_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedAt", created_at)?;
            }
            if let Some(ref modified_at) = self.modified_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModifiedAt", modified_at)?;
            }
            if let Some(ref sampling_rule) = self.sampling_rule {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamplingRule", sampling_rule)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SamplingRuleRecord {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SamplingRuleRecord, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SamplingRuleRecord;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SamplingRuleRecord")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut created_at: Option<::Value<String>> = None;
                    let mut modified_at: Option<::Value<String>> = None;
                    let mut sampling_rule: Option<::Value<SamplingRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CreatedAt" => {
                                created_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModifiedAt" => {
                                modified_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SamplingRule" => {
                                sampling_rule = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SamplingRuleRecord {
                        created_at: created_at,
                        modified_at: modified_at,
                        sampling_rule: sampling_rule,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::XRay::SamplingRule.SamplingRuleUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html) property type.
    #[derive(Debug, Default)]
    pub struct SamplingRuleUpdate {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::ValueMap<String>>,
        /// Property [`FixedRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-fixedrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fixed_rate: Option<::Value<f64>>,
        /// Property [`HTTPMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-httpmethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http_method: Option<::Value<String>>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: Option<::Value<String>>,
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: Option<::Value<u32>>,
        /// Property [`ReservoirSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-reservoirsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reservoir_size: Option<::Value<u32>>,
        /// Property [`ResourceARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: Option<::Value<String>>,
        /// Property [`RuleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-rulearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_arn: Option<::Value<String>>,
        /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-rulename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_name: Option<::Value<String>>,
        /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-servicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_name: Option<::Value<String>>,
        /// Property [`ServiceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-servicetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_type: Option<::Value<String>>,
        /// Property [`URLPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingruleupdate.html#cfn-xray-samplingrule-samplingruleupdate-urlpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SamplingRuleUpdate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            if let Some(ref fixed_rate) = self.fixed_rate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FixedRate", fixed_rate)?;
            }
            if let Some(ref http_method) = self.http_method {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HTTPMethod", http_method)?;
            }
            if let Some(ref host) = self.host {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", host)?;
            }
            if let Some(ref priority) = self.priority {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", priority)?;
            }
            if let Some(ref reservoir_size) = self.reservoir_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReservoirSize", reservoir_size)?;
            }
            if let Some(ref resource_arn) = self.resource_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", resource_arn)?;
            }
            if let Some(ref rule_arn) = self.rule_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleARN", rule_arn)?;
            }
            if let Some(ref rule_name) = self.rule_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", rule_name)?;
            }
            if let Some(ref service_name) = self.service_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", service_name)?;
            }
            if let Some(ref service_type) = self.service_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceType", service_type)?;
            }
            if let Some(ref url_path) = self.url_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "URLPath", url_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SamplingRuleUpdate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SamplingRuleUpdate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SamplingRuleUpdate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SamplingRuleUpdate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueMap<String>> = None;
                    let mut fixed_rate: Option<::Value<f64>> = None;
                    let mut http_method: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut priority: Option<::Value<u32>> = None;
                    let mut reservoir_size: Option<::Value<u32>> = None;
                    let mut resource_arn: Option<::Value<String>> = None;
                    let mut rule_arn: Option<::Value<String>> = None;
                    let mut rule_name: Option<::Value<String>> = None;
                    let mut service_name: Option<::Value<String>> = None;
                    let mut service_type: Option<::Value<String>> = None;
                    let mut url_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FixedRate" => {
                                fixed_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HTTPMethod" => {
                                http_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReservoirSize" => {
                                reservoir_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceARN" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleARN" => {
                                rule_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleName" => {
                                rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceName" => {
                                service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceType" => {
                                service_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "URLPath" => {
                                url_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SamplingRuleUpdate {
                        attributes: attributes,
                        fixed_rate: fixed_rate,
                        http_method: http_method,
                        host: host,
                        priority: priority,
                        reservoir_size: reservoir_size,
                        resource_arn: resource_arn,
                        rule_arn: rule_arn,
                        rule_name: rule_name,
                        service_name: service_name,
                        service_type: service_type,
                        url_path: url_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
