//! Types for the `Evidently` service.

/// The [`AWS::Evidently::Experiment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html) resource type.
#[derive(Debug, Default)]
pub struct Experiment {
    properties: ExperimentProperties
}

/// Properties for the `Experiment` resource.
#[derive(Debug, Default)]
pub struct ExperimentProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`MetricGoals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-metricgoals).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metric_goals: ::ValueList<self::experiment::MetricGoalObject>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OnlineAbConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-onlineabconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub online_ab_config: ::Value<self::experiment::OnlineAbConfigObject>,
    /// Property [`Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-project).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project: ::Value<String>,
    /// Property [`RandomizationSalt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-randomizationsalt).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub randomization_salt: Option<::Value<String>>,
    /// Property [`RemoveSegment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-removesegment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub remove_segment: Option<::Value<bool>>,
    /// Property [`RunningStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-runningstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub running_status: Option<::Value<self::experiment::RunningStatusObject>>,
    /// Property [`SamplingRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-samplingrate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sampling_rate: Option<::Value<u32>>,
    /// Property [`Segment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-segment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub segment: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Treatments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-experiment.html#cfn-evidently-experiment-treatments).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub treatments: ::ValueList<self::experiment::TreatmentObject>,
}

impl ::serde::Serialize for ExperimentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricGoals", &self.metric_goals)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnlineAbConfig", &self.online_ab_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Project", &self.project)?;
        if let Some(ref randomization_salt) = self.randomization_salt {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RandomizationSalt", randomization_salt)?;
        }
        if let Some(ref remove_segment) = self.remove_segment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveSegment", remove_segment)?;
        }
        if let Some(ref running_status) = self.running_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunningStatus", running_status)?;
        }
        if let Some(ref sampling_rate) = self.sampling_rate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamplingRate", sampling_rate)?;
        }
        if let Some(ref segment) = self.segment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Segment", segment)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Treatments", &self.treatments)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ExperimentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ExperimentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ExperimentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ExperimentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut metric_goals: Option<::ValueList<self::experiment::MetricGoalObject>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut online_ab_config: Option<::Value<self::experiment::OnlineAbConfigObject>> = None;
                let mut project: Option<::Value<String>> = None;
                let mut randomization_salt: Option<::Value<String>> = None;
                let mut remove_segment: Option<::Value<bool>> = None;
                let mut running_status: Option<::Value<self::experiment::RunningStatusObject>> = None;
                let mut sampling_rate: Option<::Value<u32>> = None;
                let mut segment: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut treatments: Option<::ValueList<self::experiment::TreatmentObject>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricGoals" => {
                            metric_goals = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OnlineAbConfig" => {
                            online_ab_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Project" => {
                            project = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RandomizationSalt" => {
                            randomization_salt = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RemoveSegment" => {
                            remove_segment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RunningStatus" => {
                            running_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SamplingRate" => {
                            sampling_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Segment" => {
                            segment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Treatments" => {
                            treatments = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ExperimentProperties {
                    description: description,
                    metric_goals: metric_goals.ok_or(::serde::de::Error::missing_field("MetricGoals"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    online_ab_config: online_ab_config.ok_or(::serde::de::Error::missing_field("OnlineAbConfig"))?,
                    project: project.ok_or(::serde::de::Error::missing_field("Project"))?,
                    randomization_salt: randomization_salt,
                    remove_segment: remove_segment,
                    running_status: running_status,
                    sampling_rate: sampling_rate,
                    segment: segment,
                    tags: tags,
                    treatments: treatments.ok_or(::serde::de::Error::missing_field("Treatments"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Experiment {
    type Properties = ExperimentProperties;
    const TYPE: &'static str = "AWS::Evidently::Experiment";
    fn properties(&self) -> &ExperimentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ExperimentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Experiment {}

impl From<ExperimentProperties> for Experiment {
    fn from(properties: ExperimentProperties) -> Experiment {
        Experiment { properties }
    }
}

/// The [`AWS::Evidently::Feature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-feature.html) resource type.
#[derive(Debug, Default)]
pub struct Feature {
    properties: FeatureProperties
}

/// Properties for the `Feature` resource.
#[derive(Debug, Default)]
pub struct FeatureProperties {
    /// Property [`DefaultVariation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-feature.html#cfn-evidently-feature-defaultvariation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_variation: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-feature.html#cfn-evidently-feature-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EntityOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-feature.html#cfn-evidently-feature-entityoverrides).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub entity_overrides: Option<::ValueList<self::feature::EntityOverride>>,
    /// Property [`EvaluationStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-feature.html#cfn-evidently-feature-evaluationstrategy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub evaluation_strategy: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-feature.html#cfn-evidently-feature-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-feature.html#cfn-evidently-feature-project).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-feature.html#cfn-evidently-feature-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Variations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-feature.html#cfn-evidently-feature-variations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub variations: ::ValueList<self::feature::VariationObject>,
}

impl ::serde::Serialize for FeatureProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref default_variation) = self.default_variation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultVariation", default_variation)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref entity_overrides) = self.entity_overrides {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityOverrides", entity_overrides)?;
        }
        if let Some(ref evaluation_strategy) = self.evaluation_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationStrategy", evaluation_strategy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Project", &self.project)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variations", &self.variations)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FeatureProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FeatureProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FeatureProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FeatureProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut default_variation: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut entity_overrides: Option<::ValueList<self::feature::EntityOverride>> = None;
                let mut evaluation_strategy: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut project: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut variations: Option<::ValueList<self::feature::VariationObject>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DefaultVariation" => {
                            default_variation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EntityOverrides" => {
                            entity_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EvaluationStrategy" => {
                            evaluation_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Project" => {
                            project = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Variations" => {
                            variations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FeatureProperties {
                    default_variation: default_variation,
                    description: description,
                    entity_overrides: entity_overrides,
                    evaluation_strategy: evaluation_strategy,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    project: project.ok_or(::serde::de::Error::missing_field("Project"))?,
                    tags: tags,
                    variations: variations.ok_or(::serde::de::Error::missing_field("Variations"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Feature {
    type Properties = FeatureProperties;
    const TYPE: &'static str = "AWS::Evidently::Feature";
    fn properties(&self) -> &FeatureProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FeatureProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Feature {}

impl From<FeatureProperties> for Feature {
    fn from(properties: FeatureProperties) -> Feature {
        Feature { properties }
    }
}

/// The [`AWS::Evidently::Launch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html) resource type.
#[derive(Debug, Default)]
pub struct Launch {
    properties: LaunchProperties
}

/// Properties for the `Launch` resource.
#[derive(Debug, Default)]
pub struct LaunchProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html#cfn-evidently-launch-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`ExecutionStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html#cfn-evidently-launch-executionstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_status: Option<::Value<self::launch::ExecutionStatusObject>>,
    /// Property [`Groups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html#cfn-evidently-launch-groups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub groups: ::ValueList<self::launch::LaunchGroupObject>,
    /// Property [`MetricMonitors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html#cfn-evidently-launch-metricmonitors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metric_monitors: Option<::ValueList<self::launch::MetricDefinitionObject>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html#cfn-evidently-launch-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html#cfn-evidently-launch-project).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project: ::Value<String>,
    /// Property [`RandomizationSalt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html#cfn-evidently-launch-randomizationsalt).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub randomization_salt: Option<::Value<String>>,
    /// Property [`ScheduledSplitsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html#cfn-evidently-launch-scheduledsplitsconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scheduled_splits_config: ::ValueList<self::launch::StepConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-launch.html#cfn-evidently-launch-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for LaunchProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref execution_status) = self.execution_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionStatus", execution_status)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", &self.groups)?;
        if let Some(ref metric_monitors) = self.metric_monitors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricMonitors", metric_monitors)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Project", &self.project)?;
        if let Some(ref randomization_salt) = self.randomization_salt {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RandomizationSalt", randomization_salt)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduledSplitsConfig", &self.scheduled_splits_config)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LaunchProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LaunchProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LaunchProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut execution_status: Option<::Value<self::launch::ExecutionStatusObject>> = None;
                let mut groups: Option<::ValueList<self::launch::LaunchGroupObject>> = None;
                let mut metric_monitors: Option<::ValueList<self::launch::MetricDefinitionObject>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut project: Option<::Value<String>> = None;
                let mut randomization_salt: Option<::Value<String>> = None;
                let mut scheduled_splits_config: Option<::ValueList<self::launch::StepConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionStatus" => {
                            execution_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Groups" => {
                            groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricMonitors" => {
                            metric_monitors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Project" => {
                            project = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RandomizationSalt" => {
                            randomization_salt = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduledSplitsConfig" => {
                            scheduled_splits_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LaunchProperties {
                    description: description,
                    execution_status: execution_status,
                    groups: groups.ok_or(::serde::de::Error::missing_field("Groups"))?,
                    metric_monitors: metric_monitors,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    project: project.ok_or(::serde::de::Error::missing_field("Project"))?,
                    randomization_salt: randomization_salt,
                    scheduled_splits_config: scheduled_splits_config.ok_or(::serde::de::Error::missing_field("ScheduledSplitsConfig"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Launch {
    type Properties = LaunchProperties;
    const TYPE: &'static str = "AWS::Evidently::Launch";
    fn properties(&self) -> &LaunchProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LaunchProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Launch {}

impl From<LaunchProperties> for Launch {
    fn from(properties: LaunchProperties) -> Launch {
        Launch { properties }
    }
}

/// The [`AWS::Evidently::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-project.html) resource type.
#[derive(Debug, Default)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug, Default)]
pub struct ProjectProperties {
    /// Property [`AppConfigResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-project.html#cfn-evidently-project-appconfigresource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_config_resource: Option<::Value<self::project::AppConfigResourceObject>>,
    /// Property [`DataDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-project.html#cfn-evidently-project-datadelivery).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_delivery: Option<::Value<self::project::DataDeliveryObject>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-project.html#cfn-evidently-project-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-project.html#cfn-evidently-project-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-project.html#cfn-evidently-project-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref app_config_resource) = self.app_config_resource {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppConfigResource", app_config_resource)?;
        }
        if let Some(ref data_delivery) = self.data_delivery {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataDelivery", data_delivery)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProjectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProjectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProjectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_config_resource: Option<::Value<self::project::AppConfigResourceObject>> = None;
                let mut data_delivery: Option<::Value<self::project::DataDeliveryObject>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppConfigResource" => {
                            app_config_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataDelivery" => {
                            data_delivery = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProjectProperties {
                    app_config_resource: app_config_resource,
                    data_delivery: data_delivery,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Project {
    type Properties = ProjectProperties;
    const TYPE: &'static str = "AWS::Evidently::Project";
    fn properties(&self) -> &ProjectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProjectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Project {}

impl From<ProjectProperties> for Project {
    fn from(properties: ProjectProperties) -> Project {
        Project { properties }
    }
}

/// The [`AWS::Evidently::Segment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-segment.html) resource type.
#[derive(Debug, Default)]
pub struct Segment {
    properties: SegmentProperties
}

/// Properties for the `Segment` resource.
#[derive(Debug, Default)]
pub struct SegmentProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-segment.html#cfn-evidently-segment-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-segment.html#cfn-evidently-segment-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Pattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-segment.html#cfn-evidently-segment-pattern).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pattern: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-evidently-segment.html#cfn-evidently-segment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SegmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref pattern) = self.pattern {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pattern", pattern)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SegmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SegmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SegmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SegmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut pattern: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Pattern" => {
                            pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SegmentProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    pattern: pattern,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Segment {
    type Properties = SegmentProperties;
    const TYPE: &'static str = "AWS::Evidently::Segment";
    fn properties(&self) -> &SegmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SegmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Segment {}

impl From<SegmentProperties> for Segment {
    fn from(properties: SegmentProperties) -> Segment {
        Segment { properties }
    }
}

pub mod experiment {
    //! Property types for the `Experiment` resource.

    /// The [`AWS::Evidently::Experiment.MetricGoalObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-metricgoalobject.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricGoalObject {
        /// Property [`DesiredChange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-metricgoalobject.html#cfn-evidently-experiment-metricgoalobject-desiredchange).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desired_change: ::Value<String>,
        /// Property [`EntityIdKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-metricgoalobject.html#cfn-evidently-experiment-metricgoalobject-entityidkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entity_id_key: ::Value<String>,
        /// Property [`EventPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-metricgoalobject.html#cfn-evidently-experiment-metricgoalobject-eventpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_pattern: Option<::Value<String>>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-metricgoalobject.html#cfn-evidently-experiment-metricgoalobject-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`UnitLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-metricgoalobject.html#cfn-evidently-experiment-metricgoalobject-unitlabel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit_label: Option<::Value<String>>,
        /// Property [`ValueKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-metricgoalobject.html#cfn-evidently-experiment-metricgoalobject-valuekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for MetricGoalObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredChange", &self.desired_change)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityIdKey", &self.entity_id_key)?;
            if let Some(ref event_pattern) = self.event_pattern {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventPattern", event_pattern)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            if let Some(ref unit_label) = self.unit_label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnitLabel", unit_label)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueKey", &self.value_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricGoalObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricGoalObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricGoalObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricGoalObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut desired_change: Option<::Value<String>> = None;
                    let mut entity_id_key: Option<::Value<String>> = None;
                    let mut event_pattern: Option<::Value<String>> = None;
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut unit_label: Option<::Value<String>> = None;
                    let mut value_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DesiredChange" => {
                                desired_change = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntityIdKey" => {
                                entity_id_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventPattern" => {
                                event_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnitLabel" => {
                                unit_label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueKey" => {
                                value_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricGoalObject {
                        desired_change: desired_change.ok_or(::serde::de::Error::missing_field("DesiredChange"))?,
                        entity_id_key: entity_id_key.ok_or(::serde::de::Error::missing_field("EntityIdKey"))?,
                        event_pattern: event_pattern,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        unit_label: unit_label,
                        value_key: value_key.ok_or(::serde::de::Error::missing_field("ValueKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Experiment.OnlineAbConfigObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-onlineabconfigobject.html) property type.
    #[derive(Debug, Default)]
    pub struct OnlineAbConfigObject {
        /// Property [`ControlTreatmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-onlineabconfigobject.html#cfn-evidently-experiment-onlineabconfigobject-controltreatmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub control_treatment_name: Option<::Value<String>>,
        /// Property [`TreatmentWeights`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-onlineabconfigobject.html#cfn-evidently-experiment-onlineabconfigobject-treatmentweights).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub treatment_weights: Option<::ValueList<TreatmentToWeight>>,
    }

    impl ::codec::SerializeValue for OnlineAbConfigObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref control_treatment_name) = self.control_treatment_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlTreatmentName", control_treatment_name)?;
            }
            if let Some(ref treatment_weights) = self.treatment_weights {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TreatmentWeights", treatment_weights)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OnlineAbConfigObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OnlineAbConfigObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OnlineAbConfigObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OnlineAbConfigObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut control_treatment_name: Option<::Value<String>> = None;
                    let mut treatment_weights: Option<::ValueList<TreatmentToWeight>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ControlTreatmentName" => {
                                control_treatment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TreatmentWeights" => {
                                treatment_weights = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OnlineAbConfigObject {
                        control_treatment_name: control_treatment_name,
                        treatment_weights: treatment_weights,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Experiment.RunningStatusObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-runningstatusobject.html) property type.
    #[derive(Debug, Default)]
    pub struct RunningStatusObject {
        /// Property [`AnalysisCompleteTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-runningstatusobject.html#cfn-evidently-experiment-runningstatusobject-analysiscompletetime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub analysis_complete_time: Option<::Value<String>>,
        /// Property [`DesiredState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-runningstatusobject.html#cfn-evidently-experiment-runningstatusobject-desiredstate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desired_state: Option<::Value<String>>,
        /// Property [`Reason`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-runningstatusobject.html#cfn-evidently-experiment-runningstatusobject-reason).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reason: Option<::Value<String>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-runningstatusobject.html#cfn-evidently-experiment-runningstatusobject-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for RunningStatusObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref analysis_complete_time) = self.analysis_complete_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnalysisCompleteTime", analysis_complete_time)?;
            }
            if let Some(ref desired_state) = self.desired_state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredState", desired_state)?;
            }
            if let Some(ref reason) = self.reason {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Reason", reason)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RunningStatusObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RunningStatusObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RunningStatusObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RunningStatusObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut analysis_complete_time: Option<::Value<String>> = None;
                    let mut desired_state: Option<::Value<String>> = None;
                    let mut reason: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AnalysisCompleteTime" => {
                                analysis_complete_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DesiredState" => {
                                desired_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Reason" => {
                                reason = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RunningStatusObject {
                        analysis_complete_time: analysis_complete_time,
                        desired_state: desired_state,
                        reason: reason,
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Experiment.TreatmentObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-treatmentobject.html) property type.
    #[derive(Debug, Default)]
    pub struct TreatmentObject {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-treatmentobject.html#cfn-evidently-experiment-treatmentobject-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Feature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-treatmentobject.html#cfn-evidently-experiment-treatmentobject-feature).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub feature: ::Value<String>,
        /// Property [`TreatmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-treatmentobject.html#cfn-evidently-experiment-treatmentobject-treatmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub treatment_name: ::Value<String>,
        /// Property [`Variation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-treatmentobject.html#cfn-evidently-experiment-treatmentobject-variation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variation: ::Value<String>,
    }

    impl ::codec::SerializeValue for TreatmentObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Feature", &self.feature)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TreatmentName", &self.treatment_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variation", &self.variation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TreatmentObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TreatmentObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TreatmentObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TreatmentObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut feature: Option<::Value<String>> = None;
                    let mut treatment_name: Option<::Value<String>> = None;
                    let mut variation: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Feature" => {
                                feature = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TreatmentName" => {
                                treatment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variation" => {
                                variation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TreatmentObject {
                        description: description,
                        feature: feature.ok_or(::serde::de::Error::missing_field("Feature"))?,
                        treatment_name: treatment_name.ok_or(::serde::de::Error::missing_field("TreatmentName"))?,
                        variation: variation.ok_or(::serde::de::Error::missing_field("Variation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Experiment.TreatmentToWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-treatmenttoweight.html) property type.
    #[derive(Debug, Default)]
    pub struct TreatmentToWeight {
        /// Property [`SplitWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-treatmenttoweight.html#cfn-evidently-experiment-treatmenttoweight-splitweight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub split_weight: ::Value<u32>,
        /// Property [`Treatment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-experiment-treatmenttoweight.html#cfn-evidently-experiment-treatmenttoweight-treatment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub treatment: ::Value<String>,
    }

    impl ::codec::SerializeValue for TreatmentToWeight {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SplitWeight", &self.split_weight)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Treatment", &self.treatment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TreatmentToWeight {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TreatmentToWeight, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TreatmentToWeight;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TreatmentToWeight")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut split_weight: Option<::Value<u32>> = None;
                    let mut treatment: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SplitWeight" => {
                                split_weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Treatment" => {
                                treatment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TreatmentToWeight {
                        split_weight: split_weight.ok_or(::serde::de::Error::missing_field("SplitWeight"))?,
                        treatment: treatment.ok_or(::serde::de::Error::missing_field("Treatment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod feature {
    //! Property types for the `Feature` resource.

    /// The [`AWS::Evidently::Feature.EntityOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-entityoverride.html) property type.
    #[derive(Debug, Default)]
    pub struct EntityOverride {
        /// Property [`EntityId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-entityoverride.html#cfn-evidently-feature-entityoverride-entityid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entity_id: Option<::Value<String>>,
        /// Property [`Variation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-entityoverride.html#cfn-evidently-feature-entityoverride-variation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variation: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EntityOverride {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref entity_id) = self.entity_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityId", entity_id)?;
            }
            if let Some(ref variation) = self.variation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variation", variation)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EntityOverride {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EntityOverride, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EntityOverride;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EntityOverride")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut entity_id: Option<::Value<String>> = None;
                    let mut variation: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EntityId" => {
                                entity_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variation" => {
                                variation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EntityOverride {
                        entity_id: entity_id,
                        variation: variation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Feature.VariationObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-variationobject.html) property type.
    #[derive(Debug, Default)]
    pub struct VariationObject {
        /// Property [`BooleanValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-variationobject.html#cfn-evidently-feature-variationobject-booleanvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub boolean_value: Option<::Value<bool>>,
        /// Property [`DoubleValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-variationobject.html#cfn-evidently-feature-variationobject-doublevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub double_value: Option<::Value<f64>>,
        /// Property [`LongValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-variationobject.html#cfn-evidently-feature-variationobject-longvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub long_value: Option<::Value<f64>>,
        /// Property [`StringValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-variationobject.html#cfn-evidently-feature-variationobject-stringvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_value: Option<::Value<String>>,
        /// Property [`VariationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-feature-variationobject.html#cfn-evidently-feature-variationobject-variationname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variation_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for VariationObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref boolean_value) = self.boolean_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BooleanValue", boolean_value)?;
            }
            if let Some(ref double_value) = self.double_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DoubleValue", double_value)?;
            }
            if let Some(ref long_value) = self.long_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LongValue", long_value)?;
            }
            if let Some(ref string_value) = self.string_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", string_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariationName", &self.variation_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VariationObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VariationObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VariationObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VariationObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut boolean_value: Option<::Value<bool>> = None;
                    let mut double_value: Option<::Value<f64>> = None;
                    let mut long_value: Option<::Value<f64>> = None;
                    let mut string_value: Option<::Value<String>> = None;
                    let mut variation_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BooleanValue" => {
                                boolean_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DoubleValue" => {
                                double_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LongValue" => {
                                long_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VariationName" => {
                                variation_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VariationObject {
                        boolean_value: boolean_value,
                        double_value: double_value,
                        long_value: long_value,
                        string_value: string_value,
                        variation_name: variation_name.ok_or(::serde::de::Error::missing_field("VariationName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod launch {
    //! Property types for the `Launch` resource.

    /// The [`AWS::Evidently::Launch.ExecutionStatusObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-executionstatusobject.html) property type.
    #[derive(Debug, Default)]
    pub struct ExecutionStatusObject {
        /// Property [`DesiredState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-executionstatusobject.html#cfn-evidently-launch-executionstatusobject-desiredstate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desired_state: Option<::Value<String>>,
        /// Property [`Reason`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-executionstatusobject.html#cfn-evidently-launch-executionstatusobject-reason).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reason: Option<::Value<String>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-executionstatusobject.html#cfn-evidently-launch-executionstatusobject-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for ExecutionStatusObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref desired_state) = self.desired_state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredState", desired_state)?;
            }
            if let Some(ref reason) = self.reason {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Reason", reason)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExecutionStatusObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExecutionStatusObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExecutionStatusObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExecutionStatusObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut desired_state: Option<::Value<String>> = None;
                    let mut reason: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DesiredState" => {
                                desired_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Reason" => {
                                reason = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExecutionStatusObject {
                        desired_state: desired_state,
                        reason: reason,
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Launch.GroupToWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-grouptoweight.html) property type.
    #[derive(Debug, Default)]
    pub struct GroupToWeight {
        /// Property [`GroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-grouptoweight.html#cfn-evidently-launch-grouptoweight-groupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_name: ::Value<String>,
        /// Property [`SplitWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-grouptoweight.html#cfn-evidently-launch-grouptoweight-splitweight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub split_weight: ::Value<u32>,
    }

    impl ::codec::SerializeValue for GroupToWeight {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", &self.group_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SplitWeight", &self.split_weight)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GroupToWeight {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupToWeight, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GroupToWeight;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GroupToWeight")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_name: Option<::Value<String>> = None;
                    let mut split_weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupName" => {
                                group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SplitWeight" => {
                                split_weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GroupToWeight {
                        group_name: group_name.ok_or(::serde::de::Error::missing_field("GroupName"))?,
                        split_weight: split_weight.ok_or(::serde::de::Error::missing_field("SplitWeight"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Launch.LaunchGroupObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-launchgroupobject.html) property type.
    #[derive(Debug, Default)]
    pub struct LaunchGroupObject {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-launchgroupobject.html#cfn-evidently-launch-launchgroupobject-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Feature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-launchgroupobject.html#cfn-evidently-launch-launchgroupobject-feature).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub feature: ::Value<String>,
        /// Property [`GroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-launchgroupobject.html#cfn-evidently-launch-launchgroupobject-groupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_name: ::Value<String>,
        /// Property [`Variation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-launchgroupobject.html#cfn-evidently-launch-launchgroupobject-variation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variation: ::Value<String>,
    }

    impl ::codec::SerializeValue for LaunchGroupObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Feature", &self.feature)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", &self.group_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variation", &self.variation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LaunchGroupObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchGroupObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LaunchGroupObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LaunchGroupObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut feature: Option<::Value<String>> = None;
                    let mut group_name: Option<::Value<String>> = None;
                    let mut variation: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Feature" => {
                                feature = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupName" => {
                                group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variation" => {
                                variation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LaunchGroupObject {
                        description: description,
                        feature: feature.ok_or(::serde::de::Error::missing_field("Feature"))?,
                        group_name: group_name.ok_or(::serde::de::Error::missing_field("GroupName"))?,
                        variation: variation.ok_or(::serde::de::Error::missing_field("Variation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Launch.MetricDefinitionObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-metricdefinitionobject.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricDefinitionObject {
        /// Property [`EntityIdKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-metricdefinitionobject.html#cfn-evidently-launch-metricdefinitionobject-entityidkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entity_id_key: ::Value<String>,
        /// Property [`EventPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-metricdefinitionobject.html#cfn-evidently-launch-metricdefinitionobject-eventpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_pattern: Option<::Value<String>>,
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-metricdefinitionobject.html#cfn-evidently-launch-metricdefinitionobject-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`UnitLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-metricdefinitionobject.html#cfn-evidently-launch-metricdefinitionobject-unitlabel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit_label: Option<::Value<String>>,
        /// Property [`ValueKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-metricdefinitionobject.html#cfn-evidently-launch-metricdefinitionobject-valuekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for MetricDefinitionObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityIdKey", &self.entity_id_key)?;
            if let Some(ref event_pattern) = self.event_pattern {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventPattern", event_pattern)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            if let Some(ref unit_label) = self.unit_label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnitLabel", unit_label)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueKey", &self.value_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricDefinitionObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricDefinitionObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricDefinitionObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricDefinitionObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut entity_id_key: Option<::Value<String>> = None;
                    let mut event_pattern: Option<::Value<String>> = None;
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut unit_label: Option<::Value<String>> = None;
                    let mut value_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EntityIdKey" => {
                                entity_id_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventPattern" => {
                                event_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnitLabel" => {
                                unit_label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueKey" => {
                                value_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricDefinitionObject {
                        entity_id_key: entity_id_key.ok_or(::serde::de::Error::missing_field("EntityIdKey"))?,
                        event_pattern: event_pattern,
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        unit_label: unit_label,
                        value_key: value_key.ok_or(::serde::de::Error::missing_field("ValueKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Launch.SegmentOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-segmentoverride.html) property type.
    #[derive(Debug, Default)]
    pub struct SegmentOverride {
        /// Property [`EvaluationOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-segmentoverride.html#cfn-evidently-launch-segmentoverride-evaluationorder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub evaluation_order: ::Value<u32>,
        /// Property [`Segment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-segmentoverride.html#cfn-evidently-launch-segmentoverride-segment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment: ::Value<String>,
        /// Property [`Weights`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-segmentoverride.html#cfn-evidently-launch-segmentoverride-weights).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weights: ::ValueList<GroupToWeight>,
    }

    impl ::codec::SerializeValue for SegmentOverride {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EvaluationOrder", &self.evaluation_order)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Segment", &self.segment)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weights", &self.weights)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SegmentOverride {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SegmentOverride, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SegmentOverride;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SegmentOverride")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut evaluation_order: Option<::Value<u32>> = None;
                    let mut segment: Option<::Value<String>> = None;
                    let mut weights: Option<::ValueList<GroupToWeight>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EvaluationOrder" => {
                                evaluation_order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Segment" => {
                                segment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weights" => {
                                weights = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SegmentOverride {
                        evaluation_order: evaluation_order.ok_or(::serde::de::Error::missing_field("EvaluationOrder"))?,
                        segment: segment.ok_or(::serde::de::Error::missing_field("Segment"))?,
                        weights: weights.ok_or(::serde::de::Error::missing_field("Weights"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Launch.StepConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-stepconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct StepConfig {
        /// Property [`GroupWeights`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-stepconfig.html#cfn-evidently-launch-stepconfig-groupweights).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_weights: ::ValueList<GroupToWeight>,
        /// Property [`SegmentOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-stepconfig.html#cfn-evidently-launch-stepconfig-segmentoverrides).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_overrides: Option<::ValueList<SegmentOverride>>,
        /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-launch-stepconfig.html#cfn-evidently-launch-stepconfig-starttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time: ::Value<String>,
    }

    impl ::codec::SerializeValue for StepConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupWeights", &self.group_weights)?;
            if let Some(ref segment_overrides) = self.segment_overrides {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentOverrides", segment_overrides)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", &self.start_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StepConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StepConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StepConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StepConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut group_weights: Option<::ValueList<GroupToWeight>> = None;
                    let mut segment_overrides: Option<::ValueList<SegmentOverride>> = None;
                    let mut start_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GroupWeights" => {
                                group_weights = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentOverrides" => {
                                segment_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTime" => {
                                start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StepConfig {
                        group_weights: group_weights.ok_or(::serde::de::Error::missing_field("GroupWeights"))?,
                        segment_overrides: segment_overrides,
                        start_time: start_time.ok_or(::serde::de::Error::missing_field("StartTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod project {
    //! Property types for the `Project` resource.

    /// The [`AWS::Evidently::Project.AppConfigResourceObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-appconfigresourceobject.html) property type.
    #[derive(Debug, Default)]
    pub struct AppConfigResourceObject {
        /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-appconfigresourceobject.html#cfn-evidently-project-appconfigresourceobject-applicationid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_id: ::Value<String>,
        /// Property [`EnvironmentId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-appconfigresourceobject.html#cfn-evidently-project-appconfigresourceobject-environmentid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for AppConfigResourceObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", &self.application_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentId", &self.environment_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AppConfigResourceObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AppConfigResourceObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AppConfigResourceObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AppConfigResourceObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_id: Option<::Value<String>> = None;
                    let mut environment_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationId" => {
                                application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnvironmentId" => {
                                environment_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AppConfigResourceObject {
                        application_id: application_id.ok_or(::serde::de::Error::missing_field("ApplicationId"))?,
                        environment_id: environment_id.ok_or(::serde::de::Error::missing_field("EnvironmentId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Project.DataDeliveryObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-datadeliveryobject.html) property type.
    #[derive(Debug, Default)]
    pub struct DataDeliveryObject {
        /// Property [`LogGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-datadeliveryobject.html#cfn-evidently-project-datadeliveryobject-loggroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group: Option<::Value<String>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-datadeliveryobject.html#cfn-evidently-project-datadeliveryobject-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<S3Destination>>,
    }

    impl ::codec::SerializeValue for DataDeliveryObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_group) = self.log_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroup", log_group)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataDeliveryObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataDeliveryObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataDeliveryObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataDeliveryObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_group: Option<::Value<String>> = None;
                    let mut s3: Option<::Value<S3Destination>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogGroup" => {
                                log_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataDeliveryObject {
                        log_group: log_group,
                        s3: s3,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Evidently::Project.S3Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-s3destination.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Destination {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-s3destination.html#cfn-evidently-project-s3destination-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-evidently-project-s3destination.html#cfn-evidently-project-s3destination-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Destination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Destination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Destination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Destination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Destination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Destination {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
