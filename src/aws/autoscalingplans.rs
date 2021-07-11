//! Types for the `AutoScalingPlans` service.

/// The [`AWS::AutoScalingPlans::ScalingPlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscalingplans-scalingplan.html) resource type.
#[derive(Debug, Default)]
pub struct ScalingPlan {
    properties: ScalingPlanProperties
}

/// Properties for the `ScalingPlan` resource.
#[derive(Debug, Default)]
pub struct ScalingPlanProperties {
    /// Property [`ApplicationSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscalingplans-scalingplan.html#cfn-autoscalingplans-scalingplan-applicationsource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub application_source: ::Value<self::scaling_plan::ApplicationSource>,
    /// Property [`ScalingInstructions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscalingplans-scalingplan.html#cfn-autoscalingplans-scalingplan-scalinginstructions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scaling_instructions: ::ValueList<self::scaling_plan::ScalingInstruction>,
}

impl ::serde::Serialize for ScalingPlanProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationSource", &self.application_source)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingInstructions", &self.scaling_instructions)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScalingPlanProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingPlanProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScalingPlanProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScalingPlanProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_source: Option<::Value<self::scaling_plan::ApplicationSource>> = None;
                let mut scaling_instructions: Option<::ValueList<self::scaling_plan::ScalingInstruction>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationSource" => {
                            application_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScalingInstructions" => {
                            scaling_instructions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScalingPlanProperties {
                    application_source: application_source.ok_or(::serde::de::Error::missing_field("ApplicationSource"))?,
                    scaling_instructions: scaling_instructions.ok_or(::serde::de::Error::missing_field("ScalingInstructions"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ScalingPlan {
    type Properties = ScalingPlanProperties;
    const TYPE: &'static str = "AWS::AutoScalingPlans::ScalingPlan";
    fn properties(&self) -> &ScalingPlanProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScalingPlanProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ScalingPlan {}

impl From<ScalingPlanProperties> for ScalingPlan {
    fn from(properties: ScalingPlanProperties) -> ScalingPlan {
        ScalingPlan { properties }
    }
}

pub mod scaling_plan {
    //! Property types for the `ScalingPlan` resource.

    /// The [`AWS::AutoScalingPlans::ScalingPlan.ApplicationSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-applicationsource.html) property type.
    #[derive(Debug, Default)]
    pub struct ApplicationSource {
        /// Property [`CloudFormationStackARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-applicationsource.html#cfn-autoscalingplans-scalingplan-applicationsource-cloudformationstackarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_formation_stack_arn: Option<::Value<String>>,
        /// Property [`TagFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-applicationsource.html#cfn-autoscalingplans-scalingplan-applicationsource-tagfilters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_filters: Option<::ValueList<TagFilter>>,
    }

    impl ::codec::SerializeValue for ApplicationSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_formation_stack_arn) = self.cloud_formation_stack_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudFormationStackARN", cloud_formation_stack_arn)?;
            }
            if let Some(ref tag_filters) = self.tag_filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagFilters", tag_filters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_formation_stack_arn: Option<::Value<String>> = None;
                    let mut tag_filters: Option<::ValueList<TagFilter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudFormationStackARN" => {
                                cloud_formation_stack_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagFilters" => {
                                tag_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationSource {
                        cloud_formation_stack_arn: cloud_formation_stack_arn,
                        tag_filters: tag_filters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScalingPlans::ScalingPlan.CustomizedLoadMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedloadmetricspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomizedLoadMetricSpecification {
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedloadmetricspecification.html#cfn-autoscalingplans-scalingplan-customizedloadmetricspecification-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedloadmetricspecification.html#cfn-autoscalingplans-scalingplan-customizedloadmetricspecification-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedloadmetricspecification.html#cfn-autoscalingplans-scalingplan-customizedloadmetricspecification-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: ::Value<String>,
        /// Property [`Statistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedloadmetricspecification.html#cfn-autoscalingplans-scalingplan-customizedloadmetricspecification-statistic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statistic: ::Value<String>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedloadmetricspecification.html#cfn-autoscalingplans-scalingplan-customizedloadmetricspecification-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CustomizedLoadMetricSpecification {
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

    impl ::codec::DeserializeValue for CustomizedLoadMetricSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomizedLoadMetricSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomizedLoadMetricSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomizedLoadMetricSpecification")
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

                    Ok(CustomizedLoadMetricSpecification {
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

    /// The [`AWS::AutoScalingPlans::ScalingPlan.CustomizedScalingMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedscalingmetricspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomizedScalingMetricSpecification {
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedscalingmetricspecification.html#cfn-autoscalingplans-scalingplan-customizedscalingmetricspecification-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: Option<::ValueList<MetricDimension>>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedscalingmetricspecification.html#cfn-autoscalingplans-scalingplan-customizedscalingmetricspecification-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedscalingmetricspecification.html#cfn-autoscalingplans-scalingplan-customizedscalingmetricspecification-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: ::Value<String>,
        /// Property [`Statistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedscalingmetricspecification.html#cfn-autoscalingplans-scalingplan-customizedscalingmetricspecification-statistic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statistic: ::Value<String>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-customizedscalingmetricspecification.html#cfn-autoscalingplans-scalingplan-customizedscalingmetricspecification-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CustomizedScalingMetricSpecification {
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

    impl ::codec::DeserializeValue for CustomizedScalingMetricSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomizedScalingMetricSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomizedScalingMetricSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomizedScalingMetricSpecification")
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

                    Ok(CustomizedScalingMetricSpecification {
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

    /// The [`AWS::AutoScalingPlans::ScalingPlan.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-metricdimension.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricDimension {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-metricdimension.html#cfn-autoscalingplans-scalingplan-metricdimension-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-metricdimension.html#cfn-autoscalingplans-scalingplan-metricdimension-value).
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

    /// The [`AWS::AutoScalingPlans::ScalingPlan.PredefinedLoadMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-predefinedloadmetricspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct PredefinedLoadMetricSpecification {
        /// Property [`PredefinedLoadMetricType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-predefinedloadmetricspecification.html#cfn-autoscalingplans-scalingplan-predefinedloadmetricspecification-predefinedloadmetrictype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predefined_load_metric_type: ::Value<String>,
        /// Property [`ResourceLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-predefinedloadmetricspecification.html#cfn-autoscalingplans-scalingplan-predefinedloadmetricspecification-resourcelabel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_label: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PredefinedLoadMetricSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredefinedLoadMetricType", &self.predefined_load_metric_type)?;
            if let Some(ref resource_label) = self.resource_label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceLabel", resource_label)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PredefinedLoadMetricSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PredefinedLoadMetricSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PredefinedLoadMetricSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PredefinedLoadMetricSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut predefined_load_metric_type: Option<::Value<String>> = None;
                    let mut resource_label: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PredefinedLoadMetricType" => {
                                predefined_load_metric_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceLabel" => {
                                resource_label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PredefinedLoadMetricSpecification {
                        predefined_load_metric_type: predefined_load_metric_type.ok_or(::serde::de::Error::missing_field("PredefinedLoadMetricType"))?,
                        resource_label: resource_label,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScalingPlans::ScalingPlan.PredefinedScalingMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-predefinedscalingmetricspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct PredefinedScalingMetricSpecification {
        /// Property [`PredefinedScalingMetricType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-predefinedscalingmetricspecification.html#cfn-autoscalingplans-scalingplan-predefinedscalingmetricspecification-predefinedscalingmetrictype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predefined_scaling_metric_type: ::Value<String>,
        /// Property [`ResourceLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-predefinedscalingmetricspecification.html#cfn-autoscalingplans-scalingplan-predefinedscalingmetricspecification-resourcelabel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_label: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PredefinedScalingMetricSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredefinedScalingMetricType", &self.predefined_scaling_metric_type)?;
            if let Some(ref resource_label) = self.resource_label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceLabel", resource_label)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PredefinedScalingMetricSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PredefinedScalingMetricSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PredefinedScalingMetricSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PredefinedScalingMetricSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut predefined_scaling_metric_type: Option<::Value<String>> = None;
                    let mut resource_label: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PredefinedScalingMetricType" => {
                                predefined_scaling_metric_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceLabel" => {
                                resource_label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PredefinedScalingMetricSpecification {
                        predefined_scaling_metric_type: predefined_scaling_metric_type.ok_or(::serde::de::Error::missing_field("PredefinedScalingMetricType"))?,
                        resource_label: resource_label,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScalingPlans::ScalingPlan.ScalingInstruction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html) property type.
    #[derive(Debug, Default)]
    pub struct ScalingInstruction {
        /// Property [`CustomizedLoadMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-customizedloadmetricspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub customized_load_metric_specification: Option<::Value<CustomizedLoadMetricSpecification>>,
        /// Property [`DisableDynamicScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-disabledynamicscaling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_dynamic_scaling: Option<::Value<bool>>,
        /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-maxcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_capacity: ::Value<u32>,
        /// Property [`MinCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-mincapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_capacity: ::Value<u32>,
        /// Property [`PredefinedLoadMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-predefinedloadmetricspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predefined_load_metric_specification: Option<::Value<PredefinedLoadMetricSpecification>>,
        /// Property [`PredictiveScalingMaxCapacityBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-predictivescalingmaxcapacitybehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predictive_scaling_max_capacity_behavior: Option<::Value<String>>,
        /// Property [`PredictiveScalingMaxCapacityBuffer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-predictivescalingmaxcapacitybuffer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predictive_scaling_max_capacity_buffer: Option<::Value<u32>>,
        /// Property [`PredictiveScalingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-predictivescalingmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predictive_scaling_mode: Option<::Value<String>>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-resourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id: ::Value<String>,
        /// Property [`ScalableDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-scalabledimension).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scalable_dimension: ::Value<String>,
        /// Property [`ScalingPolicyUpdateBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-scalingpolicyupdatebehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scaling_policy_update_behavior: Option<::Value<String>>,
        /// Property [`ScheduledActionBufferTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-scheduledactionbuffertime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scheduled_action_buffer_time: Option<::Value<u32>>,
        /// Property [`ServiceNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-servicenamespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_namespace: ::Value<String>,
        /// Property [`TargetTrackingConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-scalinginstruction.html#cfn-autoscalingplans-scalingplan-scalinginstruction-targettrackingconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_tracking_configurations: ::ValueList<TargetTrackingConfiguration>,
    }

    impl ::codec::SerializeValue for ScalingInstruction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref customized_load_metric_specification) = self.customized_load_metric_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomizedLoadMetricSpecification", customized_load_metric_specification)?;
            }
            if let Some(ref disable_dynamic_scaling) = self.disable_dynamic_scaling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableDynamicScaling", disable_dynamic_scaling)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", &self.max_capacity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinCapacity", &self.min_capacity)?;
            if let Some(ref predefined_load_metric_specification) = self.predefined_load_metric_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredefinedLoadMetricSpecification", predefined_load_metric_specification)?;
            }
            if let Some(ref predictive_scaling_max_capacity_behavior) = self.predictive_scaling_max_capacity_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredictiveScalingMaxCapacityBehavior", predictive_scaling_max_capacity_behavior)?;
            }
            if let Some(ref predictive_scaling_max_capacity_buffer) = self.predictive_scaling_max_capacity_buffer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredictiveScalingMaxCapacityBuffer", predictive_scaling_max_capacity_buffer)?;
            }
            if let Some(ref predictive_scaling_mode) = self.predictive_scaling_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredictiveScalingMode", predictive_scaling_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", &self.resource_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalableDimension", &self.scalable_dimension)?;
            if let Some(ref scaling_policy_update_behavior) = self.scaling_policy_update_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingPolicyUpdateBehavior", scaling_policy_update_behavior)?;
            }
            if let Some(ref scheduled_action_buffer_time) = self.scheduled_action_buffer_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduledActionBufferTime", scheduled_action_buffer_time)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNamespace", &self.service_namespace)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetTrackingConfigurations", &self.target_tracking_configurations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingInstruction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingInstruction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingInstruction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingInstruction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut customized_load_metric_specification: Option<::Value<CustomizedLoadMetricSpecification>> = None;
                    let mut disable_dynamic_scaling: Option<::Value<bool>> = None;
                    let mut max_capacity: Option<::Value<u32>> = None;
                    let mut min_capacity: Option<::Value<u32>> = None;
                    let mut predefined_load_metric_specification: Option<::Value<PredefinedLoadMetricSpecification>> = None;
                    let mut predictive_scaling_max_capacity_behavior: Option<::Value<String>> = None;
                    let mut predictive_scaling_max_capacity_buffer: Option<::Value<u32>> = None;
                    let mut predictive_scaling_mode: Option<::Value<String>> = None;
                    let mut resource_id: Option<::Value<String>> = None;
                    let mut scalable_dimension: Option<::Value<String>> = None;
                    let mut scaling_policy_update_behavior: Option<::Value<String>> = None;
                    let mut scheduled_action_buffer_time: Option<::Value<u32>> = None;
                    let mut service_namespace: Option<::Value<String>> = None;
                    let mut target_tracking_configurations: Option<::ValueList<TargetTrackingConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomizedLoadMetricSpecification" => {
                                customized_load_metric_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisableDynamicScaling" => {
                                disable_dynamic_scaling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxCapacity" => {
                                max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinCapacity" => {
                                min_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PredefinedLoadMetricSpecification" => {
                                predefined_load_metric_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PredictiveScalingMaxCapacityBehavior" => {
                                predictive_scaling_max_capacity_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PredictiveScalingMaxCapacityBuffer" => {
                                predictive_scaling_max_capacity_buffer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PredictiveScalingMode" => {
                                predictive_scaling_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScalableDimension" => {
                                scalable_dimension = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScalingPolicyUpdateBehavior" => {
                                scaling_policy_update_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduledActionBufferTime" => {
                                scheduled_action_buffer_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNamespace" => {
                                service_namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetTrackingConfigurations" => {
                                target_tracking_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingInstruction {
                        customized_load_metric_specification: customized_load_metric_specification,
                        disable_dynamic_scaling: disable_dynamic_scaling,
                        max_capacity: max_capacity.ok_or(::serde::de::Error::missing_field("MaxCapacity"))?,
                        min_capacity: min_capacity.ok_or(::serde::de::Error::missing_field("MinCapacity"))?,
                        predefined_load_metric_specification: predefined_load_metric_specification,
                        predictive_scaling_max_capacity_behavior: predictive_scaling_max_capacity_behavior,
                        predictive_scaling_max_capacity_buffer: predictive_scaling_max_capacity_buffer,
                        predictive_scaling_mode: predictive_scaling_mode,
                        resource_id: resource_id.ok_or(::serde::de::Error::missing_field("ResourceId"))?,
                        scalable_dimension: scalable_dimension.ok_or(::serde::de::Error::missing_field("ScalableDimension"))?,
                        scaling_policy_update_behavior: scaling_policy_update_behavior,
                        scheduled_action_buffer_time: scheduled_action_buffer_time,
                        service_namespace: service_namespace.ok_or(::serde::de::Error::missing_field("ServiceNamespace"))?,
                        target_tracking_configurations: target_tracking_configurations.ok_or(::serde::de::Error::missing_field("TargetTrackingConfigurations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScalingPlans::ScalingPlan.TagFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-tagfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct TagFilter {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-tagfilter.html#cfn-autoscalingplans-scalingplan-tagfilter-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-tagfilter.html#cfn-autoscalingplans-scalingplan-tagfilter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for TagFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
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
                    let mut key: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TagFilter {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AutoScalingPlans::ScalingPlan.TargetTrackingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-targettrackingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetTrackingConfiguration {
        /// Property [`CustomizedScalingMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-targettrackingconfiguration.html#cfn-autoscalingplans-scalingplan-targettrackingconfiguration-customizedscalingmetricspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub customized_scaling_metric_specification: Option<::Value<CustomizedScalingMetricSpecification>>,
        /// Property [`DisableScaleIn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-targettrackingconfiguration.html#cfn-autoscalingplans-scalingplan-targettrackingconfiguration-disablescalein).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_scale_in: Option<::Value<bool>>,
        /// Property [`EstimatedInstanceWarmup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-targettrackingconfiguration.html#cfn-autoscalingplans-scalingplan-targettrackingconfiguration-estimatedinstancewarmup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub estimated_instance_warmup: Option<::Value<u32>>,
        /// Property [`PredefinedScalingMetricSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-targettrackingconfiguration.html#cfn-autoscalingplans-scalingplan-targettrackingconfiguration-predefinedscalingmetricspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predefined_scaling_metric_specification: Option<::Value<PredefinedScalingMetricSpecification>>,
        /// Property [`ScaleInCooldown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-targettrackingconfiguration.html#cfn-autoscalingplans-scalingplan-targettrackingconfiguration-scaleincooldown).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scale_in_cooldown: Option<::Value<u32>>,
        /// Property [`ScaleOutCooldown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-targettrackingconfiguration.html#cfn-autoscalingplans-scalingplan-targettrackingconfiguration-scaleoutcooldown).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scale_out_cooldown: Option<::Value<u32>>,
        /// Property [`TargetValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscalingplans-scalingplan-targettrackingconfiguration.html#cfn-autoscalingplans-scalingplan-targettrackingconfiguration-targetvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_value: ::Value<f64>,
    }

    impl ::codec::SerializeValue for TargetTrackingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref customized_scaling_metric_specification) = self.customized_scaling_metric_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomizedScalingMetricSpecification", customized_scaling_metric_specification)?;
            }
            if let Some(ref disable_scale_in) = self.disable_scale_in {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableScaleIn", disable_scale_in)?;
            }
            if let Some(ref estimated_instance_warmup) = self.estimated_instance_warmup {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EstimatedInstanceWarmup", estimated_instance_warmup)?;
            }
            if let Some(ref predefined_scaling_metric_specification) = self.predefined_scaling_metric_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredefinedScalingMetricSpecification", predefined_scaling_metric_specification)?;
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

    impl ::codec::DeserializeValue for TargetTrackingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetTrackingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetTrackingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetTrackingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut customized_scaling_metric_specification: Option<::Value<CustomizedScalingMetricSpecification>> = None;
                    let mut disable_scale_in: Option<::Value<bool>> = None;
                    let mut estimated_instance_warmup: Option<::Value<u32>> = None;
                    let mut predefined_scaling_metric_specification: Option<::Value<PredefinedScalingMetricSpecification>> = None;
                    let mut scale_in_cooldown: Option<::Value<u32>> = None;
                    let mut scale_out_cooldown: Option<::Value<u32>> = None;
                    let mut target_value: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomizedScalingMetricSpecification" => {
                                customized_scaling_metric_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisableScaleIn" => {
                                disable_scale_in = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EstimatedInstanceWarmup" => {
                                estimated_instance_warmup = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PredefinedScalingMetricSpecification" => {
                                predefined_scaling_metric_specification = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(TargetTrackingConfiguration {
                        customized_scaling_metric_specification: customized_scaling_metric_specification,
                        disable_scale_in: disable_scale_in,
                        estimated_instance_warmup: estimated_instance_warmup,
                        predefined_scaling_metric_specification: predefined_scaling_metric_specification,
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
