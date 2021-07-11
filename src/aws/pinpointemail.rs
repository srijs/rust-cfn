//! Types for the `PinpointEmail` service.

/// The [`AWS::PinpointEmail::ConfigurationSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationset.html) resource type.
#[derive(Debug, Default)]
pub struct ConfigurationSet {
    properties: ConfigurationSetProperties
}

/// Properties for the `ConfigurationSet` resource.
#[derive(Debug, Default)]
pub struct ConfigurationSetProperties {
    /// Property [`DeliveryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationset.html#cfn-pinpointemail-configurationset-deliveryoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delivery_options: Option<::Value<self::configuration_set::DeliveryOptions>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationset.html#cfn-pinpointemail-configurationset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ReputationOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationset.html#cfn-pinpointemail-configurationset-reputationoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub reputation_options: Option<::Value<self::configuration_set::ReputationOptions>>,
    /// Property [`SendingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationset.html#cfn-pinpointemail-configurationset-sendingoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sending_options: Option<::Value<self::configuration_set::SendingOptions>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationset.html#cfn-pinpointemail-configurationset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<self::configuration_set::Tags>>,
    /// Property [`TrackingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationset.html#cfn-pinpointemail-configurationset-trackingoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tracking_options: Option<::Value<self::configuration_set::TrackingOptions>>,
}

impl ::serde::Serialize for ConfigurationSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref delivery_options) = self.delivery_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryOptions", delivery_options)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref reputation_options) = self.reputation_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReputationOptions", reputation_options)?;
        }
        if let Some(ref sending_options) = self.sending_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SendingOptions", sending_options)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref tracking_options) = self.tracking_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrackingOptions", tracking_options)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut delivery_options: Option<::Value<self::configuration_set::DeliveryOptions>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut reputation_options: Option<::Value<self::configuration_set::ReputationOptions>> = None;
                let mut sending_options: Option<::Value<self::configuration_set::SendingOptions>> = None;
                let mut tags: Option<::ValueList<self::configuration_set::Tags>> = None;
                let mut tracking_options: Option<::Value<self::configuration_set::TrackingOptions>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeliveryOptions" => {
                            delivery_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReputationOptions" => {
                            reputation_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SendingOptions" => {
                            sending_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrackingOptions" => {
                            tracking_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationSetProperties {
                    delivery_options: delivery_options,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    reputation_options: reputation_options,
                    sending_options: sending_options,
                    tags: tags,
                    tracking_options: tracking_options,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfigurationSet {
    type Properties = ConfigurationSetProperties;
    const TYPE: &'static str = "AWS::PinpointEmail::ConfigurationSet";
    fn properties(&self) -> &ConfigurationSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfigurationSet {}

impl From<ConfigurationSetProperties> for ConfigurationSet {
    fn from(properties: ConfigurationSetProperties) -> ConfigurationSet {
        ConfigurationSet { properties }
    }
}

/// The [`AWS::PinpointEmail::ConfigurationSetEventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationseteventdestination.html) resource type.
#[derive(Debug, Default)]
pub struct ConfigurationSetEventDestination {
    properties: ConfigurationSetEventDestinationProperties
}

/// Properties for the `ConfigurationSetEventDestination` resource.
#[derive(Debug, Default)]
pub struct ConfigurationSetEventDestinationProperties {
    /// Property [`ConfigurationSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationseteventdestination.html#cfn-pinpointemail-configurationseteventdestination-configurationsetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub configuration_set_name: ::Value<String>,
    /// Property [`EventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationseteventdestination.html#cfn-pinpointemail-configurationseteventdestination-eventdestination).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_destination: Option<::Value<self::configuration_set_event_destination::EventDestination>>,
    /// Property [`EventDestinationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationseteventdestination.html#cfn-pinpointemail-configurationseteventdestination-eventdestinationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_destination_name: ::Value<String>,
}

impl ::serde::Serialize for ConfigurationSetEventDestinationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationSetName", &self.configuration_set_name)?;
        if let Some(ref event_destination) = self.event_destination {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventDestination", event_destination)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventDestinationName", &self.event_destination_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigurationSetEventDestinationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationSetEventDestinationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigurationSetEventDestinationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigurationSetEventDestinationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut configuration_set_name: Option<::Value<String>> = None;
                let mut event_destination: Option<::Value<self::configuration_set_event_destination::EventDestination>> = None;
                let mut event_destination_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfigurationSetName" => {
                            configuration_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventDestination" => {
                            event_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventDestinationName" => {
                            event_destination_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationSetEventDestinationProperties {
                    configuration_set_name: configuration_set_name.ok_or(::serde::de::Error::missing_field("ConfigurationSetName"))?,
                    event_destination: event_destination,
                    event_destination_name: event_destination_name.ok_or(::serde::de::Error::missing_field("EventDestinationName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfigurationSetEventDestination {
    type Properties = ConfigurationSetEventDestinationProperties;
    const TYPE: &'static str = "AWS::PinpointEmail::ConfigurationSetEventDestination";
    fn properties(&self) -> &ConfigurationSetEventDestinationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigurationSetEventDestinationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConfigurationSetEventDestination {}

impl From<ConfigurationSetEventDestinationProperties> for ConfigurationSetEventDestination {
    fn from(properties: ConfigurationSetEventDestinationProperties) -> ConfigurationSetEventDestination {
        ConfigurationSetEventDestination { properties }
    }
}

/// The [`AWS::PinpointEmail::DedicatedIpPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-dedicatedippool.html) resource type.
#[derive(Debug, Default)]
pub struct DedicatedIpPool {
    properties: DedicatedIpPoolProperties
}

/// Properties for the `DedicatedIpPool` resource.
#[derive(Debug, Default)]
pub struct DedicatedIpPoolProperties {
    /// Property [`PoolName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-dedicatedippool.html#cfn-pinpointemail-dedicatedippool-poolname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub pool_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-dedicatedippool.html#cfn-pinpointemail-dedicatedippool-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<self::dedicated_ip_pool::Tags>>,
}

impl ::serde::Serialize for DedicatedIpPoolProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref pool_name) = self.pool_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PoolName", pool_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DedicatedIpPoolProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DedicatedIpPoolProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DedicatedIpPoolProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DedicatedIpPoolProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut pool_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<self::dedicated_ip_pool::Tags>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PoolName" => {
                            pool_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DedicatedIpPoolProperties {
                    pool_name: pool_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DedicatedIpPool {
    type Properties = DedicatedIpPoolProperties;
    const TYPE: &'static str = "AWS::PinpointEmail::DedicatedIpPool";
    fn properties(&self) -> &DedicatedIpPoolProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DedicatedIpPoolProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DedicatedIpPool {}

impl From<DedicatedIpPoolProperties> for DedicatedIpPool {
    fn from(properties: DedicatedIpPoolProperties) -> DedicatedIpPool {
        DedicatedIpPool { properties }
    }
}

/// The [`AWS::PinpointEmail::Identity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-identity.html) resource type.
#[derive(Debug, Default)]
pub struct Identity {
    properties: IdentityProperties
}

/// Properties for the `Identity` resource.
#[derive(Debug, Default)]
pub struct IdentityProperties {
    /// Property [`DkimSigningEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-identity.html#cfn-pinpointemail-identity-dkimsigningenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dkim_signing_enabled: Option<::Value<bool>>,
    /// Property [`FeedbackForwardingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-identity.html#cfn-pinpointemail-identity-feedbackforwardingenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub feedback_forwarding_enabled: Option<::Value<bool>>,
    /// Property [`MailFromAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-identity.html#cfn-pinpointemail-identity-mailfromattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mail_from_attributes: Option<::Value<self::identity::MailFromAttributes>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-identity.html#cfn-pinpointemail-identity-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-identity.html#cfn-pinpointemail-identity-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<self::identity::Tags>>,
}

impl ::serde::Serialize for IdentityProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref dkim_signing_enabled) = self.dkim_signing_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DkimSigningEnabled", dkim_signing_enabled)?;
        }
        if let Some(ref feedback_forwarding_enabled) = self.feedback_forwarding_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeedbackForwardingEnabled", feedback_forwarding_enabled)?;
        }
        if let Some(ref mail_from_attributes) = self.mail_from_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MailFromAttributes", mail_from_attributes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IdentityProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IdentityProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IdentityProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IdentityProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dkim_signing_enabled: Option<::Value<bool>> = None;
                let mut feedback_forwarding_enabled: Option<::Value<bool>> = None;
                let mut mail_from_attributes: Option<::Value<self::identity::MailFromAttributes>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<self::identity::Tags>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DkimSigningEnabled" => {
                            dkim_signing_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FeedbackForwardingEnabled" => {
                            feedback_forwarding_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MailFromAttributes" => {
                            mail_from_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(IdentityProperties {
                    dkim_signing_enabled: dkim_signing_enabled,
                    feedback_forwarding_enabled: feedback_forwarding_enabled,
                    mail_from_attributes: mail_from_attributes,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Identity {
    type Properties = IdentityProperties;
    const TYPE: &'static str = "AWS::PinpointEmail::Identity";
    fn properties(&self) -> &IdentityProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IdentityProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Identity {}

impl From<IdentityProperties> for Identity {
    fn from(properties: IdentityProperties) -> Identity {
        Identity { properties }
    }
}

pub mod configuration_set {
    //! Property types for the `ConfigurationSet` resource.

    /// The [`AWS::PinpointEmail::ConfigurationSet.DeliveryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-deliveryoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct DeliveryOptions {
        /// Property [`SendingPoolName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-deliveryoptions.html#cfn-pinpointemail-configurationset-deliveryoptions-sendingpoolname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sending_pool_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeliveryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref sending_pool_name) = self.sending_pool_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SendingPoolName", sending_pool_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeliveryOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeliveryOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeliveryOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeliveryOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sending_pool_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SendingPoolName" => {
                                sending_pool_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeliveryOptions {
                        sending_pool_name: sending_pool_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PinpointEmail::ConfigurationSet.ReputationOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-reputationoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct ReputationOptions {
        /// Property [`ReputationMetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-reputationoptions.html#cfn-pinpointemail-configurationset-reputationoptions-reputationmetricsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reputation_metrics_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ReputationOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref reputation_metrics_enabled) = self.reputation_metrics_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReputationMetricsEnabled", reputation_metrics_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReputationOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReputationOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReputationOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReputationOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut reputation_metrics_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReputationMetricsEnabled" => {
                                reputation_metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReputationOptions {
                        reputation_metrics_enabled: reputation_metrics_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PinpointEmail::ConfigurationSet.SendingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-sendingoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct SendingOptions {
        /// Property [`SendingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-sendingoptions.html#cfn-pinpointemail-configurationset-sendingoptions-sendingenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sending_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SendingOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref sending_enabled) = self.sending_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SendingEnabled", sending_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SendingOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SendingOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SendingOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SendingOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sending_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SendingEnabled" => {
                                sending_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SendingOptions {
                        sending_enabled: sending_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PinpointEmail::ConfigurationSet.Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-tags.html) property type.
    #[derive(Debug, Default)]
    pub struct Tags {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-tags.html#cfn-pinpointemail-configurationset-tags-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-tags.html#cfn-pinpointemail-configurationset-tags-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Tags {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Tags {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Tags, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Tags;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Tags")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Tags {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PinpointEmail::ConfigurationSet.TrackingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-trackingoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct TrackingOptions {
        /// Property [`CustomRedirectDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-trackingoptions.html#cfn-pinpointemail-configurationset-trackingoptions-customredirectdomain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_redirect_domain: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TrackingOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_redirect_domain) = self.custom_redirect_domain {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRedirectDomain", custom_redirect_domain)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TrackingOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TrackingOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TrackingOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TrackingOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_redirect_domain: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomRedirectDomain" => {
                                custom_redirect_domain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TrackingOptions {
                        custom_redirect_domain: custom_redirect_domain,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod configuration_set_event_destination {
    //! Property types for the `ConfigurationSetEventDestination` resource.

    /// The [`AWS::PinpointEmail::ConfigurationSetEventDestination.CloudWatchDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-cloudwatchdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchDestination {
        /// Property [`DimensionConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-cloudwatchdestination.html#cfn-pinpointemail-configurationseteventdestination-cloudwatchdestination-dimensionconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension_configurations: Option<::ValueList<DimensionConfiguration>>,
    }

    impl ::codec::SerializeValue for CloudWatchDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dimension_configurations) = self.dimension_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionConfigurations", dimension_configurations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimension_configurations: Option<::ValueList<DimensionConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DimensionConfigurations" => {
                                dimension_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchDestination {
                        dimension_configurations: dimension_configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PinpointEmail::ConfigurationSetEventDestination.DimensionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-dimensionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DimensionConfiguration {
        /// Property [`DefaultDimensionValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-dimensionconfiguration.html#cfn-pinpointemail-configurationseteventdestination-dimensionconfiguration-defaultdimensionvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_dimension_value: ::Value<String>,
        /// Property [`DimensionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-dimensionconfiguration.html#cfn-pinpointemail-configurationseteventdestination-dimensionconfiguration-dimensionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension_name: ::Value<String>,
        /// Property [`DimensionValueSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-dimensionconfiguration.html#cfn-pinpointemail-configurationseteventdestination-dimensionconfiguration-dimensionvaluesource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension_value_source: ::Value<String>,
    }

    impl ::codec::SerializeValue for DimensionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultDimensionValue", &self.default_dimension_value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionName", &self.dimension_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionValueSource", &self.dimension_value_source)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DimensionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DimensionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DimensionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DimensionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_dimension_value: Option<::Value<String>> = None;
                    let mut dimension_name: Option<::Value<String>> = None;
                    let mut dimension_value_source: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultDimensionValue" => {
                                default_dimension_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DimensionName" => {
                                dimension_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DimensionValueSource" => {
                                dimension_value_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DimensionConfiguration {
                        default_dimension_value: default_dimension_value.ok_or(::serde::de::Error::missing_field("DefaultDimensionValue"))?,
                        dimension_name: dimension_name.ok_or(::serde::de::Error::missing_field("DimensionName"))?,
                        dimension_value_source: dimension_value_source.ok_or(::serde::de::Error::missing_field("DimensionValueSource"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PinpointEmail::ConfigurationSetEventDestination.EventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-eventdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct EventDestination {
        /// Property [`CloudWatchDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-eventdestination.html#cfn-pinpointemail-configurationseteventdestination-eventdestination-cloudwatchdestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_destination: Option<::Value<CloudWatchDestination>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-eventdestination.html#cfn-pinpointemail-configurationseteventdestination-eventdestination-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`KinesisFirehoseDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-eventdestination.html#cfn-pinpointemail-configurationseteventdestination-eventdestination-kinesisfirehosedestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_firehose_destination: Option<::Value<KinesisFirehoseDestination>>,
        /// Property [`MatchingEventTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-eventdestination.html#cfn-pinpointemail-configurationseteventdestination-eventdestination-matchingeventtypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub matching_event_types: ::ValueList<String>,
        /// Property [`PinpointDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-eventdestination.html#cfn-pinpointemail-configurationseteventdestination-eventdestination-pinpointdestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pinpoint_destination: Option<::Value<PinpointDestination>>,
        /// Property [`SnsDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-eventdestination.html#cfn-pinpointemail-configurationseteventdestination-eventdestination-snsdestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sns_destination: Option<::Value<SnsDestination>>,
    }

    impl ::codec::SerializeValue for EventDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_destination) = self.cloud_watch_destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchDestination", cloud_watch_destination)?;
            }
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref kinesis_firehose_destination) = self.kinesis_firehose_destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisFirehoseDestination", kinesis_firehose_destination)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchingEventTypes", &self.matching_event_types)?;
            if let Some(ref pinpoint_destination) = self.pinpoint_destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PinpointDestination", pinpoint_destination)?;
            }
            if let Some(ref sns_destination) = self.sns_destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsDestination", sns_destination)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_destination: Option<::Value<CloudWatchDestination>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut kinesis_firehose_destination: Option<::Value<KinesisFirehoseDestination>> = None;
                    let mut matching_event_types: Option<::ValueList<String>> = None;
                    let mut pinpoint_destination: Option<::Value<PinpointDestination>> = None;
                    let mut sns_destination: Option<::Value<SnsDestination>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchDestination" => {
                                cloud_watch_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisFirehoseDestination" => {
                                kinesis_firehose_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchingEventTypes" => {
                                matching_event_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PinpointDestination" => {
                                pinpoint_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnsDestination" => {
                                sns_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventDestination {
                        cloud_watch_destination: cloud_watch_destination,
                        enabled: enabled,
                        kinesis_firehose_destination: kinesis_firehose_destination,
                        matching_event_types: matching_event_types.ok_or(::serde::de::Error::missing_field("MatchingEventTypes"))?,
                        pinpoint_destination: pinpoint_destination,
                        sns_destination: sns_destination,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PinpointEmail::ConfigurationSetEventDestination.KinesisFirehoseDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-kinesisfirehosedestination.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisFirehoseDestination {
        /// Property [`DeliveryStreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-kinesisfirehosedestination.html#cfn-pinpointemail-configurationseteventdestination-kinesisfirehosedestination-deliverystreamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_stream_arn: ::Value<String>,
        /// Property [`IamRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-kinesisfirehosedestination.html#cfn-pinpointemail-configurationseteventdestination-kinesisfirehosedestination-iamrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamArn", &self.delivery_stream_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoleArn", &self.iam_role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisFirehoseDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisFirehoseDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisFirehoseDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisFirehoseDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delivery_stream_arn: Option<::Value<String>> = None;
                    let mut iam_role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeliveryStreamArn" => {
                                delivery_stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IamRoleArn" => {
                                iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseDestination {
                        delivery_stream_arn: delivery_stream_arn.ok_or(::serde::de::Error::missing_field("DeliveryStreamArn"))?,
                        iam_role_arn: iam_role_arn.ok_or(::serde::de::Error::missing_field("IamRoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PinpointEmail::ConfigurationSetEventDestination.PinpointDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-pinpointdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct PinpointDestination {
        /// Property [`ApplicationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-pinpointdestination.html#cfn-pinpointemail-configurationseteventdestination-pinpointdestination-applicationarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PinpointDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_arn) = self.application_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationArn", application_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PinpointDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PinpointDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PinpointDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PinpointDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationArn" => {
                                application_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PinpointDestination {
                        application_arn: application_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PinpointEmail::ConfigurationSetEventDestination.SnsDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-snsdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct SnsDestination {
        /// Property [`TopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-snsdestination.html#cfn-pinpointemail-configurationseteventdestination-snsdestination-topicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnsDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", &self.topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnsDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnsDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnsDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnsDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TopicArn" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnsDestination {
                        topic_arn: topic_arn.ok_or(::serde::de::Error::missing_field("TopicArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod dedicated_ip_pool {
    //! Property types for the `DedicatedIpPool` resource.

    /// The [`AWS::PinpointEmail::DedicatedIpPool.Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-dedicatedippool-tags.html) property type.
    #[derive(Debug, Default)]
    pub struct Tags {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-dedicatedippool-tags.html#cfn-pinpointemail-dedicatedippool-tags-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-dedicatedippool-tags.html#cfn-pinpointemail-dedicatedippool-tags-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Tags {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Tags {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Tags, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Tags;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Tags")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Tags {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod identity {
    //! Property types for the `Identity` resource.

    /// The [`AWS::PinpointEmail::Identity.MailFromAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-identity-mailfromattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct MailFromAttributes {
        /// Property [`BehaviorOnMxFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-identity-mailfromattributes.html#cfn-pinpointemail-identity-mailfromattributes-behavioronmxfailure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub behavior_on_mx_failure: Option<::Value<String>>,
        /// Property [`MailFromDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-identity-mailfromattributes.html#cfn-pinpointemail-identity-mailfromattributes-mailfromdomain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mail_from_domain: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MailFromAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref behavior_on_mx_failure) = self.behavior_on_mx_failure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BehaviorOnMxFailure", behavior_on_mx_failure)?;
            }
            if let Some(ref mail_from_domain) = self.mail_from_domain {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MailFromDomain", mail_from_domain)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MailFromAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MailFromAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MailFromAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MailFromAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut behavior_on_mx_failure: Option<::Value<String>> = None;
                    let mut mail_from_domain: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BehaviorOnMxFailure" => {
                                behavior_on_mx_failure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MailFromDomain" => {
                                mail_from_domain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MailFromAttributes {
                        behavior_on_mx_failure: behavior_on_mx_failure,
                        mail_from_domain: mail_from_domain,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::PinpointEmail::Identity.Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-identity-tags.html) property type.
    #[derive(Debug, Default)]
    pub struct Tags {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-identity-tags.html#cfn-pinpointemail-identity-tags-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-identity-tags.html#cfn-pinpointemail-identity-tags-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Tags {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Tags {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Tags, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Tags;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Tags")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Tags {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
