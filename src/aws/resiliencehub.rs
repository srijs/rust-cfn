//! Types for the `ResilienceHub` service.

/// The [`AWS::ResilienceHub::App`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html) resource type.
#[derive(Debug, Default)]
pub struct App {
    properties: AppProperties
}

/// Properties for the `App` resource.
#[derive(Debug, Default)]
pub struct AppProperties {
    /// Property [`AppAssessmentSchedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html#cfn-resiliencehub-app-appassessmentschedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_assessment_schedule: Option<::Value<String>>,
    /// Property [`AppTemplateBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html#cfn-resiliencehub-app-apptemplatebody).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_template_body: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html#cfn-resiliencehub-app-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EventSubscriptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html#cfn-resiliencehub-app-eventsubscriptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_subscriptions: Option<::ValueList<self::app::EventSubscription>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html#cfn-resiliencehub-app-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`PermissionModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html#cfn-resiliencehub-app-permissionmodel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permission_model: Option<::Value<self::app::PermissionModel>>,
    /// Property [`ResiliencyPolicyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html#cfn-resiliencehub-app-resiliencypolicyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resiliency_policy_arn: Option<::Value<String>>,
    /// Property [`ResourceMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html#cfn-resiliencehub-app-resourcemappings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_mappings: ::ValueList<self::app::ResourceMapping>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-app.html#cfn-resiliencehub-app-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for AppProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref app_assessment_schedule) = self.app_assessment_schedule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppAssessmentSchedule", app_assessment_schedule)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppTemplateBody", &self.app_template_body)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref event_subscriptions) = self.event_subscriptions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSubscriptions", event_subscriptions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref permission_model) = self.permission_model {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PermissionModel", permission_model)?;
        }
        if let Some(ref resiliency_policy_arn) = self.resiliency_policy_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResiliencyPolicyArn", resiliency_policy_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceMappings", &self.resource_mappings)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AppProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AppProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AppProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AppProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_assessment_schedule: Option<::Value<String>> = None;
                let mut app_template_body: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut event_subscriptions: Option<::ValueList<self::app::EventSubscription>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut permission_model: Option<::Value<self::app::PermissionModel>> = None;
                let mut resiliency_policy_arn: Option<::Value<String>> = None;
                let mut resource_mappings: Option<::ValueList<self::app::ResourceMapping>> = None;
                let mut tags: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppAssessmentSchedule" => {
                            app_assessment_schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AppTemplateBody" => {
                            app_template_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventSubscriptions" => {
                            event_subscriptions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PermissionModel" => {
                            permission_model = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResiliencyPolicyArn" => {
                            resiliency_policy_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceMappings" => {
                            resource_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AppProperties {
                    app_assessment_schedule: app_assessment_schedule,
                    app_template_body: app_template_body.ok_or(::serde::de::Error::missing_field("AppTemplateBody"))?,
                    description: description,
                    event_subscriptions: event_subscriptions,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    permission_model: permission_model,
                    resiliency_policy_arn: resiliency_policy_arn,
                    resource_mappings: resource_mappings.ok_or(::serde::de::Error::missing_field("ResourceMappings"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for App {
    type Properties = AppProperties;
    const TYPE: &'static str = "AWS::ResilienceHub::App";
    fn properties(&self) -> &AppProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AppProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for App {}

impl From<AppProperties> for App {
    fn from(properties: AppProperties) -> App {
        App { properties }
    }
}

/// The [`AWS::ResilienceHub::ResiliencyPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-resiliencypolicy.html) resource type.
#[derive(Debug, Default)]
pub struct ResiliencyPolicy {
    properties: ResiliencyPolicyProperties
}

/// Properties for the `ResiliencyPolicy` resource.
#[derive(Debug, Default)]
pub struct ResiliencyPolicyProperties {
    /// Property [`DataLocationConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-resiliencypolicy.html#cfn-resiliencehub-resiliencypolicy-datalocationconstraint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_location_constraint: Option<::Value<String>>,
    /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-resiliencypolicy.html#cfn-resiliencehub-resiliencypolicy-policy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy: ::ValueMap<self::resiliency_policy::FailurePolicy>,
    /// Property [`PolicyDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-resiliencypolicy.html#cfn-resiliencehub-resiliencypolicy-policydescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_description: Option<::Value<String>>,
    /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-resiliencypolicy.html#cfn-resiliencehub-resiliencypolicy-policyname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-resiliencypolicy.html#cfn-resiliencehub-resiliencypolicy-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueMap<String>>,
    /// Property [`Tier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resiliencehub-resiliencypolicy.html#cfn-resiliencehub-resiliencypolicy-tier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tier: ::Value<String>,
}

impl ::serde::Serialize for ResiliencyPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref data_location_constraint) = self.data_location_constraint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLocationConstraint", data_location_constraint)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", &self.policy)?;
        if let Some(ref policy_description) = self.policy_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDescription", policy_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tier", &self.tier)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResiliencyPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResiliencyPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResiliencyPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResiliencyPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_location_constraint: Option<::Value<String>> = None;
                let mut policy: Option<::ValueMap<self::resiliency_policy::FailurePolicy>> = None;
                let mut policy_description: Option<::Value<String>> = None;
                let mut policy_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueMap<String>> = None;
                let mut tier: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataLocationConstraint" => {
                            data_location_constraint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policy" => {
                            policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyDescription" => {
                            policy_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyName" => {
                            policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tier" => {
                            tier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResiliencyPolicyProperties {
                    data_location_constraint: data_location_constraint,
                    policy: policy.ok_or(::serde::de::Error::missing_field("Policy"))?,
                    policy_description: policy_description,
                    policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    tags: tags,
                    tier: tier.ok_or(::serde::de::Error::missing_field("Tier"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResiliencyPolicy {
    type Properties = ResiliencyPolicyProperties;
    const TYPE: &'static str = "AWS::ResilienceHub::ResiliencyPolicy";
    fn properties(&self) -> &ResiliencyPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResiliencyPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResiliencyPolicy {}

impl From<ResiliencyPolicyProperties> for ResiliencyPolicy {
    fn from(properties: ResiliencyPolicyProperties) -> ResiliencyPolicy {
        ResiliencyPolicy { properties }
    }
}

pub mod app {
    //! Property types for the `App` resource.

    /// The [`AWS::ResilienceHub::App.EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-eventsubscription.html) property type.
    #[derive(Debug, Default)]
    pub struct EventSubscription {
        /// Property [`EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-eventsubscription.html#cfn-resiliencehub-app-eventsubscription-eventtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_type: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-eventsubscription.html#cfn-resiliencehub-app-eventsubscription-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`SnsTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-eventsubscription.html#cfn-resiliencehub-app-eventsubscription-snstopicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sns_topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EventSubscription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventType", &self.event_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref sns_topic_arn) = self.sns_topic_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArn", sns_topic_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventSubscription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSubscription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventSubscription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventSubscription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_type: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut sns_topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventType" => {
                                event_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnsTopicArn" => {
                                sns_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventSubscription {
                        event_type: event_type.ok_or(::serde::de::Error::missing_field("EventType"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        sns_topic_arn: sns_topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ResilienceHub::App.PermissionModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-permissionmodel.html) property type.
    #[derive(Debug, Default)]
    pub struct PermissionModel {
        /// Property [`CrossAccountRoleArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-permissionmodel.html#cfn-resiliencehub-app-permissionmodel-crossaccountrolearns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cross_account_role_arns: Option<::ValueList<String>>,
        /// Property [`InvokerRoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-permissionmodel.html#cfn-resiliencehub-app-permissionmodel-invokerrolename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invoker_role_name: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-permissionmodel.html#cfn-resiliencehub-app-permissionmodel-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for PermissionModel {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cross_account_role_arns) = self.cross_account_role_arns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrossAccountRoleArns", cross_account_role_arns)?;
            }
            if let Some(ref invoker_role_name) = self.invoker_role_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvokerRoleName", invoker_role_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PermissionModel {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PermissionModel, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PermissionModel;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PermissionModel")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cross_account_role_arns: Option<::ValueList<String>> = None;
                    let mut invoker_role_name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CrossAccountRoleArns" => {
                                cross_account_role_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InvokerRoleName" => {
                                invoker_role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PermissionModel {
                        cross_account_role_arns: cross_account_role_arns,
                        invoker_role_name: invoker_role_name,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ResilienceHub::App.PhysicalResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-physicalresourceid.html) property type.
    #[derive(Debug, Default)]
    pub struct PhysicalResourceId {
        /// Property [`AwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-physicalresourceid.html#cfn-resiliencehub-app-physicalresourceid-awsaccountid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_account_id: Option<::Value<String>>,
        /// Property [`AwsRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-physicalresourceid.html#cfn-resiliencehub-app-physicalresourceid-awsregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_region: Option<::Value<String>>,
        /// Property [`Identifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-physicalresourceid.html#cfn-resiliencehub-app-physicalresourceid-identifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identifier: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-physicalresourceid.html#cfn-resiliencehub-app-physicalresourceid-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for PhysicalResourceId {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_account_id) = self.aws_account_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", aws_account_id)?;
            }
            if let Some(ref aws_region) = self.aws_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsRegion", aws_region)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Identifier", &self.identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PhysicalResourceId {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PhysicalResourceId, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PhysicalResourceId;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PhysicalResourceId")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_account_id: Option<::Value<String>> = None;
                    let mut aws_region: Option<::Value<String>> = None;
                    let mut identifier: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsAccountId" => {
                                aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AwsRegion" => {
                                aws_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Identifier" => {
                                identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PhysicalResourceId {
                        aws_account_id: aws_account_id,
                        aws_region: aws_region,
                        identifier: identifier.ok_or(::serde::de::Error::missing_field("Identifier"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ResilienceHub::App.ResourceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-resourcemapping.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceMapping {
        /// Property [`EksSourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-resourcemapping.html#cfn-resiliencehub-app-resourcemapping-ekssourcename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub eks_source_name: Option<::Value<String>>,
        /// Property [`LogicalStackName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-resourcemapping.html#cfn-resiliencehub-app-resourcemapping-logicalstackname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logical_stack_name: Option<::Value<String>>,
        /// Property [`MappingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-resourcemapping.html#cfn-resiliencehub-app-resourcemapping-mappingtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mapping_type: ::Value<String>,
        /// Property [`PhysicalResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-resourcemapping.html#cfn-resiliencehub-app-resourcemapping-physicalresourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub physical_resource_id: ::Value<PhysicalResourceId>,
        /// Property [`ResourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-resourcemapping.html#cfn-resiliencehub-app-resourcemapping-resourcename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_name: Option<::Value<String>>,
        /// Property [`TerraformSourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-app-resourcemapping.html#cfn-resiliencehub-app-resourcemapping-terraformsourcename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub terraform_source_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ResourceMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref eks_source_name) = self.eks_source_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EksSourceName", eks_source_name)?;
            }
            if let Some(ref logical_stack_name) = self.logical_stack_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogicalStackName", logical_stack_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MappingType", &self.mapping_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhysicalResourceId", &self.physical_resource_id)?;
            if let Some(ref resource_name) = self.resource_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceName", resource_name)?;
            }
            if let Some(ref terraform_source_name) = self.terraform_source_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TerraformSourceName", terraform_source_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut eks_source_name: Option<::Value<String>> = None;
                    let mut logical_stack_name: Option<::Value<String>> = None;
                    let mut mapping_type: Option<::Value<String>> = None;
                    let mut physical_resource_id: Option<::Value<PhysicalResourceId>> = None;
                    let mut resource_name: Option<::Value<String>> = None;
                    let mut terraform_source_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EksSourceName" => {
                                eks_source_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogicalStackName" => {
                                logical_stack_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MappingType" => {
                                mapping_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PhysicalResourceId" => {
                                physical_resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceName" => {
                                resource_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TerraformSourceName" => {
                                terraform_source_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceMapping {
                        eks_source_name: eks_source_name,
                        logical_stack_name: logical_stack_name,
                        mapping_type: mapping_type.ok_or(::serde::de::Error::missing_field("MappingType"))?,
                        physical_resource_id: physical_resource_id.ok_or(::serde::de::Error::missing_field("PhysicalResourceId"))?,
                        resource_name: resource_name,
                        terraform_source_name: terraform_source_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod resiliency_policy {
    //! Property types for the `ResiliencyPolicy` resource.

    /// The [`AWS::ResilienceHub::ResiliencyPolicy.FailurePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-resiliencypolicy-failurepolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct FailurePolicy {
        /// Property [`RpoInSecs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-resiliencypolicy-failurepolicy.html#cfn-resiliencehub-resiliencypolicy-failurepolicy-rpoinsecs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rpo_in_secs: ::Value<u32>,
        /// Property [`RtoInSecs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resiliencehub-resiliencypolicy-failurepolicy.html#cfn-resiliencehub-resiliencypolicy-failurepolicy-rtoinsecs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rto_in_secs: ::Value<u32>,
    }

    impl ::codec::SerializeValue for FailurePolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RpoInSecs", &self.rpo_in_secs)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RtoInSecs", &self.rto_in_secs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FailurePolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FailurePolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FailurePolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FailurePolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rpo_in_secs: Option<::Value<u32>> = None;
                    let mut rto_in_secs: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RpoInSecs" => {
                                rpo_in_secs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RtoInSecs" => {
                                rto_in_secs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FailurePolicy {
                        rpo_in_secs: rpo_in_secs.ok_or(::serde::de::Error::missing_field("RpoInSecs"))?,
                        rto_in_secs: rto_in_secs.ok_or(::serde::de::Error::missing_field("RtoInSecs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
