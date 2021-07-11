//! Types for the `LicenseManager` service.

/// The [`AWS::LicenseManager::Grant`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-grant.html) resource type.
#[derive(Debug, Default)]
pub struct Grant {
    properties: GrantProperties
}

/// Properties for the `Grant` resource.
#[derive(Debug, Default)]
pub struct GrantProperties {
    /// Property [`AllowedOperations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-grant.html#cfn-licensemanager-grant-allowedoperations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allowed_operations: Option<::ValueList<String>>,
    /// Property [`GrantName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-grant.html#cfn-licensemanager-grant-grantname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub grant_name: Option<::Value<String>>,
    /// Property [`HomeRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-grant.html#cfn-licensemanager-grant-homeregion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub home_region: Option<::Value<String>>,
    /// Property [`LicenseArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-grant.html#cfn-licensemanager-grant-licensearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub license_arn: Option<::Value<String>>,
    /// Property [`Principals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-grant.html#cfn-licensemanager-grant-principals).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub principals: Option<::ValueList<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-grant.html#cfn-licensemanager-grant-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
}

impl ::serde::Serialize for GrantProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allowed_operations) = self.allowed_operations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedOperations", allowed_operations)?;
        }
        if let Some(ref grant_name) = self.grant_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GrantName", grant_name)?;
        }
        if let Some(ref home_region) = self.home_region {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HomeRegion", home_region)?;
        }
        if let Some(ref license_arn) = self.license_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LicenseArn", license_arn)?;
        }
        if let Some(ref principals) = self.principals {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principals", principals)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GrantProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GrantProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GrantProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GrantProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allowed_operations: Option<::ValueList<String>> = None;
                let mut grant_name: Option<::Value<String>> = None;
                let mut home_region: Option<::Value<String>> = None;
                let mut license_arn: Option<::Value<String>> = None;
                let mut principals: Option<::ValueList<String>> = None;
                let mut status: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowedOperations" => {
                            allowed_operations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GrantName" => {
                            grant_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HomeRegion" => {
                            home_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LicenseArn" => {
                            license_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principals" => {
                            principals = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GrantProperties {
                    allowed_operations: allowed_operations,
                    grant_name: grant_name,
                    home_region: home_region,
                    license_arn: license_arn,
                    principals: principals,
                    status: status,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Grant {
    type Properties = GrantProperties;
    const TYPE: &'static str = "AWS::LicenseManager::Grant";
    fn properties(&self) -> &GrantProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GrantProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Grant {}

impl From<GrantProperties> for Grant {
    fn from(properties: GrantProperties) -> Grant {
        Grant { properties }
    }
}

/// The [`AWS::LicenseManager::License`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html) resource type.
#[derive(Debug, Default)]
pub struct License {
    properties: LicenseProperties
}

/// Properties for the `License` resource.
#[derive(Debug, Default)]
pub struct LicenseProperties {
    /// Property [`Beneficiary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-beneficiary).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub beneficiary: Option<::Value<String>>,
    /// Property [`ConsumptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-consumptionconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub consumption_configuration: ::Value<self::license::ConsumptionConfiguration>,
    /// Property [`Entitlements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-entitlements).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub entitlements: ::ValueList<self::license::Entitlement>,
    /// Property [`HomeRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-homeregion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub home_region: ::Value<String>,
    /// Property [`Issuer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-issuer).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub issuer: ::Value<self::license::IssuerData>,
    /// Property [`LicenseMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-licensemetadata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub license_metadata: Option<::ValueList<self::license::Metadata>>,
    /// Property [`LicenseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-licensename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub license_name: ::Value<String>,
    /// Property [`ProductName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-productname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub product_name: ::Value<String>,
    /// Property [`ProductSKU`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-productsku).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub product_sku: Option<::Value<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Validity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-licensemanager-license.html#cfn-licensemanager-license-validity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub validity: ::Value<self::license::ValidityDateFormat>,
}

impl ::serde::Serialize for LicenseProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref beneficiary) = self.beneficiary {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Beneficiary", beneficiary)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsumptionConfiguration", &self.consumption_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Entitlements", &self.entitlements)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HomeRegion", &self.home_region)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Issuer", &self.issuer)?;
        if let Some(ref license_metadata) = self.license_metadata {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LicenseMetadata", license_metadata)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LicenseName", &self.license_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductName", &self.product_name)?;
        if let Some(ref product_sku) = self.product_sku {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductSKU", product_sku)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Validity", &self.validity)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LicenseProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LicenseProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LicenseProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LicenseProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut beneficiary: Option<::Value<String>> = None;
                let mut consumption_configuration: Option<::Value<self::license::ConsumptionConfiguration>> = None;
                let mut entitlements: Option<::ValueList<self::license::Entitlement>> = None;
                let mut home_region: Option<::Value<String>> = None;
                let mut issuer: Option<::Value<self::license::IssuerData>> = None;
                let mut license_metadata: Option<::ValueList<self::license::Metadata>> = None;
                let mut license_name: Option<::Value<String>> = None;
                let mut product_name: Option<::Value<String>> = None;
                let mut product_sku: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut validity: Option<::Value<self::license::ValidityDateFormat>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Beneficiary" => {
                            beneficiary = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConsumptionConfiguration" => {
                            consumption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Entitlements" => {
                            entitlements = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HomeRegion" => {
                            home_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Issuer" => {
                            issuer = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LicenseMetadata" => {
                            license_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LicenseName" => {
                            license_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductName" => {
                            product_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductSKU" => {
                            product_sku = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Validity" => {
                            validity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LicenseProperties {
                    beneficiary: beneficiary,
                    consumption_configuration: consumption_configuration.ok_or(::serde::de::Error::missing_field("ConsumptionConfiguration"))?,
                    entitlements: entitlements.ok_or(::serde::de::Error::missing_field("Entitlements"))?,
                    home_region: home_region.ok_or(::serde::de::Error::missing_field("HomeRegion"))?,
                    issuer: issuer.ok_or(::serde::de::Error::missing_field("Issuer"))?,
                    license_metadata: license_metadata,
                    license_name: license_name.ok_or(::serde::de::Error::missing_field("LicenseName"))?,
                    product_name: product_name.ok_or(::serde::de::Error::missing_field("ProductName"))?,
                    product_sku: product_sku,
                    status: status,
                    validity: validity.ok_or(::serde::de::Error::missing_field("Validity"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for License {
    type Properties = LicenseProperties;
    const TYPE: &'static str = "AWS::LicenseManager::License";
    fn properties(&self) -> &LicenseProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LicenseProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for License {}

impl From<LicenseProperties> for License {
    fn from(properties: LicenseProperties) -> License {
        License { properties }
    }
}

pub mod license {
    //! Property types for the `License` resource.

    /// The [`AWS::LicenseManager::License.BorrowConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-borrowconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct BorrowConfiguration {
        /// Property [`AllowEarlyCheckIn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-borrowconfiguration.html#cfn-licensemanager-license-borrowconfiguration-allowearlycheckin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_early_check_in: ::Value<bool>,
        /// Property [`MaxTimeToLiveInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-borrowconfiguration.html#cfn-licensemanager-license-borrowconfiguration-maxtimetoliveinminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_time_to_live_in_minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for BorrowConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowEarlyCheckIn", &self.allow_early_check_in)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTimeToLiveInMinutes", &self.max_time_to_live_in_minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BorrowConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BorrowConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BorrowConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BorrowConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_early_check_in: Option<::Value<bool>> = None;
                    let mut max_time_to_live_in_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowEarlyCheckIn" => {
                                allow_early_check_in = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxTimeToLiveInMinutes" => {
                                max_time_to_live_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BorrowConfiguration {
                        allow_early_check_in: allow_early_check_in.ok_or(::serde::de::Error::missing_field("AllowEarlyCheckIn"))?,
                        max_time_to_live_in_minutes: max_time_to_live_in_minutes.ok_or(::serde::de::Error::missing_field("MaxTimeToLiveInMinutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LicenseManager::License.ConsumptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-consumptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ConsumptionConfiguration {
        /// Property [`BorrowConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-consumptionconfiguration.html#cfn-licensemanager-license-consumptionconfiguration-borrowconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub borrow_configuration: Option<::Value<BorrowConfiguration>>,
        /// Property [`ProvisionalConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-consumptionconfiguration.html#cfn-licensemanager-license-consumptionconfiguration-provisionalconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provisional_configuration: Option<::Value<ProvisionalConfiguration>>,
        /// Property [`RenewType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-consumptionconfiguration.html#cfn-licensemanager-license-consumptionconfiguration-renewtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub renew_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConsumptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref borrow_configuration) = self.borrow_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BorrowConfiguration", borrow_configuration)?;
            }
            if let Some(ref provisional_configuration) = self.provisional_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionalConfiguration", provisional_configuration)?;
            }
            if let Some(ref renew_type) = self.renew_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RenewType", renew_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConsumptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConsumptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConsumptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConsumptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut borrow_configuration: Option<::Value<BorrowConfiguration>> = None;
                    let mut provisional_configuration: Option<::Value<ProvisionalConfiguration>> = None;
                    let mut renew_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BorrowConfiguration" => {
                                borrow_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProvisionalConfiguration" => {
                                provisional_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RenewType" => {
                                renew_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConsumptionConfiguration {
                        borrow_configuration: borrow_configuration,
                        provisional_configuration: provisional_configuration,
                        renew_type: renew_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LicenseManager::License.Entitlement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-entitlement.html) property type.
    #[derive(Debug, Default)]
    pub struct Entitlement {
        /// Property [`AllowCheckIn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-entitlement.html#cfn-licensemanager-license-entitlement-allowcheckin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_check_in: Option<::Value<bool>>,
        /// Property [`MaxCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-entitlement.html#cfn-licensemanager-license-entitlement-maxcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_count: Option<::Value<u32>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-entitlement.html#cfn-licensemanager-license-entitlement-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Overage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-entitlement.html#cfn-licensemanager-license-entitlement-overage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub overage: Option<::Value<bool>>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-entitlement.html#cfn-licensemanager-license-entitlement-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-entitlement.html#cfn-licensemanager-license-entitlement-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Entitlement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_check_in) = self.allow_check_in {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowCheckIn", allow_check_in)?;
            }
            if let Some(ref max_count) = self.max_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCount", max_count)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref overage) = self.overage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Overage", overage)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Entitlement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Entitlement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Entitlement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Entitlement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_check_in: Option<::Value<bool>> = None;
                    let mut max_count: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut overage: Option<::Value<bool>> = None;
                    let mut unit: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowCheckIn" => {
                                allow_check_in = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxCount" => {
                                max_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Overage" => {
                                overage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Entitlement {
                        allow_check_in: allow_check_in,
                        max_count: max_count,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        overage: overage,
                        unit: unit.ok_or(::serde::de::Error::missing_field("Unit"))?,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LicenseManager::License.IssuerData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-issuerdata.html) property type.
    #[derive(Debug, Default)]
    pub struct IssuerData {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-issuerdata.html#cfn-licensemanager-license-issuerdata-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`SignKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-issuerdata.html#cfn-licensemanager-license-issuerdata-signkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sign_key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IssuerData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref sign_key) = self.sign_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SignKey", sign_key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IssuerData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IssuerData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IssuerData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IssuerData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut sign_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SignKey" => {
                                sign_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IssuerData {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        sign_key: sign_key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LicenseManager::License.Metadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-metadata.html) property type.
    #[derive(Debug, Default)]
    pub struct Metadata {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-metadata.html#cfn-licensemanager-license-metadata-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-metadata.html#cfn-licensemanager-license-metadata-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Metadata {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Metadata {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Metadata, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Metadata;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Metadata")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Metadata {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LicenseManager::License.ProvisionalConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-provisionalconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisionalConfiguration {
        /// Property [`MaxTimeToLiveInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-provisionalconfiguration.html#cfn-licensemanager-license-provisionalconfiguration-maxtimetoliveinminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_time_to_live_in_minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ProvisionalConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxTimeToLiveInMinutes", &self.max_time_to_live_in_minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProvisionalConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisionalConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisionalConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisionalConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_time_to_live_in_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxTimeToLiveInMinutes" => {
                                max_time_to_live_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisionalConfiguration {
                        max_time_to_live_in_minutes: max_time_to_live_in_minutes.ok_or(::serde::de::Error::missing_field("MaxTimeToLiveInMinutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LicenseManager::License.ValidityDateFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-validitydateformat.html) property type.
    #[derive(Debug, Default)]
    pub struct ValidityDateFormat {
        /// Property [`Begin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-validitydateformat.html#cfn-licensemanager-license-validitydateformat-begin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub begin: ::Value<String>,
        /// Property [`End`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-licensemanager-license-validitydateformat.html#cfn-licensemanager-license-validitydateformat-end).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end: ::Value<String>,
    }

    impl ::codec::SerializeValue for ValidityDateFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Begin", &self.begin)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "End", &self.end)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ValidityDateFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ValidityDateFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ValidityDateFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ValidityDateFormat")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut begin: Option<::Value<String>> = None;
                    let mut end: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Begin" => {
                                begin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "End" => {
                                end = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ValidityDateFormat {
                        begin: begin.ok_or(::serde::de::Error::missing_field("Begin"))?,
                        end: end.ok_or(::serde::de::Error::missing_field("End"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
