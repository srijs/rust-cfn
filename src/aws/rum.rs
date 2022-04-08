//! Types for the `RUM` service.

/// The [`AWS::RUM::AppMonitor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rum-appmonitor.html) resource type.
#[derive(Debug, Default)]
pub struct AppMonitor {
    properties: AppMonitorProperties
}

/// Properties for the `AppMonitor` resource.
#[derive(Debug, Default)]
pub struct AppMonitorProperties {
    /// Property [`AppMonitorConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rum-appmonitor.html#cfn-rum-appmonitor-appmonitorconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_monitor_configuration: Option<::Value<self::app_monitor::AppMonitorConfiguration>>,
    /// Property [`CwLogEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rum-appmonitor.html#cfn-rum-appmonitor-cwlogenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cw_log_enabled: Option<::Value<bool>>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rum-appmonitor.html#cfn-rum-appmonitor-domain).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rum-appmonitor.html#cfn-rum-appmonitor-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rum-appmonitor.html#cfn-rum-appmonitor-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AppMonitorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref app_monitor_configuration) = self.app_monitor_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppMonitorConfiguration", app_monitor_configuration)?;
        }
        if let Some(ref cw_log_enabled) = self.cw_log_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CwLogEnabled", cw_log_enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", &self.domain)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AppMonitorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AppMonitorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AppMonitorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AppMonitorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_monitor_configuration: Option<::Value<self::app_monitor::AppMonitorConfiguration>> = None;
                let mut cw_log_enabled: Option<::Value<bool>> = None;
                let mut domain: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppMonitorConfiguration" => {
                            app_monitor_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CwLogEnabled" => {
                            cw_log_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(AppMonitorProperties {
                    app_monitor_configuration: app_monitor_configuration,
                    cw_log_enabled: cw_log_enabled,
                    domain: domain.ok_or(::serde::de::Error::missing_field("Domain"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AppMonitor {
    type Properties = AppMonitorProperties;
    const TYPE: &'static str = "AWS::RUM::AppMonitor";
    fn properties(&self) -> &AppMonitorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AppMonitorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AppMonitor {}

impl From<AppMonitorProperties> for AppMonitor {
    fn from(properties: AppMonitorProperties) -> AppMonitor {
        AppMonitor { properties }
    }
}

pub mod app_monitor {
    //! Property types for the `AppMonitor` resource.

    /// The [`AWS::RUM::AppMonitor.AppMonitorConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AppMonitorConfiguration {
        /// Property [`AllowCookies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html#cfn-rum-appmonitor-appmonitorconfiguration-allowcookies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_cookies: Option<::Value<bool>>,
        /// Property [`EnableXRay`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html#cfn-rum-appmonitor-appmonitorconfiguration-enablexray).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_x_ray: Option<::Value<bool>>,
        /// Property [`ExcludedPages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html#cfn-rum-appmonitor-appmonitorconfiguration-excludedpages).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded_pages: Option<::ValueList<String>>,
        /// Property [`FavoritePages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html#cfn-rum-appmonitor-appmonitorconfiguration-favoritepages).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub favorite_pages: Option<::ValueList<String>>,
        /// Property [`GuestRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html#cfn-rum-appmonitor-appmonitorconfiguration-guestrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub guest_role_arn: Option<::Value<String>>,
        /// Property [`IdentityPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html#cfn-rum-appmonitor-appmonitorconfiguration-identitypoolid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identity_pool_id: Option<::Value<String>>,
        /// Property [`IncludedPages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html#cfn-rum-appmonitor-appmonitorconfiguration-includedpages).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub included_pages: Option<::ValueList<String>>,
        /// Property [`SessionSampleRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html#cfn-rum-appmonitor-appmonitorconfiguration-sessionsamplerate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_sample_rate: Option<::Value<f64>>,
        /// Property [`Telemetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html#cfn-rum-appmonitor-appmonitorconfiguration-telemetries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub telemetries: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AppMonitorConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_cookies) = self.allow_cookies {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowCookies", allow_cookies)?;
            }
            if let Some(ref enable_x_ray) = self.enable_x_ray {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableXRay", enable_x_ray)?;
            }
            if let Some(ref excluded_pages) = self.excluded_pages {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedPages", excluded_pages)?;
            }
            if let Some(ref favorite_pages) = self.favorite_pages {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FavoritePages", favorite_pages)?;
            }
            if let Some(ref guest_role_arn) = self.guest_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GuestRoleArn", guest_role_arn)?;
            }
            if let Some(ref identity_pool_id) = self.identity_pool_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityPoolId", identity_pool_id)?;
            }
            if let Some(ref included_pages) = self.included_pages {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludedPages", included_pages)?;
            }
            if let Some(ref session_sample_rate) = self.session_sample_rate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionSampleRate", session_sample_rate)?;
            }
            if let Some(ref telemetries) = self.telemetries {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Telemetries", telemetries)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AppMonitorConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AppMonitorConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AppMonitorConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AppMonitorConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_cookies: Option<::Value<bool>> = None;
                    let mut enable_x_ray: Option<::Value<bool>> = None;
                    let mut excluded_pages: Option<::ValueList<String>> = None;
                    let mut favorite_pages: Option<::ValueList<String>> = None;
                    let mut guest_role_arn: Option<::Value<String>> = None;
                    let mut identity_pool_id: Option<::Value<String>> = None;
                    let mut included_pages: Option<::ValueList<String>> = None;
                    let mut session_sample_rate: Option<::Value<f64>> = None;
                    let mut telemetries: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowCookies" => {
                                allow_cookies = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableXRay" => {
                                enable_x_ray = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludedPages" => {
                                excluded_pages = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FavoritePages" => {
                                favorite_pages = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GuestRoleArn" => {
                                guest_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdentityPoolId" => {
                                identity_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludedPages" => {
                                included_pages = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionSampleRate" => {
                                session_sample_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Telemetries" => {
                                telemetries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AppMonitorConfiguration {
                        allow_cookies: allow_cookies,
                        enable_x_ray: enable_x_ray,
                        excluded_pages: excluded_pages,
                        favorite_pages: favorite_pages,
                        guest_role_arn: guest_role_arn,
                        identity_pool_id: identity_pool_id,
                        included_pages: included_pages,
                        session_sample_rate: session_sample_rate,
                        telemetries: telemetries,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
