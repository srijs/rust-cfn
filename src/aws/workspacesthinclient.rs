//! Types for the `WorkSpacesThinClient` service.

/// The [`AWS::WorkSpacesThinClient::Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html) resource type.
#[derive(Debug, Default)]
pub struct Environment {
    properties: EnvironmentProperties
}

/// Properties for the `Environment` resource.
#[derive(Debug, Default)]
pub struct EnvironmentProperties {
    /// Property [`DesiredSoftwareSetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html#cfn-workspacesthinclient-environment-desiredsoftwaresetid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub desired_software_set_id: Option<::Value<String>>,
    /// Property [`DesktopArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html#cfn-workspacesthinclient-environment-desktoparn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub desktop_arn: ::Value<String>,
    /// Property [`DesktopEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html#cfn-workspacesthinclient-environment-desktopendpoint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub desktop_endpoint: Option<::Value<String>>,
    /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html#cfn-workspacesthinclient-environment-kmskeyarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_arn: Option<::Value<String>>,
    /// Property [`MaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html#cfn-workspacesthinclient-environment-maintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maintenance_window: Option<::Value<self::environment::MaintenanceWindow>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html#cfn-workspacesthinclient-environment-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`SoftwareSetUpdateMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html#cfn-workspacesthinclient-environment-softwaresetupdatemode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub software_set_update_mode: Option<::Value<String>>,
    /// Property [`SoftwareSetUpdateSchedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html#cfn-workspacesthinclient-environment-softwaresetupdateschedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub software_set_update_schedule: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html#cfn-workspacesthinclient-environment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EnvironmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref desired_software_set_id) = self.desired_software_set_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredSoftwareSetId", desired_software_set_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesktopArn", &self.desktop_arn)?;
        if let Some(ref desktop_endpoint) = self.desktop_endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesktopEndpoint", desktop_endpoint)?;
        }
        if let Some(ref kms_key_arn) = self.kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
        }
        if let Some(ref maintenance_window) = self.maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindow", maintenance_window)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref software_set_update_mode) = self.software_set_update_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SoftwareSetUpdateMode", software_set_update_mode)?;
        }
        if let Some(ref software_set_update_schedule) = self.software_set_update_schedule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SoftwareSetUpdateSchedule", software_set_update_schedule)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnvironmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnvironmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnvironmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut desired_software_set_id: Option<::Value<String>> = None;
                let mut desktop_arn: Option<::Value<String>> = None;
                let mut desktop_endpoint: Option<::Value<String>> = None;
                let mut kms_key_arn: Option<::Value<String>> = None;
                let mut maintenance_window: Option<::Value<self::environment::MaintenanceWindow>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut software_set_update_mode: Option<::Value<String>> = None;
                let mut software_set_update_schedule: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DesiredSoftwareSetId" => {
                            desired_software_set_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DesktopArn" => {
                            desktop_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DesktopEndpoint" => {
                            desktop_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyArn" => {
                            kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaintenanceWindow" => {
                            maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SoftwareSetUpdateMode" => {
                            software_set_update_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SoftwareSetUpdateSchedule" => {
                            software_set_update_schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnvironmentProperties {
                    desired_software_set_id: desired_software_set_id,
                    desktop_arn: desktop_arn.ok_or(::serde::de::Error::missing_field("DesktopArn"))?,
                    desktop_endpoint: desktop_endpoint,
                    kms_key_arn: kms_key_arn,
                    maintenance_window: maintenance_window,
                    name: name,
                    software_set_update_mode: software_set_update_mode,
                    software_set_update_schedule: software_set_update_schedule,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Environment {
    type Properties = EnvironmentProperties;
    const TYPE: &'static str = "AWS::WorkSpacesThinClient::Environment";
    fn properties(&self) -> &EnvironmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Environment {}

impl From<EnvironmentProperties> for Environment {
    fn from(properties: EnvironmentProperties) -> Environment {
        Environment { properties }
    }
}

pub mod environment {
    //! Property types for the `Environment` resource.

    /// The [`AWS::WorkSpacesThinClient::Environment.MaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesthinclient-environment-maintenancewindow.html) property type.
    #[derive(Debug, Default)]
    pub struct MaintenanceWindow {
        /// Property [`ApplyTimeOf`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesthinclient-environment-maintenancewindow.html#cfn-workspacesthinclient-environment-maintenancewindow-applytimeof).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub apply_time_of: Option<::Value<String>>,
        /// Property [`DaysOfTheWeek`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesthinclient-environment-maintenancewindow.html#cfn-workspacesthinclient-environment-maintenancewindow-daysoftheweek).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub days_of_the_week: Option<::ValueList<String>>,
        /// Property [`EndTimeHour`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesthinclient-environment-maintenancewindow.html#cfn-workspacesthinclient-environment-maintenancewindow-endtimehour).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_time_hour: Option<::Value<u32>>,
        /// Property [`EndTimeMinute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesthinclient-environment-maintenancewindow.html#cfn-workspacesthinclient-environment-maintenancewindow-endtimeminute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_time_minute: Option<::Value<u32>>,
        /// Property [`StartTimeHour`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesthinclient-environment-maintenancewindow.html#cfn-workspacesthinclient-environment-maintenancewindow-starttimehour).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time_hour: Option<::Value<u32>>,
        /// Property [`StartTimeMinute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesthinclient-environment-maintenancewindow.html#cfn-workspacesthinclient-environment-maintenancewindow-starttimeminute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time_minute: Option<::Value<u32>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesthinclient-environment-maintenancewindow.html#cfn-workspacesthinclient-environment-maintenancewindow-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for MaintenanceWindow {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref apply_time_of) = self.apply_time_of {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplyTimeOf", apply_time_of)?;
            }
            if let Some(ref days_of_the_week) = self.days_of_the_week {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DaysOfTheWeek", days_of_the_week)?;
            }
            if let Some(ref end_time_hour) = self.end_time_hour {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTimeHour", end_time_hour)?;
            }
            if let Some(ref end_time_minute) = self.end_time_minute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTimeMinute", end_time_minute)?;
            }
            if let Some(ref start_time_hour) = self.start_time_hour {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTimeHour", start_time_hour)?;
            }
            if let Some(ref start_time_minute) = self.start_time_minute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTimeMinute", start_time_minute)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindow {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindow, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindow;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindow")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut apply_time_of: Option<::Value<String>> = None;
                    let mut days_of_the_week: Option<::ValueList<String>> = None;
                    let mut end_time_hour: Option<::Value<u32>> = None;
                    let mut end_time_minute: Option<::Value<u32>> = None;
                    let mut start_time_hour: Option<::Value<u32>> = None;
                    let mut start_time_minute: Option<::Value<u32>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplyTimeOf" => {
                                apply_time_of = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DaysOfTheWeek" => {
                                days_of_the_week = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndTimeHour" => {
                                end_time_hour = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndTimeMinute" => {
                                end_time_minute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTimeHour" => {
                                start_time_hour = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTimeMinute" => {
                                start_time_minute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindow {
                        apply_time_of: apply_time_of,
                        days_of_the_week: days_of_the_week,
                        end_time_hour: end_time_hour,
                        end_time_minute: end_time_minute,
                        start_time_hour: start_time_hour,
                        start_time_minute: start_time_minute,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
