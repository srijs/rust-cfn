//! Types for the `SES` service.

/// The [`AWS::SES::ConfigurationSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html) resource type.
#[derive(Debug, Default)]
pub struct ConfigurationSet {
    properties: ConfigurationSetProperties
}

/// Properties for the `ConfigurationSet` resource.
#[derive(Debug, Default)]
pub struct ConfigurationSetProperties {
    /// Property [`DeliveryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html#cfn-ses-configurationset-deliveryoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delivery_options: Option<::Value<self::configuration_set::DeliveryOptions>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html#cfn-ses-configurationset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ReputationOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html#cfn-ses-configurationset-reputationoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub reputation_options: Option<::Value<self::configuration_set::ReputationOptions>>,
    /// Property [`SendingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html#cfn-ses-configurationset-sendingoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sending_options: Option<::Value<self::configuration_set::SendingOptions>>,
    /// Property [`SuppressionOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html#cfn-ses-configurationset-suppressionoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub suppression_options: Option<::Value<self::configuration_set::SuppressionOptions>>,
    /// Property [`TrackingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html#cfn-ses-configurationset-trackingoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tracking_options: Option<::Value<self::configuration_set::TrackingOptions>>,
    /// Property [`VdmOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html#cfn-ses-configurationset-vdmoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vdm_options: Option<::Value<self::configuration_set::VdmOptions>>,
}

impl ::serde::Serialize for ConfigurationSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref delivery_options) = self.delivery_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryOptions", delivery_options)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref reputation_options) = self.reputation_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReputationOptions", reputation_options)?;
        }
        if let Some(ref sending_options) = self.sending_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SendingOptions", sending_options)?;
        }
        if let Some(ref suppression_options) = self.suppression_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuppressionOptions", suppression_options)?;
        }
        if let Some(ref tracking_options) = self.tracking_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrackingOptions", tracking_options)?;
        }
        if let Some(ref vdm_options) = self.vdm_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VdmOptions", vdm_options)?;
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
                let mut suppression_options: Option<::Value<self::configuration_set::SuppressionOptions>> = None;
                let mut tracking_options: Option<::Value<self::configuration_set::TrackingOptions>> = None;
                let mut vdm_options: Option<::Value<self::configuration_set::VdmOptions>> = None;

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
                        "SuppressionOptions" => {
                            suppression_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrackingOptions" => {
                            tracking_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VdmOptions" => {
                            vdm_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationSetProperties {
                    delivery_options: delivery_options,
                    name: name,
                    reputation_options: reputation_options,
                    sending_options: sending_options,
                    suppression_options: suppression_options,
                    tracking_options: tracking_options,
                    vdm_options: vdm_options,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfigurationSet {
    type Properties = ConfigurationSetProperties;
    const TYPE: &'static str = "AWS::SES::ConfigurationSet";
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

/// The [`AWS::SES::ConfigurationSetEventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationseteventdestination.html) resource type.
#[derive(Debug, Default)]
pub struct ConfigurationSetEventDestination {
    properties: ConfigurationSetEventDestinationProperties
}

/// Properties for the `ConfigurationSetEventDestination` resource.
#[derive(Debug, Default)]
pub struct ConfigurationSetEventDestinationProperties {
    /// Property [`ConfigurationSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationseteventdestination.html#cfn-ses-configurationseteventdestination-configurationsetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub configuration_set_name: ::Value<String>,
    /// Property [`EventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationseteventdestination.html#cfn-ses-configurationseteventdestination-eventdestination).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_destination: ::Value<self::configuration_set_event_destination::EventDestination>,
}

impl ::serde::Serialize for ConfigurationSetEventDestinationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationSetName", &self.configuration_set_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventDestination", &self.event_destination)?;
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

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfigurationSetName" => {
                            configuration_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventDestination" => {
                            event_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConfigurationSetEventDestinationProperties {
                    configuration_set_name: configuration_set_name.ok_or(::serde::de::Error::missing_field("ConfigurationSetName"))?,
                    event_destination: event_destination.ok_or(::serde::de::Error::missing_field("EventDestination"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConfigurationSetEventDestination {
    type Properties = ConfigurationSetEventDestinationProperties;
    const TYPE: &'static str = "AWS::SES::ConfigurationSetEventDestination";
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

/// The [`AWS::SES::ContactList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-contactlist.html) resource type.
#[derive(Debug, Default)]
pub struct ContactList {
    properties: ContactListProperties
}

/// Properties for the `ContactList` resource.
#[derive(Debug, Default)]
pub struct ContactListProperties {
    /// Property [`ContactListName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-contactlist.html#cfn-ses-contactlist-contactlistname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub contact_list_name: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-contactlist.html#cfn-ses-contactlist-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-contactlist.html#cfn-ses-contactlist-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Topics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-contactlist.html#cfn-ses-contactlist-topics).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub topics: Option<::ValueList<self::contact_list::Topic>>,
}

impl ::serde::Serialize for ContactListProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref contact_list_name) = self.contact_list_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactListName", contact_list_name)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref topics) = self.topics {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Topics", topics)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContactListProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContactListProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContactListProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContactListProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut contact_list_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut topics: Option<::ValueList<self::contact_list::Topic>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContactListName" => {
                            contact_list_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Topics" => {
                            topics = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContactListProperties {
                    contact_list_name: contact_list_name,
                    description: description,
                    tags: tags,
                    topics: topics,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ContactList {
    type Properties = ContactListProperties;
    const TYPE: &'static str = "AWS::SES::ContactList";
    fn properties(&self) -> &ContactListProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContactListProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ContactList {}

impl From<ContactListProperties> for ContactList {
    fn from(properties: ContactListProperties) -> ContactList {
        ContactList { properties }
    }
}

/// The [`AWS::SES::DedicatedIpPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-dedicatedippool.html) resource type.
#[derive(Debug, Default)]
pub struct DedicatedIpPool {
    properties: DedicatedIpPoolProperties
}

/// Properties for the `DedicatedIpPool` resource.
#[derive(Debug, Default)]
pub struct DedicatedIpPoolProperties {
    /// Property [`PoolName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-dedicatedippool.html#cfn-ses-dedicatedippool-poolname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub pool_name: Option<::Value<String>>,
    /// Property [`ScalingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-dedicatedippool.html#cfn-ses-dedicatedippool-scalingmode).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub scaling_mode: Option<::Value<String>>,
}

impl ::serde::Serialize for DedicatedIpPoolProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref pool_name) = self.pool_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PoolName", pool_name)?;
        }
        if let Some(ref scaling_mode) = self.scaling_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingMode", scaling_mode)?;
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
                let mut scaling_mode: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PoolName" => {
                            pool_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScalingMode" => {
                            scaling_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DedicatedIpPoolProperties {
                    pool_name: pool_name,
                    scaling_mode: scaling_mode,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DedicatedIpPool {
    type Properties = DedicatedIpPoolProperties;
    const TYPE: &'static str = "AWS::SES::DedicatedIpPool";
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

/// The [`AWS::SES::EmailIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-emailidentity.html) resource type.
#[derive(Debug, Default)]
pub struct EmailIdentity {
    properties: EmailIdentityProperties
}

/// Properties for the `EmailIdentity` resource.
#[derive(Debug, Default)]
pub struct EmailIdentityProperties {
    /// Property [`ConfigurationSetAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-emailidentity.html#cfn-ses-emailidentity-configurationsetattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration_set_attributes: Option<::Value<self::email_identity::ConfigurationSetAttributes>>,
    /// Property [`DkimAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-emailidentity.html#cfn-ses-emailidentity-dkimattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dkim_attributes: Option<::Value<self::email_identity::DkimAttributes>>,
    /// Property [`DkimSigningAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-emailidentity.html#cfn-ses-emailidentity-dkimsigningattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dkim_signing_attributes: Option<::Value<self::email_identity::DkimSigningAttributes>>,
    /// Property [`EmailIdentity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-emailidentity.html#cfn-ses-emailidentity-emailidentity).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub email_identity: ::Value<String>,
    /// Property [`FeedbackAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-emailidentity.html#cfn-ses-emailidentity-feedbackattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub feedback_attributes: Option<::Value<self::email_identity::FeedbackAttributes>>,
    /// Property [`MailFromAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-emailidentity.html#cfn-ses-emailidentity-mailfromattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mail_from_attributes: Option<::Value<self::email_identity::MailFromAttributes>>,
}

impl ::serde::Serialize for EmailIdentityProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref configuration_set_attributes) = self.configuration_set_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationSetAttributes", configuration_set_attributes)?;
        }
        if let Some(ref dkim_attributes) = self.dkim_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DkimAttributes", dkim_attributes)?;
        }
        if let Some(ref dkim_signing_attributes) = self.dkim_signing_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DkimSigningAttributes", dkim_signing_attributes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailIdentity", &self.email_identity)?;
        if let Some(ref feedback_attributes) = self.feedback_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeedbackAttributes", feedback_attributes)?;
        }
        if let Some(ref mail_from_attributes) = self.mail_from_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MailFromAttributes", mail_from_attributes)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EmailIdentityProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EmailIdentityProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EmailIdentityProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EmailIdentityProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut configuration_set_attributes: Option<::Value<self::email_identity::ConfigurationSetAttributes>> = None;
                let mut dkim_attributes: Option<::Value<self::email_identity::DkimAttributes>> = None;
                let mut dkim_signing_attributes: Option<::Value<self::email_identity::DkimSigningAttributes>> = None;
                let mut email_identity: Option<::Value<String>> = None;
                let mut feedback_attributes: Option<::Value<self::email_identity::FeedbackAttributes>> = None;
                let mut mail_from_attributes: Option<::Value<self::email_identity::MailFromAttributes>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfigurationSetAttributes" => {
                            configuration_set_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DkimAttributes" => {
                            dkim_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DkimSigningAttributes" => {
                            dkim_signing_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EmailIdentity" => {
                            email_identity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FeedbackAttributes" => {
                            feedback_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MailFromAttributes" => {
                            mail_from_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EmailIdentityProperties {
                    configuration_set_attributes: configuration_set_attributes,
                    dkim_attributes: dkim_attributes,
                    dkim_signing_attributes: dkim_signing_attributes,
                    email_identity: email_identity.ok_or(::serde::de::Error::missing_field("EmailIdentity"))?,
                    feedback_attributes: feedback_attributes,
                    mail_from_attributes: mail_from_attributes,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EmailIdentity {
    type Properties = EmailIdentityProperties;
    const TYPE: &'static str = "AWS::SES::EmailIdentity";
    fn properties(&self) -> &EmailIdentityProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EmailIdentityProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EmailIdentity {}

impl From<EmailIdentityProperties> for EmailIdentity {
    fn from(properties: EmailIdentityProperties) -> EmailIdentity {
        EmailIdentity { properties }
    }
}

/// The [`AWS::SES::ReceiptFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptfilter.html) resource type.
#[derive(Debug, Default)]
pub struct ReceiptFilter {
    properties: ReceiptFilterProperties
}

/// Properties for the `ReceiptFilter` resource.
#[derive(Debug, Default)]
pub struct ReceiptFilterProperties {
    /// Property [`Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptfilter.html#cfn-ses-receiptfilter-filter).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub filter: ::Value<self::receipt_filter::Filter>,
}

impl ::serde::Serialize for ReceiptFilterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filter", &self.filter)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReceiptFilterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReceiptFilterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReceiptFilterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReceiptFilterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut filter: Option<::Value<self::receipt_filter::Filter>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Filter" => {
                            filter = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReceiptFilterProperties {
                    filter: filter.ok_or(::serde::de::Error::missing_field("Filter"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReceiptFilter {
    type Properties = ReceiptFilterProperties;
    const TYPE: &'static str = "AWS::SES::ReceiptFilter";
    fn properties(&self) -> &ReceiptFilterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReceiptFilterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReceiptFilter {}

impl From<ReceiptFilterProperties> for ReceiptFilter {
    fn from(properties: ReceiptFilterProperties) -> ReceiptFilter {
        ReceiptFilter { properties }
    }
}

/// The [`AWS::SES::ReceiptRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptrule.html) resource type.
#[derive(Debug, Default)]
pub struct ReceiptRule {
    properties: ReceiptRuleProperties
}

/// Properties for the `ReceiptRule` resource.
#[derive(Debug, Default)]
pub struct ReceiptRuleProperties {
    /// Property [`After`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptrule.html#cfn-ses-receiptrule-after).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub after: Option<::Value<String>>,
    /// Property [`Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptrule.html#cfn-ses-receiptrule-rule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rule: ::Value<self::receipt_rule::Rule>,
    /// Property [`RuleSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptrule.html#cfn-ses-receiptrule-rulesetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rule_set_name: ::Value<String>,
}

impl ::serde::Serialize for ReceiptRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref after) = self.after {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "After", after)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rule", &self.rule)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleSetName", &self.rule_set_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReceiptRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReceiptRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReceiptRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReceiptRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut after: Option<::Value<String>> = None;
                let mut rule: Option<::Value<self::receipt_rule::Rule>> = None;
                let mut rule_set_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "After" => {
                            after = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Rule" => {
                            rule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuleSetName" => {
                            rule_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReceiptRuleProperties {
                    after: after,
                    rule: rule.ok_or(::serde::de::Error::missing_field("Rule"))?,
                    rule_set_name: rule_set_name.ok_or(::serde::de::Error::missing_field("RuleSetName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReceiptRule {
    type Properties = ReceiptRuleProperties;
    const TYPE: &'static str = "AWS::SES::ReceiptRule";
    fn properties(&self) -> &ReceiptRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReceiptRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReceiptRule {}

impl From<ReceiptRuleProperties> for ReceiptRule {
    fn from(properties: ReceiptRuleProperties) -> ReceiptRule {
        ReceiptRule { properties }
    }
}

/// The [`AWS::SES::ReceiptRuleSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptruleset.html) resource type.
#[derive(Debug, Default)]
pub struct ReceiptRuleSet {
    properties: ReceiptRuleSetProperties
}

/// Properties for the `ReceiptRuleSet` resource.
#[derive(Debug, Default)]
pub struct ReceiptRuleSetProperties {
    /// Property [`RuleSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptruleset.html#cfn-ses-receiptruleset-rulesetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rule_set_name: Option<::Value<String>>,
}

impl ::serde::Serialize for ReceiptRuleSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref rule_set_name) = self.rule_set_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleSetName", rule_set_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReceiptRuleSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReceiptRuleSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReceiptRuleSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReceiptRuleSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut rule_set_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RuleSetName" => {
                            rule_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReceiptRuleSetProperties {
                    rule_set_name: rule_set_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReceiptRuleSet {
    type Properties = ReceiptRuleSetProperties;
    const TYPE: &'static str = "AWS::SES::ReceiptRuleSet";
    fn properties(&self) -> &ReceiptRuleSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReceiptRuleSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReceiptRuleSet {}

impl From<ReceiptRuleSetProperties> for ReceiptRuleSet {
    fn from(properties: ReceiptRuleSetProperties) -> ReceiptRuleSet {
        ReceiptRuleSet { properties }
    }
}

/// The [`AWS::SES::Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-template.html) resource type.
#[derive(Debug, Default)]
pub struct Template {
    properties: TemplateProperties
}

/// Properties for the `Template` resource.
#[derive(Debug, Default)]
pub struct TemplateProperties {
    /// Property [`Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-template.html#cfn-ses-template-template).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template: Option<::Value<self::template::Template>>,
}

impl ::serde::Serialize for TemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref template) = self.template {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Template", template)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut template: Option<::Value<self::template::Template>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Template" => {
                            template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TemplateProperties {
                    template: template,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Template {
    type Properties = TemplateProperties;
    const TYPE: &'static str = "AWS::SES::Template";
    fn properties(&self) -> &TemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Template {}

impl From<TemplateProperties> for Template {
    fn from(properties: TemplateProperties) -> Template {
        Template { properties }
    }
}

/// The [`AWS::SES::VdmAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-vdmattributes.html) resource type.
#[derive(Debug, Default)]
pub struct VdmAttributes {
    properties: VdmAttributesProperties
}

/// Properties for the `VdmAttributes` resource.
#[derive(Debug, Default)]
pub struct VdmAttributesProperties {
    /// Property [`DashboardAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-vdmattributes.html#cfn-ses-vdmattributes-dashboardattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dashboard_attributes: Option<::Value<self::vdm_attributes::DashboardAttributes>>,
    /// Property [`GuardianAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-vdmattributes.html#cfn-ses-vdmattributes-guardianattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub guardian_attributes: Option<::Value<self::vdm_attributes::GuardianAttributes>>,
}

impl ::serde::Serialize for VdmAttributesProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref dashboard_attributes) = self.dashboard_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardAttributes", dashboard_attributes)?;
        }
        if let Some(ref guardian_attributes) = self.guardian_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GuardianAttributes", guardian_attributes)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VdmAttributesProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VdmAttributesProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VdmAttributesProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VdmAttributesProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dashboard_attributes: Option<::Value<self::vdm_attributes::DashboardAttributes>> = None;
                let mut guardian_attributes: Option<::Value<self::vdm_attributes::GuardianAttributes>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DashboardAttributes" => {
                            dashboard_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GuardianAttributes" => {
                            guardian_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VdmAttributesProperties {
                    dashboard_attributes: dashboard_attributes,
                    guardian_attributes: guardian_attributes,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VdmAttributes {
    type Properties = VdmAttributesProperties;
    const TYPE: &'static str = "AWS::SES::VdmAttributes";
    fn properties(&self) -> &VdmAttributesProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VdmAttributesProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VdmAttributes {}

impl From<VdmAttributesProperties> for VdmAttributes {
    fn from(properties: VdmAttributesProperties) -> VdmAttributes {
        VdmAttributes { properties }
    }
}

pub mod configuration_set {
    //! Property types for the `ConfigurationSet` resource.

    /// The [`AWS::SES::ConfigurationSet.DashboardOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-dashboardoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct DashboardOptions {
        /// Property [`EngagementMetrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-dashboardoptions.html#cfn-ses-configurationset-dashboardoptions-engagementmetrics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub engagement_metrics: ::Value<String>,
    }

    impl ::codec::SerializeValue for DashboardOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngagementMetrics", &self.engagement_metrics)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashboardOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashboardOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashboardOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashboardOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut engagement_metrics: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EngagementMetrics" => {
                                engagement_metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashboardOptions {
                        engagement_metrics: engagement_metrics.ok_or(::serde::de::Error::missing_field("EngagementMetrics"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ConfigurationSet.DeliveryOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-deliveryoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct DeliveryOptions {
        /// Property [`SendingPoolName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-deliveryoptions.html#cfn-ses-configurationset-deliveryoptions-sendingpoolname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sending_pool_name: Option<::Value<String>>,
        /// Property [`TlsPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-deliveryoptions.html#cfn-ses-configurationset-deliveryoptions-tlspolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls_policy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeliveryOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref sending_pool_name) = self.sending_pool_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SendingPoolName", sending_pool_name)?;
            }
            if let Some(ref tls_policy) = self.tls_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TlsPolicy", tls_policy)?;
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
                    let mut tls_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SendingPoolName" => {
                                sending_pool_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TlsPolicy" => {
                                tls_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeliveryOptions {
                        sending_pool_name: sending_pool_name,
                        tls_policy: tls_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ConfigurationSet.GuardianOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-guardianoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct GuardianOptions {
        /// Property [`OptimizedSharedDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-guardianoptions.html#cfn-ses-configurationset-guardianoptions-optimizedshareddelivery).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub optimized_shared_delivery: ::Value<String>,
    }

    impl ::codec::SerializeValue for GuardianOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptimizedSharedDelivery", &self.optimized_shared_delivery)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GuardianOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GuardianOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GuardianOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GuardianOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut optimized_shared_delivery: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OptimizedSharedDelivery" => {
                                optimized_shared_delivery = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GuardianOptions {
                        optimized_shared_delivery: optimized_shared_delivery.ok_or(::serde::de::Error::missing_field("OptimizedSharedDelivery"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ConfigurationSet.ReputationOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-reputationoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct ReputationOptions {
        /// Property [`ReputationMetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-reputationoptions.html#cfn-ses-configurationset-reputationoptions-reputationmetricsenabled).
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

    /// The [`AWS::SES::ConfigurationSet.SendingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-sendingoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct SendingOptions {
        /// Property [`SendingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-sendingoptions.html#cfn-ses-configurationset-sendingoptions-sendingenabled).
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

    /// The [`AWS::SES::ConfigurationSet.SuppressionOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-suppressionoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct SuppressionOptions {
        /// Property [`SuppressedReasons`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-suppressionoptions.html#cfn-ses-configurationset-suppressionoptions-suppressedreasons).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suppressed_reasons: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SuppressionOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref suppressed_reasons) = self.suppressed_reasons {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuppressedReasons", suppressed_reasons)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SuppressionOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SuppressionOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SuppressionOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SuppressionOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut suppressed_reasons: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SuppressedReasons" => {
                                suppressed_reasons = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SuppressionOptions {
                        suppressed_reasons: suppressed_reasons,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ConfigurationSet.TrackingOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-trackingoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct TrackingOptions {
        /// Property [`CustomRedirectDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-trackingoptions.html#cfn-ses-configurationset-trackingoptions-customredirectdomain).
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

    /// The [`AWS::SES::ConfigurationSet.VdmOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-vdmoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct VdmOptions {
        /// Property [`DashboardOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-vdmoptions.html#cfn-ses-configurationset-vdmoptions-dashboardoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dashboard_options: Option<::Value<DashboardOptions>>,
        /// Property [`GuardianOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-vdmoptions.html#cfn-ses-configurationset-vdmoptions-guardianoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub guardian_options: Option<::Value<GuardianOptions>>,
    }

    impl ::codec::SerializeValue for VdmOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dashboard_options) = self.dashboard_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardOptions", dashboard_options)?;
            }
            if let Some(ref guardian_options) = self.guardian_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GuardianOptions", guardian_options)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VdmOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VdmOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VdmOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VdmOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dashboard_options: Option<::Value<DashboardOptions>> = None;
                    let mut guardian_options: Option<::Value<GuardianOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DashboardOptions" => {
                                dashboard_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GuardianOptions" => {
                                guardian_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VdmOptions {
                        dashboard_options: dashboard_options,
                        guardian_options: guardian_options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod configuration_set_event_destination {
    //! Property types for the `ConfigurationSetEventDestination` resource.

    /// The [`AWS::SES::ConfigurationSetEventDestination.CloudWatchDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-cloudwatchdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchDestination {
        /// Property [`DimensionConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-cloudwatchdestination.html#cfn-ses-configurationseteventdestination-cloudwatchdestination-dimensionconfigurations).
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

    /// The [`AWS::SES::ConfigurationSetEventDestination.DimensionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-dimensionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DimensionConfiguration {
        /// Property [`DefaultDimensionValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-dimensionconfiguration.html#cfn-ses-configurationseteventdestination-dimensionconfiguration-defaultdimensionvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_dimension_value: ::Value<String>,
        /// Property [`DimensionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-dimensionconfiguration.html#cfn-ses-configurationseteventdestination-dimensionconfiguration-dimensionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension_name: ::Value<String>,
        /// Property [`DimensionValueSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-dimensionconfiguration.html#cfn-ses-configurationseteventdestination-dimensionconfiguration-dimensionvaluesource).
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

    /// The [`AWS::SES::ConfigurationSetEventDestination.EventDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct EventDestination {
        /// Property [`CloudWatchDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventdestination.html#cfn-ses-configurationseteventdestination-eventdestination-cloudwatchdestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_destination: Option<::Value<CloudWatchDestination>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventdestination.html#cfn-ses-configurationseteventdestination-eventdestination-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`KinesisFirehoseDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventdestination.html#cfn-ses-configurationseteventdestination-eventdestination-kinesisfirehosedestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_firehose_destination: Option<::Value<KinesisFirehoseDestination>>,
        /// Property [`MatchingEventTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventdestination.html#cfn-ses-configurationseteventdestination-eventdestination-matchingeventtypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub matching_event_types: ::ValueList<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventdestination.html#cfn-ses-configurationseteventdestination-eventdestination-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`SnsDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventdestination.html#cfn-ses-configurationseteventdestination-eventdestination-snsdestination).
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
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
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
                    let mut name: Option<::Value<String>> = None;
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
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        name: name,
                        sns_destination: sns_destination,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ConfigurationSetEventDestination.KinesisFirehoseDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-kinesisfirehosedestination.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisFirehoseDestination {
        /// Property [`DeliveryStreamARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-kinesisfirehosedestination.html#cfn-ses-configurationseteventdestination-kinesisfirehosedestination-deliverystreamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_stream_arn: ::Value<String>,
        /// Property [`IAMRoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-kinesisfirehosedestination.html#cfn-ses-configurationseteventdestination-kinesisfirehosedestination-iamrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamARN", &self.delivery_stream_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IAMRoleARN", &self.iam_role_arn)?;
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
                            "DeliveryStreamARN" => {
                                delivery_stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IAMRoleARN" => {
                                iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseDestination {
                        delivery_stream_arn: delivery_stream_arn.ok_or(::serde::de::Error::missing_field("DeliveryStreamARN"))?,
                        iam_role_arn: iam_role_arn.ok_or(::serde::de::Error::missing_field("IAMRoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ConfigurationSetEventDestination.SnsDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-snsdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct SnsDestination {
        /// Property [`TopicARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-snsdestination.html#cfn-ses-configurationseteventdestination-snsdestination-topicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnsDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicARN", &self.topic_arn)?;
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
                            "TopicARN" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnsDestination {
                        topic_arn: topic_arn.ok_or(::serde::de::Error::missing_field("TopicARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod contact_list {
    //! Property types for the `ContactList` resource.

    /// The [`AWS::SES::ContactList.Topic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-contactlist-topic.html) property type.
    #[derive(Debug, Default)]
    pub struct Topic {
        /// Property [`DefaultSubscriptionStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-contactlist-topic.html#cfn-ses-contactlist-topic-defaultsubscriptionstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_subscription_status: ::Value<String>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-contactlist-topic.html#cfn-ses-contactlist-topic-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-contactlist-topic.html#cfn-ses-contactlist-topic-displayname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub display_name: ::Value<String>,
        /// Property [`TopicName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-contactlist-topic.html#cfn-ses-contactlist-topic-topicname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Topic {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultSubscriptionStatus", &self.default_subscription_status)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", &self.display_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicName", &self.topic_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Topic {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Topic, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Topic;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Topic")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_subscription_status: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut display_name: Option<::Value<String>> = None;
                    let mut topic_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultSubscriptionStatus" => {
                                default_subscription_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisplayName" => {
                                display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicName" => {
                                topic_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Topic {
                        default_subscription_status: default_subscription_status.ok_or(::serde::de::Error::missing_field("DefaultSubscriptionStatus"))?,
                        description: description,
                        display_name: display_name.ok_or(::serde::de::Error::missing_field("DisplayName"))?,
                        topic_name: topic_name.ok_or(::serde::de::Error::missing_field("TopicName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod email_identity {
    //! Property types for the `EmailIdentity` resource.

    /// The [`AWS::SES::EmailIdentity.ConfigurationSetAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-configurationsetattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfigurationSetAttributes {
        /// Property [`ConfigurationSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-configurationsetattributes.html#cfn-ses-emailidentity-configurationsetattributes-configurationsetname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configuration_set_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConfigurationSetAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref configuration_set_name) = self.configuration_set_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationSetName", configuration_set_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigurationSetAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationSetAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigurationSetAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigurationSetAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut configuration_set_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfigurationSetName" => {
                                configuration_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigurationSetAttributes {
                        configuration_set_name: configuration_set_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::EmailIdentity.DkimAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-dkimattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct DkimAttributes {
        /// Property [`SigningEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-dkimattributes.html#cfn-ses-emailidentity-dkimattributes-signingenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub signing_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for DkimAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref signing_enabled) = self.signing_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigningEnabled", signing_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DkimAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DkimAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DkimAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DkimAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut signing_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SigningEnabled" => {
                                signing_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DkimAttributes {
                        signing_enabled: signing_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::EmailIdentity.DkimSigningAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-dkimsigningattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct DkimSigningAttributes {
        /// Property [`DomainSigningPrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-dkimsigningattributes.html#cfn-ses-emailidentity-dkimsigningattributes-domainsigningprivatekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_signing_private_key: Option<::Value<String>>,
        /// Property [`DomainSigningSelector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-dkimsigningattributes.html#cfn-ses-emailidentity-dkimsigningattributes-domainsigningselector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_signing_selector: Option<::Value<String>>,
        /// Property [`NextSigningKeyLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-dkimsigningattributes.html#cfn-ses-emailidentity-dkimsigningattributes-nextsigningkeylength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next_signing_key_length: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DkimSigningAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref domain_signing_private_key) = self.domain_signing_private_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainSigningPrivateKey", domain_signing_private_key)?;
            }
            if let Some(ref domain_signing_selector) = self.domain_signing_selector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainSigningSelector", domain_signing_selector)?;
            }
            if let Some(ref next_signing_key_length) = self.next_signing_key_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NextSigningKeyLength", next_signing_key_length)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DkimSigningAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DkimSigningAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DkimSigningAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DkimSigningAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut domain_signing_private_key: Option<::Value<String>> = None;
                    let mut domain_signing_selector: Option<::Value<String>> = None;
                    let mut next_signing_key_length: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DomainSigningPrivateKey" => {
                                domain_signing_private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DomainSigningSelector" => {
                                domain_signing_selector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NextSigningKeyLength" => {
                                next_signing_key_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DkimSigningAttributes {
                        domain_signing_private_key: domain_signing_private_key,
                        domain_signing_selector: domain_signing_selector,
                        next_signing_key_length: next_signing_key_length,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::EmailIdentity.FeedbackAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-feedbackattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct FeedbackAttributes {
        /// Property [`EmailForwardingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-feedbackattributes.html#cfn-ses-emailidentity-feedbackattributes-emailforwardingenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_forwarding_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for FeedbackAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref email_forwarding_enabled) = self.email_forwarding_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailForwardingEnabled", email_forwarding_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FeedbackAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FeedbackAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FeedbackAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FeedbackAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut email_forwarding_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EmailForwardingEnabled" => {
                                email_forwarding_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FeedbackAttributes {
                        email_forwarding_enabled: email_forwarding_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::EmailIdentity.MailFromAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-mailfromattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct MailFromAttributes {
        /// Property [`BehaviorOnMxFailure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-mailfromattributes.html#cfn-ses-emailidentity-mailfromattributes-behavioronmxfailure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub behavior_on_mx_failure: Option<::Value<String>>,
        /// Property [`MailFromDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-mailfromattributes.html#cfn-ses-emailidentity-mailfromattributes-mailfromdomain).
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
}

pub mod receipt_filter {
    //! Property types for the `ReceiptFilter` resource.

    /// The [`AWS::SES::ReceiptFilter.Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-filter.html) property type.
    #[derive(Debug, Default)]
    pub struct Filter {
        /// Property [`IpFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-filter.html#cfn-ses-receiptfilter-filter-ipfilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_filter: ::Value<IpFilter>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-filter.html#cfn-ses-receiptfilter-filter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Filter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpFilter", &self.ip_filter)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Filter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Filter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Filter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Filter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip_filter: Option<::Value<IpFilter>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IpFilter" => {
                                ip_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Filter {
                        ip_filter: ip_filter.ok_or(::serde::de::Error::missing_field("IpFilter"))?,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptFilter.IpFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-ipfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct IpFilter {
        /// Property [`Cidr`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-ipfilter.html#cfn-ses-receiptfilter-ipfilter-cidr).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cidr: ::Value<String>,
        /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-ipfilter.html#cfn-ses-receiptfilter-ipfilter-policy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy: ::Value<String>,
    }

    impl ::codec::SerializeValue for IpFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cidr", &self.cidr)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", &self.policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IpFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IpFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IpFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IpFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cidr: Option<::Value<String>> = None;
                    let mut policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cidr" => {
                                cidr = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Policy" => {
                                policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IpFilter {
                        cidr: cidr.ok_or(::serde::de::Error::missing_field("Cidr"))?,
                        policy: policy.ok_or(::serde::de::Error::missing_field("Policy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod receipt_rule {
    //! Property types for the `ReceiptRule` resource.

    /// The [`AWS::SES::ReceiptRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`AddHeaderAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html#cfn-ses-receiptrule-action-addheaderaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub add_header_action: Option<::Value<AddHeaderAction>>,
        /// Property [`BounceAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html#cfn-ses-receiptrule-action-bounceaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bounce_action: Option<::Value<BounceAction>>,
        /// Property [`LambdaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html#cfn-ses-receiptrule-action-lambdaaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_action: Option<::Value<LambdaAction>>,
        /// Property [`S3Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html#cfn-ses-receiptrule-action-s3action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_action: Option<::Value<S3Action>>,
        /// Property [`SNSAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html#cfn-ses-receiptrule-action-snsaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sns_action: Option<::Value<SNSAction>>,
        /// Property [`StopAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html#cfn-ses-receiptrule-action-stopaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stop_action: Option<::Value<StopAction>>,
        /// Property [`WorkmailAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html#cfn-ses-receiptrule-action-workmailaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workmail_action: Option<::Value<WorkmailAction>>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref add_header_action) = self.add_header_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddHeaderAction", add_header_action)?;
            }
            if let Some(ref bounce_action) = self.bounce_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BounceAction", bounce_action)?;
            }
            if let Some(ref lambda_action) = self.lambda_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaAction", lambda_action)?;
            }
            if let Some(ref s3_action) = self.s3_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Action", s3_action)?;
            }
            if let Some(ref sns_action) = self.sns_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SNSAction", sns_action)?;
            }
            if let Some(ref stop_action) = self.stop_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StopAction", stop_action)?;
            }
            if let Some(ref workmail_action) = self.workmail_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkmailAction", workmail_action)?;
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
                    let mut add_header_action: Option<::Value<AddHeaderAction>> = None;
                    let mut bounce_action: Option<::Value<BounceAction>> = None;
                    let mut lambda_action: Option<::Value<LambdaAction>> = None;
                    let mut s3_action: Option<::Value<S3Action>> = None;
                    let mut sns_action: Option<::Value<SNSAction>> = None;
                    let mut stop_action: Option<::Value<StopAction>> = None;
                    let mut workmail_action: Option<::Value<WorkmailAction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddHeaderAction" => {
                                add_header_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BounceAction" => {
                                bounce_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaAction" => {
                                lambda_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Action" => {
                                s3_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SNSAction" => {
                                sns_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StopAction" => {
                                stop_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkmailAction" => {
                                workmail_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        add_header_action: add_header_action,
                        bounce_action: bounce_action,
                        lambda_action: lambda_action,
                        s3_action: s3_action,
                        sns_action: sns_action,
                        stop_action: stop_action,
                        workmail_action: workmail_action,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.AddHeaderAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-addheaderaction.html) property type.
    #[derive(Debug, Default)]
    pub struct AddHeaderAction {
        /// Property [`HeaderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-addheaderaction.html#cfn-ses-receiptrule-addheaderaction-headername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_name: ::Value<String>,
        /// Property [`HeaderValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-addheaderaction.html#cfn-ses-receiptrule-addheaderaction-headervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for AddHeaderAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderName", &self.header_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderValue", &self.header_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AddHeaderAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AddHeaderAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AddHeaderAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AddHeaderAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_name: Option<::Value<String>> = None;
                    let mut header_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderName" => {
                                header_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderValue" => {
                                header_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AddHeaderAction {
                        header_name: header_name.ok_or(::serde::de::Error::missing_field("HeaderName"))?,
                        header_value: header_value.ok_or(::serde::de::Error::missing_field("HeaderValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.BounceAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-bounceaction.html) property type.
    #[derive(Debug, Default)]
    pub struct BounceAction {
        /// Property [`Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-bounceaction.html#cfn-ses-receiptrule-bounceaction-message).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message: ::Value<String>,
        /// Property [`Sender`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-bounceaction.html#cfn-ses-receiptrule-bounceaction-sender).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sender: ::Value<String>,
        /// Property [`SmtpReplyCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-bounceaction.html#cfn-ses-receiptrule-bounceaction-smtpreplycode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub smtp_reply_code: ::Value<String>,
        /// Property [`StatusCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-bounceaction.html#cfn-ses-receiptrule-bounceaction-statuscode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status_code: Option<::Value<String>>,
        /// Property [`TopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-bounceaction.html#cfn-ses-receiptrule-bounceaction-topicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BounceAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", &self.message)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sender", &self.sender)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmtpReplyCode", &self.smtp_reply_code)?;
            if let Some(ref status_code) = self.status_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusCode", status_code)?;
            }
            if let Some(ref topic_arn) = self.topic_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", topic_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BounceAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BounceAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BounceAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BounceAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message: Option<::Value<String>> = None;
                    let mut sender: Option<::Value<String>> = None;
                    let mut smtp_reply_code: Option<::Value<String>> = None;
                    let mut status_code: Option<::Value<String>> = None;
                    let mut topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Message" => {
                                message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sender" => {
                                sender = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SmtpReplyCode" => {
                                smtp_reply_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatusCode" => {
                                status_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicArn" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BounceAction {
                        message: message.ok_or(::serde::de::Error::missing_field("Message"))?,
                        sender: sender.ok_or(::serde::de::Error::missing_field("Sender"))?,
                        smtp_reply_code: smtp_reply_code.ok_or(::serde::de::Error::missing_field("SmtpReplyCode"))?,
                        status_code: status_code,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.LambdaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-lambdaaction.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaAction {
        /// Property [`FunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-lambdaaction.html#cfn-ses-receiptrule-lambdaaction-functionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_arn: ::Value<String>,
        /// Property [`InvocationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-lambdaaction.html#cfn-ses-receiptrule-lambdaaction-invocationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invocation_type: Option<::Value<String>>,
        /// Property [`TopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-lambdaaction.html#cfn-ses-receiptrule-lambdaaction-topicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionArn", &self.function_arn)?;
            if let Some(ref invocation_type) = self.invocation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationType", invocation_type)?;
            }
            if let Some(ref topic_arn) = self.topic_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", topic_arn)?;
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
                    let mut invocation_type: Option<::Value<String>> = None;
                    let mut topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionArn" => {
                                function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InvocationType" => {
                                invocation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicArn" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaAction {
                        function_arn: function_arn.ok_or(::serde::de::Error::missing_field("FunctionArn"))?,
                        invocation_type: invocation_type,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-rule.html) property type.
    #[derive(Debug, Default)]
    pub struct Rule {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-rule.html#cfn-ses-receiptrule-rule-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: Option<::ValueList<Action>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-rule.html#cfn-ses-receiptrule-rule-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-rule.html#cfn-ses-receiptrule-rule-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Recipients`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-rule.html#cfn-ses-receiptrule-rule-recipients).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recipients: Option<::ValueList<String>>,
        /// Property [`ScanEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-rule.html#cfn-ses-receiptrule-rule-scanenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scan_enabled: Option<::Value<bool>>,
        /// Property [`TlsPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-rule.html#cfn-ses-receiptrule-rule-tlspolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls_policy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref actions) = self.actions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", actions)?;
            }
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref recipients) = self.recipients {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Recipients", recipients)?;
            }
            if let Some(ref scan_enabled) = self.scan_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScanEnabled", scan_enabled)?;
            }
            if let Some(ref tls_policy) = self.tls_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TlsPolicy", tls_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Rule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Rule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Rule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Rule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<Action>> = None;
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut recipients: Option<::ValueList<String>> = None;
                    let mut scan_enabled: Option<::Value<bool>> = None;
                    let mut tls_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Recipients" => {
                                recipients = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScanEnabled" => {
                                scan_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TlsPolicy" => {
                                tls_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Rule {
                        actions: actions,
                        enabled: enabled,
                        name: name,
                        recipients: recipients,
                        scan_enabled: scan_enabled,
                        tls_policy: tls_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.S3Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-s3action.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Action {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-s3action.html#cfn-ses-receiptrule-s3action-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-s3action.html#cfn-ses-receiptrule-s3action-kmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_arn: Option<::Value<String>>,
        /// Property [`ObjectKeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-s3action.html#cfn-ses-receiptrule-s3action-objectkeyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_key_prefix: Option<::Value<String>>,
        /// Property [`TopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-s3action.html#cfn-ses-receiptrule-s3action-topicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref kms_key_arn) = self.kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
            }
            if let Some(ref object_key_prefix) = self.object_key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectKeyPrefix", object_key_prefix)?;
            }
            if let Some(ref topic_arn) = self.topic_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", topic_arn)?;
            }
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
                    let mut kms_key_arn: Option<::Value<String>> = None;
                    let mut object_key_prefix: Option<::Value<String>> = None;
                    let mut topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectKeyPrefix" => {
                                object_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicArn" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Action {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        kms_key_arn: kms_key_arn,
                        object_key_prefix: object_key_prefix,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.SNSAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-snsaction.html) property type.
    #[derive(Debug, Default)]
    pub struct SNSAction {
        /// Property [`Encoding`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-snsaction.html#cfn-ses-receiptrule-snsaction-encoding).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encoding: Option<::Value<String>>,
        /// Property [`TopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-snsaction.html#cfn-ses-receiptrule-snsaction-topicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SNSAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encoding) = self.encoding {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encoding", encoding)?;
            }
            if let Some(ref topic_arn) = self.topic_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", topic_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SNSAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SNSAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SNSAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SNSAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encoding: Option<::Value<String>> = None;
                    let mut topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Encoding" => {
                                encoding = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicArn" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SNSAction {
                        encoding: encoding,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.StopAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-stopaction.html) property type.
    #[derive(Debug, Default)]
    pub struct StopAction {
        /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-stopaction.html#cfn-ses-receiptrule-stopaction-scope).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope: ::Value<String>,
        /// Property [`TopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-stopaction.html#cfn-ses-receiptrule-stopaction-topicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StopAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", &self.scope)?;
            if let Some(ref topic_arn) = self.topic_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", topic_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StopAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StopAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StopAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StopAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut scope: Option<::Value<String>> = None;
                    let mut topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Scope" => {
                                scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicArn" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StopAction {
                        scope: scope.ok_or(::serde::de::Error::missing_field("Scope"))?,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::ReceiptRule.WorkmailAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-workmailaction.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkmailAction {
        /// Property [`OrganizationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-workmailaction.html#cfn-ses-receiptrule-workmailaction-organizationarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub organization_arn: ::Value<String>,
        /// Property [`TopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-workmailaction.html#cfn-ses-receiptrule-workmailaction-topicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for WorkmailAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationArn", &self.organization_arn)?;
            if let Some(ref topic_arn) = self.topic_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", topic_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkmailAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkmailAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkmailAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkmailAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut organization_arn: Option<::Value<String>> = None;
                    let mut topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OrganizationArn" => {
                                organization_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TopicArn" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkmailAction {
                        organization_arn: organization_arn.ok_or(::serde::de::Error::missing_field("OrganizationArn"))?,
                        topic_arn: topic_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod template {
    //! Property types for the `Template` resource.

    /// The [`AWS::SES::Template.Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-template-template.html) property type.
    #[derive(Debug, Default)]
    pub struct Template {
        /// Property [`HtmlPart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-template-template.html#cfn-ses-template-template-htmlpart).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub html_part: Option<::Value<String>>,
        /// Property [`SubjectPart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-template-template.html#cfn-ses-template-template-subjectpart).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject_part: ::Value<String>,
        /// Property [`TemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-template-template.html#cfn-ses-template-template-templatename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub template_name: Option<::Value<String>>,
        /// Property [`TextPart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-template-template.html#cfn-ses-template-template-textpart).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_part: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Template {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref html_part) = self.html_part {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HtmlPart", html_part)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubjectPart", &self.subject_part)?;
            if let Some(ref template_name) = self.template_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", template_name)?;
            }
            if let Some(ref text_part) = self.text_part {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextPart", text_part)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Template {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Template, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Template;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Template")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut html_part: Option<::Value<String>> = None;
                    let mut subject_part: Option<::Value<String>> = None;
                    let mut template_name: Option<::Value<String>> = None;
                    let mut text_part: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HtmlPart" => {
                                html_part = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubjectPart" => {
                                subject_part = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemplateName" => {
                                template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextPart" => {
                                text_part = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Template {
                        html_part: html_part,
                        subject_part: subject_part.ok_or(::serde::de::Error::missing_field("SubjectPart"))?,
                        template_name: template_name,
                        text_part: text_part,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod vdm_attributes {
    //! Property types for the `VdmAttributes` resource.

    /// The [`AWS::SES::VdmAttributes.DashboardAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-vdmattributes-dashboardattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct DashboardAttributes {
        /// Property [`EngagementMetrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-vdmattributes-dashboardattributes.html#cfn-ses-vdmattributes-dashboardattributes-engagementmetrics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub engagement_metrics: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DashboardAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref engagement_metrics) = self.engagement_metrics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngagementMetrics", engagement_metrics)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashboardAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashboardAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashboardAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashboardAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut engagement_metrics: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EngagementMetrics" => {
                                engagement_metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashboardAttributes {
                        engagement_metrics: engagement_metrics,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SES::VdmAttributes.GuardianAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-vdmattributes-guardianattributes.html) property type.
    #[derive(Debug, Default)]
    pub struct GuardianAttributes {
        /// Property [`OptimizedSharedDelivery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-vdmattributes-guardianattributes.html#cfn-ses-vdmattributes-guardianattributes-optimizedshareddelivery).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub optimized_shared_delivery: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GuardianAttributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref optimized_shared_delivery) = self.optimized_shared_delivery {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptimizedSharedDelivery", optimized_shared_delivery)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GuardianAttributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GuardianAttributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GuardianAttributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GuardianAttributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut optimized_shared_delivery: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OptimizedSharedDelivery" => {
                                optimized_shared_delivery = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GuardianAttributes {
                        optimized_shared_delivery: optimized_shared_delivery,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
