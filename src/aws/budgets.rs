//! Types for the `Budgets` service.

/// The [`AWS::Budgets::Budget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budget.html) resource type.
#[derive(Debug, Default)]
pub struct Budget {
    properties: BudgetProperties
}

/// Properties for the `Budget` resource.
#[derive(Debug, Default)]
pub struct BudgetProperties {
    /// Property [`Budget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budget.html#cfn-budgets-budget-budget).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub budget: ::Value<self::budget::BudgetData>,
    /// Property [`NotificationsWithSubscribers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budget.html#cfn-budgets-budget-notificationswithsubscribers).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub notifications_with_subscribers: Option<::ValueList<self::budget::NotificationWithSubscribers>>,
}

impl ::serde::Serialize for BudgetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Budget", &self.budget)?;
        if let Some(ref notifications_with_subscribers) = self.notifications_with_subscribers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationsWithSubscribers", notifications_with_subscribers)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BudgetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BudgetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BudgetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BudgetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut budget: Option<::Value<self::budget::BudgetData>> = None;
                let mut notifications_with_subscribers: Option<::ValueList<self::budget::NotificationWithSubscribers>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Budget" => {
                            budget = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationsWithSubscribers" => {
                            notifications_with_subscribers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BudgetProperties {
                    budget: budget.ok_or(::serde::de::Error::missing_field("Budget"))?,
                    notifications_with_subscribers: notifications_with_subscribers,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Budget {
    type Properties = BudgetProperties;
    const TYPE: &'static str = "AWS::Budgets::Budget";
    fn properties(&self) -> &BudgetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BudgetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Budget {}

impl From<BudgetProperties> for Budget {
    fn from(properties: BudgetProperties) -> Budget {
        Budget { properties }
    }
}

/// The [`AWS::Budgets::BudgetsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budgetsaction.html) resource type.
#[derive(Debug, Default)]
pub struct BudgetsAction {
    properties: BudgetsActionProperties
}

/// Properties for the `BudgetsAction` resource.
#[derive(Debug, Default)]
pub struct BudgetsActionProperties {
    /// Property [`ActionThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budgetsaction.html#cfn-budgets-budgetsaction-actionthreshold).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub action_threshold: ::Value<self::budgets_action::ActionThreshold>,
    /// Property [`ActionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budgetsaction.html#cfn-budgets-budgetsaction-actiontype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub action_type: ::Value<String>,
    /// Property [`ApprovalModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budgetsaction.html#cfn-budgets-budgetsaction-approvalmodel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub approval_model: Option<::Value<String>>,
    /// Property [`BudgetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budgetsaction.html#cfn-budgets-budgetsaction-budgetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub budget_name: ::Value<String>,
    /// Property [`Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budgetsaction.html#cfn-budgets-budgetsaction-definition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub definition: ::Value<self::budgets_action::Definition>,
    /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budgetsaction.html#cfn-budgets-budgetsaction-executionrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_role_arn: ::Value<String>,
    /// Property [`NotificationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budgetsaction.html#cfn-budgets-budgetsaction-notificationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_type: ::Value<String>,
    /// Property [`Subscribers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-budgets-budgetsaction.html#cfn-budgets-budgetsaction-subscribers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subscribers: ::ValueList<self::budgets_action::Subscriber>,
}

impl ::serde::Serialize for BudgetsActionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionThreshold", &self.action_threshold)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionType", &self.action_type)?;
        if let Some(ref approval_model) = self.approval_model {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovalModel", approval_model)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BudgetName", &self.budget_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Definition", &self.definition)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", &self.execution_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationType", &self.notification_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subscribers", &self.subscribers)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BudgetsActionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BudgetsActionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BudgetsActionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BudgetsActionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action_threshold: Option<::Value<self::budgets_action::ActionThreshold>> = None;
                let mut action_type: Option<::Value<String>> = None;
                let mut approval_model: Option<::Value<String>> = None;
                let mut budget_name: Option<::Value<String>> = None;
                let mut definition: Option<::Value<self::budgets_action::Definition>> = None;
                let mut execution_role_arn: Option<::Value<String>> = None;
                let mut notification_type: Option<::Value<String>> = None;
                let mut subscribers: Option<::ValueList<self::budgets_action::Subscriber>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActionThreshold" => {
                            action_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ActionType" => {
                            action_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApprovalModel" => {
                            approval_model = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BudgetName" => {
                            budget_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Definition" => {
                            definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionRoleArn" => {
                            execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationType" => {
                            notification_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subscribers" => {
                            subscribers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BudgetsActionProperties {
                    action_threshold: action_threshold.ok_or(::serde::de::Error::missing_field("ActionThreshold"))?,
                    action_type: action_type.ok_or(::serde::de::Error::missing_field("ActionType"))?,
                    approval_model: approval_model,
                    budget_name: budget_name.ok_or(::serde::de::Error::missing_field("BudgetName"))?,
                    definition: definition.ok_or(::serde::de::Error::missing_field("Definition"))?,
                    execution_role_arn: execution_role_arn.ok_or(::serde::de::Error::missing_field("ExecutionRoleArn"))?,
                    notification_type: notification_type.ok_or(::serde::de::Error::missing_field("NotificationType"))?,
                    subscribers: subscribers.ok_or(::serde::de::Error::missing_field("Subscribers"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BudgetsAction {
    type Properties = BudgetsActionProperties;
    const TYPE: &'static str = "AWS::Budgets::BudgetsAction";
    fn properties(&self) -> &BudgetsActionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BudgetsActionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BudgetsAction {}

impl From<BudgetsActionProperties> for BudgetsAction {
    fn from(properties: BudgetsActionProperties) -> BudgetsAction {
        BudgetsAction { properties }
    }
}

pub mod budget {
    //! Property types for the `Budget` resource.

    /// The [`AWS::Budgets::Budget.BudgetData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-budgetdata.html) property type.
    #[derive(Debug, Default)]
    pub struct BudgetData {
        /// Property [`BudgetLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-budgetdata.html#cfn-budgets-budget-budgetdata-budgetlimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub budget_limit: Option<::Value<Spend>>,
        /// Property [`BudgetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-budgetdata.html#cfn-budgets-budget-budgetdata-budgetname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub budget_name: Option<::Value<String>>,
        /// Property [`BudgetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-budgetdata.html#cfn-budgets-budget-budgetdata-budgettype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub budget_type: ::Value<String>,
        /// Property [`CostFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-budgetdata.html#cfn-budgets-budget-budgetdata-costfilters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cost_filters: Option<::Value<::json::Value>>,
        /// Property [`CostTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-budgetdata.html#cfn-budgets-budget-budgetdata-costtypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cost_types: Option<::Value<CostTypes>>,
        /// Property [`PlannedBudgetLimits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-budgetdata.html#cfn-budgets-budget-budgetdata-plannedbudgetlimits).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub planned_budget_limits: Option<::Value<::json::Value>>,
        /// Property [`TimePeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-budgetdata.html#cfn-budgets-budget-budgetdata-timeperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_period: Option<::Value<TimePeriod>>,
        /// Property [`TimeUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-budgetdata.html#cfn-budgets-budget-budgetdata-timeunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_unit: ::Value<String>,
    }

    impl ::codec::SerializeValue for BudgetData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref budget_limit) = self.budget_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BudgetLimit", budget_limit)?;
            }
            if let Some(ref budget_name) = self.budget_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BudgetName", budget_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BudgetType", &self.budget_type)?;
            if let Some(ref cost_filters) = self.cost_filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CostFilters", cost_filters)?;
            }
            if let Some(ref cost_types) = self.cost_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CostTypes", cost_types)?;
            }
            if let Some(ref planned_budget_limits) = self.planned_budget_limits {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlannedBudgetLimits", planned_budget_limits)?;
            }
            if let Some(ref time_period) = self.time_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimePeriod", time_period)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeUnit", &self.time_unit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BudgetData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BudgetData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BudgetData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BudgetData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut budget_limit: Option<::Value<Spend>> = None;
                    let mut budget_name: Option<::Value<String>> = None;
                    let mut budget_type: Option<::Value<String>> = None;
                    let mut cost_filters: Option<::Value<::json::Value>> = None;
                    let mut cost_types: Option<::Value<CostTypes>> = None;
                    let mut planned_budget_limits: Option<::Value<::json::Value>> = None;
                    let mut time_period: Option<::Value<TimePeriod>> = None;
                    let mut time_unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BudgetLimit" => {
                                budget_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BudgetName" => {
                                budget_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BudgetType" => {
                                budget_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CostFilters" => {
                                cost_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CostTypes" => {
                                cost_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlannedBudgetLimits" => {
                                planned_budget_limits = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimePeriod" => {
                                time_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeUnit" => {
                                time_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BudgetData {
                        budget_limit: budget_limit,
                        budget_name: budget_name,
                        budget_type: budget_type.ok_or(::serde::de::Error::missing_field("BudgetType"))?,
                        cost_filters: cost_filters,
                        cost_types: cost_types,
                        planned_budget_limits: planned_budget_limits,
                        time_period: time_period,
                        time_unit: time_unit.ok_or(::serde::de::Error::missing_field("TimeUnit"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::Budget.CostTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html) property type.
    #[derive(Debug, Default)]
    pub struct CostTypes {
        /// Property [`IncludeCredit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-includecredit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_credit: Option<::Value<bool>>,
        /// Property [`IncludeDiscount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-includediscount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_discount: Option<::Value<bool>>,
        /// Property [`IncludeOtherSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-includeothersubscription).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_other_subscription: Option<::Value<bool>>,
        /// Property [`IncludeRecurring`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-includerecurring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_recurring: Option<::Value<bool>>,
        /// Property [`IncludeRefund`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-includerefund).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_refund: Option<::Value<bool>>,
        /// Property [`IncludeSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-includesubscription).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_subscription: Option<::Value<bool>>,
        /// Property [`IncludeSupport`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-includesupport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_support: Option<::Value<bool>>,
        /// Property [`IncludeTax`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-includetax).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_tax: Option<::Value<bool>>,
        /// Property [`IncludeUpfront`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-includeupfront).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_upfront: Option<::Value<bool>>,
        /// Property [`UseAmortized`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-useamortized).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_amortized: Option<::Value<bool>>,
        /// Property [`UseBlended`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-costtypes.html#cfn-budgets-budget-costtypes-useblended).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_blended: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for CostTypes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref include_credit) = self.include_credit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeCredit", include_credit)?;
            }
            if let Some(ref include_discount) = self.include_discount {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeDiscount", include_discount)?;
            }
            if let Some(ref include_other_subscription) = self.include_other_subscription {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeOtherSubscription", include_other_subscription)?;
            }
            if let Some(ref include_recurring) = self.include_recurring {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeRecurring", include_recurring)?;
            }
            if let Some(ref include_refund) = self.include_refund {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeRefund", include_refund)?;
            }
            if let Some(ref include_subscription) = self.include_subscription {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeSubscription", include_subscription)?;
            }
            if let Some(ref include_support) = self.include_support {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeSupport", include_support)?;
            }
            if let Some(ref include_tax) = self.include_tax {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeTax", include_tax)?;
            }
            if let Some(ref include_upfront) = self.include_upfront {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeUpfront", include_upfront)?;
            }
            if let Some(ref use_amortized) = self.use_amortized {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseAmortized", use_amortized)?;
            }
            if let Some(ref use_blended) = self.use_blended {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseBlended", use_blended)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CostTypes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CostTypes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CostTypes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CostTypes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut include_credit: Option<::Value<bool>> = None;
                    let mut include_discount: Option<::Value<bool>> = None;
                    let mut include_other_subscription: Option<::Value<bool>> = None;
                    let mut include_recurring: Option<::Value<bool>> = None;
                    let mut include_refund: Option<::Value<bool>> = None;
                    let mut include_subscription: Option<::Value<bool>> = None;
                    let mut include_support: Option<::Value<bool>> = None;
                    let mut include_tax: Option<::Value<bool>> = None;
                    let mut include_upfront: Option<::Value<bool>> = None;
                    let mut use_amortized: Option<::Value<bool>> = None;
                    let mut use_blended: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IncludeCredit" => {
                                include_credit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeDiscount" => {
                                include_discount = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeOtherSubscription" => {
                                include_other_subscription = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeRecurring" => {
                                include_recurring = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeRefund" => {
                                include_refund = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeSubscription" => {
                                include_subscription = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeSupport" => {
                                include_support = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeTax" => {
                                include_tax = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeUpfront" => {
                                include_upfront = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseAmortized" => {
                                use_amortized = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseBlended" => {
                                use_blended = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CostTypes {
                        include_credit: include_credit,
                        include_discount: include_discount,
                        include_other_subscription: include_other_subscription,
                        include_recurring: include_recurring,
                        include_refund: include_refund,
                        include_subscription: include_subscription,
                        include_support: include_support,
                        include_tax: include_tax,
                        include_upfront: include_upfront,
                        use_amortized: use_amortized,
                        use_blended: use_blended,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::Budget.Notification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-notification.html) property type.
    #[derive(Debug, Default)]
    pub struct Notification {
        /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-notification.html#cfn-budgets-budget-notification-comparisonoperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison_operator: ::Value<String>,
        /// Property [`NotificationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-notification.html#cfn-budgets-budget-notification-notificationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_type: ::Value<String>,
        /// Property [`Threshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-notification.html#cfn-budgets-budget-notification-threshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub threshold: ::Value<f64>,
        /// Property [`ThresholdType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-notification.html#cfn-budgets-budget-notification-thresholdtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub threshold_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Notification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", &self.comparison_operator)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationType", &self.notification_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Threshold", &self.threshold)?;
            if let Some(ref threshold_type) = self.threshold_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThresholdType", threshold_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Notification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Notification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Notification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Notification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison_operator: Option<::Value<String>> = None;
                    let mut notification_type: Option<::Value<String>> = None;
                    let mut threshold: Option<::Value<f64>> = None;
                    let mut threshold_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationType" => {
                                notification_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Threshold" => {
                                threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThresholdType" => {
                                threshold_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Notification {
                        comparison_operator: comparison_operator.ok_or(::serde::de::Error::missing_field("ComparisonOperator"))?,
                        notification_type: notification_type.ok_or(::serde::de::Error::missing_field("NotificationType"))?,
                        threshold: threshold.ok_or(::serde::de::Error::missing_field("Threshold"))?,
                        threshold_type: threshold_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::Budget.NotificationWithSubscribers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-notificationwithsubscribers.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationWithSubscribers {
        /// Property [`Notification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-notificationwithsubscribers.html#cfn-budgets-budget-notificationwithsubscribers-notification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification: ::Value<Notification>,
        /// Property [`Subscribers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-notificationwithsubscribers.html#cfn-budgets-budget-notificationwithsubscribers-subscribers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subscribers: ::ValueList<Subscriber>,
    }

    impl ::codec::SerializeValue for NotificationWithSubscribers {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Notification", &self.notification)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subscribers", &self.subscribers)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationWithSubscribers {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationWithSubscribers, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationWithSubscribers;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationWithSubscribers")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut notification: Option<::Value<Notification>> = None;
                    let mut subscribers: Option<::ValueList<Subscriber>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Notification" => {
                                notification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subscribers" => {
                                subscribers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationWithSubscribers {
                        notification: notification.ok_or(::serde::de::Error::missing_field("Notification"))?,
                        subscribers: subscribers.ok_or(::serde::de::Error::missing_field("Subscribers"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::Budget.Spend`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-spend.html) property type.
    #[derive(Debug, Default)]
    pub struct Spend {
        /// Property [`Amount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-spend.html#cfn-budgets-budget-spend-amount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub amount: ::Value<f64>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-spend.html#cfn-budgets-budget-spend-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: ::Value<String>,
    }

    impl ::codec::SerializeValue for Spend {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Amount", &self.amount)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Spend {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Spend, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Spend;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Spend")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut amount: Option<::Value<f64>> = None;
                    let mut unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Amount" => {
                                amount = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Spend {
                        amount: amount.ok_or(::serde::de::Error::missing_field("Amount"))?,
                        unit: unit.ok_or(::serde::de::Error::missing_field("Unit"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::Budget.Subscriber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-subscriber.html) property type.
    #[derive(Debug, Default)]
    pub struct Subscriber {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-subscriber.html#cfn-budgets-budget-subscriber-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: ::Value<String>,
        /// Property [`SubscriptionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-subscriber.html#cfn-budgets-budget-subscriber-subscriptiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subscription_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Subscriber {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", &self.address)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubscriptionType", &self.subscription_type)?;
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
                    let mut subscription_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubscriptionType" => {
                                subscription_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Subscriber {
                        address: address.ok_or(::serde::de::Error::missing_field("Address"))?,
                        subscription_type: subscription_type.ok_or(::serde::de::Error::missing_field("SubscriptionType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::Budget.TimePeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-timeperiod.html) property type.
    #[derive(Debug, Default)]
    pub struct TimePeriod {
        /// Property [`End`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-timeperiod.html#cfn-budgets-budget-timeperiod-end).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end: Option<::Value<String>>,
        /// Property [`Start`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budget-timeperiod.html#cfn-budgets-budget-timeperiod-start).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TimePeriod {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref end) = self.end {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "End", end)?;
            }
            if let Some(ref start) = self.start {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Start", start)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimePeriod {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimePeriod, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimePeriod;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimePeriod")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end: Option<::Value<String>> = None;
                    let mut start: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "End" => {
                                end = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Start" => {
                                start = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimePeriod {
                        end: end,
                        start: start,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod budgets_action {
    //! Property types for the `BudgetsAction` resource.

    /// The [`AWS::Budgets::BudgetsAction.ActionThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-actionthreshold.html) property type.
    #[derive(Debug, Default)]
    pub struct ActionThreshold {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-actionthreshold.html#cfn-budgets-budgetsaction-actionthreshold-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-actionthreshold.html#cfn-budgets-budgetsaction-actionthreshold-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<f64>,
    }

    impl ::codec::SerializeValue for ActionThreshold {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActionThreshold {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActionThreshold, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActionThreshold;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActionThreshold")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<f64>> = None;

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

                    Ok(ActionThreshold {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::BudgetsAction.Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-definition.html) property type.
    #[derive(Debug, Default)]
    pub struct Definition {
        /// Property [`IamActionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-definition.html#cfn-budgets-budgetsaction-definition-iamactiondefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_action_definition: Option<::Value<IamActionDefinition>>,
        /// Property [`ScpActionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-definition.html#cfn-budgets-budgetsaction-definition-scpactiondefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scp_action_definition: Option<::Value<ScpActionDefinition>>,
        /// Property [`SsmActionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-definition.html#cfn-budgets-budgetsaction-definition-ssmactiondefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssm_action_definition: Option<::Value<SsmActionDefinition>>,
    }

    impl ::codec::SerializeValue for Definition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iam_action_definition) = self.iam_action_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamActionDefinition", iam_action_definition)?;
            }
            if let Some(ref scp_action_definition) = self.scp_action_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScpActionDefinition", scp_action_definition)?;
            }
            if let Some(ref ssm_action_definition) = self.ssm_action_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SsmActionDefinition", ssm_action_definition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Definition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Definition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Definition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Definition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iam_action_definition: Option<::Value<IamActionDefinition>> = None;
                    let mut scp_action_definition: Option<::Value<ScpActionDefinition>> = None;
                    let mut ssm_action_definition: Option<::Value<SsmActionDefinition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IamActionDefinition" => {
                                iam_action_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScpActionDefinition" => {
                                scp_action_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SsmActionDefinition" => {
                                ssm_action_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Definition {
                        iam_action_definition: iam_action_definition,
                        scp_action_definition: scp_action_definition,
                        ssm_action_definition: ssm_action_definition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::BudgetsAction.IamActionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-iamactiondefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct IamActionDefinition {
        /// Property [`Groups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-iamactiondefinition.html#cfn-budgets-budgetsaction-iamactiondefinition-groups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub groups: Option<::ValueList<String>>,
        /// Property [`PolicyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-iamactiondefinition.html#cfn-budgets-budgetsaction-iamactiondefinition-policyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_arn: ::Value<String>,
        /// Property [`Roles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-iamactiondefinition.html#cfn-budgets-budgetsaction-iamactiondefinition-roles).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub roles: Option<::ValueList<String>>,
        /// Property [`Users`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-iamactiondefinition.html#cfn-budgets-budgetsaction-iamactiondefinition-users).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub users: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for IamActionDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref groups) = self.groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", groups)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyArn", &self.policy_arn)?;
            if let Some(ref roles) = self.roles {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Roles", roles)?;
            }
            if let Some(ref users) = self.users {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Users", users)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IamActionDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IamActionDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IamActionDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IamActionDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut groups: Option<::ValueList<String>> = None;
                    let mut policy_arn: Option<::Value<String>> = None;
                    let mut roles: Option<::ValueList<String>> = None;
                    let mut users: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Groups" => {
                                groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyArn" => {
                                policy_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Roles" => {
                                roles = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Users" => {
                                users = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IamActionDefinition {
                        groups: groups,
                        policy_arn: policy_arn.ok_or(::serde::de::Error::missing_field("PolicyArn"))?,
                        roles: roles,
                        users: users,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::BudgetsAction.ScpActionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-scpactiondefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct ScpActionDefinition {
        /// Property [`PolicyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-scpactiondefinition.html#cfn-budgets-budgetsaction-scpactiondefinition-policyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_id: ::Value<String>,
        /// Property [`TargetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-scpactiondefinition.html#cfn-budgets-budgetsaction-scpactiondefinition-targetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ScpActionDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyId", &self.policy_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetIds", &self.target_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScpActionDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScpActionDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScpActionDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScpActionDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_id: Option<::Value<String>> = None;
                    let mut target_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyId" => {
                                policy_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetIds" => {
                                target_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScpActionDefinition {
                        policy_id: policy_id.ok_or(::serde::de::Error::missing_field("PolicyId"))?,
                        target_ids: target_ids.ok_or(::serde::de::Error::missing_field("TargetIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::BudgetsAction.SsmActionDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-ssmactiondefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct SsmActionDefinition {
        /// Property [`InstanceIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-ssmactiondefinition.html#cfn-budgets-budgetsaction-ssmactiondefinition-instanceids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_ids: ::ValueList<String>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-ssmactiondefinition.html#cfn-budgets-budgetsaction-ssmactiondefinition-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: ::Value<String>,
        /// Property [`Subtype`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-ssmactiondefinition.html#cfn-budgets-budgetsaction-ssmactiondefinition-subtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subtype: ::Value<String>,
    }

    impl ::codec::SerializeValue for SsmActionDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceIds", &self.instance_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subtype", &self.subtype)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SsmActionDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SsmActionDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SsmActionDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SsmActionDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_ids: Option<::ValueList<String>> = None;
                    let mut region: Option<::Value<String>> = None;
                    let mut subtype: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceIds" => {
                                instance_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subtype" => {
                                subtype = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SsmActionDefinition {
                        instance_ids: instance_ids.ok_or(::serde::de::Error::missing_field("InstanceIds"))?,
                        region: region.ok_or(::serde::de::Error::missing_field("Region"))?,
                        subtype: subtype.ok_or(::serde::de::Error::missing_field("Subtype"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Budgets::BudgetsAction.Subscriber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-subscriber.html) property type.
    #[derive(Debug, Default)]
    pub struct Subscriber {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-subscriber.html#cfn-budgets-budgetsaction-subscriber-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-budgets-budgetsaction-subscriber.html#cfn-budgets-budgetsaction-subscriber-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Subscriber {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", &self.address)?;
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
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Subscriber {
                        address: address.ok_or(::serde::de::Error::missing_field("Address"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
