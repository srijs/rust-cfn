//! Types for the `IoT` service.

/// The [`AWS::IoT::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html) resource type.
#[derive(Debug, Default)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug, Default)]
pub struct CertificateProperties {
    /// Property [`CertificateSigningRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html#cfn-iot-certificate-certificatesigningrequest).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_signing_request: ::Value<String>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html#cfn-iot-certificate-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: ::Value<String>,
}

impl ::serde::Serialize for CertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateSigningRequest", &self.certificate_signing_request)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CertificateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CertificateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate_signing_request: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateSigningRequest" => {
                            certificate_signing_request = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CertificateProperties {
                    certificate_signing_request: certificate_signing_request.ok_or(::serde::de::Error::missing_field("CertificateSigningRequest"))?,
                    status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::IoT::Certificate";
    fn properties(&self) -> &CertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Certificate {}

impl From<CertificateProperties> for Certificate {
    fn from(properties: CertificateProperties) -> Certificate {
        Certificate { properties }
    }
}

/// The [`AWS::IoT::Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policy.html) resource type.
#[derive(Debug, Default)]
pub struct Policy {
    properties: PolicyProperties
}

/// Properties for the `Policy` resource.
#[derive(Debug, Default)]
pub struct PolicyProperties {
    /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policy.html#cfn-iot-policy-policydocument).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub policy_document: ::Value<::json::Value>,
    /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policy.html#cfn-iot-policy-policyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub policy_name: Option<::Value<String>>,
}

impl ::serde::Serialize for PolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        if let Some(ref policy_name) = self.policy_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", policy_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut policy_document: Option<::Value<::json::Value>> = None;
                let mut policy_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PolicyDocument" => {
                            policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyName" => {
                            policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PolicyProperties {
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                    policy_name: policy_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Policy {
    type Properties = PolicyProperties;
    const TYPE: &'static str = "AWS::IoT::Policy";
    fn properties(&self) -> &PolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Policy {}

impl From<PolicyProperties> for Policy {
    fn from(properties: PolicyProperties) -> Policy {
        Policy { properties }
    }
}

/// The [`AWS::IoT::PolicyPrincipalAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policyprincipalattachment.html) resource type.
#[derive(Debug, Default)]
pub struct PolicyPrincipalAttachment {
    properties: PolicyPrincipalAttachmentProperties
}

/// Properties for the `PolicyPrincipalAttachment` resource.
#[derive(Debug, Default)]
pub struct PolicyPrincipalAttachmentProperties {
    /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policyprincipalattachment.html#cfn-iot-policyprincipalattachment-policyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub policy_name: ::Value<String>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policyprincipalattachment.html#cfn-iot-policyprincipalattachment-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: ::Value<String>,
}

impl ::serde::Serialize for PolicyPrincipalAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PolicyPrincipalAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyPrincipalAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyPrincipalAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PolicyPrincipalAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut policy_name: Option<::Value<String>> = None;
                let mut principal: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PolicyName" => {
                            policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PolicyPrincipalAttachmentProperties {
                    policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PolicyPrincipalAttachment {
    type Properties = PolicyPrincipalAttachmentProperties;
    const TYPE: &'static str = "AWS::IoT::PolicyPrincipalAttachment";
    fn properties(&self) -> &PolicyPrincipalAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PolicyPrincipalAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PolicyPrincipalAttachment {}

impl From<PolicyPrincipalAttachmentProperties> for PolicyPrincipalAttachment {
    fn from(properties: PolicyPrincipalAttachmentProperties) -> PolicyPrincipalAttachment {
        PolicyPrincipalAttachment { properties }
    }
}

/// The [`AWS::IoT::Thing`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thing.html) resource type.
#[derive(Debug, Default)]
pub struct Thing {
    properties: ThingProperties
}

/// Properties for the `Thing` resource.
#[derive(Debug, Default)]
pub struct ThingProperties {
    /// Property [`AttributePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thing.html#cfn-iot-thing-attributepayload).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attribute_payload: Option<::Value<self::thing::AttributePayload>>,
    /// Property [`ThingName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thing.html#cfn-iot-thing-thingname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub thing_name: Option<::Value<String>>,
}

impl ::serde::Serialize for ThingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref attribute_payload) = self.attribute_payload {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributePayload", attribute_payload)?;
        }
        if let Some(ref thing_name) = self.thing_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingName", thing_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ThingProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ThingProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThingProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ThingProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attribute_payload: Option<::Value<self::thing::AttributePayload>> = None;
                let mut thing_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AttributePayload" => {
                            attribute_payload = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThingName" => {
                            thing_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ThingProperties {
                    attribute_payload: attribute_payload,
                    thing_name: thing_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Thing {
    type Properties = ThingProperties;
    const TYPE: &'static str = "AWS::IoT::Thing";
    fn properties(&self) -> &ThingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThingProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Thing {}

impl From<ThingProperties> for Thing {
    fn from(properties: ThingProperties) -> Thing {
        Thing { properties }
    }
}

/// The [`AWS::IoT::ThingPrincipalAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingprincipalattachment.html) resource type.
#[derive(Debug, Default)]
pub struct ThingPrincipalAttachment {
    properties: ThingPrincipalAttachmentProperties
}

/// Properties for the `ThingPrincipalAttachment` resource.
#[derive(Debug, Default)]
pub struct ThingPrincipalAttachmentProperties {
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingprincipalattachment.html#cfn-iot-thingprincipalattachment-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: ::Value<String>,
    /// Property [`ThingName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingprincipalattachment.html#cfn-iot-thingprincipalattachment-thingname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub thing_name: ::Value<String>,
}

impl ::serde::Serialize for ThingPrincipalAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingName", &self.thing_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ThingPrincipalAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ThingPrincipalAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThingPrincipalAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ThingPrincipalAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut principal: Option<::Value<String>> = None;
                let mut thing_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThingName" => {
                            thing_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ThingPrincipalAttachmentProperties {
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    thing_name: thing_name.ok_or(::serde::de::Error::missing_field("ThingName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ThingPrincipalAttachment {
    type Properties = ThingPrincipalAttachmentProperties;
    const TYPE: &'static str = "AWS::IoT::ThingPrincipalAttachment";
    fn properties(&self) -> &ThingPrincipalAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThingPrincipalAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ThingPrincipalAttachment {}

impl From<ThingPrincipalAttachmentProperties> for ThingPrincipalAttachment {
    fn from(properties: ThingPrincipalAttachmentProperties) -> ThingPrincipalAttachment {
        ThingPrincipalAttachment { properties }
    }
}

/// The [`AWS::IoT::TopicRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicrule.html) resource type.
#[derive(Debug, Default)]
pub struct TopicRule {
    properties: TopicRuleProperties
}

/// Properties for the `TopicRule` resource.
#[derive(Debug, Default)]
pub struct TopicRuleProperties {
    /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicrule.html#cfn-iot-topicrule-rulename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rule_name: Option<::Value<String>>,
    /// Property [`TopicRulePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicrule.html#cfn-iot-topicrule-topicrulepayload).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub topic_rule_payload: ::Value<self::topic_rule::TopicRulePayload>,
}

impl ::serde::Serialize for TopicRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref rule_name) = self.rule_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", rule_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicRulePayload", &self.topic_rule_payload)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TopicRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TopicRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TopicRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TopicRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut rule_name: Option<::Value<String>> = None;
                let mut topic_rule_payload: Option<::Value<self::topic_rule::TopicRulePayload>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RuleName" => {
                            rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TopicRulePayload" => {
                            topic_rule_payload = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TopicRuleProperties {
                    rule_name: rule_name,
                    topic_rule_payload: topic_rule_payload.ok_or(::serde::de::Error::missing_field("TopicRulePayload"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TopicRule {
    type Properties = TopicRuleProperties;
    const TYPE: &'static str = "AWS::IoT::TopicRule";
    fn properties(&self) -> &TopicRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TopicRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TopicRule {}

impl From<TopicRuleProperties> for TopicRule {
    fn from(properties: TopicRuleProperties) -> TopicRule {
        TopicRule { properties }
    }
}

pub mod thing {
    //! Property types for the `Thing` resource.

    /// The [`AWS::IoT::Thing.AttributePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thing-attributepayload.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributePayload {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thing-attributepayload.html#cfn-iot-thing-attributepayload-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for AttributePayload {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributePayload {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributePayload, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributePayload;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributePayload")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributePayload {
                        attributes: attributes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod topic_rule {
    //! Property types for the `TopicRule` resource.

    /// The [`AWS::IoT::TopicRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`CloudwatchAlarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-cloudwatchalarm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloudwatch_alarm: Option<::Value<CloudwatchAlarmAction>>,
        /// Property [`CloudwatchMetric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-cloudwatchmetric).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloudwatch_metric: Option<::Value<CloudwatchMetricAction>>,
        /// Property [`DynamoDB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-dynamodb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamo_db: Option<::Value<DynamoDBAction>>,
        /// Property [`DynamoDBv2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-dynamodbv2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamo_d_bv2: Option<::Value<DynamoDBv2Action>>,
        /// Property [`Elasticsearch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-elasticsearch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub elasticsearch: Option<::Value<ElasticsearchAction>>,
        /// Property [`Firehose`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-firehose).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub firehose: Option<::Value<FirehoseAction>>,
        /// Property [`Kinesis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-kinesis).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis: Option<::Value<KinesisAction>>,
        /// Property [`Lambda`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-lambda).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda: Option<::Value<LambdaAction>>,
        /// Property [`Republish`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-republish).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub republish: Option<::Value<RepublishAction>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<S3Action>>,
        /// Property [`Sns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-sns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sns: Option<::Value<SnsAction>>,
        /// Property [`Sqs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-sqs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sqs: Option<::Value<SqsAction>>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloudwatch_alarm) = self.cloudwatch_alarm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudwatchAlarm", cloudwatch_alarm)?;
            }
            if let Some(ref cloudwatch_metric) = self.cloudwatch_metric {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudwatchMetric", cloudwatch_metric)?;
            }
            if let Some(ref dynamo_db) = self.dynamo_db {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDB", dynamo_db)?;
            }
            if let Some(ref dynamo_d_bv2) = self.dynamo_d_bv2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDBv2", dynamo_d_bv2)?;
            }
            if let Some(ref elasticsearch) = self.elasticsearch {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Elasticsearch", elasticsearch)?;
            }
            if let Some(ref firehose) = self.firehose {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Firehose", firehose)?;
            }
            if let Some(ref kinesis) = self.kinesis {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Kinesis", kinesis)?;
            }
            if let Some(ref lambda) = self.lambda {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lambda", lambda)?;
            }
            if let Some(ref republish) = self.republish {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Republish", republish)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            if let Some(ref sns) = self.sns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sns", sns)?;
            }
            if let Some(ref sqs) = self.sqs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sqs", sqs)?;
            }
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
                    let mut cloudwatch_alarm: Option<::Value<CloudwatchAlarmAction>> = None;
                    let mut cloudwatch_metric: Option<::Value<CloudwatchMetricAction>> = None;
                    let mut dynamo_db: Option<::Value<DynamoDBAction>> = None;
                    let mut dynamo_d_bv2: Option<::Value<DynamoDBv2Action>> = None;
                    let mut elasticsearch: Option<::Value<ElasticsearchAction>> = None;
                    let mut firehose: Option<::Value<FirehoseAction>> = None;
                    let mut kinesis: Option<::Value<KinesisAction>> = None;
                    let mut lambda: Option<::Value<LambdaAction>> = None;
                    let mut republish: Option<::Value<RepublishAction>> = None;
                    let mut s3: Option<::Value<S3Action>> = None;
                    let mut sns: Option<::Value<SnsAction>> = None;
                    let mut sqs: Option<::Value<SqsAction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudwatchAlarm" => {
                                cloudwatch_alarm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudwatchMetric" => {
                                cloudwatch_metric = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DynamoDB" => {
                                dynamo_db = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DynamoDBv2" => {
                                dynamo_d_bv2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Elasticsearch" => {
                                elasticsearch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Firehose" => {
                                firehose = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Kinesis" => {
                                kinesis = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Lambda" => {
                                lambda = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Republish" => {
                                republish = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sns" => {
                                sns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sqs" => {
                                sqs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        cloudwatch_alarm: cloudwatch_alarm,
                        cloudwatch_metric: cloudwatch_metric,
                        dynamo_db: dynamo_db,
                        dynamo_d_bv2: dynamo_d_bv2,
                        elasticsearch: elasticsearch,
                        firehose: firehose,
                        kinesis: kinesis,
                        lambda: lambda,
                        republish: republish,
                        s3: s3,
                        sns: sns,
                        sqs: sqs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.CloudwatchAlarmAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudwatchAlarmAction {
        /// Property [`AlarmName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html#cfn-iot-topicrule-cloudwatchalarmaction-alarmname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarm_name: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html#cfn-iot-topicrule-cloudwatchalarmaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`StateReason`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html#cfn-iot-topicrule-cloudwatchalarmaction-statereason).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state_reason: ::Value<String>,
        /// Property [`StateValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html#cfn-iot-topicrule-cloudwatchalarmaction-statevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudwatchAlarmAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmName", &self.alarm_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StateReason", &self.state_reason)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StateValue", &self.state_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudwatchAlarmAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudwatchAlarmAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudwatchAlarmAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudwatchAlarmAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut state_reason: Option<::Value<String>> = None;
                    let mut state_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmName" => {
                                alarm_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StateReason" => {
                                state_reason = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StateValue" => {
                                state_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudwatchAlarmAction {
                        alarm_name: alarm_name.ok_or(::serde::de::Error::missing_field("AlarmName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        state_reason: state_reason.ok_or(::serde::de::Error::missing_field("StateReason"))?,
                        state_value: state_value.ok_or(::serde::de::Error::missing_field("StateValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.CloudwatchMetricAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudwatchMetricAction {
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`MetricNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-metricnamespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_namespace: ::Value<String>,
        /// Property [`MetricTimestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-metrictimestamp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_timestamp: Option<::Value<String>>,
        /// Property [`MetricUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-metricunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_unit: ::Value<String>,
        /// Property [`MetricValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-metricvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_value: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudwatchMetricAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricNamespace", &self.metric_namespace)?;
            if let Some(ref metric_timestamp) = self.metric_timestamp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricTimestamp", metric_timestamp)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricUnit", &self.metric_unit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricValue", &self.metric_value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudwatchMetricAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudwatchMetricAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudwatchMetricAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudwatchMetricAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut metric_namespace: Option<::Value<String>> = None;
                    let mut metric_timestamp: Option<::Value<String>> = None;
                    let mut metric_unit: Option<::Value<String>> = None;
                    let mut metric_value: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricNamespace" => {
                                metric_namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricTimestamp" => {
                                metric_timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricUnit" => {
                                metric_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricValue" => {
                                metric_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudwatchMetricAction {
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        metric_namespace: metric_namespace.ok_or(::serde::de::Error::missing_field("MetricNamespace"))?,
                        metric_timestamp: metric_timestamp,
                        metric_unit: metric_unit.ok_or(::serde::de::Error::missing_field("MetricUnit"))?,
                        metric_value: metric_value.ok_or(::serde::de::Error::missing_field("MetricValue"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.DynamoDBAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html) property type.
    #[derive(Debug, Default)]
    pub struct DynamoDBAction {
        /// Property [`HashKeyField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-hashkeyfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_key_field: ::Value<String>,
        /// Property [`HashKeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-hashkeytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_key_type: Option<::Value<String>>,
        /// Property [`HashKeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-hashkeyvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_key_value: ::Value<String>,
        /// Property [`PayloadField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-payloadfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload_field: Option<::Value<String>>,
        /// Property [`RangeKeyField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-rangekeyfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range_key_field: Option<::Value<String>>,
        /// Property [`RangeKeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-rangekeytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range_key_type: Option<::Value<String>>,
        /// Property [`RangeKeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-rangekeyvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range_key_value: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for DynamoDBAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashKeyField", &self.hash_key_field)?;
            if let Some(ref hash_key_type) = self.hash_key_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashKeyType", hash_key_type)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashKeyValue", &self.hash_key_value)?;
            if let Some(ref payload_field) = self.payload_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadField", payload_field)?;
            }
            if let Some(ref range_key_field) = self.range_key_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RangeKeyField", range_key_field)?;
            }
            if let Some(ref range_key_type) = self.range_key_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RangeKeyType", range_key_type)?;
            }
            if let Some(ref range_key_value) = self.range_key_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RangeKeyValue", range_key_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynamoDBAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamoDBAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamoDBAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamoDBAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hash_key_field: Option<::Value<String>> = None;
                    let mut hash_key_type: Option<::Value<String>> = None;
                    let mut hash_key_value: Option<::Value<String>> = None;
                    let mut payload_field: Option<::Value<String>> = None;
                    let mut range_key_field: Option<::Value<String>> = None;
                    let mut range_key_type: Option<::Value<String>> = None;
                    let mut range_key_value: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HashKeyField" => {
                                hash_key_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HashKeyType" => {
                                hash_key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HashKeyValue" => {
                                hash_key_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PayloadField" => {
                                payload_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RangeKeyField" => {
                                range_key_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RangeKeyType" => {
                                range_key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RangeKeyValue" => {
                                range_key_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamoDBAction {
                        hash_key_field: hash_key_field.ok_or(::serde::de::Error::missing_field("HashKeyField"))?,
                        hash_key_type: hash_key_type,
                        hash_key_value: hash_key_value.ok_or(::serde::de::Error::missing_field("HashKeyValue"))?,
                        payload_field: payload_field,
                        range_key_field: range_key_field,
                        range_key_type: range_key_type,
                        range_key_value: range_key_value,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.DynamoDBv2Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbv2action.html) property type.
    #[derive(Debug, Default)]
    pub struct DynamoDBv2Action {
        /// Property [`PutItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbv2action.html#cfn-iot-topicrule-dynamodbv2action-putitem).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub put_item: Option<::Value<PutItemInput>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbv2action.html#cfn-iot-topicrule-dynamodbv2action-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DynamoDBv2Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref put_item) = self.put_item {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PutItem", put_item)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynamoDBv2Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamoDBv2Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamoDBv2Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamoDBv2Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut put_item: Option<::Value<PutItemInput>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PutItem" => {
                                put_item = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamoDBv2Action {
                        put_item: put_item,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.ElasticsearchAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html) property type.
    #[derive(Debug, Default)]
    pub struct ElasticsearchAction {
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html#cfn-iot-topicrule-elasticsearchaction-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html#cfn-iot-topicrule-elasticsearchaction-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Index`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html#cfn-iot-topicrule-elasticsearchaction-index).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html#cfn-iot-topicrule-elasticsearchaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html#cfn-iot-topicrule-elasticsearchaction-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for ElasticsearchAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", &self.endpoint)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Index", &self.index)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticsearchAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticsearchAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticsearchAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticsearchAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut index: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Index" => {
                                index = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticsearchAction {
                        endpoint: endpoint.ok_or(::serde::de::Error::missing_field("Endpoint"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        index: index.ok_or(::serde::de::Error::missing_field("Index"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.FirehoseAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html) property type.
    #[derive(Debug, Default)]
    pub struct FirehoseAction {
        /// Property [`DeliveryStreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html#cfn-iot-topicrule-firehoseaction-deliverystreamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_stream_name: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html#cfn-iot-topicrule-firehoseaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`Separator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html#cfn-iot-topicrule-firehoseaction-separator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub separator: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FirehoseAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamName", &self.delivery_stream_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref separator) = self.separator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Separator", separator)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FirehoseAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FirehoseAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FirehoseAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FirehoseAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_stream_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut separator: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryStreamName" => {
                                delivery_stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Separator" => {
                                separator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FirehoseAction {
                        delivery_stream_name: delivery_stream_name.ok_or(::serde::de::Error::missing_field("DeliveryStreamName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        separator: separator,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.KinesisAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisAction {
        /// Property [`PartitionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html#cfn-iot-topicrule-kinesisaction-partitionkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition_key: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html#cfn-iot-topicrule-kinesisaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`StreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html#cfn-iot-topicrule-kinesisaction-streamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref partition_key) = self.partition_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionKey", partition_key)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamName", &self.stream_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut partition_key: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut stream_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PartitionKey" => {
                                partition_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamName" => {
                                stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisAction {
                        partition_key: partition_key,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        stream_name: stream_name.ok_or(::serde::de::Error::missing_field("StreamName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.LambdaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-lambdaaction.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaAction {
        /// Property [`FunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-lambdaaction.html#cfn-iot-topicrule-lambdaaction-functionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref function_arn) = self.function_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionArn", function_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut function_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionArn" => {
                                function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaAction {
                        function_arn: function_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.PutItemInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putiteminput.html) property type.
    #[derive(Debug, Default)]
    pub struct PutItemInput {
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putiteminput.html#cfn-iot-topicrule-putiteminput-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for PutItemInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PutItemInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PutItemInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PutItemInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PutItemInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PutItemInput {
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.RepublishAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html) property type.
    #[derive(Debug, Default)]
    pub struct RepublishAction {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html#cfn-iot-topicrule-republishaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`Topic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html#cfn-iot-topicrule-republishaction-topic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic: ::Value<String>,
    }

    impl ::codec::SerializeValue for RepublishAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Topic", &self.topic)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RepublishAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RepublishAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RepublishAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RepublishAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut topic: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Topic" => {
                                topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RepublishAction {
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        topic: topic.ok_or(::serde::de::Error::missing_field("Topic"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.S3Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Action {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html#cfn-iot-topicrule-s3action-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html#cfn-iot-topicrule-s3action-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html#cfn-iot-topicrule-s3action-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Action {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.SnsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html) property type.
    #[derive(Debug, Default)]
    pub struct SnsAction {
        /// Property [`MessageFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html#cfn-iot-topicrule-snsaction-messageformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_format: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html#cfn-iot-topicrule-snsaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html#cfn-iot-topicrule-snsaction-targetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnsAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref message_format) = self.message_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageFormat", message_format)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", &self.target_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnsAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnsAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnsAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnsAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message_format: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut target_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MessageFormat" => {
                                message_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetArn" => {
                                target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnsAction {
                        message_format: message_format,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        target_arn: target_arn.ok_or(::serde::de::Error::missing_field("TargetArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.SqsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html) property type.
    #[derive(Debug, Default)]
    pub struct SqsAction {
        /// Property [`QueueUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html#cfn-iot-topicrule-sqsaction-queueurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_url: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html#cfn-iot-topicrule-sqsaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`UseBase64`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html#cfn-iot-topicrule-sqsaction-usebase64).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_base64: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SqsAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueUrl", &self.queue_url)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref use_base64) = self.use_base64 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseBase64", use_base64)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SqsAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SqsAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SqsAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SqsAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut queue_url: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut use_base64: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "QueueUrl" => {
                                queue_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseBase64" => {
                                use_base64 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SqsAction {
                        queue_url: queue_url.ok_or(::serde::de::Error::missing_field("QueueUrl"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        use_base64: use_base64,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.TopicRulePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html) property type.
    #[derive(Debug, Default)]
    pub struct TopicRulePayload {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::ValueList<Action>,
        /// Property [`AwsIotSqlVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-awsiotsqlversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_iot_sql_version: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`RuleDisabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-ruledisabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_disabled: ::Value<bool>,
        /// Property [`Sql`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-sql).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sql: ::Value<String>,
    }

    impl ::codec::SerializeValue for TopicRulePayload {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            if let Some(ref aws_iot_sql_version) = self.aws_iot_sql_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsIotSqlVersion", aws_iot_sql_version)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleDisabled", &self.rule_disabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sql", &self.sql)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TopicRulePayload {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TopicRulePayload, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TopicRulePayload;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TopicRulePayload")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<Action>> = None;
                    let mut aws_iot_sql_version: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut rule_disabled: Option<::Value<bool>> = None;
                    let mut sql: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AwsIotSqlVersion" => {
                                aws_iot_sql_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleDisabled" => {
                                rule_disabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sql" => {
                                sql = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TopicRulePayload {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        aws_iot_sql_version: aws_iot_sql_version,
                        description: description,
                        rule_disabled: rule_disabled.ok_or(::serde::de::Error::missing_field("RuleDisabled"))?,
                        sql: sql.ok_or(::serde::de::Error::missing_field("Sql"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
