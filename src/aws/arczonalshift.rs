//! Types for the `ARCZonalShift` service.

/// The [`AWS::ARCZonalShift::ZonalAutoshiftConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-arczonalshift-zonalautoshiftconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct ZonalAutoshiftConfiguration {
    properties: ZonalAutoshiftConfigurationProperties
}

/// Properties for the `ZonalAutoshiftConfiguration` resource.
#[derive(Debug, Default)]
pub struct ZonalAutoshiftConfigurationProperties {
    /// Property [`PracticeRunConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-arczonalshift-zonalautoshiftconfiguration.html#cfn-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub practice_run_configuration: Option<::Value<self::zonal_autoshift_configuration::PracticeRunConfiguration>>,
    /// Property [`ResourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-arczonalshift-zonalautoshiftconfiguration.html#cfn-arczonalshift-zonalautoshiftconfiguration-resourceidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_identifier: Option<::Value<String>>,
    /// Property [`ZonalAutoshiftStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-arczonalshift-zonalautoshiftconfiguration.html#cfn-arczonalshift-zonalautoshiftconfiguration-zonalautoshiftstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub zonal_autoshift_status: Option<::Value<String>>,
}

impl ::serde::Serialize for ZonalAutoshiftConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref practice_run_configuration) = self.practice_run_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PracticeRunConfiguration", practice_run_configuration)?;
        }
        if let Some(ref resource_identifier) = self.resource_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceIdentifier", resource_identifier)?;
        }
        if let Some(ref zonal_autoshift_status) = self.zonal_autoshift_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ZonalAutoshiftStatus", zonal_autoshift_status)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ZonalAutoshiftConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ZonalAutoshiftConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ZonalAutoshiftConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ZonalAutoshiftConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut practice_run_configuration: Option<::Value<self::zonal_autoshift_configuration::PracticeRunConfiguration>> = None;
                let mut resource_identifier: Option<::Value<String>> = None;
                let mut zonal_autoshift_status: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PracticeRunConfiguration" => {
                            practice_run_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceIdentifier" => {
                            resource_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ZonalAutoshiftStatus" => {
                            zonal_autoshift_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ZonalAutoshiftConfigurationProperties {
                    practice_run_configuration: practice_run_configuration,
                    resource_identifier: resource_identifier,
                    zonal_autoshift_status: zonal_autoshift_status,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ZonalAutoshiftConfiguration {
    type Properties = ZonalAutoshiftConfigurationProperties;
    const TYPE: &'static str = "AWS::ARCZonalShift::ZonalAutoshiftConfiguration";
    fn properties(&self) -> &ZonalAutoshiftConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ZonalAutoshiftConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ZonalAutoshiftConfiguration {}

impl From<ZonalAutoshiftConfigurationProperties> for ZonalAutoshiftConfiguration {
    fn from(properties: ZonalAutoshiftConfigurationProperties) -> ZonalAutoshiftConfiguration {
        ZonalAutoshiftConfiguration { properties }
    }
}

pub mod zonal_autoshift_configuration {
    //! Property types for the `ZonalAutoshiftConfiguration` resource.

    /// The [`AWS::ARCZonalShift::ZonalAutoshiftConfiguration.ControlCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arczonalshift-zonalautoshiftconfiguration-controlcondition.html) property type.
    #[derive(Debug, Default)]
    pub struct ControlCondition {
        /// Property [`AlarmIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arczonalshift-zonalautoshiftconfiguration-controlcondition.html#cfn-arczonalshift-zonalautoshiftconfiguration-controlcondition-alarmidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarm_identifier: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arczonalshift-zonalautoshiftconfiguration-controlcondition.html#cfn-arczonalshift-zonalautoshiftconfiguration-controlcondition-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for ControlCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmIdentifier", &self.alarm_identifier)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ControlCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ControlCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ControlCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ControlCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_identifier: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmIdentifier" => {
                                alarm_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ControlCondition {
                        alarm_identifier: alarm_identifier.ok_or(::serde::de::Error::missing_field("AlarmIdentifier"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ARCZonalShift::ZonalAutoshiftConfiguration.PracticeRunConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct PracticeRunConfiguration {
        /// Property [`BlockedDates`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration.html#cfn-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration-blockeddates).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub blocked_dates: Option<::ValueList<String>>,
        /// Property [`BlockedWindows`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration.html#cfn-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration-blockedwindows).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub blocked_windows: Option<::ValueList<String>>,
        /// Property [`BlockingAlarms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration.html#cfn-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration-blockingalarms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub blocking_alarms: Option<::ValueList<ControlCondition>>,
        /// Property [`OutcomeAlarms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration.html#cfn-arczonalshift-zonalautoshiftconfiguration-practicerunconfiguration-outcomealarms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outcome_alarms: ::ValueList<ControlCondition>,
    }

    impl ::codec::SerializeValue for PracticeRunConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref blocked_dates) = self.blocked_dates {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockedDates", blocked_dates)?;
            }
            if let Some(ref blocked_windows) = self.blocked_windows {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockedWindows", blocked_windows)?;
            }
            if let Some(ref blocking_alarms) = self.blocking_alarms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockingAlarms", blocking_alarms)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutcomeAlarms", &self.outcome_alarms)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PracticeRunConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PracticeRunConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PracticeRunConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PracticeRunConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut blocked_dates: Option<::ValueList<String>> = None;
                    let mut blocked_windows: Option<::ValueList<String>> = None;
                    let mut blocking_alarms: Option<::ValueList<ControlCondition>> = None;
                    let mut outcome_alarms: Option<::ValueList<ControlCondition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockedDates" => {
                                blocked_dates = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BlockedWindows" => {
                                blocked_windows = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BlockingAlarms" => {
                                blocking_alarms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutcomeAlarms" => {
                                outcome_alarms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PracticeRunConfiguration {
                        blocked_dates: blocked_dates,
                        blocked_windows: blocked_windows,
                        blocking_alarms: blocking_alarms,
                        outcome_alarms: outcome_alarms.ok_or(::serde::de::Error::missing_field("OutcomeAlarms"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
