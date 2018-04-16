//! Types for the `Inspector` service.

/// The [`AWS::Inspector::AssessmentTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttarget.html) resource type.
#[derive(Debug, Default)]
pub struct AssessmentTarget {
    properties: AssessmentTargetProperties
}

/// Properties for the `AssessmentTarget` resource.
#[derive(Debug, Default)]
pub struct AssessmentTargetProperties {
    /// Property [`AssessmentTargetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttarget.html#cfn-inspector-assessmenttarget-assessmenttargetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub assessment_target_name: Option<::Value<String>>,
    /// Property [`ResourceGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttarget.html#cfn-inspector-assessmenttarget-resourcegrouparn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_group_arn: ::Value<String>,
}

impl ::serde::Serialize for AssessmentTargetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref assessment_target_name) = self.assessment_target_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssessmentTargetName", assessment_target_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceGroupArn", &self.resource_group_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssessmentTargetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssessmentTargetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssessmentTargetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssessmentTargetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut assessment_target_name: Option<::Value<String>> = None;
                let mut resource_group_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssessmentTargetName" => {
                            assessment_target_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceGroupArn" => {
                            resource_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AssessmentTargetProperties {
                    assessment_target_name: assessment_target_name,
                    resource_group_arn: resource_group_arn.ok_or(::serde::de::Error::missing_field("ResourceGroupArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AssessmentTarget {
    type Properties = AssessmentTargetProperties;
    const TYPE: &'static str = "AWS::Inspector::AssessmentTarget";
    fn properties(&self) -> &AssessmentTargetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssessmentTargetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AssessmentTarget {}

impl From<AssessmentTargetProperties> for AssessmentTarget {
    fn from(properties: AssessmentTargetProperties) -> AssessmentTarget {
        AssessmentTarget { properties }
    }
}

/// The [`AWS::Inspector::AssessmentTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttemplate.html) resource type.
#[derive(Debug, Default)]
pub struct AssessmentTemplate {
    properties: AssessmentTemplateProperties
}

/// Properties for the `AssessmentTemplate` resource.
#[derive(Debug, Default)]
pub struct AssessmentTemplateProperties {
    /// Property [`AssessmentTargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttemplate.html#cfn-inspector-assessmenttemplate-assessmenttargetarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub assessment_target_arn: ::Value<String>,
    /// Property [`AssessmentTemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttemplate.html#cfn-inspector-assessmenttemplate-assessmenttemplatename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub assessment_template_name: Option<::Value<String>>,
    /// Property [`DurationInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttemplate.html#cfn-inspector-assessmenttemplate-durationinseconds).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub duration_in_seconds: ::Value<u32>,
    /// Property [`RulesPackageArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttemplate.html#cfn-inspector-assessmenttemplate-rulespackagearns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rules_package_arns: ::ValueList<String>,
    /// Property [`UserAttributesForFindings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-assessmenttemplate.html#cfn-inspector-assessmenttemplate-userattributesforfindings).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_attributes_for_findings: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AssessmentTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssessmentTargetArn", &self.assessment_target_arn)?;
        if let Some(ref assessment_template_name) = self.assessment_template_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssessmentTemplateName", assessment_template_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInSeconds", &self.duration_in_seconds)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RulesPackageArns", &self.rules_package_arns)?;
        if let Some(ref user_attributes_for_findings) = self.user_attributes_for_findings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserAttributesForFindings", user_attributes_for_findings)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssessmentTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssessmentTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssessmentTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssessmentTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut assessment_target_arn: Option<::Value<String>> = None;
                let mut assessment_template_name: Option<::Value<String>> = None;
                let mut duration_in_seconds: Option<::Value<u32>> = None;
                let mut rules_package_arns: Option<::ValueList<String>> = None;
                let mut user_attributes_for_findings: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssessmentTargetArn" => {
                            assessment_target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssessmentTemplateName" => {
                            assessment_template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DurationInSeconds" => {
                            duration_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RulesPackageArns" => {
                            rules_package_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserAttributesForFindings" => {
                            user_attributes_for_findings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AssessmentTemplateProperties {
                    assessment_target_arn: assessment_target_arn.ok_or(::serde::de::Error::missing_field("AssessmentTargetArn"))?,
                    assessment_template_name: assessment_template_name,
                    duration_in_seconds: duration_in_seconds.ok_or(::serde::de::Error::missing_field("DurationInSeconds"))?,
                    rules_package_arns: rules_package_arns.ok_or(::serde::de::Error::missing_field("RulesPackageArns"))?,
                    user_attributes_for_findings: user_attributes_for_findings,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AssessmentTemplate {
    type Properties = AssessmentTemplateProperties;
    const TYPE: &'static str = "AWS::Inspector::AssessmentTemplate";
    fn properties(&self) -> &AssessmentTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssessmentTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AssessmentTemplate {}

impl From<AssessmentTemplateProperties> for AssessmentTemplate {
    fn from(properties: AssessmentTemplateProperties) -> AssessmentTemplate {
        AssessmentTemplate { properties }
    }
}

/// The [`AWS::Inspector::ResourceGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-resourcegroup.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceGroup {
    properties: ResourceGroupProperties
}

/// Properties for the `ResourceGroup` resource.
#[derive(Debug, Default)]
pub struct ResourceGroupProperties {
    /// Property [`ResourceGroupTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspector-resourcegroup.html#cfn-inspector-resourcegroup-resourcegrouptags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_group_tags: ::ValueList<::Tag>,
}

impl ::serde::Serialize for ResourceGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceGroupTags", &self.resource_group_tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resource_group_tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceGroupTags" => {
                            resource_group_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceGroupProperties {
                    resource_group_tags: resource_group_tags.ok_or(::serde::de::Error::missing_field("ResourceGroupTags"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceGroup {
    type Properties = ResourceGroupProperties;
    const TYPE: &'static str = "AWS::Inspector::ResourceGroup";
    fn properties(&self) -> &ResourceGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceGroup {}

impl From<ResourceGroupProperties> for ResourceGroup {
    fn from(properties: ResourceGroupProperties) -> ResourceGroup {
        ResourceGroup { properties }
    }
}
