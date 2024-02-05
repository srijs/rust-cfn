//! Types for the `BillingConductor` service.

/// The [`AWS::BillingConductor::BillingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-billinggroup.html) resource type.
#[derive(Debug, Default)]
pub struct BillingGroup {
    properties: BillingGroupProperties
}

/// Properties for the `BillingGroup` resource.
#[derive(Debug, Default)]
pub struct BillingGroupProperties {
    /// Property [`AccountGrouping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-billinggroup.html#cfn-billingconductor-billinggroup-accountgrouping).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub account_grouping: ::Value<self::billing_group::AccountGrouping>,
    /// Property [`ComputationPreference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-billinggroup.html#cfn-billingconductor-billinggroup-computationpreference).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub computation_preference: ::Value<self::billing_group::ComputationPreference>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-billinggroup.html#cfn-billingconductor-billinggroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-billinggroup.html#cfn-billingconductor-billinggroup-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`PrimaryAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-billinggroup.html#cfn-billingconductor-billinggroup-primaryaccountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub primary_account_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-billinggroup.html#cfn-billingconductor-billinggroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for BillingGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountGrouping", &self.account_grouping)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputationPreference", &self.computation_preference)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryAccountId", &self.primary_account_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BillingGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BillingGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BillingGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BillingGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_grouping: Option<::Value<self::billing_group::AccountGrouping>> = None;
                let mut computation_preference: Option<::Value<self::billing_group::ComputationPreference>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut primary_account_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountGrouping" => {
                            account_grouping = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComputationPreference" => {
                            computation_preference = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrimaryAccountId" => {
                            primary_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BillingGroupProperties {
                    account_grouping: account_grouping.ok_or(::serde::de::Error::missing_field("AccountGrouping"))?,
                    computation_preference: computation_preference.ok_or(::serde::de::Error::missing_field("ComputationPreference"))?,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    primary_account_id: primary_account_id.ok_or(::serde::de::Error::missing_field("PrimaryAccountId"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BillingGroup {
    type Properties = BillingGroupProperties;
    const TYPE: &'static str = "AWS::BillingConductor::BillingGroup";
    fn properties(&self) -> &BillingGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BillingGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BillingGroup {}

impl From<BillingGroupProperties> for BillingGroup {
    fn from(properties: BillingGroupProperties) -> BillingGroup {
        BillingGroup { properties }
    }
}

/// The [`AWS::BillingConductor::CustomLineItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-customlineitem.html) resource type.
#[derive(Debug, Default)]
pub struct CustomLineItem {
    properties: CustomLineItemProperties
}

/// Properties for the `CustomLineItem` resource.
#[derive(Debug, Default)]
pub struct CustomLineItemProperties {
    /// Property [`AccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-customlineitem.html#cfn-billingconductor-customlineitem-accountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub account_id: Option<::Value<String>>,
    /// Property [`BillingGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-customlineitem.html#cfn-billingconductor-customlineitem-billinggrouparn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub billing_group_arn: ::Value<String>,
    /// Property [`BillingPeriodRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-customlineitem.html#cfn-billingconductor-customlineitem-billingperiodrange).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub billing_period_range: Option<::Value<self::custom_line_item::BillingPeriodRange>>,
    /// Property [`CustomLineItemChargeDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-customlineitem.html#cfn-billingconductor-customlineitem-customlineitemchargedetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_line_item_charge_details: Option<::Value<self::custom_line_item::CustomLineItemChargeDetails>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-customlineitem.html#cfn-billingconductor-customlineitem-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-customlineitem.html#cfn-billingconductor-customlineitem-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-customlineitem.html#cfn-billingconductor-customlineitem-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CustomLineItemProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref account_id) = self.account_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountId", account_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingGroupArn", &self.billing_group_arn)?;
        if let Some(ref billing_period_range) = self.billing_period_range {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingPeriodRange", billing_period_range)?;
        }
        if let Some(ref custom_line_item_charge_details) = self.custom_line_item_charge_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomLineItemChargeDetails", custom_line_item_charge_details)?;
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

impl<'de> ::serde::Deserialize<'de> for CustomLineItemProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomLineItemProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomLineItemProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CustomLineItemProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_id: Option<::Value<String>> = None;
                let mut billing_group_arn: Option<::Value<String>> = None;
                let mut billing_period_range: Option<::Value<self::custom_line_item::BillingPeriodRange>> = None;
                let mut custom_line_item_charge_details: Option<::Value<self::custom_line_item::CustomLineItemChargeDetails>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountId" => {
                            account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BillingGroupArn" => {
                            billing_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BillingPeriodRange" => {
                            billing_period_range = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomLineItemChargeDetails" => {
                            custom_line_item_charge_details = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(CustomLineItemProperties {
                    account_id: account_id,
                    billing_group_arn: billing_group_arn.ok_or(::serde::de::Error::missing_field("BillingGroupArn"))?,
                    billing_period_range: billing_period_range,
                    custom_line_item_charge_details: custom_line_item_charge_details,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CustomLineItem {
    type Properties = CustomLineItemProperties;
    const TYPE: &'static str = "AWS::BillingConductor::CustomLineItem";
    fn properties(&self) -> &CustomLineItemProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomLineItemProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CustomLineItem {}

impl From<CustomLineItemProperties> for CustomLineItem {
    fn from(properties: CustomLineItemProperties) -> CustomLineItem {
        CustomLineItem { properties }
    }
}

/// The [`AWS::BillingConductor::PricingPlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingplan.html) resource type.
#[derive(Debug, Default)]
pub struct PricingPlan {
    properties: PricingPlanProperties
}

/// Properties for the `PricingPlan` resource.
#[derive(Debug, Default)]
pub struct PricingPlanProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingplan.html#cfn-billingconductor-pricingplan-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingplan.html#cfn-billingconductor-pricingplan-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`PricingRuleArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingplan.html#cfn-billingconductor-pricingplan-pricingrulearns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pricing_rule_arns: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingplan.html#cfn-billingconductor-pricingplan-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PricingPlanProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref pricing_rule_arns) = self.pricing_rule_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PricingRuleArns", pricing_rule_arns)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PricingPlanProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PricingPlanProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PricingPlanProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PricingPlanProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut pricing_rule_arns: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PricingRuleArns" => {
                            pricing_rule_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PricingPlanProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    pricing_rule_arns: pricing_rule_arns,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PricingPlan {
    type Properties = PricingPlanProperties;
    const TYPE: &'static str = "AWS::BillingConductor::PricingPlan";
    fn properties(&self) -> &PricingPlanProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PricingPlanProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PricingPlan {}

impl From<PricingPlanProperties> for PricingPlan {
    fn from(properties: PricingPlanProperties) -> PricingPlan {
        PricingPlan { properties }
    }
}

/// The [`AWS::BillingConductor::PricingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html) resource type.
#[derive(Debug, Default)]
pub struct PricingRule {
    properties: PricingRuleProperties
}

/// Properties for the `PricingRule` resource.
#[derive(Debug, Default)]
pub struct PricingRuleProperties {
    /// Property [`BillingEntity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-billingentity).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub billing_entity: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`ModifierPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-modifierpercentage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub modifier_percentage: Option<::Value<f64>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Operation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-operation).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub operation: Option<::Value<String>>,
    /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-scope).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scope: ::Value<String>,
    /// Property [`Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-service).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Tiering`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-tiering).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tiering: Option<::Value<self::pricing_rule::Tiering>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: ::Value<String>,
    /// Property [`UsageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-billingconductor-pricingrule.html#cfn-billingconductor-pricingrule-usagetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub usage_type: Option<::Value<String>>,
}

impl ::serde::Serialize for PricingRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref billing_entity) = self.billing_entity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingEntity", billing_entity)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref modifier_percentage) = self.modifier_percentage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModifierPercentage", modifier_percentage)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref operation) = self.operation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Operation", operation)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", &self.scope)?;
        if let Some(ref service) = self.service {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Service", service)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref tiering) = self.tiering {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tiering", tiering)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        if let Some(ref usage_type) = self.usage_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsageType", usage_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PricingRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PricingRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PricingRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PricingRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut billing_entity: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut modifier_percentage: Option<::Value<f64>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut operation: Option<::Value<String>> = None;
                let mut scope: Option<::Value<String>> = None;
                let mut service: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut tiering: Option<::Value<self::pricing_rule::Tiering>> = None;
                let mut r#type: Option<::Value<String>> = None;
                let mut usage_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BillingEntity" => {
                            billing_entity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModifierPercentage" => {
                            modifier_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Operation" => {
                            operation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scope" => {
                            scope = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Service" => {
                            service = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tiering" => {
                            tiering = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UsageType" => {
                            usage_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PricingRuleProperties {
                    billing_entity: billing_entity,
                    description: description,
                    modifier_percentage: modifier_percentage,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    operation: operation,
                    scope: scope.ok_or(::serde::de::Error::missing_field("Scope"))?,
                    service: service,
                    tags: tags,
                    tiering: tiering,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    usage_type: usage_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PricingRule {
    type Properties = PricingRuleProperties;
    const TYPE: &'static str = "AWS::BillingConductor::PricingRule";
    fn properties(&self) -> &PricingRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PricingRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PricingRule {}

impl From<PricingRuleProperties> for PricingRule {
    fn from(properties: PricingRuleProperties) -> PricingRule {
        PricingRule { properties }
    }
}

pub mod billing_group {
    //! Property types for the `BillingGroup` resource.

    /// The [`AWS::BillingConductor::BillingGroup.AccountGrouping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-billinggroup-accountgrouping.html) property type.
    #[derive(Debug, Default)]
    pub struct AccountGrouping {
        /// Property [`AutoAssociate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-billinggroup-accountgrouping.html#cfn-billingconductor-billinggroup-accountgrouping-autoassociate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_associate: Option<::Value<bool>>,
        /// Property [`LinkedAccountIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-billinggroup-accountgrouping.html#cfn-billingconductor-billinggroup-accountgrouping-linkedaccountids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub linked_account_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AccountGrouping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_associate) = self.auto_associate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoAssociate", auto_associate)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LinkedAccountIds", &self.linked_account_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccountGrouping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccountGrouping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccountGrouping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccountGrouping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_associate: Option<::Value<bool>> = None;
                    let mut linked_account_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoAssociate" => {
                                auto_associate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LinkedAccountIds" => {
                                linked_account_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccountGrouping {
                        auto_associate: auto_associate,
                        linked_account_ids: linked_account_ids.ok_or(::serde::de::Error::missing_field("LinkedAccountIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::BillingConductor::BillingGroup.ComputationPreference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-billinggroup-computationpreference.html) property type.
    #[derive(Debug, Default)]
    pub struct ComputationPreference {
        /// Property [`PricingPlanArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-billinggroup-computationpreference.html#cfn-billingconductor-billinggroup-computationpreference-pricingplanarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pricing_plan_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for ComputationPreference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PricingPlanArn", &self.pricing_plan_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComputationPreference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComputationPreference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComputationPreference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComputationPreference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pricing_plan_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PricingPlanArn" => {
                                pricing_plan_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComputationPreference {
                        pricing_plan_arn: pricing_plan_arn.ok_or(::serde::de::Error::missing_field("PricingPlanArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod custom_line_item {
    //! Property types for the `CustomLineItem` resource.

    /// The [`AWS::BillingConductor::CustomLineItem.BillingPeriodRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-billingperiodrange.html) property type.
    #[derive(Debug, Default)]
    pub struct BillingPeriodRange {
        /// Property [`ExclusiveEndBillingPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-billingperiodrange.html#cfn-billingconductor-customlineitem-billingperiodrange-exclusiveendbillingperiod).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub exclusive_end_billing_period: Option<::Value<String>>,
        /// Property [`InclusiveStartBillingPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-billingperiodrange.html#cfn-billingconductor-customlineitem-billingperiodrange-inclusivestartbillingperiod).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub inclusive_start_billing_period: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BillingPeriodRange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exclusive_end_billing_period) = self.exclusive_end_billing_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExclusiveEndBillingPeriod", exclusive_end_billing_period)?;
            }
            if let Some(ref inclusive_start_billing_period) = self.inclusive_start_billing_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InclusiveStartBillingPeriod", inclusive_start_billing_period)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BillingPeriodRange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BillingPeriodRange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BillingPeriodRange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BillingPeriodRange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exclusive_end_billing_period: Option<::Value<String>> = None;
                    let mut inclusive_start_billing_period: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExclusiveEndBillingPeriod" => {
                                exclusive_end_billing_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InclusiveStartBillingPeriod" => {
                                inclusive_start_billing_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BillingPeriodRange {
                        exclusive_end_billing_period: exclusive_end_billing_period,
                        inclusive_start_billing_period: inclusive_start_billing_period,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::BillingConductor::CustomLineItem.CustomLineItemChargeDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitemchargedetails.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomLineItemChargeDetails {
        /// Property [`Flat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitemchargedetails.html#cfn-billingconductor-customlineitem-customlineitemchargedetails-flat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flat: Option<::Value<CustomLineItemFlatChargeDetails>>,
        /// Property [`LineItemFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitemchargedetails.html#cfn-billingconductor-customlineitem-customlineitemchargedetails-lineitemfilters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub line_item_filters: Option<::ValueList<LineItemFilter>>,
        /// Property [`Percentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitemchargedetails.html#cfn-billingconductor-customlineitem-customlineitemchargedetails-percentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub percentage: Option<::Value<CustomLineItemPercentageChargeDetails>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitemchargedetails.html#cfn-billingconductor-customlineitem-customlineitemchargedetails-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomLineItemChargeDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref flat) = self.flat {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Flat", flat)?;
            }
            if let Some(ref line_item_filters) = self.line_item_filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LineItemFilters", line_item_filters)?;
            }
            if let Some(ref percentage) = self.percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Percentage", percentage)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomLineItemChargeDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomLineItemChargeDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomLineItemChargeDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomLineItemChargeDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut flat: Option<::Value<CustomLineItemFlatChargeDetails>> = None;
                    let mut line_item_filters: Option<::ValueList<LineItemFilter>> = None;
                    let mut percentage: Option<::Value<CustomLineItemPercentageChargeDetails>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Flat" => {
                                flat = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LineItemFilters" => {
                                line_item_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Percentage" => {
                                percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomLineItemChargeDetails {
                        flat: flat,
                        line_item_filters: line_item_filters,
                        percentage: percentage,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::BillingConductor::CustomLineItem.CustomLineItemFlatChargeDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitemflatchargedetails.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomLineItemFlatChargeDetails {
        /// Property [`ChargeValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitemflatchargedetails.html#cfn-billingconductor-customlineitem-customlineitemflatchargedetails-chargevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub charge_value: ::Value<f64>,
    }

    impl ::codec::SerializeValue for CustomLineItemFlatChargeDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChargeValue", &self.charge_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomLineItemFlatChargeDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomLineItemFlatChargeDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomLineItemFlatChargeDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomLineItemFlatChargeDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut charge_value: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChargeValue" => {
                                charge_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomLineItemFlatChargeDetails {
                        charge_value: charge_value.ok_or(::serde::de::Error::missing_field("ChargeValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::BillingConductor::CustomLineItem.CustomLineItemPercentageChargeDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitempercentagechargedetails.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomLineItemPercentageChargeDetails {
        /// Property [`ChildAssociatedResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitempercentagechargedetails.html#cfn-billingconductor-customlineitem-customlineitempercentagechargedetails-childassociatedresources).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub child_associated_resources: Option<::ValueList<String>>,
        /// Property [`PercentageValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-customlineitempercentagechargedetails.html#cfn-billingconductor-customlineitem-customlineitempercentagechargedetails-percentagevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub percentage_value: ::Value<f64>,
    }

    impl ::codec::SerializeValue for CustomLineItemPercentageChargeDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref child_associated_resources) = self.child_associated_resources {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChildAssociatedResources", child_associated_resources)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PercentageValue", &self.percentage_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomLineItemPercentageChargeDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomLineItemPercentageChargeDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomLineItemPercentageChargeDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomLineItemPercentageChargeDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut child_associated_resources: Option<::ValueList<String>> = None;
                    let mut percentage_value: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChildAssociatedResources" => {
                                child_associated_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PercentageValue" => {
                                percentage_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomLineItemPercentageChargeDetails {
                        child_associated_resources: child_associated_resources,
                        percentage_value: percentage_value.ok_or(::serde::de::Error::missing_field("PercentageValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::BillingConductor::CustomLineItem.LineItemFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-lineitemfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct LineItemFilter {
        /// Property [`Attribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-lineitemfilter.html#cfn-billingconductor-customlineitem-lineitemfilter-attribute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute: ::Value<String>,
        /// Property [`MatchOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-lineitemfilter.html#cfn-billingconductor-customlineitem-lineitemfilter-matchoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_option: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-customlineitem-lineitemfilter.html#cfn-billingconductor-customlineitem-lineitemfilter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for LineItemFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attribute", &self.attribute)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchOption", &self.match_option)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LineItemFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LineItemFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LineItemFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LineItemFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute: Option<::Value<String>> = None;
                    let mut match_option: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attribute" => {
                                attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchOption" => {
                                match_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LineItemFilter {
                        attribute: attribute.ok_or(::serde::de::Error::missing_field("Attribute"))?,
                        match_option: match_option.ok_or(::serde::de::Error::missing_field("MatchOption"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod pricing_rule {
    //! Property types for the `PricingRule` resource.

    /// The [`AWS::BillingConductor::PricingRule.FreeTier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-pricingrule-freetier.html) property type.
    #[derive(Debug, Default)]
    pub struct FreeTier {
        /// Property [`Activated`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-pricingrule-freetier.html#cfn-billingconductor-pricingrule-freetier-activated).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub activated: ::Value<bool>,
    }

    impl ::codec::SerializeValue for FreeTier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Activated", &self.activated)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FreeTier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FreeTier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FreeTier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FreeTier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut activated: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Activated" => {
                                activated = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FreeTier {
                        activated: activated.ok_or(::serde::de::Error::missing_field("Activated"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::BillingConductor::PricingRule.Tiering`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-pricingrule-tiering.html) property type.
    #[derive(Debug, Default)]
    pub struct Tiering {
        /// Property [`FreeTier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-billingconductor-pricingrule-tiering.html#cfn-billingconductor-pricingrule-tiering-freetier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub free_tier: Option<::Value<FreeTier>>,
    }

    impl ::codec::SerializeValue for Tiering {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref free_tier) = self.free_tier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FreeTier", free_tier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Tiering {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Tiering, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Tiering;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Tiering")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut free_tier: Option<::Value<FreeTier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FreeTier" => {
                                free_tier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Tiering {
                        free_tier: free_tier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
