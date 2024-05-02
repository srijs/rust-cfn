//! Types for the `Connect` service.

/// The [`AWS::Connect::ApprovedOrigin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-approvedorigin.html) resource type.
#[derive(Debug, Default)]
pub struct ApprovedOrigin {
    properties: ApprovedOriginProperties
}

/// Properties for the `ApprovedOrigin` resource.
#[derive(Debug, Default)]
pub struct ApprovedOriginProperties {
    /// Property [`InstanceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-approvedorigin.html#cfn-connect-approvedorigin-instanceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_id: ::Value<String>,
    /// Property [`Origin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-approvedorigin.html#cfn-connect-approvedorigin-origin).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub origin: ::Value<String>,
}

impl ::serde::Serialize for ApprovedOriginProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Origin", &self.origin)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApprovedOriginProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApprovedOriginProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApprovedOriginProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApprovedOriginProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_id: Option<::Value<String>> = None;
                let mut origin: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceId" => {
                            instance_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Origin" => {
                            origin = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApprovedOriginProperties {
                    instance_id: instance_id.ok_or(::serde::de::Error::missing_field("InstanceId"))?,
                    origin: origin.ok_or(::serde::de::Error::missing_field("Origin"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApprovedOrigin {
    type Properties = ApprovedOriginProperties;
    const TYPE: &'static str = "AWS::Connect::ApprovedOrigin";
    fn properties(&self) -> &ApprovedOriginProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApprovedOriginProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApprovedOrigin {}

impl From<ApprovedOriginProperties> for ApprovedOrigin {
    fn from(properties: ApprovedOriginProperties) -> ApprovedOrigin {
        ApprovedOrigin { properties }
    }
}

/// The [`AWS::Connect::ContactFlow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html) resource type.
#[derive(Debug, Default)]
pub struct ContactFlow {
    properties: ContactFlowProperties
}

/// Properties for the `ContactFlow` resource.
#[derive(Debug, Default)]
pub struct ContactFlowProperties {
    /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-content).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for ContactFlowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContactFlowProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContactFlowProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContactFlowProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContactFlowProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Content" => {
                            content = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContactFlowProperties {
                    content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    state: state,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ContactFlow {
    type Properties = ContactFlowProperties;
    const TYPE: &'static str = "AWS::Connect::ContactFlow";
    fn properties(&self) -> &ContactFlowProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContactFlowProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ContactFlow {}

impl From<ContactFlowProperties> for ContactFlow {
    fn from(properties: ContactFlowProperties) -> ContactFlow {
        ContactFlow { properties }
    }
}

/// The [`AWS::Connect::ContactFlowModule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html) resource type.
#[derive(Debug, Default)]
pub struct ContactFlowModule {
    properties: ContactFlowModuleProperties
}

/// Properties for the `ContactFlowModule` resource.
#[derive(Debug, Default)]
pub struct ContactFlowModuleProperties {
    /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-content).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ContactFlowModuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContactFlowModuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContactFlowModuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContactFlowModuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContactFlowModuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Content" => {
                            content = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ContactFlowModuleProperties {
                    content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    state: state,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ContactFlowModule {
    type Properties = ContactFlowModuleProperties;
    const TYPE: &'static str = "AWS::Connect::ContactFlowModule";
    fn properties(&self) -> &ContactFlowModuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContactFlowModuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ContactFlowModule {}

impl From<ContactFlowModuleProperties> for ContactFlowModule {
    fn from(properties: ContactFlowModuleProperties) -> ContactFlowModule {
        ContactFlowModule { properties }
    }
}

/// The [`AWS::Connect::EvaluationForm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-evaluationform.html) resource type.
#[derive(Debug, Default)]
pub struct EvaluationForm {
    properties: EvaluationFormProperties
}

/// Properties for the `EvaluationForm` resource.
#[derive(Debug, Default)]
pub struct EvaluationFormProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-evaluationform.html#cfn-connect-evaluationform-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-evaluationform.html#cfn-connect-evaluationform-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-evaluationform.html#cfn-connect-evaluationform-items).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub items: ::ValueList<self::evaluation_form::EvaluationFormBaseItem>,
    /// Property [`ScoringStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-evaluationform.html#cfn-connect-evaluationform-scoringstrategy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scoring_strategy: Option<::Value<self::evaluation_form::ScoringStrategy>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-evaluationform.html#cfn-connect-evaluationform-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-evaluationform.html#cfn-connect-evaluationform-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-evaluationform.html#cfn-connect-evaluationform-title).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub title: ::Value<String>,
}

impl ::serde::Serialize for EvaluationFormProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", &self.items)?;
        if let Some(ref scoring_strategy) = self.scoring_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScoringStrategy", scoring_strategy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", &self.title)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EvaluationFormProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EvaluationFormProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EvaluationFormProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut items: Option<::ValueList<self::evaluation_form::EvaluationFormBaseItem>> = None;
                let mut scoring_strategy: Option<::Value<self::evaluation_form::ScoringStrategy>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut title: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Items" => {
                            items = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScoringStrategy" => {
                            scoring_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Title" => {
                            title = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EvaluationFormProperties {
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    items: items.ok_or(::serde::de::Error::missing_field("Items"))?,
                    scoring_strategy: scoring_strategy,
                    status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    tags: tags,
                    title: title.ok_or(::serde::de::Error::missing_field("Title"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EvaluationForm {
    type Properties = EvaluationFormProperties;
    const TYPE: &'static str = "AWS::Connect::EvaluationForm";
    fn properties(&self) -> &EvaluationFormProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EvaluationFormProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EvaluationForm {}

impl From<EvaluationFormProperties> for EvaluationForm {
    fn from(properties: EvaluationFormProperties) -> EvaluationForm {
        EvaluationForm { properties }
    }
}

/// The [`AWS::Connect::HoursOfOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html) resource type.
#[derive(Debug, Default)]
pub struct HoursOfOperation {
    properties: HoursOfOperationProperties
}

/// Properties for the `HoursOfOperation` resource.
#[derive(Debug, Default)]
pub struct HoursOfOperationProperties {
    /// Property [`Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-config).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub config: ::ValueList<self::hours_of_operation::HoursOfOperationConfig>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TimeZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-timezone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub time_zone: ::Value<String>,
}

impl ::serde::Serialize for HoursOfOperationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Config", &self.config)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeZone", &self.time_zone)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HoursOfOperationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HoursOfOperationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HoursOfOperationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HoursOfOperationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut config: Option<::ValueList<self::hours_of_operation::HoursOfOperationConfig>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut time_zone: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Config" => {
                            config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeZone" => {
                            time_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(HoursOfOperationProperties {
                    config: config.ok_or(::serde::de::Error::missing_field("Config"))?,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    time_zone: time_zone.ok_or(::serde::de::Error::missing_field("TimeZone"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for HoursOfOperation {
    type Properties = HoursOfOperationProperties;
    const TYPE: &'static str = "AWS::Connect::HoursOfOperation";
    fn properties(&self) -> &HoursOfOperationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HoursOfOperationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for HoursOfOperation {}

impl From<HoursOfOperationProperties> for HoursOfOperation {
    fn from(properties: HoursOfOperationProperties) -> HoursOfOperation {
        HoursOfOperation { properties }
    }
}

/// The [`AWS::Connect::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html) resource type.
#[derive(Debug, Default)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Debug, Default)]
pub struct InstanceProperties {
    /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html#cfn-connect-instance-attributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes: ::Value<self::instance::Attributes>,
    /// Property [`DirectoryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html#cfn-connect-instance-directoryid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub directory_id: Option<::Value<String>>,
    /// Property [`IdentityManagementType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html#cfn-connect-instance-identitymanagementtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub identity_management_type: ::Value<String>,
    /// Property [`InstanceAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html#cfn-connect-instance-instancealias).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_alias: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html#cfn-connect-instance-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for InstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
        if let Some(ref directory_id) = self.directory_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryId", directory_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityManagementType", &self.identity_management_type)?;
        if let Some(ref instance_alias) = self.instance_alias {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceAlias", instance_alias)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attributes: Option<::Value<self::instance::Attributes>> = None;
                let mut directory_id: Option<::Value<String>> = None;
                let mut identity_management_type: Option<::Value<String>> = None;
                let mut instance_alias: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Attributes" => {
                            attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DirectoryId" => {
                            directory_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityManagementType" => {
                            identity_management_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceAlias" => {
                            instance_alias = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InstanceProperties {
                    attributes: attributes.ok_or(::serde::de::Error::missing_field("Attributes"))?,
                    directory_id: directory_id,
                    identity_management_type: identity_management_type.ok_or(::serde::de::Error::missing_field("IdentityManagementType"))?,
                    instance_alias: instance_alias,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Instance {
    type Properties = InstanceProperties;
    const TYPE: &'static str = "AWS::Connect::Instance";
    fn properties(&self) -> &InstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Instance {}

impl From<InstanceProperties> for Instance {
    fn from(properties: InstanceProperties) -> Instance {
        Instance { properties }
    }
}

/// The [`AWS::Connect::InstanceStorageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html) resource type.
#[derive(Debug, Default)]
pub struct InstanceStorageConfig {
    properties: InstanceStorageConfigProperties
}

/// Properties for the `InstanceStorageConfig` resource.
#[derive(Debug, Default)]
pub struct InstanceStorageConfigProperties {
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-instancearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`KinesisFirehoseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-kinesisfirehoseconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kinesis_firehose_config: Option<::Value<self::instance_storage_config::KinesisFirehoseConfig>>,
    /// Property [`KinesisStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-kinesisstreamconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kinesis_stream_config: Option<::Value<self::instance_storage_config::KinesisStreamConfig>>,
    /// Property [`KinesisVideoStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-kinesisvideostreamconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kinesis_video_stream_config: Option<::Value<self::instance_storage_config::KinesisVideoStreamConfig>>,
    /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-resourcetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_type: ::Value<String>,
    /// Property [`S3Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-s3config).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_config: Option<::Value<self::instance_storage_config::S3Config>>,
    /// Property [`StorageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-storagetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub storage_type: ::Value<String>,
}

impl ::serde::Serialize for InstanceStorageConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        if let Some(ref kinesis_firehose_config) = self.kinesis_firehose_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisFirehoseConfig", kinesis_firehose_config)?;
        }
        if let Some(ref kinesis_stream_config) = self.kinesis_stream_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamConfig", kinesis_stream_config)?;
        }
        if let Some(ref kinesis_video_stream_config) = self.kinesis_video_stream_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisVideoStreamConfig", kinesis_video_stream_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", &self.resource_type)?;
        if let Some(ref s3_config) = self.s3_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Config", s3_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageType", &self.storage_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceStorageConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceStorageConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceStorageConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceStorageConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_arn: Option<::Value<String>> = None;
                let mut kinesis_firehose_config: Option<::Value<self::instance_storage_config::KinesisFirehoseConfig>> = None;
                let mut kinesis_stream_config: Option<::Value<self::instance_storage_config::KinesisStreamConfig>> = None;
                let mut kinesis_video_stream_config: Option<::Value<self::instance_storage_config::KinesisVideoStreamConfig>> = None;
                let mut resource_type: Option<::Value<String>> = None;
                let mut s3_config: Option<::Value<self::instance_storage_config::S3Config>> = None;
                let mut storage_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisFirehoseConfig" => {
                            kinesis_firehose_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisStreamConfig" => {
                            kinesis_stream_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisVideoStreamConfig" => {
                            kinesis_video_stream_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceType" => {
                            resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Config" => {
                            s3_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageType" => {
                            storage_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InstanceStorageConfigProperties {
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    kinesis_firehose_config: kinesis_firehose_config,
                    kinesis_stream_config: kinesis_stream_config,
                    kinesis_video_stream_config: kinesis_video_stream_config,
                    resource_type: resource_type.ok_or(::serde::de::Error::missing_field("ResourceType"))?,
                    s3_config: s3_config,
                    storage_type: storage_type.ok_or(::serde::de::Error::missing_field("StorageType"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InstanceStorageConfig {
    type Properties = InstanceStorageConfigProperties;
    const TYPE: &'static str = "AWS::Connect::InstanceStorageConfig";
    fn properties(&self) -> &InstanceStorageConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceStorageConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InstanceStorageConfig {}

impl From<InstanceStorageConfigProperties> for InstanceStorageConfig {
    fn from(properties: InstanceStorageConfigProperties) -> InstanceStorageConfig {
        InstanceStorageConfig { properties }
    }
}

/// The [`AWS::Connect::IntegrationAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-integrationassociation.html) resource type.
#[derive(Debug, Default)]
pub struct IntegrationAssociation {
    properties: IntegrationAssociationProperties
}

/// Properties for the `IntegrationAssociation` resource.
#[derive(Debug, Default)]
pub struct IntegrationAssociationProperties {
    /// Property [`InstanceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-integrationassociation.html#cfn-connect-integrationassociation-instanceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_id: ::Value<String>,
    /// Property [`IntegrationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-integrationassociation.html#cfn-connect-integrationassociation-integrationarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub integration_arn: ::Value<String>,
    /// Property [`IntegrationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-integrationassociation.html#cfn-connect-integrationassociation-integrationtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub integration_type: ::Value<String>,
}

impl ::serde::Serialize for IntegrationAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationArn", &self.integration_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationType", &self.integration_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IntegrationAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegrationAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IntegrationAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IntegrationAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_id: Option<::Value<String>> = None;
                let mut integration_arn: Option<::Value<String>> = None;
                let mut integration_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceId" => {
                            instance_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IntegrationArn" => {
                            integration_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IntegrationType" => {
                            integration_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IntegrationAssociationProperties {
                    instance_id: instance_id.ok_or(::serde::de::Error::missing_field("InstanceId"))?,
                    integration_arn: integration_arn.ok_or(::serde::de::Error::missing_field("IntegrationArn"))?,
                    integration_type: integration_type.ok_or(::serde::de::Error::missing_field("IntegrationType"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IntegrationAssociation {
    type Properties = IntegrationAssociationProperties;
    const TYPE: &'static str = "AWS::Connect::IntegrationAssociation";
    fn properties(&self) -> &IntegrationAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IntegrationAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IntegrationAssociation {}

impl From<IntegrationAssociationProperties> for IntegrationAssociation {
    fn from(properties: IntegrationAssociationProperties) -> IntegrationAssociation {
        IntegrationAssociation { properties }
    }
}

/// The [`AWS::Connect::PhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html) resource type.
#[derive(Debug, Default)]
pub struct PhoneNumber {
    properties: PhoneNumberProperties
}

/// Properties for the `PhoneNumber` resource.
#[derive(Debug, Default)]
pub struct PhoneNumberProperties {
    /// Property [`CountryCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-countrycode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub country_code: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-prefix).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub prefix: Option<::Value<String>>,
    /// Property [`SourcePhoneNumberArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-sourcephonenumberarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_phone_number_arn: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-targetarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_arn: ::Value<String>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: Option<::Value<String>>,
}

impl ::serde::Serialize for PhoneNumberProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref country_code) = self.country_code {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CountryCode", country_code)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref prefix) = self.prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
        }
        if let Some(ref source_phone_number_arn) = self.source_phone_number_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePhoneNumberArn", source_phone_number_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", &self.target_arn)?;
        if let Some(ref r#type) = self.r#type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PhoneNumberProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PhoneNumberProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PhoneNumberProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PhoneNumberProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut country_code: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut prefix: Option<::Value<String>> = None;
                let mut source_phone_number_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_arn: Option<::Value<String>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CountryCode" => {
                            country_code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Prefix" => {
                            prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourcePhoneNumberArn" => {
                            source_phone_number_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetArn" => {
                            target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PhoneNumberProperties {
                    country_code: country_code,
                    description: description,
                    prefix: prefix,
                    source_phone_number_arn: source_phone_number_arn,
                    tags: tags,
                    target_arn: target_arn.ok_or(::serde::de::Error::missing_field("TargetArn"))?,
                    r#type: r#type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PhoneNumber {
    type Properties = PhoneNumberProperties;
    const TYPE: &'static str = "AWS::Connect::PhoneNumber";
    fn properties(&self) -> &PhoneNumberProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PhoneNumberProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PhoneNumber {}

impl From<PhoneNumberProperties> for PhoneNumber {
    fn from(properties: PhoneNumberProperties) -> PhoneNumber {
        PhoneNumber { properties }
    }
}

/// The [`AWS::Connect::PredefinedAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-predefinedattribute.html) resource type.
#[derive(Debug, Default)]
pub struct PredefinedAttribute {
    properties: PredefinedAttributeProperties
}

/// Properties for the `PredefinedAttribute` resource.
#[derive(Debug, Default)]
pub struct PredefinedAttributeProperties {
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-predefinedattribute.html#cfn-connect-predefinedattribute-instancearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-predefinedattribute.html#cfn-connect-predefinedattribute-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-predefinedattribute.html#cfn-connect-predefinedattribute-values).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub values: ::Value<self::predefined_attribute::Values>,
}

impl ::serde::Serialize for PredefinedAttributeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PredefinedAttributeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PredefinedAttributeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PredefinedAttributeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PredefinedAttributeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut values: Option<::Value<self::predefined_attribute::Values>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Values" => {
                            values = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PredefinedAttributeProperties {
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PredefinedAttribute {
    type Properties = PredefinedAttributeProperties;
    const TYPE: &'static str = "AWS::Connect::PredefinedAttribute";
    fn properties(&self) -> &PredefinedAttributeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PredefinedAttributeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PredefinedAttribute {}

impl From<PredefinedAttributeProperties> for PredefinedAttribute {
    fn from(properties: PredefinedAttributeProperties) -> PredefinedAttribute {
        PredefinedAttribute { properties }
    }
}

/// The [`AWS::Connect::Prompt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-prompt.html) resource type.
#[derive(Debug, Default)]
pub struct Prompt {
    properties: PromptProperties
}

/// Properties for the `Prompt` resource.
#[derive(Debug, Default)]
pub struct PromptProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-prompt.html#cfn-connect-prompt-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-prompt.html#cfn-connect-prompt-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-prompt.html#cfn-connect-prompt-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-prompt.html#cfn-connect-prompt-s3uri).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_uri: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-prompt.html#cfn-connect-prompt-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PromptProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref s3_uri) = self.s3_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", s3_uri)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PromptProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PromptProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PromptProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PromptProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut s3_uri: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Uri" => {
                            s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PromptProperties {
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    s3_uri: s3_uri,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Prompt {
    type Properties = PromptProperties;
    const TYPE: &'static str = "AWS::Connect::Prompt";
    fn properties(&self) -> &PromptProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PromptProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Prompt {}

impl From<PromptProperties> for Prompt {
    fn from(properties: PromptProperties) -> Prompt {
        Prompt { properties }
    }
}

/// The [`AWS::Connect::Queue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html) resource type.
#[derive(Debug, Default)]
pub struct Queue {
    properties: QueueProperties
}

/// Properties for the `Queue` resource.
#[derive(Debug, Default)]
pub struct QueueProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html#cfn-connect-queue-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`HoursOfOperationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html#cfn-connect-queue-hoursofoperationarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hours_of_operation_arn: ::Value<String>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html#cfn-connect-queue-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`MaxContacts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html#cfn-connect-queue-maxcontacts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_contacts: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html#cfn-connect-queue-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OutboundCallerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html#cfn-connect-queue-outboundcallerconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub outbound_caller_config: Option<::Value<self::queue::OutboundCallerConfig>>,
    /// Property [`QuickConnectArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html#cfn-connect-queue-quickconnectarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub quick_connect_arns: Option<::ValueList<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html#cfn-connect-queue-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html#cfn-connect-queue-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for QueueProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HoursOfOperationArn", &self.hours_of_operation_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        if let Some(ref max_contacts) = self.max_contacts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxContacts", max_contacts)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref outbound_caller_config) = self.outbound_caller_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutboundCallerConfig", outbound_caller_config)?;
        }
        if let Some(ref quick_connect_arns) = self.quick_connect_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuickConnectArns", quick_connect_arns)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for QueueProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<QueueProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = QueueProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type QueueProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut hours_of_operation_arn: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut max_contacts: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut outbound_caller_config: Option<::Value<self::queue::OutboundCallerConfig>> = None;
                let mut quick_connect_arns: Option<::ValueList<String>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HoursOfOperationArn" => {
                            hours_of_operation_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxContacts" => {
                            max_contacts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutboundCallerConfig" => {
                            outbound_caller_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QuickConnectArns" => {
                            quick_connect_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(QueueProperties {
                    description: description,
                    hours_of_operation_arn: hours_of_operation_arn.ok_or(::serde::de::Error::missing_field("HoursOfOperationArn"))?,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    max_contacts: max_contacts,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    outbound_caller_config: outbound_caller_config,
                    quick_connect_arns: quick_connect_arns,
                    status: status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Queue {
    type Properties = QueueProperties;
    const TYPE: &'static str = "AWS::Connect::Queue";
    fn properties(&self) -> &QueueProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut QueueProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Queue {}

impl From<QueueProperties> for Queue {
    fn from(properties: QueueProperties) -> Queue {
        Queue { properties }
    }
}

/// The [`AWS::Connect::QuickConnect`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html) resource type.
#[derive(Debug, Default)]
pub struct QuickConnect {
    properties: QuickConnectProperties
}

/// Properties for the `QuickConnect` resource.
#[derive(Debug, Default)]
pub struct QuickConnectProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`QuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-quickconnectconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub quick_connect_config: ::Value<self::quick_connect::QuickConnectConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for QuickConnectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuickConnectConfig", &self.quick_connect_config)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for QuickConnectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<QuickConnectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = QuickConnectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type QuickConnectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut quick_connect_config: Option<::Value<self::quick_connect::QuickConnectConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QuickConnectConfig" => {
                            quick_connect_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(QuickConnectProperties {
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    quick_connect_config: quick_connect_config.ok_or(::serde::de::Error::missing_field("QuickConnectConfig"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for QuickConnect {
    type Properties = QuickConnectProperties;
    const TYPE: &'static str = "AWS::Connect::QuickConnect";
    fn properties(&self) -> &QuickConnectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut QuickConnectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for QuickConnect {}

impl From<QuickConnectProperties> for QuickConnect {
    fn from(properties: QuickConnectProperties) -> QuickConnect {
        QuickConnect { properties }
    }
}

/// The [`AWS::Connect::RoutingProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-routingprofile.html) resource type.
#[derive(Debug, Default)]
pub struct RoutingProfile {
    properties: RoutingProfileProperties
}

/// Properties for the `RoutingProfile` resource.
#[derive(Debug, Default)]
pub struct RoutingProfileProperties {
    /// Property [`AgentAvailabilityTimer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-routingprofile.html#cfn-connect-routingprofile-agentavailabilitytimer).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub agent_availability_timer: Option<::Value<String>>,
    /// Property [`DefaultOutboundQueueArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-routingprofile.html#cfn-connect-routingprofile-defaultoutboundqueuearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_outbound_queue_arn: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-routingprofile.html#cfn-connect-routingprofile-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-routingprofile.html#cfn-connect-routingprofile-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`MediaConcurrencies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-routingprofile.html#cfn-connect-routingprofile-mediaconcurrencies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub media_concurrencies: ::ValueList<self::routing_profile::MediaConcurrency>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-routingprofile.html#cfn-connect-routingprofile-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`QueueConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-routingprofile.html#cfn-connect-routingprofile-queueconfigs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub queue_configs: Option<::ValueList<self::routing_profile::RoutingProfileQueueConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-routingprofile.html#cfn-connect-routingprofile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for RoutingProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref agent_availability_timer) = self.agent_availability_timer {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentAvailabilityTimer", agent_availability_timer)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultOutboundQueueArn", &self.default_outbound_queue_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MediaConcurrencies", &self.media_concurrencies)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref queue_configs) = self.queue_configs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueConfigs", queue_configs)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RoutingProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RoutingProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RoutingProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RoutingProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut agent_availability_timer: Option<::Value<String>> = None;
                let mut default_outbound_queue_arn: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut media_concurrencies: Option<::ValueList<self::routing_profile::MediaConcurrency>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut queue_configs: Option<::ValueList<self::routing_profile::RoutingProfileQueueConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AgentAvailabilityTimer" => {
                            agent_availability_timer = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultOutboundQueueArn" => {
                            default_outbound_queue_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MediaConcurrencies" => {
                            media_concurrencies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueueConfigs" => {
                            queue_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RoutingProfileProperties {
                    agent_availability_timer: agent_availability_timer,
                    default_outbound_queue_arn: default_outbound_queue_arn.ok_or(::serde::de::Error::missing_field("DefaultOutboundQueueArn"))?,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    media_concurrencies: media_concurrencies.ok_or(::serde::de::Error::missing_field("MediaConcurrencies"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    queue_configs: queue_configs,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RoutingProfile {
    type Properties = RoutingProfileProperties;
    const TYPE: &'static str = "AWS::Connect::RoutingProfile";
    fn properties(&self) -> &RoutingProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RoutingProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RoutingProfile {}

impl From<RoutingProfileProperties> for RoutingProfile {
    fn from(properties: RoutingProfileProperties) -> RoutingProfile {
        RoutingProfile { properties }
    }
}

/// The [`AWS::Connect::Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-rule.html) resource type.
#[derive(Debug, Default)]
pub struct Rule {
    properties: RuleProperties
}

/// Properties for the `Rule` resource.
#[derive(Debug, Default)]
pub struct RuleProperties {
    /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-rule.html#cfn-connect-rule-actions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions: ::Value<self::rule::Actions>,
    /// Property [`Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-rule.html#cfn-connect-rule-function).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub function: ::Value<String>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-rule.html#cfn-connect-rule-instancearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-rule.html#cfn-connect-rule-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`PublishStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-rule.html#cfn-connect-rule-publishstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub publish_status: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-rule.html#cfn-connect-rule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TriggerEventSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-rule.html#cfn-connect-rule-triggereventsource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub trigger_event_source: ::Value<self::rule::RuleTriggerEventSource>,
}

impl ::serde::Serialize for RuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Function", &self.function)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublishStatus", &self.publish_status)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerEventSource", &self.trigger_event_source)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions: Option<::Value<self::rule::Actions>> = None;
                let mut function: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut publish_status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut trigger_event_source: Option<::Value<self::rule::RuleTriggerEventSource>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Actions" => {
                            actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Function" => {
                            function = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PublishStatus" => {
                            publish_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TriggerEventSource" => {
                            trigger_event_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RuleProperties {
                    actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                    function: function.ok_or(::serde::de::Error::missing_field("Function"))?,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    publish_status: publish_status.ok_or(::serde::de::Error::missing_field("PublishStatus"))?,
                    tags: tags,
                    trigger_event_source: trigger_event_source.ok_or(::serde::de::Error::missing_field("TriggerEventSource"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Rule {
    type Properties = RuleProperties;
    const TYPE: &'static str = "AWS::Connect::Rule";
    fn properties(&self) -> &RuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Rule {}

impl From<RuleProperties> for Rule {
    fn from(properties: RuleProperties) -> Rule {
        Rule { properties }
    }
}

/// The [`AWS::Connect::SecurityKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securitykey.html) resource type.
#[derive(Debug, Default)]
pub struct SecurityKey {
    properties: SecurityKeyProperties
}

/// Properties for the `SecurityKey` resource.
#[derive(Debug, Default)]
pub struct SecurityKeyProperties {
    /// Property [`InstanceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securitykey.html#cfn-connect-securitykey-instanceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_id: ::Value<String>,
    /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securitykey.html#cfn-connect-securitykey-key).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key: ::Value<String>,
}

impl ::serde::Serialize for SecurityKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecurityKeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityKeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityKeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecurityKeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_id: Option<::Value<String>> = None;
                let mut key: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceId" => {
                            instance_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Key" => {
                            key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SecurityKeyProperties {
                    instance_id: instance_id.ok_or(::serde::de::Error::missing_field("InstanceId"))?,
                    key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SecurityKey {
    type Properties = SecurityKeyProperties;
    const TYPE: &'static str = "AWS::Connect::SecurityKey";
    fn properties(&self) -> &SecurityKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityKeyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityKey {}

impl From<SecurityKeyProperties> for SecurityKey {
    fn from(properties: SecurityKeyProperties) -> SecurityKey {
        SecurityKey { properties }
    }
}

/// The [`AWS::Connect::SecurityProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securityprofile.html) resource type.
#[derive(Debug, Default)]
pub struct SecurityProfile {
    properties: SecurityProfileProperties
}

/// Properties for the `SecurityProfile` resource.
#[derive(Debug, Default)]
pub struct SecurityProfileProperties {
    /// Property [`AllowedAccessControlTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securityprofile.html#cfn-connect-securityprofile-allowedaccesscontroltags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allowed_access_control_tags: Option<::ValueList<::Tag>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securityprofile.html#cfn-connect-securityprofile-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securityprofile.html#cfn-connect-securityprofile-instancearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securityprofile.html#cfn-connect-securityprofile-permissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions: Option<::ValueList<String>>,
    /// Property [`SecurityProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securityprofile.html#cfn-connect-securityprofile-securityprofilename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_profile_name: ::Value<String>,
    /// Property [`TagRestrictedResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securityprofile.html#cfn-connect-securityprofile-tagrestrictedresources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tag_restricted_resources: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securityprofile.html#cfn-connect-securityprofile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SecurityProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allowed_access_control_tags) = self.allowed_access_control_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedAccessControlTags", allowed_access_control_tags)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        if let Some(ref permissions) = self.permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityProfileName", &self.security_profile_name)?;
        if let Some(ref tag_restricted_resources) = self.tag_restricted_resources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagRestrictedResources", tag_restricted_resources)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecurityProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecurityProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allowed_access_control_tags: Option<::ValueList<::Tag>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut permissions: Option<::ValueList<String>> = None;
                let mut security_profile_name: Option<::Value<String>> = None;
                let mut tag_restricted_resources: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowedAccessControlTags" => {
                            allowed_access_control_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Permissions" => {
                            permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityProfileName" => {
                            security_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TagRestrictedResources" => {
                            tag_restricted_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SecurityProfileProperties {
                    allowed_access_control_tags: allowed_access_control_tags,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    permissions: permissions,
                    security_profile_name: security_profile_name.ok_or(::serde::de::Error::missing_field("SecurityProfileName"))?,
                    tag_restricted_resources: tag_restricted_resources,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SecurityProfile {
    type Properties = SecurityProfileProperties;
    const TYPE: &'static str = "AWS::Connect::SecurityProfile";
    fn properties(&self) -> &SecurityProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityProfile {}

impl From<SecurityProfileProperties> for SecurityProfile {
    fn from(properties: SecurityProfileProperties) -> SecurityProfile {
        SecurityProfile { properties }
    }
}

/// The [`AWS::Connect::TaskTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html) resource type.
#[derive(Debug, Default)]
pub struct TaskTemplate {
    properties: TaskTemplateProperties
}

/// Properties for the `TaskTemplate` resource.
#[derive(Debug, Default)]
pub struct TaskTemplateProperties {
    /// Property [`ClientToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-clienttoken).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub client_token: Option<::Value<String>>,
    /// Property [`Constraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-constraints).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub constraints: Option<::Value<self::task_template::Constraints>>,
    /// Property [`ContactFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-contactflowarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub contact_flow_arn: Option<::Value<String>>,
    /// Property [`Defaults`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-defaults).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub defaults: Option<::ValueList<self::task_template::DefaultFieldValue>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Fields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-fields).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fields: Option<::ValueList<self::task_template::Field>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for TaskTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref client_token) = self.client_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientToken", client_token)?;
        }
        if let Some(ref constraints) = self.constraints {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Constraints", constraints)?;
        }
        if let Some(ref contact_flow_arn) = self.contact_flow_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactFlowArn", contact_flow_arn)?;
        }
        if let Some(ref defaults) = self.defaults {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Defaults", defaults)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref fields) = self.fields {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", fields)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TaskTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TaskTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TaskTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut client_token: Option<::Value<String>> = None;
                let mut constraints: Option<::Value<self::task_template::Constraints>> = None;
                let mut contact_flow_arn: Option<::Value<String>> = None;
                let mut defaults: Option<::ValueList<self::task_template::DefaultFieldValue>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut fields: Option<::ValueList<self::task_template::Field>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClientToken" => {
                            client_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Constraints" => {
                            constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContactFlowArn" => {
                            contact_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Defaults" => {
                            defaults = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Fields" => {
                            fields = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TaskTemplateProperties {
                    client_token: client_token,
                    constraints: constraints,
                    contact_flow_arn: contact_flow_arn,
                    defaults: defaults,
                    description: description,
                    fields: fields,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name,
                    status: status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TaskTemplate {
    type Properties = TaskTemplateProperties;
    const TYPE: &'static str = "AWS::Connect::TaskTemplate";
    fn properties(&self) -> &TaskTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TaskTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TaskTemplate {}

impl From<TaskTemplateProperties> for TaskTemplate {
    fn from(properties: TaskTemplateProperties) -> TaskTemplate {
        TaskTemplate { properties }
    }
}

/// The [`AWS::Connect::TrafficDistributionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-trafficdistributiongroup.html) resource type.
#[derive(Debug, Default)]
pub struct TrafficDistributionGroup {
    properties: TrafficDistributionGroupProperties
}

/// Properties for the `TrafficDistributionGroup` resource.
#[derive(Debug, Default)]
pub struct TrafficDistributionGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-trafficdistributiongroup.html#cfn-connect-trafficdistributiongroup-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-trafficdistributiongroup.html#cfn-connect-trafficdistributiongroup-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-trafficdistributiongroup.html#cfn-connect-trafficdistributiongroup-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-trafficdistributiongroup.html#cfn-connect-trafficdistributiongroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for TrafficDistributionGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TrafficDistributionGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TrafficDistributionGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TrafficDistributionGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TrafficDistributionGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(TrafficDistributionGroupProperties {
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TrafficDistributionGroup {
    type Properties = TrafficDistributionGroupProperties;
    const TYPE: &'static str = "AWS::Connect::TrafficDistributionGroup";
    fn properties(&self) -> &TrafficDistributionGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrafficDistributionGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TrafficDistributionGroup {}

impl From<TrafficDistributionGroupProperties> for TrafficDistributionGroup {
    fn from(properties: TrafficDistributionGroupProperties) -> TrafficDistributionGroup {
        TrafficDistributionGroup { properties }
    }
}

/// The [`AWS::Connect::User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html) resource type.
#[derive(Debug, Default)]
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Debug, Default)]
pub struct UserProperties {
    /// Property [`DirectoryUserId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-directoryuserid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub directory_user_id: Option<::Value<String>>,
    /// Property [`HierarchyGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-hierarchygrouparn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hierarchy_group_arn: Option<::Value<String>>,
    /// Property [`IdentityInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-identityinfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_info: Option<::Value<self::user::UserIdentityInfo>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-password).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub password: Option<::Value<String>>,
    /// Property [`PhoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-phoneconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub phone_config: ::Value<self::user::UserPhoneConfig>,
    /// Property [`RoutingProfileArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-routingprofilearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub routing_profile_arn: ::Value<String>,
    /// Property [`SecurityProfileArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-securityprofilearns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_profile_arns: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserProficiencies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-userproficiencies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_proficiencies: Option<::ValueList<self::user::UserProficiency>>,
    /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-username).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub username: ::Value<String>,
}

impl ::serde::Serialize for UserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref directory_user_id) = self.directory_user_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryUserId", directory_user_id)?;
        }
        if let Some(ref hierarchy_group_arn) = self.hierarchy_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HierarchyGroupArn", hierarchy_group_arn)?;
        }
        if let Some(ref identity_info) = self.identity_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityInfo", identity_info)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        if let Some(ref password) = self.password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneConfig", &self.phone_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoutingProfileArn", &self.routing_profile_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityProfileArns", &self.security_profile_arns)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref user_proficiencies) = self.user_proficiencies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserProficiencies", user_proficiencies)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut directory_user_id: Option<::Value<String>> = None;
                let mut hierarchy_group_arn: Option<::Value<String>> = None;
                let mut identity_info: Option<::Value<self::user::UserIdentityInfo>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut password: Option<::Value<String>> = None;
                let mut phone_config: Option<::Value<self::user::UserPhoneConfig>> = None;
                let mut routing_profile_arn: Option<::Value<String>> = None;
                let mut security_profile_arns: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_proficiencies: Option<::ValueList<self::user::UserProficiency>> = None;
                let mut username: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DirectoryUserId" => {
                            directory_user_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HierarchyGroupArn" => {
                            hierarchy_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityInfo" => {
                            identity_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Password" => {
                            password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PhoneConfig" => {
                            phone_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoutingProfileArn" => {
                            routing_profile_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityProfileArns" => {
                            security_profile_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserProficiencies" => {
                            user_proficiencies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Username" => {
                            username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserProperties {
                    directory_user_id: directory_user_id,
                    hierarchy_group_arn: hierarchy_group_arn,
                    identity_info: identity_info,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    password: password,
                    phone_config: phone_config.ok_or(::serde::de::Error::missing_field("PhoneConfig"))?,
                    routing_profile_arn: routing_profile_arn.ok_or(::serde::de::Error::missing_field("RoutingProfileArn"))?,
                    security_profile_arns: security_profile_arns.ok_or(::serde::de::Error::missing_field("SecurityProfileArns"))?,
                    tags: tags,
                    user_proficiencies: user_proficiencies,
                    username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for User {
    type Properties = UserProperties;
    const TYPE: &'static str = "AWS::Connect::User";
    fn properties(&self) -> &UserProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for User {}

impl From<UserProperties> for User {
    fn from(properties: UserProperties) -> User {
        User { properties }
    }
}

/// The [`AWS::Connect::UserHierarchyGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html) resource type.
#[derive(Debug, Default)]
pub struct UserHierarchyGroup {
    properties: UserHierarchyGroupProperties
}

/// Properties for the `UserHierarchyGroup` resource.
#[derive(Debug, Default)]
pub struct UserHierarchyGroupProperties {
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html#cfn-connect-userhierarchygroup-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html#cfn-connect-userhierarchygroup-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ParentGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html#cfn-connect-userhierarchygroup-parentgrouparn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parent_group_arn: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html#cfn-connect-userhierarchygroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for UserHierarchyGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref parent_group_arn) = self.parent_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentGroupArn", parent_group_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserHierarchyGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserHierarchyGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserHierarchyGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserHierarchyGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parent_group_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParentGroupArn" => {
                            parent_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserHierarchyGroupProperties {
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    parent_group_arn: parent_group_arn,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserHierarchyGroup {
    type Properties = UserHierarchyGroupProperties;
    const TYPE: &'static str = "AWS::Connect::UserHierarchyGroup";
    fn properties(&self) -> &UserHierarchyGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserHierarchyGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserHierarchyGroup {}

impl From<UserHierarchyGroupProperties> for UserHierarchyGroup {
    fn from(properties: UserHierarchyGroupProperties) -> UserHierarchyGroup {
        UserHierarchyGroup { properties }
    }
}

/// The [`AWS::Connect::View`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-view.html) resource type.
#[derive(Debug, Default)]
pub struct View {
    properties: ViewProperties
}

/// Properties for the `View` resource.
#[derive(Debug, Default)]
pub struct ViewProperties {
    /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-view.html#cfn-connect-view-actions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub actions: ::ValueList<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-view.html#cfn-connect-view-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-view.html#cfn-connect-view-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-view.html#cfn-connect-view-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-view.html#cfn-connect-view-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-view.html#cfn-connect-view-template).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template: ::Value<::json::Value>,
}

impl ::serde::Serialize for ViewProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Template", &self.template)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ViewProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ViewProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ViewProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ViewProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut actions: Option<::ValueList<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut template: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Actions" => {
                            actions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Template" => {
                            template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ViewProperties {
                    actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    template: template.ok_or(::serde::de::Error::missing_field("Template"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for View {
    type Properties = ViewProperties;
    const TYPE: &'static str = "AWS::Connect::View";
    fn properties(&self) -> &ViewProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ViewProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for View {}

impl From<ViewProperties> for View {
    fn from(properties: ViewProperties) -> View {
        View { properties }
    }
}

/// The [`AWS::Connect::ViewVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-viewversion.html) resource type.
#[derive(Debug, Default)]
pub struct ViewVersion {
    properties: ViewVersionProperties
}

/// Properties for the `ViewVersion` resource.
#[derive(Debug, Default)]
pub struct ViewVersionProperties {
    /// Property [`VersionDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-viewversion.html#cfn-connect-viewversion-versiondescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub version_description: Option<::Value<String>>,
    /// Property [`ViewArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-viewversion.html#cfn-connect-viewversion-viewarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub view_arn: ::Value<String>,
    /// Property [`ViewContentSha256`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-viewversion.html#cfn-connect-viewversion-viewcontentsha256).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub view_content_sha256: Option<::Value<String>>,
}

impl ::serde::Serialize for ViewVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref version_description) = self.version_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionDescription", version_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewArn", &self.view_arn)?;
        if let Some(ref view_content_sha256) = self.view_content_sha256 {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewContentSha256", view_content_sha256)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ViewVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ViewVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ViewVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ViewVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut version_description: Option<::Value<String>> = None;
                let mut view_arn: Option<::Value<String>> = None;
                let mut view_content_sha256: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "VersionDescription" => {
                            version_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ViewArn" => {
                            view_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ViewContentSha256" => {
                            view_content_sha256 = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ViewVersionProperties {
                    version_description: version_description,
                    view_arn: view_arn.ok_or(::serde::de::Error::missing_field("ViewArn"))?,
                    view_content_sha256: view_content_sha256,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ViewVersion {
    type Properties = ViewVersionProperties;
    const TYPE: &'static str = "AWS::Connect::ViewVersion";
    fn properties(&self) -> &ViewVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ViewVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ViewVersion {}

impl From<ViewVersionProperties> for ViewVersion {
    fn from(properties: ViewVersionProperties) -> ViewVersion {
        ViewVersion { properties }
    }
}

pub mod evaluation_form {
    //! Property types for the `EvaluationForm` resource.

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormBaseItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformbaseitem.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormBaseItem {
        /// Property [`Section`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformbaseitem.html#cfn-connect-evaluationform-evaluationformbaseitem-section).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub section: ::Value<EvaluationFormSection>,
    }

    impl ::codec::SerializeValue for EvaluationFormBaseItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Section", &self.section)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormBaseItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormBaseItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormBaseItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormBaseItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut section: Option<::Value<EvaluationFormSection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Section" => {
                                section = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormBaseItem {
                        section: section.ok_or(::serde::de::Error::missing_field("Section"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformitem.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormItem {
        /// Property [`Question`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformitem.html#cfn-connect-evaluationform-evaluationformitem-question).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub question: Option<::Value<EvaluationFormQuestion>>,
        /// Property [`Section`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformitem.html#cfn-connect-evaluationform-evaluationformitem-section).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub section: Option<::Value<EvaluationFormSection>>,
    }

    impl ::codec::SerializeValue for EvaluationFormItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref question) = self.question {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Question", question)?;
            }
            if let Some(ref section) = self.section {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Section", section)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut question: Option<::Value<EvaluationFormQuestion>> = None;
                    let mut section: Option<::Value<EvaluationFormSection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Question" => {
                                question = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Section" => {
                                section = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormItem {
                        question: question,
                        section: section,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormNumericQuestionAutomation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionautomation.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormNumericQuestionAutomation {
        /// Property [`PropertyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionautomation.html#cfn-connect-evaluationform-evaluationformnumericquestionautomation-propertyvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_value: ::Value<NumericQuestionPropertyValueAutomation>,
    }

    impl ::codec::SerializeValue for EvaluationFormNumericQuestionAutomation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyValue", &self.property_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormNumericQuestionAutomation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormNumericQuestionAutomation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormNumericQuestionAutomation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormNumericQuestionAutomation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut property_value: Option<::Value<NumericQuestionPropertyValueAutomation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PropertyValue" => {
                                property_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormNumericQuestionAutomation {
                        property_value: property_value.ok_or(::serde::de::Error::missing_field("PropertyValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormNumericQuestionOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionoption.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormNumericQuestionOption {
        /// Property [`AutomaticFail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionoption.html#cfn-connect-evaluationform-evaluationformnumericquestionoption-automaticfail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automatic_fail: Option<::Value<bool>>,
        /// Property [`MaxValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionoption.html#cfn-connect-evaluationform-evaluationformnumericquestionoption-maxvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_value: ::Value<u32>,
        /// Property [`MinValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionoption.html#cfn-connect-evaluationform-evaluationformnumericquestionoption-minvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_value: ::Value<u32>,
        /// Property [`Score`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionoption.html#cfn-connect-evaluationform-evaluationformnumericquestionoption-score).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub score: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EvaluationFormNumericQuestionOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref automatic_fail) = self.automatic_fail {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticFail", automatic_fail)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxValue", &self.max_value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinValue", &self.min_value)?;
            if let Some(ref score) = self.score {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Score", score)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormNumericQuestionOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormNumericQuestionOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormNumericQuestionOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormNumericQuestionOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut automatic_fail: Option<::Value<bool>> = None;
                    let mut max_value: Option<::Value<u32>> = None;
                    let mut min_value: Option<::Value<u32>> = None;
                    let mut score: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutomaticFail" => {
                                automatic_fail = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxValue" => {
                                max_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinValue" => {
                                min_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Score" => {
                                score = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormNumericQuestionOption {
                        automatic_fail: automatic_fail,
                        max_value: max_value.ok_or(::serde::de::Error::missing_field("MaxValue"))?,
                        min_value: min_value.ok_or(::serde::de::Error::missing_field("MinValue"))?,
                        score: score,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormNumericQuestionProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormNumericQuestionProperties {
        /// Property [`Automation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionproperties.html#cfn-connect-evaluationform-evaluationformnumericquestionproperties-automation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automation: Option<::Value<EvaluationFormNumericQuestionAutomation>>,
        /// Property [`MaxValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionproperties.html#cfn-connect-evaluationform-evaluationformnumericquestionproperties-maxvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_value: ::Value<u32>,
        /// Property [`MinValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionproperties.html#cfn-connect-evaluationform-evaluationformnumericquestionproperties-minvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_value: ::Value<u32>,
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionproperties.html#cfn-connect-evaluationform-evaluationformnumericquestionproperties-options).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub options: Option<::ValueList<EvaluationFormNumericQuestionOption>>,
    }

    impl ::codec::SerializeValue for EvaluationFormNumericQuestionProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref automation) = self.automation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Automation", automation)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxValue", &self.max_value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinValue", &self.min_value)?;
            if let Some(ref options) = self.options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", options)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormNumericQuestionProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormNumericQuestionProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormNumericQuestionProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormNumericQuestionProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut automation: Option<::Value<EvaluationFormNumericQuestionAutomation>> = None;
                    let mut max_value: Option<::Value<u32>> = None;
                    let mut min_value: Option<::Value<u32>> = None;
                    let mut options: Option<::ValueList<EvaluationFormNumericQuestionOption>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Automation" => {
                                automation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxValue" => {
                                max_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinValue" => {
                                min_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormNumericQuestionProperties {
                        automation: automation,
                        max_value: max_value.ok_or(::serde::de::Error::missing_field("MaxValue"))?,
                        min_value: min_value.ok_or(::serde::de::Error::missing_field("MinValue"))?,
                        options: options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormQuestion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestion.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormQuestion {
        /// Property [`Instructions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestion.html#cfn-connect-evaluationform-evaluationformquestion-instructions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instructions: Option<::Value<String>>,
        /// Property [`NotApplicableEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestion.html#cfn-connect-evaluationform-evaluationformquestion-notapplicableenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub not_applicable_enabled: Option<::Value<bool>>,
        /// Property [`QuestionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestion.html#cfn-connect-evaluationform-evaluationformquestion-questiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub question_type: ::Value<String>,
        /// Property [`QuestionTypeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestion.html#cfn-connect-evaluationform-evaluationformquestion-questiontypeproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub question_type_properties: Option<::Value<EvaluationFormQuestionTypeProperties>>,
        /// Property [`RefId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestion.html#cfn-connect-evaluationform-evaluationformquestion-refid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ref_id: ::Value<String>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestion.html#cfn-connect-evaluationform-evaluationformquestion-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: ::Value<String>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestion.html#cfn-connect-evaluationform-evaluationformquestion-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for EvaluationFormQuestion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref instructions) = self.instructions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Instructions", instructions)?;
            }
            if let Some(ref not_applicable_enabled) = self.not_applicable_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotApplicableEnabled", not_applicable_enabled)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuestionType", &self.question_type)?;
            if let Some(ref question_type_properties) = self.question_type_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuestionTypeProperties", question_type_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefId", &self.ref_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", &self.title)?;
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormQuestion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormQuestion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormQuestion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormQuestion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instructions: Option<::Value<String>> = None;
                    let mut not_applicable_enabled: Option<::Value<bool>> = None;
                    let mut question_type: Option<::Value<String>> = None;
                    let mut question_type_properties: Option<::Value<EvaluationFormQuestionTypeProperties>> = None;
                    let mut ref_id: Option<::Value<String>> = None;
                    let mut title: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Instructions" => {
                                instructions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotApplicableEnabled" => {
                                not_applicable_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QuestionType" => {
                                question_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QuestionTypeProperties" => {
                                question_type_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RefId" => {
                                ref_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormQuestion {
                        instructions: instructions,
                        not_applicable_enabled: not_applicable_enabled,
                        question_type: question_type.ok_or(::serde::de::Error::missing_field("QuestionType"))?,
                        question_type_properties: question_type_properties,
                        ref_id: ref_id.ok_or(::serde::de::Error::missing_field("RefId"))?,
                        title: title.ok_or(::serde::de::Error::missing_field("Title"))?,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormQuestionTypeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestiontypeproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormQuestionTypeProperties {
        /// Property [`Numeric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestiontypeproperties.html#cfn-connect-evaluationform-evaluationformquestiontypeproperties-numeric).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub numeric: Option<::Value<EvaluationFormNumericQuestionProperties>>,
        /// Property [`SingleSelect`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestiontypeproperties.html#cfn-connect-evaluationform-evaluationformquestiontypeproperties-singleselect).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub single_select: Option<::Value<EvaluationFormSingleSelectQuestionProperties>>,
    }

    impl ::codec::SerializeValue for EvaluationFormQuestionTypeProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref numeric) = self.numeric {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Numeric", numeric)?;
            }
            if let Some(ref single_select) = self.single_select {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleSelect", single_select)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormQuestionTypeProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormQuestionTypeProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormQuestionTypeProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormQuestionTypeProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut numeric: Option<::Value<EvaluationFormNumericQuestionProperties>> = None;
                    let mut single_select: Option<::Value<EvaluationFormSingleSelectQuestionProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Numeric" => {
                                numeric = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SingleSelect" => {
                                single_select = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormQuestionTypeProperties {
                        numeric: numeric,
                        single_select: single_select,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormSection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsection.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormSection {
        /// Property [`Instructions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsection.html#cfn-connect-evaluationform-evaluationformsection-instructions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instructions: Option<::Value<String>>,
        /// Property [`Items`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsection.html#cfn-connect-evaluationform-evaluationformsection-items).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub items: Option<::ValueList<EvaluationFormItem>>,
        /// Property [`RefId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsection.html#cfn-connect-evaluationform-evaluationformsection-refid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ref_id: ::Value<String>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsection.html#cfn-connect-evaluationform-evaluationformsection-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: ::Value<String>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsection.html#cfn-connect-evaluationform-evaluationformsection-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for EvaluationFormSection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref instructions) = self.instructions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Instructions", instructions)?;
            }
            if let Some(ref items) = self.items {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Items", items)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefId", &self.ref_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", &self.title)?;
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormSection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormSection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormSection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormSection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instructions: Option<::Value<String>> = None;
                    let mut items: Option<::ValueList<EvaluationFormItem>> = None;
                    let mut ref_id: Option<::Value<String>> = None;
                    let mut title: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Instructions" => {
                                instructions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Items" => {
                                items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RefId" => {
                                ref_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormSection {
                        instructions: instructions,
                        items: items,
                        ref_id: ref_id.ok_or(::serde::de::Error::missing_field("RefId"))?,
                        title: title.ok_or(::serde::de::Error::missing_field("Title"))?,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormSingleSelectQuestionAutomation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionautomation.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormSingleSelectQuestionAutomation {
        /// Property [`DefaultOptionRefId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionautomation.html#cfn-connect-evaluationform-evaluationformsingleselectquestionautomation-defaultoptionrefid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_option_ref_id: Option<::Value<String>>,
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionautomation.html#cfn-connect-evaluationform-evaluationformsingleselectquestionautomation-options).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub options: ::ValueList<EvaluationFormSingleSelectQuestionAutomationOption>,
    }

    impl ::codec::SerializeValue for EvaluationFormSingleSelectQuestionAutomation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_option_ref_id) = self.default_option_ref_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultOptionRefId", default_option_ref_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", &self.options)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormSingleSelectQuestionAutomation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormSingleSelectQuestionAutomation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormSingleSelectQuestionAutomation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormSingleSelectQuestionAutomation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_option_ref_id: Option<::Value<String>> = None;
                    let mut options: Option<::ValueList<EvaluationFormSingleSelectQuestionAutomationOption>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultOptionRefId" => {
                                default_option_ref_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormSingleSelectQuestionAutomation {
                        default_option_ref_id: default_option_ref_id,
                        options: options.ok_or(::serde::de::Error::missing_field("Options"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormSingleSelectQuestionAutomationOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionautomationoption.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormSingleSelectQuestionAutomationOption {
        /// Property [`RuleCategory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionautomationoption.html#cfn-connect-evaluationform-evaluationformsingleselectquestionautomationoption-rulecategory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_category: ::Value<SingleSelectQuestionRuleCategoryAutomation>,
    }

    impl ::codec::SerializeValue for EvaluationFormSingleSelectQuestionAutomationOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleCategory", &self.rule_category)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormSingleSelectQuestionAutomationOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormSingleSelectQuestionAutomationOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormSingleSelectQuestionAutomationOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormSingleSelectQuestionAutomationOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rule_category: Option<::Value<SingleSelectQuestionRuleCategoryAutomation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RuleCategory" => {
                                rule_category = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormSingleSelectQuestionAutomationOption {
                        rule_category: rule_category.ok_or(::serde::de::Error::missing_field("RuleCategory"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormSingleSelectQuestionOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionoption.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormSingleSelectQuestionOption {
        /// Property [`AutomaticFail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionoption.html#cfn-connect-evaluationform-evaluationformsingleselectquestionoption-automaticfail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automatic_fail: Option<::Value<bool>>,
        /// Property [`RefId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionoption.html#cfn-connect-evaluationform-evaluationformsingleselectquestionoption-refid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ref_id: ::Value<String>,
        /// Property [`Score`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionoption.html#cfn-connect-evaluationform-evaluationformsingleselectquestionoption-score).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub score: Option<::Value<u32>>,
        /// Property [`Text`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionoption.html#cfn-connect-evaluationform-evaluationformsingleselectquestionoption-text).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text: ::Value<String>,
    }

    impl ::codec::SerializeValue for EvaluationFormSingleSelectQuestionOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref automatic_fail) = self.automatic_fail {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticFail", automatic_fail)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefId", &self.ref_id)?;
            if let Some(ref score) = self.score {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Score", score)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Text", &self.text)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormSingleSelectQuestionOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormSingleSelectQuestionOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormSingleSelectQuestionOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormSingleSelectQuestionOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut automatic_fail: Option<::Value<bool>> = None;
                    let mut ref_id: Option<::Value<String>> = None;
                    let mut score: Option<::Value<u32>> = None;
                    let mut text: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutomaticFail" => {
                                automatic_fail = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RefId" => {
                                ref_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Score" => {
                                score = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Text" => {
                                text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormSingleSelectQuestionOption {
                        automatic_fail: automatic_fail,
                        ref_id: ref_id.ok_or(::serde::de::Error::missing_field("RefId"))?,
                        score: score,
                        text: text.ok_or(::serde::de::Error::missing_field("Text"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.EvaluationFormSingleSelectQuestionProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct EvaluationFormSingleSelectQuestionProperties {
        /// Property [`Automation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionproperties.html#cfn-connect-evaluationform-evaluationformsingleselectquestionproperties-automation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automation: Option<::Value<EvaluationFormSingleSelectQuestionAutomation>>,
        /// Property [`DisplayAs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionproperties.html#cfn-connect-evaluationform-evaluationformsingleselectquestionproperties-displayas).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub display_as: Option<::Value<String>>,
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionproperties.html#cfn-connect-evaluationform-evaluationformsingleselectquestionproperties-options).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub options: ::ValueList<EvaluationFormSingleSelectQuestionOption>,
    }

    impl ::codec::SerializeValue for EvaluationFormSingleSelectQuestionProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref automation) = self.automation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Automation", automation)?;
            }
            if let Some(ref display_as) = self.display_as {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayAs", display_as)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", &self.options)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EvaluationFormSingleSelectQuestionProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EvaluationFormSingleSelectQuestionProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EvaluationFormSingleSelectQuestionProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EvaluationFormSingleSelectQuestionProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut automation: Option<::Value<EvaluationFormSingleSelectQuestionAutomation>> = None;
                    let mut display_as: Option<::Value<String>> = None;
                    let mut options: Option<::ValueList<EvaluationFormSingleSelectQuestionOption>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Automation" => {
                                automation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisplayAs" => {
                                display_as = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EvaluationFormSingleSelectQuestionProperties {
                        automation: automation,
                        display_as: display_as,
                        options: options.ok_or(::serde::de::Error::missing_field("Options"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.NumericQuestionPropertyValueAutomation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-numericquestionpropertyvalueautomation.html) property type.
    #[derive(Debug, Default)]
    pub struct NumericQuestionPropertyValueAutomation {
        /// Property [`Label`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-numericquestionpropertyvalueautomation.html#cfn-connect-evaluationform-numericquestionpropertyvalueautomation-label).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub label: ::Value<String>,
    }

    impl ::codec::SerializeValue for NumericQuestionPropertyValueAutomation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Label", &self.label)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NumericQuestionPropertyValueAutomation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NumericQuestionPropertyValueAutomation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NumericQuestionPropertyValueAutomation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NumericQuestionPropertyValueAutomation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut label: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Label" => {
                                label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NumericQuestionPropertyValueAutomation {
                        label: label.ok_or(::serde::de::Error::missing_field("Label"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.ScoringStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-scoringstrategy.html) property type.
    #[derive(Debug, Default)]
    pub struct ScoringStrategy {
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-scoringstrategy.html#cfn-connect-evaluationform-scoringstrategy-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: ::Value<String>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-scoringstrategy.html#cfn-connect-evaluationform-scoringstrategy-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for ScoringStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", &self.mode)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScoringStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScoringStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScoringStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScoringStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mode: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScoringStrategy {
                        mode: mode.ok_or(::serde::de::Error::missing_field("Mode"))?,
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::EvaluationForm.SingleSelectQuestionRuleCategoryAutomation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-singleselectquestionrulecategoryautomation.html) property type.
    #[derive(Debug, Default)]
    pub struct SingleSelectQuestionRuleCategoryAutomation {
        /// Property [`Category`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-singleselectquestionrulecategoryautomation.html#cfn-connect-evaluationform-singleselectquestionrulecategoryautomation-category).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub category: ::Value<String>,
        /// Property [`Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-singleselectquestionrulecategoryautomation.html#cfn-connect-evaluationform-singleselectquestionrulecategoryautomation-condition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition: ::Value<String>,
        /// Property [`OptionRefId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-singleselectquestionrulecategoryautomation.html#cfn-connect-evaluationform-singleselectquestionrulecategoryautomation-optionrefid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub option_ref_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for SingleSelectQuestionRuleCategoryAutomation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Category", &self.category)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Condition", &self.condition)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionRefId", &self.option_ref_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SingleSelectQuestionRuleCategoryAutomation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SingleSelectQuestionRuleCategoryAutomation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SingleSelectQuestionRuleCategoryAutomation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SingleSelectQuestionRuleCategoryAutomation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut category: Option<::Value<String>> = None;
                    let mut condition: Option<::Value<String>> = None;
                    let mut option_ref_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Category" => {
                                category = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Condition" => {
                                condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptionRefId" => {
                                option_ref_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SingleSelectQuestionRuleCategoryAutomation {
                        category: category.ok_or(::serde::de::Error::missing_field("Category"))?,
                        condition: condition.ok_or(::serde::de::Error::missing_field("Condition"))?,
                        option_ref_id: option_ref_id.ok_or(::serde::de::Error::missing_field("OptionRefId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod hours_of_operation {
    //! Property types for the `HoursOfOperation` resource.

    /// The [`AWS::Connect::HoursOfOperation.HoursOfOperationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HoursOfOperationConfig {
        /// Property [`Day`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html#cfn-connect-hoursofoperation-hoursofoperationconfig-day).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub day: ::Value<String>,
        /// Property [`EndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html#cfn-connect-hoursofoperation-hoursofoperationconfig-endtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_time: ::Value<HoursOfOperationTimeSlice>,
        /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html#cfn-connect-hoursofoperation-hoursofoperationconfig-starttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time: ::Value<HoursOfOperationTimeSlice>,
    }

    impl ::codec::SerializeValue for HoursOfOperationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Day", &self.day)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTime", &self.end_time)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", &self.start_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HoursOfOperationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HoursOfOperationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HoursOfOperationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HoursOfOperationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut day: Option<::Value<String>> = None;
                    let mut end_time: Option<::Value<HoursOfOperationTimeSlice>> = None;
                    let mut start_time: Option<::Value<HoursOfOperationTimeSlice>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Day" => {
                                day = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndTime" => {
                                end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTime" => {
                                start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HoursOfOperationConfig {
                        day: day.ok_or(::serde::de::Error::missing_field("Day"))?,
                        end_time: end_time.ok_or(::serde::de::Error::missing_field("EndTime"))?,
                        start_time: start_time.ok_or(::serde::de::Error::missing_field("StartTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::HoursOfOperation.HoursOfOperationTimeSlice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationtimeslice.html) property type.
    #[derive(Debug, Default)]
    pub struct HoursOfOperationTimeSlice {
        /// Property [`Hours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationtimeslice.html#cfn-connect-hoursofoperation-hoursofoperationtimeslice-hours).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hours: ::Value<u32>,
        /// Property [`Minutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationtimeslice.html#cfn-connect-hoursofoperation-hoursofoperationtimeslice-minutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for HoursOfOperationTimeSlice {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hours", &self.hours)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Minutes", &self.minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HoursOfOperationTimeSlice {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HoursOfOperationTimeSlice, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HoursOfOperationTimeSlice;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HoursOfOperationTimeSlice")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hours: Option<::Value<u32>> = None;
                    let mut minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hours" => {
                                hours = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Minutes" => {
                                minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HoursOfOperationTimeSlice {
                        hours: hours.ok_or(::serde::de::Error::missing_field("Hours"))?,
                        minutes: minutes.ok_or(::serde::de::Error::missing_field("Minutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod instance {
    //! Property types for the `Instance` resource.

    /// The [`AWS::Connect::Instance.Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html) property type.
    #[derive(Debug, Default)]
    pub struct Attributes {
        /// Property [`AutoResolveBestVoices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-autoresolvebestvoices).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_resolve_best_voices: Option<::Value<bool>>,
        /// Property [`ContactLens`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-contactlens).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_lens: Option<::Value<bool>>,
        /// Property [`ContactflowLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-contactflowlogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contactflow_logs: Option<::Value<bool>>,
        /// Property [`EarlyMedia`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-earlymedia).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub early_media: Option<::Value<bool>>,
        /// Property [`InboundCalls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-inboundcalls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inbound_calls: ::Value<bool>,
        /// Property [`OutboundCalls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-outboundcalls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outbound_calls: ::Value<bool>,
        /// Property [`UseCustomTTSVoices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-usecustomttsvoices).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_custom_tts_voices: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for Attributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_resolve_best_voices) = self.auto_resolve_best_voices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoResolveBestVoices", auto_resolve_best_voices)?;
            }
            if let Some(ref contact_lens) = self.contact_lens {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactLens", contact_lens)?;
            }
            if let Some(ref contactflow_logs) = self.contactflow_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactflowLogs", contactflow_logs)?;
            }
            if let Some(ref early_media) = self.early_media {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EarlyMedia", early_media)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InboundCalls", &self.inbound_calls)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutboundCalls", &self.outbound_calls)?;
            if let Some(ref use_custom_tts_voices) = self.use_custom_tts_voices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseCustomTTSVoices", use_custom_tts_voices)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Attributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Attributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Attributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Attributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_resolve_best_voices: Option<::Value<bool>> = None;
                    let mut contact_lens: Option<::Value<bool>> = None;
                    let mut contactflow_logs: Option<::Value<bool>> = None;
                    let mut early_media: Option<::Value<bool>> = None;
                    let mut inbound_calls: Option<::Value<bool>> = None;
                    let mut outbound_calls: Option<::Value<bool>> = None;
                    let mut use_custom_tts_voices: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoResolveBestVoices" => {
                                auto_resolve_best_voices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContactLens" => {
                                contact_lens = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContactflowLogs" => {
                                contactflow_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EarlyMedia" => {
                                early_media = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InboundCalls" => {
                                inbound_calls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutboundCalls" => {
                                outbound_calls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseCustomTTSVoices" => {
                                use_custom_tts_voices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Attributes {
                        auto_resolve_best_voices: auto_resolve_best_voices,
                        contact_lens: contact_lens,
                        contactflow_logs: contactflow_logs,
                        early_media: early_media,
                        inbound_calls: inbound_calls.ok_or(::serde::de::Error::missing_field("InboundCalls"))?,
                        outbound_calls: outbound_calls.ok_or(::serde::de::Error::missing_field("OutboundCalls"))?,
                        use_custom_tts_voices: use_custom_tts_voices,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod instance_storage_config {
    //! Property types for the `InstanceStorageConfig` resource.

    /// The [`AWS::Connect::InstanceStorageConfig.EncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-encryptionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionConfig {
        /// Property [`EncryptionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-encryptionconfig.html#cfn-connect-instancestorageconfig-encryptionconfig-encryptiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_type: ::Value<String>,
        /// Property [`KeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-encryptionconfig.html#cfn-connect-instancestorageconfig-encryptionconfig-keyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for EncryptionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionType", &self.encryption_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyId", &self.key_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_type: Option<::Value<String>> = None;
                    let mut key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionType" => {
                                encryption_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyId" => {
                                key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfig {
                        encryption_type: encryption_type.ok_or(::serde::de::Error::missing_field("EncryptionType"))?,
                        key_id: key_id.ok_or(::serde::de::Error::missing_field("KeyId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::InstanceStorageConfig.KinesisFirehoseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisfirehoseconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisFirehoseConfig {
        /// Property [`FirehoseArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisfirehoseconfig.html#cfn-connect-instancestorageconfig-kinesisfirehoseconfig-firehosearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub firehose_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirehoseArn", &self.firehose_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisFirehoseConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisFirehoseConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisFirehoseConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisFirehoseConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut firehose_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FirehoseArn" => {
                                firehose_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseConfig {
                        firehose_arn: firehose_arn.ok_or(::serde::de::Error::missing_field("FirehoseArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::InstanceStorageConfig.KinesisStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisstreamconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisStreamConfig {
        /// Property [`StreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisstreamconfig.html#cfn-connect-instancestorageconfig-kinesisstreamconfig-streamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisStreamConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamArn", &self.stream_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisStreamConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisStreamConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisStreamConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisStreamConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut stream_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StreamArn" => {
                                stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisStreamConfig {
                        stream_arn: stream_arn.ok_or(::serde::de::Error::missing_field("StreamArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::InstanceStorageConfig.KinesisVideoStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisvideostreamconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisVideoStreamConfig {
        /// Property [`EncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisvideostreamconfig.html#cfn-connect-instancestorageconfig-kinesisvideostreamconfig-encryptionconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_config: ::Value<EncryptionConfig>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisvideostreamconfig.html#cfn-connect-instancestorageconfig-kinesisvideostreamconfig-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: ::Value<String>,
        /// Property [`RetentionPeriodHours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisvideostreamconfig.html#cfn-connect-instancestorageconfig-kinesisvideostreamconfig-retentionperiodhours).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retention_period_hours: ::Value<f64>,
    }

    impl ::codec::SerializeValue for KinesisVideoStreamConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfig", &self.encryption_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", &self.prefix)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetentionPeriodHours", &self.retention_period_hours)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisVideoStreamConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisVideoStreamConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisVideoStreamConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisVideoStreamConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_config: Option<::Value<EncryptionConfig>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut retention_period_hours: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionConfig" => {
                                encryption_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetentionPeriodHours" => {
                                retention_period_hours = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisVideoStreamConfig {
                        encryption_config: encryption_config.ok_or(::serde::de::Error::missing_field("EncryptionConfig"))?,
                        prefix: prefix.ok_or(::serde::de::Error::missing_field("Prefix"))?,
                        retention_period_hours: retention_period_hours.ok_or(::serde::de::Error::missing_field("RetentionPeriodHours"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::InstanceStorageConfig.S3Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-s3config.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Config {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-s3config.html#cfn-connect-instancestorageconfig-s3config-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-s3config.html#cfn-connect-instancestorageconfig-s3config-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: ::Value<String>,
        /// Property [`EncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-s3config.html#cfn-connect-instancestorageconfig-s3config-encryptionconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_config: Option<::Value<EncryptionConfig>>,
    }

    impl ::codec::SerializeValue for S3Config {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", &self.bucket_prefix)?;
            if let Some(ref encryption_config) = self.encryption_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfig", encryption_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Config {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Config, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Config;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Config")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut encryption_config: Option<::Value<EncryptionConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionConfig" => {
                                encryption_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Config {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        bucket_prefix: bucket_prefix.ok_or(::serde::de::Error::missing_field("BucketPrefix"))?,
                        encryption_config: encryption_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod predefined_attribute {
    //! Property types for the `PredefinedAttribute` resource.

    /// The [`AWS::Connect::PredefinedAttribute.Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-predefinedattribute-values.html) property type.
    #[derive(Debug, Default)]
    pub struct Values {
        /// Property [`StringList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-predefinedattribute-values.html#cfn-connect-predefinedattribute-values-stringlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_list: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Values {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref string_list) = self.string_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringList", string_list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Values {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Values, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Values;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Values")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut string_list: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StringList" => {
                                string_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Values {
                        string_list: string_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod queue {
    //! Property types for the `Queue` resource.

    /// The [`AWS::Connect::Queue.OutboundCallerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-queue-outboundcallerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct OutboundCallerConfig {
        /// Property [`OutboundCallerIdName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-queue-outboundcallerconfig.html#cfn-connect-queue-outboundcallerconfig-outboundcalleridname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outbound_caller_id_name: Option<::Value<String>>,
        /// Property [`OutboundCallerIdNumberArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-queue-outboundcallerconfig.html#cfn-connect-queue-outboundcallerconfig-outboundcalleridnumberarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outbound_caller_id_number_arn: Option<::Value<String>>,
        /// Property [`OutboundFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-queue-outboundcallerconfig.html#cfn-connect-queue-outboundcallerconfig-outboundflowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outbound_flow_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OutboundCallerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref outbound_caller_id_name) = self.outbound_caller_id_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutboundCallerIdName", outbound_caller_id_name)?;
            }
            if let Some(ref outbound_caller_id_number_arn) = self.outbound_caller_id_number_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutboundCallerIdNumberArn", outbound_caller_id_number_arn)?;
            }
            if let Some(ref outbound_flow_arn) = self.outbound_flow_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutboundFlowArn", outbound_flow_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutboundCallerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutboundCallerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutboundCallerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutboundCallerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut outbound_caller_id_name: Option<::Value<String>> = None;
                    let mut outbound_caller_id_number_arn: Option<::Value<String>> = None;
                    let mut outbound_flow_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OutboundCallerIdName" => {
                                outbound_caller_id_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutboundCallerIdNumberArn" => {
                                outbound_caller_id_number_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutboundFlowArn" => {
                                outbound_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutboundCallerConfig {
                        outbound_caller_id_name: outbound_caller_id_name,
                        outbound_caller_id_number_arn: outbound_caller_id_number_arn,
                        outbound_flow_arn: outbound_flow_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod quick_connect {
    //! Property types for the `QuickConnect` resource.

    /// The [`AWS::Connect::QuickConnect.PhoneNumberQuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-phonenumberquickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PhoneNumberQuickConnectConfig {
        /// Property [`PhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-phonenumberquickconnectconfig.html#cfn-connect-quickconnect-phonenumberquickconnectconfig-phonenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_number: ::Value<String>,
    }

    impl ::codec::SerializeValue for PhoneNumberQuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneNumber", &self.phone_number)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PhoneNumberQuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PhoneNumberQuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PhoneNumberQuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PhoneNumberQuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut phone_number: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PhoneNumber" => {
                                phone_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PhoneNumberQuickConnectConfig {
                        phone_number: phone_number.ok_or(::serde::de::Error::missing_field("PhoneNumber"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::QuickConnect.QueueQuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-queuequickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QueueQuickConnectConfig {
        /// Property [`ContactFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-queuequickconnectconfig.html#cfn-connect-quickconnect-queuequickconnectconfig-contactflowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_flow_arn: ::Value<String>,
        /// Property [`QueueArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-queuequickconnectconfig.html#cfn-connect-quickconnect-queuequickconnectconfig-queuearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for QueueQuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactFlowArn", &self.contact_flow_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueArn", &self.queue_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueueQuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueueQuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueueQuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueueQuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contact_flow_arn: Option<::Value<String>> = None;
                    let mut queue_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContactFlowArn" => {
                                contact_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueArn" => {
                                queue_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueueQuickConnectConfig {
                        contact_flow_arn: contact_flow_arn.ok_or(::serde::de::Error::missing_field("ContactFlowArn"))?,
                        queue_arn: queue_arn.ok_or(::serde::de::Error::missing_field("QueueArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::QuickConnect.QuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QuickConnectConfig {
        /// Property [`PhoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-phoneconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_config: Option<::Value<PhoneNumberQuickConnectConfig>>,
        /// Property [`QueueConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-queueconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_config: Option<::Value<QueueQuickConnectConfig>>,
        /// Property [`QuickConnectType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-quickconnecttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quick_connect_type: ::Value<String>,
        /// Property [`UserConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-userconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_config: Option<::Value<UserQuickConnectConfig>>,
    }

    impl ::codec::SerializeValue for QuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref phone_config) = self.phone_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneConfig", phone_config)?;
            }
            if let Some(ref queue_config) = self.queue_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueConfig", queue_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuickConnectType", &self.quick_connect_type)?;
            if let Some(ref user_config) = self.user_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserConfig", user_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut phone_config: Option<::Value<PhoneNumberQuickConnectConfig>> = None;
                    let mut queue_config: Option<::Value<QueueQuickConnectConfig>> = None;
                    let mut quick_connect_type: Option<::Value<String>> = None;
                    let mut user_config: Option<::Value<UserQuickConnectConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PhoneConfig" => {
                                phone_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueConfig" => {
                                queue_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QuickConnectType" => {
                                quick_connect_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserConfig" => {
                                user_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QuickConnectConfig {
                        phone_config: phone_config,
                        queue_config: queue_config,
                        quick_connect_type: quick_connect_type.ok_or(::serde::de::Error::missing_field("QuickConnectType"))?,
                        user_config: user_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::QuickConnect.UserQuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-userquickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UserQuickConnectConfig {
        /// Property [`ContactFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-userquickconnectconfig.html#cfn-connect-quickconnect-userquickconnectconfig-contactflowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_flow_arn: ::Value<String>,
        /// Property [`UserArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-userquickconnectconfig.html#cfn-connect-quickconnect-userquickconnectconfig-userarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for UserQuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactFlowArn", &self.contact_flow_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserArn", &self.user_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserQuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserQuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserQuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserQuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contact_flow_arn: Option<::Value<String>> = None;
                    let mut user_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContactFlowArn" => {
                                contact_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserArn" => {
                                user_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserQuickConnectConfig {
                        contact_flow_arn: contact_flow_arn.ok_or(::serde::de::Error::missing_field("ContactFlowArn"))?,
                        user_arn: user_arn.ok_or(::serde::de::Error::missing_field("UserArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod routing_profile {
    //! Property types for the `RoutingProfile` resource.

    /// The [`AWS::Connect::RoutingProfile.CrossChannelBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-crosschannelbehavior.html) property type.
    #[derive(Debug, Default)]
    pub struct CrossChannelBehavior {
        /// Property [`BehaviorType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-crosschannelbehavior.html#cfn-connect-routingprofile-crosschannelbehavior-behaviortype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub behavior_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for CrossChannelBehavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BehaviorType", &self.behavior_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CrossChannelBehavior {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CrossChannelBehavior, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CrossChannelBehavior;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CrossChannelBehavior")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut behavior_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BehaviorType" => {
                                behavior_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CrossChannelBehavior {
                        behavior_type: behavior_type.ok_or(::serde::de::Error::missing_field("BehaviorType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::RoutingProfile.MediaConcurrency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-mediaconcurrency.html) property type.
    #[derive(Debug, Default)]
    pub struct MediaConcurrency {
        /// Property [`Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-mediaconcurrency.html#cfn-connect-routingprofile-mediaconcurrency-channel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel: ::Value<String>,
        /// Property [`Concurrency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-mediaconcurrency.html#cfn-connect-routingprofile-mediaconcurrency-concurrency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub concurrency: ::Value<u32>,
        /// Property [`CrossChannelBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-mediaconcurrency.html#cfn-connect-routingprofile-mediaconcurrency-crosschannelbehavior).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cross_channel_behavior: Option<::Value<CrossChannelBehavior>>,
    }

    impl ::codec::SerializeValue for MediaConcurrency {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Channel", &self.channel)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Concurrency", &self.concurrency)?;
            if let Some(ref cross_channel_behavior) = self.cross_channel_behavior {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CrossChannelBehavior", cross_channel_behavior)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MediaConcurrency {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MediaConcurrency, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MediaConcurrency;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MediaConcurrency")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut channel: Option<::Value<String>> = None;
                    let mut concurrency: Option<::Value<u32>> = None;
                    let mut cross_channel_behavior: Option<::Value<CrossChannelBehavior>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Channel" => {
                                channel = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Concurrency" => {
                                concurrency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CrossChannelBehavior" => {
                                cross_channel_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MediaConcurrency {
                        channel: channel.ok_or(::serde::de::Error::missing_field("Channel"))?,
                        concurrency: concurrency.ok_or(::serde::de::Error::missing_field("Concurrency"))?,
                        cross_channel_behavior: cross_channel_behavior,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::RoutingProfile.RoutingProfileQueueConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-routingprofilequeueconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RoutingProfileQueueConfig {
        /// Property [`Delay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-routingprofilequeueconfig.html#cfn-connect-routingprofile-routingprofilequeueconfig-delay).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delay: ::Value<u32>,
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-routingprofilequeueconfig.html#cfn-connect-routingprofile-routingprofilequeueconfig-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: ::Value<u32>,
        /// Property [`QueueReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-routingprofilequeueconfig.html#cfn-connect-routingprofile-routingprofilequeueconfig-queuereference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_reference: ::Value<RoutingProfileQueueReference>,
    }

    impl ::codec::SerializeValue for RoutingProfileQueueConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Delay", &self.delay)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueReference", &self.queue_reference)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RoutingProfileQueueConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RoutingProfileQueueConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RoutingProfileQueueConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RoutingProfileQueueConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delay: Option<::Value<u32>> = None;
                    let mut priority: Option<::Value<u32>> = None;
                    let mut queue_reference: Option<::Value<RoutingProfileQueueReference>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Delay" => {
                                delay = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueReference" => {
                                queue_reference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RoutingProfileQueueConfig {
                        delay: delay.ok_or(::serde::de::Error::missing_field("Delay"))?,
                        priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                        queue_reference: queue_reference.ok_or(::serde::de::Error::missing_field("QueueReference"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::RoutingProfile.RoutingProfileQueueReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-routingprofilequeuereference.html) property type.
    #[derive(Debug, Default)]
    pub struct RoutingProfileQueueReference {
        /// Property [`Channel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-routingprofilequeuereference.html#cfn-connect-routingprofile-routingprofilequeuereference-channel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel: ::Value<String>,
        /// Property [`QueueArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-routingprofilequeuereference.html#cfn-connect-routingprofile-routingprofilequeuereference-queuearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for RoutingProfileQueueReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Channel", &self.channel)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueArn", &self.queue_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RoutingProfileQueueReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RoutingProfileQueueReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RoutingProfileQueueReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RoutingProfileQueueReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut channel: Option<::Value<String>> = None;
                    let mut queue_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Channel" => {
                                channel = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueArn" => {
                                queue_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RoutingProfileQueueReference {
                        channel: channel.ok_or(::serde::de::Error::missing_field("Channel"))?,
                        queue_arn: queue_arn.ok_or(::serde::de::Error::missing_field("QueueArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod rule {
    //! Property types for the `Rule` resource.

    /// The [`AWS::Connect::Rule.Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-actions.html) property type.
    #[derive(Debug, Default)]
    pub struct Actions {
        /// Property [`AssignContactCategoryActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-actions.html#cfn-connect-rule-actions-assigncontactcategoryactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assign_contact_category_actions: Option<::ValueList<::json::Value>>,
        /// Property [`CreateCaseActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-actions.html#cfn-connect-rule-actions-createcaseactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub create_case_actions: Option<::ValueList<CreateCaseAction>>,
        /// Property [`EndAssociatedTasksActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-actions.html#cfn-connect-rule-actions-endassociatedtasksactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_associated_tasks_actions: Option<::ValueList<::json::Value>>,
        /// Property [`EventBridgeActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-actions.html#cfn-connect-rule-actions-eventbridgeactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_bridge_actions: Option<::ValueList<EventBridgeAction>>,
        /// Property [`SendNotificationActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-actions.html#cfn-connect-rule-actions-sendnotificationactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub send_notification_actions: Option<::ValueList<SendNotificationAction>>,
        /// Property [`TaskActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-actions.html#cfn-connect-rule-actions-taskactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_actions: Option<::ValueList<TaskAction>>,
        /// Property [`UpdateCaseActions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-actions.html#cfn-connect-rule-actions-updatecaseactions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_case_actions: Option<::ValueList<UpdateCaseAction>>,
    }

    impl ::codec::SerializeValue for Actions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref assign_contact_category_actions) = self.assign_contact_category_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignContactCategoryActions", assign_contact_category_actions)?;
            }
            if let Some(ref create_case_actions) = self.create_case_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateCaseActions", create_case_actions)?;
            }
            if let Some(ref end_associated_tasks_actions) = self.end_associated_tasks_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndAssociatedTasksActions", end_associated_tasks_actions)?;
            }
            if let Some(ref event_bridge_actions) = self.event_bridge_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventBridgeActions", event_bridge_actions)?;
            }
            if let Some(ref send_notification_actions) = self.send_notification_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SendNotificationActions", send_notification_actions)?;
            }
            if let Some(ref task_actions) = self.task_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskActions", task_actions)?;
            }
            if let Some(ref update_case_actions) = self.update_case_actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateCaseActions", update_case_actions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Actions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Actions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Actions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Actions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut assign_contact_category_actions: Option<::ValueList<::json::Value>> = None;
                    let mut create_case_actions: Option<::ValueList<CreateCaseAction>> = None;
                    let mut end_associated_tasks_actions: Option<::ValueList<::json::Value>> = None;
                    let mut event_bridge_actions: Option<::ValueList<EventBridgeAction>> = None;
                    let mut send_notification_actions: Option<::ValueList<SendNotificationAction>> = None;
                    let mut task_actions: Option<::ValueList<TaskAction>> = None;
                    let mut update_case_actions: Option<::ValueList<UpdateCaseAction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssignContactCategoryActions" => {
                                assign_contact_category_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreateCaseActions" => {
                                create_case_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndAssociatedTasksActions" => {
                                end_associated_tasks_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventBridgeActions" => {
                                event_bridge_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SendNotificationActions" => {
                                send_notification_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskActions" => {
                                task_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateCaseActions" => {
                                update_case_actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Actions {
                        assign_contact_category_actions: assign_contact_category_actions,
                        create_case_actions: create_case_actions,
                        end_associated_tasks_actions: end_associated_tasks_actions,
                        event_bridge_actions: event_bridge_actions,
                        send_notification_actions: send_notification_actions,
                        task_actions: task_actions,
                        update_case_actions: update_case_actions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::Rule.CreateCaseAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-createcaseaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CreateCaseAction {
        /// Property [`Fields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-createcaseaction.html#cfn-connect-rule-createcaseaction-fields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fields: ::ValueList<Field>,
        /// Property [`TemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-createcaseaction.html#cfn-connect-rule-createcaseaction-templateid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub template_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for CreateCaseAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", &self.fields)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateId", &self.template_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CreateCaseAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CreateCaseAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CreateCaseAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CreateCaseAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut fields: Option<::ValueList<Field>> = None;
                    let mut template_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Fields" => {
                                fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemplateId" => {
                                template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CreateCaseAction {
                        fields: fields.ok_or(::serde::de::Error::missing_field("Fields"))?,
                        template_id: template_id.ok_or(::serde::de::Error::missing_field("TemplateId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::Rule.EventBridgeAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-eventbridgeaction.html) property type.
    #[derive(Debug, Default)]
    pub struct EventBridgeAction {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-eventbridgeaction.html#cfn-connect-rule-eventbridgeaction-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for EventBridgeAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventBridgeAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventBridgeAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventBridgeAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventBridgeAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventBridgeAction {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::Rule.Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-field.html) property type.
    #[derive(Debug, Default)]
    pub struct Field {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-field.html#cfn-connect-rule-field-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-field.html#cfn-connect-rule-field-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<FieldValue>,
    }

    impl ::codec::SerializeValue for Field {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Field {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Field, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Field;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Field")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut value: Option<::Value<FieldValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Field {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::Rule.FieldValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-fieldvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldValue {
        /// Property [`BooleanValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-fieldvalue.html#cfn-connect-rule-fieldvalue-booleanvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub boolean_value: Option<::Value<bool>>,
        /// Property [`DoubleValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-fieldvalue.html#cfn-connect-rule-fieldvalue-doublevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub double_value: Option<::Value<f64>>,
        /// Property [`EmptyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-fieldvalue.html#cfn-connect-rule-fieldvalue-emptyvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub empty_value: Option<::Value<::json::Value>>,
        /// Property [`StringValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-fieldvalue.html#cfn-connect-rule-fieldvalue-stringvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FieldValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref boolean_value) = self.boolean_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BooleanValue", boolean_value)?;
            }
            if let Some(ref double_value) = self.double_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DoubleValue", double_value)?;
            }
            if let Some(ref empty_value) = self.empty_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmptyValue", empty_value)?;
            }
            if let Some(ref string_value) = self.string_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", string_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut boolean_value: Option<::Value<bool>> = None;
                    let mut double_value: Option<::Value<f64>> = None;
                    let mut empty_value: Option<::Value<::json::Value>> = None;
                    let mut string_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BooleanValue" => {
                                boolean_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DoubleValue" => {
                                double_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmptyValue" => {
                                empty_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldValue {
                        boolean_value: boolean_value,
                        double_value: double_value,
                        empty_value: empty_value,
                        string_value: string_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::Rule.NotificationRecipientType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-notificationrecipienttype.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationRecipientType {
        /// Property [`UserArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-notificationrecipienttype.html#cfn-connect-rule-notificationrecipienttype-userarns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_arns: Option<::ValueList<String>>,
        /// Property [`UserTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-notificationrecipienttype.html#cfn-connect-rule-notificationrecipienttype-usertags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_tags: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for NotificationRecipientType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref user_arns) = self.user_arns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserArns", user_arns)?;
            }
            if let Some(ref user_tags) = self.user_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserTags", user_tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationRecipientType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationRecipientType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationRecipientType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationRecipientType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut user_arns: Option<::ValueList<String>> = None;
                    let mut user_tags: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "UserArns" => {
                                user_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserTags" => {
                                user_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationRecipientType {
                        user_arns: user_arns,
                        user_tags: user_tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::Rule.Reference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-reference.html) property type.
    #[derive(Debug, Default)]
    pub struct Reference {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-reference.html#cfn-connect-rule-reference-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-reference.html#cfn-connect-rule-reference-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Reference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Reference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Reference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Reference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Reference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Reference {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::Rule.RuleTriggerEventSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-ruletriggereventsource.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleTriggerEventSource {
        /// Property [`EventSourceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-ruletriggereventsource.html#cfn-connect-rule-ruletriggereventsource-eventsourcename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub event_source_name: ::Value<String>,
        /// Property [`IntegrationAssociationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-ruletriggereventsource.html#cfn-connect-rule-ruletriggereventsource-integrationassociationarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub integration_association_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RuleTriggerEventSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventSourceName", &self.event_source_name)?;
            if let Some(ref integration_association_arn) = self.integration_association_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationAssociationArn", integration_association_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleTriggerEventSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleTriggerEventSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleTriggerEventSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleTriggerEventSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_source_name: Option<::Value<String>> = None;
                    let mut integration_association_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventSourceName" => {
                                event_source_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegrationAssociationArn" => {
                                integration_association_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleTriggerEventSource {
                        event_source_name: event_source_name.ok_or(::serde::de::Error::missing_field("EventSourceName"))?,
                        integration_association_arn: integration_association_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::Rule.SendNotificationAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-sendnotificationaction.html) property type.
    #[derive(Debug, Default)]
    pub struct SendNotificationAction {
        /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-sendnotificationaction.html#cfn-connect-rule-sendnotificationaction-content).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content: ::Value<String>,
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-sendnotificationaction.html#cfn-connect-rule-sendnotificationaction-contenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_type: ::Value<String>,
        /// Property [`DeliveryMethod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-sendnotificationaction.html#cfn-connect-rule-sendnotificationaction-deliverymethod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_method: ::Value<String>,
        /// Property [`Recipient`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-sendnotificationaction.html#cfn-connect-rule-sendnotificationaction-recipient).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recipient: ::Value<NotificationRecipientType>,
        /// Property [`Subject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-sendnotificationaction.html#cfn-connect-rule-sendnotificationaction-subject).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SendNotificationAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", &self.content_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryMethod", &self.delivery_method)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Recipient", &self.recipient)?;
            if let Some(ref subject) = self.subject {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subject", subject)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SendNotificationAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SendNotificationAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SendNotificationAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SendNotificationAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content: Option<::Value<String>> = None;
                    let mut content_type: Option<::Value<String>> = None;
                    let mut delivery_method: Option<::Value<String>> = None;
                    let mut recipient: Option<::Value<NotificationRecipientType>> = None;
                    let mut subject: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Content" => {
                                content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeliveryMethod" => {
                                delivery_method = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Recipient" => {
                                recipient = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subject" => {
                                subject = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SendNotificationAction {
                        content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                        content_type: content_type.ok_or(::serde::de::Error::missing_field("ContentType"))?,
                        delivery_method: delivery_method.ok_or(::serde::de::Error::missing_field("DeliveryMethod"))?,
                        recipient: recipient.ok_or(::serde::de::Error::missing_field("Recipient"))?,
                        subject: subject,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::Rule.TaskAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-taskaction.html) property type.
    #[derive(Debug, Default)]
    pub struct TaskAction {
        /// Property [`ContactFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-taskaction.html#cfn-connect-rule-taskaction-contactflowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_flow_arn: ::Value<String>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-taskaction.html#cfn-connect-rule-taskaction-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-taskaction.html#cfn-connect-rule-taskaction-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`References`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-taskaction.html#cfn-connect-rule-taskaction-references).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub references: Option<::ValueMap<Reference>>,
    }

    impl ::codec::SerializeValue for TaskAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactFlowArn", &self.contact_flow_arn)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref references) = self.references {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "References", references)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TaskAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TaskAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TaskAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contact_flow_arn: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut references: Option<::ValueMap<Reference>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContactFlowArn" => {
                                contact_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "References" => {
                                references = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskAction {
                        contact_flow_arn: contact_flow_arn.ok_or(::serde::de::Error::missing_field("ContactFlowArn"))?,
                        description: description,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        references: references,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::Rule.UpdateCaseAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-updatecaseaction.html) property type.
    #[derive(Debug, Default)]
    pub struct UpdateCaseAction {
        /// Property [`Fields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-updatecaseaction.html#cfn-connect-rule-updatecaseaction-fields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fields: ::ValueList<Field>,
    }

    impl ::codec::SerializeValue for UpdateCaseAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", &self.fields)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UpdateCaseAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UpdateCaseAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UpdateCaseAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UpdateCaseAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut fields: Option<::ValueList<Field>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Fields" => {
                                fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UpdateCaseAction {
                        fields: fields.ok_or(::serde::de::Error::missing_field("Fields"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod task_template {
    //! Property types for the `TaskTemplate` resource.

    /// The [`AWS::Connect::TaskTemplate.Constraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-constraints.html) property type.
    #[derive(Debug, Default)]
    pub struct Constraints {
        /// Property [`InvisibleFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-constraints.html#cfn-connect-tasktemplate-constraints-invisiblefields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invisible_fields: Option<::ValueList<InvisibleFieldInfo>>,
        /// Property [`ReadOnlyFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-constraints.html#cfn-connect-tasktemplate-constraints-readonlyfields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_only_fields: Option<::ValueList<ReadOnlyFieldInfo>>,
        /// Property [`RequiredFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-constraints.html#cfn-connect-tasktemplate-constraints-requiredfields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub required_fields: Option<::ValueList<RequiredFieldInfo>>,
    }

    impl ::codec::SerializeValue for Constraints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref invisible_fields) = self.invisible_fields {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvisibleFields", invisible_fields)?;
            }
            if let Some(ref read_only_fields) = self.read_only_fields {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnlyFields", read_only_fields)?;
            }
            if let Some(ref required_fields) = self.required_fields {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequiredFields", required_fields)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Constraints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Constraints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Constraints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Constraints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut invisible_fields: Option<::ValueList<InvisibleFieldInfo>> = None;
                    let mut read_only_fields: Option<::ValueList<ReadOnlyFieldInfo>> = None;
                    let mut required_fields: Option<::ValueList<RequiredFieldInfo>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InvisibleFields" => {
                                invisible_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadOnlyFields" => {
                                read_only_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequiredFields" => {
                                required_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Constraints {
                        invisible_fields: invisible_fields,
                        read_only_fields: read_only_fields,
                        required_fields: required_fields,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::TaskTemplate.DefaultFieldValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-defaultfieldvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultFieldValue {
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-defaultfieldvalue.html#cfn-connect-tasktemplate-defaultfieldvalue-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-defaultfieldvalue.html#cfn-connect-tasktemplate-defaultfieldvalue-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<FieldIdentifier>,
    }

    impl ::codec::SerializeValue for DefaultFieldValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", &self.default_value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultFieldValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultFieldValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultFieldValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultFieldValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_value: Option<::Value<String>> = None;
                    let mut id: Option<::Value<FieldIdentifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultFieldValue {
                        default_value: default_value.ok_or(::serde::de::Error::missing_field("DefaultValue"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::TaskTemplate.Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html) property type.
    #[derive(Debug, Default)]
    pub struct Field {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html#cfn-connect-tasktemplate-field-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html#cfn-connect-tasktemplate-field-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<FieldIdentifier>,
        /// Property [`SingleSelectOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html#cfn-connect-tasktemplate-field-singleselectoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub single_select_options: Option<::ValueList<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html#cfn-connect-tasktemplate-field-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Field {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref single_select_options) = self.single_select_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleSelectOptions", single_select_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Field {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Field, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Field;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Field")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut id: Option<::Value<FieldIdentifier>> = None;
                    let mut single_select_options: Option<::ValueList<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SingleSelectOptions" => {
                                single_select_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Field {
                        description: description,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        single_select_options: single_select_options,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::TaskTemplate.FieldIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-fieldidentifier.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldIdentifier {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-fieldidentifier.html#cfn-connect-tasktemplate-fieldidentifier-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldIdentifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldIdentifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldIdentifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldIdentifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldIdentifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldIdentifier {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::TaskTemplate.InvisibleFieldInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-invisiblefieldinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct InvisibleFieldInfo {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-invisiblefieldinfo.html#cfn-connect-tasktemplate-invisiblefieldinfo-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<FieldIdentifier>,
    }

    impl ::codec::SerializeValue for InvisibleFieldInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InvisibleFieldInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InvisibleFieldInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InvisibleFieldInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InvisibleFieldInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<FieldIdentifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InvisibleFieldInfo {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::TaskTemplate.ReadOnlyFieldInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-readonlyfieldinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct ReadOnlyFieldInfo {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-readonlyfieldinfo.html#cfn-connect-tasktemplate-readonlyfieldinfo-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<FieldIdentifier>,
    }

    impl ::codec::SerializeValue for ReadOnlyFieldInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReadOnlyFieldInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReadOnlyFieldInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReadOnlyFieldInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReadOnlyFieldInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<FieldIdentifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReadOnlyFieldInfo {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::TaskTemplate.RequiredFieldInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-requiredfieldinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct RequiredFieldInfo {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-requiredfieldinfo.html#cfn-connect-tasktemplate-requiredfieldinfo-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<FieldIdentifier>,
    }

    impl ::codec::SerializeValue for RequiredFieldInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RequiredFieldInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RequiredFieldInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RequiredFieldInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RequiredFieldInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<FieldIdentifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RequiredFieldInfo {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user {
    //! Property types for the `User` resource.

    /// The [`AWS::Connect::User.UserIdentityInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct UserIdentityInfo {
        /// Property [`Email`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-email).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email: Option<::Value<String>>,
        /// Property [`FirstName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-firstname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub first_name: Option<::Value<String>>,
        /// Property [`LastName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-lastname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_name: Option<::Value<String>>,
        /// Property [`Mobile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-mobile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mobile: Option<::Value<String>>,
        /// Property [`SecondaryEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-secondaryemail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secondary_email: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UserIdentityInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref email) = self.email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Email", email)?;
            }
            if let Some(ref first_name) = self.first_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirstName", first_name)?;
            }
            if let Some(ref last_name) = self.last_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastName", last_name)?;
            }
            if let Some(ref mobile) = self.mobile {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mobile", mobile)?;
            }
            if let Some(ref secondary_email) = self.secondary_email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryEmail", secondary_email)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserIdentityInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserIdentityInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserIdentityInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserIdentityInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut email: Option<::Value<String>> = None;
                    let mut first_name: Option<::Value<String>> = None;
                    let mut last_name: Option<::Value<String>> = None;
                    let mut mobile: Option<::Value<String>> = None;
                    let mut secondary_email: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Email" => {
                                email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirstName" => {
                                first_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastName" => {
                                last_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mobile" => {
                                mobile = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondaryEmail" => {
                                secondary_email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserIdentityInfo {
                        email: email,
                        first_name: first_name,
                        last_name: last_name,
                        mobile: mobile,
                        secondary_email: secondary_email,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::User.UserPhoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UserPhoneConfig {
        /// Property [`AfterContactWorkTimeLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-aftercontactworktimelimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub after_contact_work_time_limit: Option<::Value<u32>>,
        /// Property [`AutoAccept`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-autoaccept).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_accept: Option<::Value<bool>>,
        /// Property [`DeskPhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-deskphonenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desk_phone_number: Option<::Value<String>>,
        /// Property [`PhoneType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-phonetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for UserPhoneConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref after_contact_work_time_limit) = self.after_contact_work_time_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AfterContactWorkTimeLimit", after_contact_work_time_limit)?;
            }
            if let Some(ref auto_accept) = self.auto_accept {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoAccept", auto_accept)?;
            }
            if let Some(ref desk_phone_number) = self.desk_phone_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeskPhoneNumber", desk_phone_number)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneType", &self.phone_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserPhoneConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPhoneConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserPhoneConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserPhoneConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut after_contact_work_time_limit: Option<::Value<u32>> = None;
                    let mut auto_accept: Option<::Value<bool>> = None;
                    let mut desk_phone_number: Option<::Value<String>> = None;
                    let mut phone_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AfterContactWorkTimeLimit" => {
                                after_contact_work_time_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AutoAccept" => {
                                auto_accept = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeskPhoneNumber" => {
                                desk_phone_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PhoneType" => {
                                phone_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserPhoneConfig {
                        after_contact_work_time_limit: after_contact_work_time_limit,
                        auto_accept: auto_accept,
                        desk_phone_number: desk_phone_number,
                        phone_type: phone_type.ok_or(::serde::de::Error::missing_field("PhoneType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::User.UserProficiency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userproficiency.html) property type.
    #[derive(Debug, Default)]
    pub struct UserProficiency {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userproficiency.html#cfn-connect-user-userproficiency-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: ::Value<String>,
        /// Property [`AttributeValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userproficiency.html#cfn-connect-user-userproficiency-attributevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_value: ::Value<String>,
        /// Property [`Level`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userproficiency.html#cfn-connect-user-userproficiency-level).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub level: ::Value<f64>,
    }

    impl ::codec::SerializeValue for UserProficiency {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", &self.attribute_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeValue", &self.attribute_value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Level", &self.level)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserProficiency {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProficiency, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserProficiency;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserProficiency")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_name: Option<::Value<String>> = None;
                    let mut attribute_value: Option<::Value<String>> = None;
                    let mut level: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AttributeValue" => {
                                attribute_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Level" => {
                                level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserProficiency {
                        attribute_name: attribute_name.ok_or(::serde::de::Error::missing_field("AttributeName"))?,
                        attribute_value: attribute_value.ok_or(::serde::de::Error::missing_field("AttributeValue"))?,
                        level: level.ok_or(::serde::de::Error::missing_field("Level"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
