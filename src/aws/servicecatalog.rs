//! Types for the `ServiceCatalog` service.

/// The [`AWS::ServiceCatalog::AcceptedPortfolioShare`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-acceptedportfolioshare.html) resource type.
#[derive(Debug, Default)]
pub struct AcceptedPortfolioShare {
    properties: AcceptedPortfolioShareProperties
}

/// Properties for the `AcceptedPortfolioShare` resource.
#[derive(Debug, Default)]
pub struct AcceptedPortfolioShareProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-acceptedportfolioshare.html#cfn-servicecatalog-acceptedportfolioshare-acceptlanguage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`PortfolioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-acceptedportfolioshare.html#cfn-servicecatalog-acceptedportfolioshare-portfolioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portfolio_id: ::Value<String>,
}

impl ::serde::Serialize for AcceptedPortfolioShareProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortfolioId", &self.portfolio_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AcceptedPortfolioShareProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AcceptedPortfolioShareProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AcceptedPortfolioShareProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AcceptedPortfolioShareProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut portfolio_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortfolioId" => {
                            portfolio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AcceptedPortfolioShareProperties {
                    accept_language: accept_language,
                    portfolio_id: portfolio_id.ok_or(::serde::de::Error::missing_field("PortfolioId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AcceptedPortfolioShare {
    type Properties = AcceptedPortfolioShareProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::AcceptedPortfolioShare";
    fn properties(&self) -> &AcceptedPortfolioShareProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AcceptedPortfolioShareProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AcceptedPortfolioShare {}

impl From<AcceptedPortfolioShareProperties> for AcceptedPortfolioShare {
    fn from(properties: AcceptedPortfolioShareProperties) -> AcceptedPortfolioShare {
        AcceptedPortfolioShare { properties }
    }
}

/// The [`AWS::ServiceCatalog::CloudFormationProduct`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html) resource type.
#[derive(Debug, Default)]
pub struct CloudFormationProduct {
    properties: CloudFormationProductProperties
}

/// Properties for the `CloudFormationProduct` resource.
#[derive(Debug, Default)]
pub struct CloudFormationProductProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-acceptlanguage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Distributor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-distributor).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub distributor: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Owner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-owner).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub owner: ::Value<String>,
    /// Property [`ProvisioningArtifactParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-provisioningartifactparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioning_artifact_parameters: ::ValueList<self::cloud_formation_product::ProvisioningArtifactProperties>,
    /// Property [`ReplaceProvisioningArtifacts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-replaceprovisioningartifacts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replace_provisioning_artifacts: Option<::Value<bool>>,
    /// Property [`SupportDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-supportdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub support_description: Option<::Value<String>>,
    /// Property [`SupportEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-supportemail).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub support_email: Option<::Value<String>>,
    /// Property [`SupportUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-supporturl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub support_url: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationproduct.html#cfn-servicecatalog-cloudformationproduct-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CloudFormationProductProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref distributor) = self.distributor {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Distributor", distributor)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Owner", &self.owner)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisioningArtifactParameters", &self.provisioning_artifact_parameters)?;
        if let Some(ref replace_provisioning_artifacts) = self.replace_provisioning_artifacts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplaceProvisioningArtifacts", replace_provisioning_artifacts)?;
        }
        if let Some(ref support_description) = self.support_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportDescription", support_description)?;
        }
        if let Some(ref support_email) = self.support_email {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportEmail", support_email)?;
        }
        if let Some(ref support_url) = self.support_url {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportUrl", support_url)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CloudFormationProductProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudFormationProductProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CloudFormationProductProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CloudFormationProductProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut distributor: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut owner: Option<::Value<String>> = None;
                let mut provisioning_artifact_parameters: Option<::ValueList<self::cloud_formation_product::ProvisioningArtifactProperties>> = None;
                let mut replace_provisioning_artifacts: Option<::Value<bool>> = None;
                let mut support_description: Option<::Value<String>> = None;
                let mut support_email: Option<::Value<String>> = None;
                let mut support_url: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Distributor" => {
                            distributor = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Owner" => {
                            owner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisioningArtifactParameters" => {
                            provisioning_artifact_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplaceProvisioningArtifacts" => {
                            replace_provisioning_artifacts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SupportDescription" => {
                            support_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SupportEmail" => {
                            support_email = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SupportUrl" => {
                            support_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CloudFormationProductProperties {
                    accept_language: accept_language,
                    description: description,
                    distributor: distributor,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    owner: owner.ok_or(::serde::de::Error::missing_field("Owner"))?,
                    provisioning_artifact_parameters: provisioning_artifact_parameters.ok_or(::serde::de::Error::missing_field("ProvisioningArtifactParameters"))?,
                    replace_provisioning_artifacts: replace_provisioning_artifacts,
                    support_description: support_description,
                    support_email: support_email,
                    support_url: support_url,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CloudFormationProduct {
    type Properties = CloudFormationProductProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::CloudFormationProduct";
    fn properties(&self) -> &CloudFormationProductProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CloudFormationProductProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CloudFormationProduct {}

impl From<CloudFormationProductProperties> for CloudFormationProduct {
    fn from(properties: CloudFormationProductProperties) -> CloudFormationProduct {
        CloudFormationProduct { properties }
    }
}

/// The [`AWS::ServiceCatalog::CloudFormationProvisionedProduct`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html) resource type.
#[derive(Debug, Default)]
pub struct CloudFormationProvisionedProduct {
    properties: CloudFormationProvisionedProductProperties
}

/// Properties for the `CloudFormationProvisionedProduct` resource.
#[derive(Debug, Default)]
pub struct CloudFormationProvisionedProductProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-acceptlanguage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`NotificationArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-notificationarns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub notification_arns: Option<::ValueList<String>>,
    /// Property [`PathId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-pathid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub path_id: Option<::Value<String>>,
    /// Property [`PathName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-pathname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub path_name: Option<::Value<String>>,
    /// Property [`ProductId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-productid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub product_id: Option<::Value<String>>,
    /// Property [`ProductName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-productname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub product_name: Option<::Value<String>>,
    /// Property [`ProvisionedProductName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisionedproductname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub provisioned_product_name: Option<::Value<String>>,
    /// Property [`ProvisioningArtifactId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningartifactid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioning_artifact_id: Option<::Value<String>>,
    /// Property [`ProvisioningArtifactName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningartifactname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioning_artifact_name: Option<::Value<String>>,
    /// Property [`ProvisioningParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioning_parameters: Option<::ValueList<self::cloud_formation_provisioned_product::ProvisioningParameter>>,
    /// Property [`ProvisioningPreferences`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioning_preferences: Option<::Value<self::cloud_formation_provisioned_product::ProvisioningPreferences>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-cloudformationprovisionedproduct.html#cfn-servicecatalog-cloudformationprovisionedproduct-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CloudFormationProvisionedProductProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        if let Some(ref notification_arns) = self.notification_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationArns", notification_arns)?;
        }
        if let Some(ref path_id) = self.path_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathId", path_id)?;
        }
        if let Some(ref path_name) = self.path_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathName", path_name)?;
        }
        if let Some(ref product_id) = self.product_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductId", product_id)?;
        }
        if let Some(ref product_name) = self.product_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductName", product_name)?;
        }
        if let Some(ref provisioned_product_name) = self.provisioned_product_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedProductName", provisioned_product_name)?;
        }
        if let Some(ref provisioning_artifact_id) = self.provisioning_artifact_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisioningArtifactId", provisioning_artifact_id)?;
        }
        if let Some(ref provisioning_artifact_name) = self.provisioning_artifact_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisioningArtifactName", provisioning_artifact_name)?;
        }
        if let Some(ref provisioning_parameters) = self.provisioning_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisioningParameters", provisioning_parameters)?;
        }
        if let Some(ref provisioning_preferences) = self.provisioning_preferences {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisioningPreferences", provisioning_preferences)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CloudFormationProvisionedProductProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudFormationProvisionedProductProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CloudFormationProvisionedProductProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CloudFormationProvisionedProductProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut notification_arns: Option<::ValueList<String>> = None;
                let mut path_id: Option<::Value<String>> = None;
                let mut path_name: Option<::Value<String>> = None;
                let mut product_id: Option<::Value<String>> = None;
                let mut product_name: Option<::Value<String>> = None;
                let mut provisioned_product_name: Option<::Value<String>> = None;
                let mut provisioning_artifact_id: Option<::Value<String>> = None;
                let mut provisioning_artifact_name: Option<::Value<String>> = None;
                let mut provisioning_parameters: Option<::ValueList<self::cloud_formation_provisioned_product::ProvisioningParameter>> = None;
                let mut provisioning_preferences: Option<::Value<self::cloud_formation_provisioned_product::ProvisioningPreferences>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationArns" => {
                            notification_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PathId" => {
                            path_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PathName" => {
                            path_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductId" => {
                            product_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductName" => {
                            product_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisionedProductName" => {
                            provisioned_product_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisioningArtifactId" => {
                            provisioning_artifact_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisioningArtifactName" => {
                            provisioning_artifact_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisioningParameters" => {
                            provisioning_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisioningPreferences" => {
                            provisioning_preferences = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CloudFormationProvisionedProductProperties {
                    accept_language: accept_language,
                    notification_arns: notification_arns,
                    path_id: path_id,
                    path_name: path_name,
                    product_id: product_id,
                    product_name: product_name,
                    provisioned_product_name: provisioned_product_name,
                    provisioning_artifact_id: provisioning_artifact_id,
                    provisioning_artifact_name: provisioning_artifact_name,
                    provisioning_parameters: provisioning_parameters,
                    provisioning_preferences: provisioning_preferences,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CloudFormationProvisionedProduct {
    type Properties = CloudFormationProvisionedProductProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::CloudFormationProvisionedProduct";
    fn properties(&self) -> &CloudFormationProvisionedProductProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CloudFormationProvisionedProductProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CloudFormationProvisionedProduct {}

impl From<CloudFormationProvisionedProductProperties> for CloudFormationProvisionedProduct {
    fn from(properties: CloudFormationProvisionedProductProperties) -> CloudFormationProvisionedProduct {
        CloudFormationProvisionedProduct { properties }
    }
}

/// The [`AWS::ServiceCatalog::LaunchNotificationConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchnotificationconstraint.html) resource type.
#[derive(Debug, Default)]
pub struct LaunchNotificationConstraint {
    properties: LaunchNotificationConstraintProperties
}

/// Properties for the `LaunchNotificationConstraint` resource.
#[derive(Debug, Default)]
pub struct LaunchNotificationConstraintProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchnotificationconstraint.html#cfn-servicecatalog-launchnotificationconstraint-acceptlanguage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchnotificationconstraint.html#cfn-servicecatalog-launchnotificationconstraint-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`NotificationArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchnotificationconstraint.html#cfn-servicecatalog-launchnotificationconstraint-notificationarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_arns: ::ValueList<String>,
    /// Property [`PortfolioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchnotificationconstraint.html#cfn-servicecatalog-launchnotificationconstraint-portfolioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portfolio_id: ::Value<String>,
    /// Property [`ProductId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchnotificationconstraint.html#cfn-servicecatalog-launchnotificationconstraint-productid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub product_id: ::Value<String>,
}

impl ::serde::Serialize for LaunchNotificationConstraintProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationArns", &self.notification_arns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortfolioId", &self.portfolio_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductId", &self.product_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LaunchNotificationConstraintProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchNotificationConstraintProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LaunchNotificationConstraintProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LaunchNotificationConstraintProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut notification_arns: Option<::ValueList<String>> = None;
                let mut portfolio_id: Option<::Value<String>> = None;
                let mut product_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationArns" => {
                            notification_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortfolioId" => {
                            portfolio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductId" => {
                            product_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LaunchNotificationConstraintProperties {
                    accept_language: accept_language,
                    description: description,
                    notification_arns: notification_arns.ok_or(::serde::de::Error::missing_field("NotificationArns"))?,
                    portfolio_id: portfolio_id.ok_or(::serde::de::Error::missing_field("PortfolioId"))?,
                    product_id: product_id.ok_or(::serde::de::Error::missing_field("ProductId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LaunchNotificationConstraint {
    type Properties = LaunchNotificationConstraintProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::LaunchNotificationConstraint";
    fn properties(&self) -> &LaunchNotificationConstraintProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LaunchNotificationConstraintProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LaunchNotificationConstraint {}

impl From<LaunchNotificationConstraintProperties> for LaunchNotificationConstraint {
    fn from(properties: LaunchNotificationConstraintProperties) -> LaunchNotificationConstraint {
        LaunchNotificationConstraint { properties }
    }
}

/// The [`AWS::ServiceCatalog::LaunchRoleConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchroleconstraint.html) resource type.
#[derive(Debug, Default)]
pub struct LaunchRoleConstraint {
    properties: LaunchRoleConstraintProperties
}

/// Properties for the `LaunchRoleConstraint` resource.
#[derive(Debug, Default)]
pub struct LaunchRoleConstraintProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchroleconstraint.html#cfn-servicecatalog-launchroleconstraint-acceptlanguage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchroleconstraint.html#cfn-servicecatalog-launchroleconstraint-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`LocalRoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchroleconstraint.html#cfn-servicecatalog-launchroleconstraint-localrolename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub local_role_name: Option<::Value<String>>,
    /// Property [`PortfolioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchroleconstraint.html#cfn-servicecatalog-launchroleconstraint-portfolioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portfolio_id: ::Value<String>,
    /// Property [`ProductId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchroleconstraint.html#cfn-servicecatalog-launchroleconstraint-productid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub product_id: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchroleconstraint.html#cfn-servicecatalog-launchroleconstraint-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for LaunchRoleConstraintProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref local_role_name) = self.local_role_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalRoleName", local_role_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortfolioId", &self.portfolio_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductId", &self.product_id)?;
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LaunchRoleConstraintProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchRoleConstraintProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LaunchRoleConstraintProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LaunchRoleConstraintProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut local_role_name: Option<::Value<String>> = None;
                let mut portfolio_id: Option<::Value<String>> = None;
                let mut product_id: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LocalRoleName" => {
                            local_role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortfolioId" => {
                            portfolio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductId" => {
                            product_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LaunchRoleConstraintProperties {
                    accept_language: accept_language,
                    description: description,
                    local_role_name: local_role_name,
                    portfolio_id: portfolio_id.ok_or(::serde::de::Error::missing_field("PortfolioId"))?,
                    product_id: product_id.ok_or(::serde::de::Error::missing_field("ProductId"))?,
                    role_arn: role_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LaunchRoleConstraint {
    type Properties = LaunchRoleConstraintProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::LaunchRoleConstraint";
    fn properties(&self) -> &LaunchRoleConstraintProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LaunchRoleConstraintProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LaunchRoleConstraint {}

impl From<LaunchRoleConstraintProperties> for LaunchRoleConstraint {
    fn from(properties: LaunchRoleConstraintProperties) -> LaunchRoleConstraint {
        LaunchRoleConstraint { properties }
    }
}

/// The [`AWS::ServiceCatalog::LaunchTemplateConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchtemplateconstraint.html) resource type.
#[derive(Debug, Default)]
pub struct LaunchTemplateConstraint {
    properties: LaunchTemplateConstraintProperties
}

/// Properties for the `LaunchTemplateConstraint` resource.
#[derive(Debug, Default)]
pub struct LaunchTemplateConstraintProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchtemplateconstraint.html#cfn-servicecatalog-launchtemplateconstraint-acceptlanguage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchtemplateconstraint.html#cfn-servicecatalog-launchtemplateconstraint-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`PortfolioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchtemplateconstraint.html#cfn-servicecatalog-launchtemplateconstraint-portfolioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portfolio_id: ::Value<String>,
    /// Property [`ProductId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchtemplateconstraint.html#cfn-servicecatalog-launchtemplateconstraint-productid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub product_id: ::Value<String>,
    /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-launchtemplateconstraint.html#cfn-servicecatalog-launchtemplateconstraint-rules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rules: ::Value<String>,
}

impl ::serde::Serialize for LaunchTemplateConstraintProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortfolioId", &self.portfolio_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductId", &self.product_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LaunchTemplateConstraintProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LaunchTemplateConstraintProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LaunchTemplateConstraintProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LaunchTemplateConstraintProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut portfolio_id: Option<::Value<String>> = None;
                let mut product_id: Option<::Value<String>> = None;
                let mut rules: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortfolioId" => {
                            portfolio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductId" => {
                            product_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Rules" => {
                            rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LaunchTemplateConstraintProperties {
                    accept_language: accept_language,
                    description: description,
                    portfolio_id: portfolio_id.ok_or(::serde::de::Error::missing_field("PortfolioId"))?,
                    product_id: product_id.ok_or(::serde::de::Error::missing_field("ProductId"))?,
                    rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LaunchTemplateConstraint {
    type Properties = LaunchTemplateConstraintProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::LaunchTemplateConstraint";
    fn properties(&self) -> &LaunchTemplateConstraintProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LaunchTemplateConstraintProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LaunchTemplateConstraint {}

impl From<LaunchTemplateConstraintProperties> for LaunchTemplateConstraint {
    fn from(properties: LaunchTemplateConstraintProperties) -> LaunchTemplateConstraint {
        LaunchTemplateConstraint { properties }
    }
}

/// The [`AWS::ServiceCatalog::Portfolio`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolio.html) resource type.
#[derive(Debug, Default)]
pub struct Portfolio {
    properties: PortfolioProperties
}

/// Properties for the `Portfolio` resource.
#[derive(Debug, Default)]
pub struct PortfolioProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolio.html#cfn-servicecatalog-portfolio-acceptlanguage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolio.html#cfn-servicecatalog-portfolio-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolio.html#cfn-servicecatalog-portfolio-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: ::Value<String>,
    /// Property [`ProviderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolio.html#cfn-servicecatalog-portfolio-providername).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provider_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolio.html#cfn-servicecatalog-portfolio-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PortfolioProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", &self.display_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderName", &self.provider_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PortfolioProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PortfolioProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PortfolioProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PortfolioProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut provider_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProviderName" => {
                            provider_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PortfolioProperties {
                    accept_language: accept_language,
                    description: description,
                    display_name: display_name.ok_or(::serde::de::Error::missing_field("DisplayName"))?,
                    provider_name: provider_name.ok_or(::serde::de::Error::missing_field("ProviderName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Portfolio {
    type Properties = PortfolioProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::Portfolio";
    fn properties(&self) -> &PortfolioProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PortfolioProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Portfolio {}

impl From<PortfolioProperties> for Portfolio {
    fn from(properties: PortfolioProperties) -> Portfolio {
        Portfolio { properties }
    }
}

/// The [`AWS::ServiceCatalog::PortfolioPrincipalAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioprincipalassociation.html) resource type.
#[derive(Debug, Default)]
pub struct PortfolioPrincipalAssociation {
    properties: PortfolioPrincipalAssociationProperties
}

/// Properties for the `PortfolioPrincipalAssociation` resource.
#[derive(Debug, Default)]
pub struct PortfolioPrincipalAssociationProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioprincipalassociation.html#cfn-servicecatalog-portfolioprincipalassociation-acceptlanguage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`PortfolioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioprincipalassociation.html#cfn-servicecatalog-portfolioprincipalassociation-portfolioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portfolio_id: ::Value<String>,
    /// Property [`PrincipalARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioprincipalassociation.html#cfn-servicecatalog-portfolioprincipalassociation-principalarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal_arn: ::Value<String>,
    /// Property [`PrincipalType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioprincipalassociation.html#cfn-servicecatalog-portfolioprincipalassociation-principaltype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal_type: ::Value<String>,
}

impl ::serde::Serialize for PortfolioPrincipalAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortfolioId", &self.portfolio_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrincipalARN", &self.principal_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrincipalType", &self.principal_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PortfolioPrincipalAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PortfolioPrincipalAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PortfolioPrincipalAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PortfolioPrincipalAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut portfolio_id: Option<::Value<String>> = None;
                let mut principal_arn: Option<::Value<String>> = None;
                let mut principal_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortfolioId" => {
                            portfolio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrincipalARN" => {
                            principal_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrincipalType" => {
                            principal_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PortfolioPrincipalAssociationProperties {
                    accept_language: accept_language,
                    portfolio_id: portfolio_id.ok_or(::serde::de::Error::missing_field("PortfolioId"))?,
                    principal_arn: principal_arn.ok_or(::serde::de::Error::missing_field("PrincipalARN"))?,
                    principal_type: principal_type.ok_or(::serde::de::Error::missing_field("PrincipalType"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PortfolioPrincipalAssociation {
    type Properties = PortfolioPrincipalAssociationProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::PortfolioPrincipalAssociation";
    fn properties(&self) -> &PortfolioPrincipalAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PortfolioPrincipalAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PortfolioPrincipalAssociation {}

impl From<PortfolioPrincipalAssociationProperties> for PortfolioPrincipalAssociation {
    fn from(properties: PortfolioPrincipalAssociationProperties) -> PortfolioPrincipalAssociation {
        PortfolioPrincipalAssociation { properties }
    }
}

/// The [`AWS::ServiceCatalog::PortfolioProductAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioproductassociation.html) resource type.
#[derive(Debug, Default)]
pub struct PortfolioProductAssociation {
    properties: PortfolioProductAssociationProperties
}

/// Properties for the `PortfolioProductAssociation` resource.
#[derive(Debug, Default)]
pub struct PortfolioProductAssociationProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioproductassociation.html#cfn-servicecatalog-portfolioproductassociation-acceptlanguage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`PortfolioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioproductassociation.html#cfn-servicecatalog-portfolioproductassociation-portfolioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portfolio_id: ::Value<String>,
    /// Property [`ProductId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioproductassociation.html#cfn-servicecatalog-portfolioproductassociation-productid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub product_id: ::Value<String>,
    /// Property [`SourcePortfolioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioproductassociation.html#cfn-servicecatalog-portfolioproductassociation-sourceportfolioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_portfolio_id: Option<::Value<String>>,
}

impl ::serde::Serialize for PortfolioProductAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortfolioId", &self.portfolio_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductId", &self.product_id)?;
        if let Some(ref source_portfolio_id) = self.source_portfolio_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePortfolioId", source_portfolio_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PortfolioProductAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PortfolioProductAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PortfolioProductAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PortfolioProductAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut portfolio_id: Option<::Value<String>> = None;
                let mut product_id: Option<::Value<String>> = None;
                let mut source_portfolio_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortfolioId" => {
                            portfolio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductId" => {
                            product_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourcePortfolioId" => {
                            source_portfolio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PortfolioProductAssociationProperties {
                    accept_language: accept_language,
                    portfolio_id: portfolio_id.ok_or(::serde::de::Error::missing_field("PortfolioId"))?,
                    product_id: product_id.ok_or(::serde::de::Error::missing_field("ProductId"))?,
                    source_portfolio_id: source_portfolio_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PortfolioProductAssociation {
    type Properties = PortfolioProductAssociationProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::PortfolioProductAssociation";
    fn properties(&self) -> &PortfolioProductAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PortfolioProductAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PortfolioProductAssociation {}

impl From<PortfolioProductAssociationProperties> for PortfolioProductAssociation {
    fn from(properties: PortfolioProductAssociationProperties) -> PortfolioProductAssociation {
        PortfolioProductAssociation { properties }
    }
}

/// The [`AWS::ServiceCatalog::PortfolioShare`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioshare.html) resource type.
#[derive(Debug, Default)]
pub struct PortfolioShare {
    properties: PortfolioShareProperties
}

/// Properties for the `PortfolioShare` resource.
#[derive(Debug, Default)]
pub struct PortfolioShareProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioshare.html#cfn-servicecatalog-portfolioshare-acceptlanguage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`AccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioshare.html#cfn-servicecatalog-portfolioshare-accountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub account_id: ::Value<String>,
    /// Property [`PortfolioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioshare.html#cfn-servicecatalog-portfolioshare-portfolioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portfolio_id: ::Value<String>,
    /// Property [`ShareTagOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-portfolioshare.html#cfn-servicecatalog-portfolioshare-sharetagoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub share_tag_options: Option<::Value<bool>>,
}

impl ::serde::Serialize for PortfolioShareProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountId", &self.account_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortfolioId", &self.portfolio_id)?;
        if let Some(ref share_tag_options) = self.share_tag_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShareTagOptions", share_tag_options)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PortfolioShareProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PortfolioShareProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PortfolioShareProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PortfolioShareProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut account_id: Option<::Value<String>> = None;
                let mut portfolio_id: Option<::Value<String>> = None;
                let mut share_tag_options: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AccountId" => {
                            account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortfolioId" => {
                            portfolio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ShareTagOptions" => {
                            share_tag_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PortfolioShareProperties {
                    accept_language: accept_language,
                    account_id: account_id.ok_or(::serde::de::Error::missing_field("AccountId"))?,
                    portfolio_id: portfolio_id.ok_or(::serde::de::Error::missing_field("PortfolioId"))?,
                    share_tag_options: share_tag_options,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PortfolioShare {
    type Properties = PortfolioShareProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::PortfolioShare";
    fn properties(&self) -> &PortfolioShareProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PortfolioShareProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PortfolioShare {}

impl From<PortfolioShareProperties> for PortfolioShare {
    fn from(properties: PortfolioShareProperties) -> PortfolioShare {
        PortfolioShare { properties }
    }
}

/// The [`AWS::ServiceCatalog::ResourceUpdateConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-resourceupdateconstraint.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceUpdateConstraint {
    properties: ResourceUpdateConstraintProperties
}

/// Properties for the `ResourceUpdateConstraint` resource.
#[derive(Debug, Default)]
pub struct ResourceUpdateConstraintProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-resourceupdateconstraint.html#cfn-servicecatalog-resourceupdateconstraint-acceptlanguage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-resourceupdateconstraint.html#cfn-servicecatalog-resourceupdateconstraint-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`PortfolioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-resourceupdateconstraint.html#cfn-servicecatalog-resourceupdateconstraint-portfolioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portfolio_id: ::Value<String>,
    /// Property [`ProductId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-resourceupdateconstraint.html#cfn-servicecatalog-resourceupdateconstraint-productid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub product_id: ::Value<String>,
    /// Property [`TagUpdateOnProvisionedProduct`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-resourceupdateconstraint.html#cfn-servicecatalog-resourceupdateconstraint-tagupdateonprovisionedproduct).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tag_update_on_provisioned_product: ::Value<String>,
}

impl ::serde::Serialize for ResourceUpdateConstraintProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortfolioId", &self.portfolio_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductId", &self.product_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagUpdateOnProvisionedProduct", &self.tag_update_on_provisioned_product)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceUpdateConstraintProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceUpdateConstraintProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceUpdateConstraintProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceUpdateConstraintProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut portfolio_id: Option<::Value<String>> = None;
                let mut product_id: Option<::Value<String>> = None;
                let mut tag_update_on_provisioned_product: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortfolioId" => {
                            portfolio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductId" => {
                            product_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TagUpdateOnProvisionedProduct" => {
                            tag_update_on_provisioned_product = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceUpdateConstraintProperties {
                    accept_language: accept_language,
                    description: description,
                    portfolio_id: portfolio_id.ok_or(::serde::de::Error::missing_field("PortfolioId"))?,
                    product_id: product_id.ok_or(::serde::de::Error::missing_field("ProductId"))?,
                    tag_update_on_provisioned_product: tag_update_on_provisioned_product.ok_or(::serde::de::Error::missing_field("TagUpdateOnProvisionedProduct"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceUpdateConstraint {
    type Properties = ResourceUpdateConstraintProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::ResourceUpdateConstraint";
    fn properties(&self) -> &ResourceUpdateConstraintProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceUpdateConstraintProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceUpdateConstraint {}

impl From<ResourceUpdateConstraintProperties> for ResourceUpdateConstraint {
    fn from(properties: ResourceUpdateConstraintProperties) -> ResourceUpdateConstraint {
        ResourceUpdateConstraint { properties }
    }
}

/// The [`AWS::ServiceCatalog::ServiceAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceaction.html) resource type.
#[derive(Debug, Default)]
pub struct ServiceAction {
    properties: ServiceActionProperties
}

/// Properties for the `ServiceAction` resource.
#[derive(Debug, Default)]
pub struct ServiceActionProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceaction.html#cfn-servicecatalog-serviceaction-acceptlanguage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceaction.html#cfn-servicecatalog-serviceaction-definition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub definition: ::ValueList<self::service_action::DefinitionParameter>,
    /// Property [`DefinitionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceaction.html#cfn-servicecatalog-serviceaction-definitiontype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub definition_type: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceaction.html#cfn-servicecatalog-serviceaction-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceaction.html#cfn-servicecatalog-serviceaction-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for ServiceActionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Definition", &self.definition)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefinitionType", &self.definition_type)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServiceActionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceActionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceActionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServiceActionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut definition: Option<::ValueList<self::service_action::DefinitionParameter>> = None;
                let mut definition_type: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Definition" => {
                            definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefinitionType" => {
                            definition_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServiceActionProperties {
                    accept_language: accept_language,
                    definition: definition.ok_or(::serde::de::Error::missing_field("Definition"))?,
                    definition_type: definition_type.ok_or(::serde::de::Error::missing_field("DefinitionType"))?,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ServiceAction {
    type Properties = ServiceActionProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::ServiceAction";
    fn properties(&self) -> &ServiceActionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServiceActionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ServiceAction {}

impl From<ServiceActionProperties> for ServiceAction {
    fn from(properties: ServiceActionProperties) -> ServiceAction {
        ServiceAction { properties }
    }
}

/// The [`AWS::ServiceCatalog::ServiceActionAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceactionassociation.html) resource type.
#[derive(Debug, Default)]
pub struct ServiceActionAssociation {
    properties: ServiceActionAssociationProperties
}

/// Properties for the `ServiceActionAssociation` resource.
#[derive(Debug, Default)]
pub struct ServiceActionAssociationProperties {
    /// Property [`ProductId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceactionassociation.html#cfn-servicecatalog-serviceactionassociation-productid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub product_id: ::Value<String>,
    /// Property [`ProvisioningArtifactId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceactionassociation.html#cfn-servicecatalog-serviceactionassociation-provisioningartifactid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub provisioning_artifact_id: ::Value<String>,
    /// Property [`ServiceActionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-serviceactionassociation.html#cfn-servicecatalog-serviceactionassociation-serviceactionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_action_id: ::Value<String>,
}

impl ::serde::Serialize for ServiceActionAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductId", &self.product_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisioningArtifactId", &self.provisioning_artifact_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceActionId", &self.service_action_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServiceActionAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceActionAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceActionAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServiceActionAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut product_id: Option<::Value<String>> = None;
                let mut provisioning_artifact_id: Option<::Value<String>> = None;
                let mut service_action_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ProductId" => {
                            product_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisioningArtifactId" => {
                            provisioning_artifact_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceActionId" => {
                            service_action_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServiceActionAssociationProperties {
                    product_id: product_id.ok_or(::serde::de::Error::missing_field("ProductId"))?,
                    provisioning_artifact_id: provisioning_artifact_id.ok_or(::serde::de::Error::missing_field("ProvisioningArtifactId"))?,
                    service_action_id: service_action_id.ok_or(::serde::de::Error::missing_field("ServiceActionId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ServiceActionAssociation {
    type Properties = ServiceActionAssociationProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::ServiceActionAssociation";
    fn properties(&self) -> &ServiceActionAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServiceActionAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ServiceActionAssociation {}

impl From<ServiceActionAssociationProperties> for ServiceActionAssociation {
    fn from(properties: ServiceActionAssociationProperties) -> ServiceActionAssociation {
        ServiceActionAssociation { properties }
    }
}

/// The [`AWS::ServiceCatalog::StackSetConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html) resource type.
#[derive(Debug, Default)]
pub struct StackSetConstraint {
    properties: StackSetConstraintProperties
}

/// Properties for the `StackSetConstraint` resource.
#[derive(Debug, Default)]
pub struct StackSetConstraintProperties {
    /// Property [`AcceptLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html#cfn-servicecatalog-stacksetconstraint-acceptlanguage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accept_language: Option<::Value<String>>,
    /// Property [`AccountList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html#cfn-servicecatalog-stacksetconstraint-accountlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub account_list: ::ValueList<String>,
    /// Property [`AdminRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html#cfn-servicecatalog-stacksetconstraint-adminrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub admin_role: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html#cfn-servicecatalog-stacksetconstraint-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`ExecutionRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html#cfn-servicecatalog-stacksetconstraint-executionrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub execution_role: ::Value<String>,
    /// Property [`PortfolioId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html#cfn-servicecatalog-stacksetconstraint-portfolioid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portfolio_id: ::Value<String>,
    /// Property [`ProductId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html#cfn-servicecatalog-stacksetconstraint-productid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub product_id: ::Value<String>,
    /// Property [`RegionList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html#cfn-servicecatalog-stacksetconstraint-regionlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub region_list: ::ValueList<String>,
    /// Property [`StackInstanceControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-stacksetconstraint.html#cfn-servicecatalog-stacksetconstraint-stackinstancecontrol).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stack_instance_control: ::Value<String>,
}

impl ::serde::Serialize for StackSetConstraintProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accept_language) = self.accept_language {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceptLanguage", accept_language)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountList", &self.account_list)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminRole", &self.admin_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRole", &self.execution_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortfolioId", &self.portfolio_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductId", &self.product_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionList", &self.region_list)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackInstanceControl", &self.stack_instance_control)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StackSetConstraintProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StackSetConstraintProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StackSetConstraintProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StackSetConstraintProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accept_language: Option<::Value<String>> = None;
                let mut account_list: Option<::ValueList<String>> = None;
                let mut admin_role: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut execution_role: Option<::Value<String>> = None;
                let mut portfolio_id: Option<::Value<String>> = None;
                let mut product_id: Option<::Value<String>> = None;
                let mut region_list: Option<::ValueList<String>> = None;
                let mut stack_instance_control: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceptLanguage" => {
                            accept_language = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AccountList" => {
                            account_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdminRole" => {
                            admin_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionRole" => {
                            execution_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortfolioId" => {
                            portfolio_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductId" => {
                            product_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RegionList" => {
                            region_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackInstanceControl" => {
                            stack_instance_control = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StackSetConstraintProperties {
                    accept_language: accept_language,
                    account_list: account_list.ok_or(::serde::de::Error::missing_field("AccountList"))?,
                    admin_role: admin_role.ok_or(::serde::de::Error::missing_field("AdminRole"))?,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    execution_role: execution_role.ok_or(::serde::de::Error::missing_field("ExecutionRole"))?,
                    portfolio_id: portfolio_id.ok_or(::serde::de::Error::missing_field("PortfolioId"))?,
                    product_id: product_id.ok_or(::serde::de::Error::missing_field("ProductId"))?,
                    region_list: region_list.ok_or(::serde::de::Error::missing_field("RegionList"))?,
                    stack_instance_control: stack_instance_control.ok_or(::serde::de::Error::missing_field("StackInstanceControl"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StackSetConstraint {
    type Properties = StackSetConstraintProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::StackSetConstraint";
    fn properties(&self) -> &StackSetConstraintProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StackSetConstraintProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StackSetConstraint {}

impl From<StackSetConstraintProperties> for StackSetConstraint {
    fn from(properties: StackSetConstraintProperties) -> StackSetConstraint {
        StackSetConstraint { properties }
    }
}

/// The [`AWS::ServiceCatalog::TagOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-tagoption.html) resource type.
#[derive(Debug, Default)]
pub struct TagOption {
    properties: TagOptionProperties
}

/// Properties for the `TagOption` resource.
#[derive(Debug, Default)]
pub struct TagOptionProperties {
    /// Property [`Active`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-tagoption.html#cfn-servicecatalog-tagoption-active).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub active: Option<::Value<bool>>,
    /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-tagoption.html#cfn-servicecatalog-tagoption-key).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key: ::Value<String>,
    /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-tagoption.html#cfn-servicecatalog-tagoption-value).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub value: ::Value<String>,
}

impl ::serde::Serialize for TagOptionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref active) = self.active {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Active", active)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TagOptionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TagOptionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TagOptionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TagOptionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut active: Option<::Value<bool>> = None;
                let mut key: Option<::Value<String>> = None;
                let mut value: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Active" => {
                            active = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Key" => {
                            key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Value" => {
                            value = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TagOptionProperties {
                    active: active,
                    key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                    value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TagOption {
    type Properties = TagOptionProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::TagOption";
    fn properties(&self) -> &TagOptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TagOptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TagOption {}

impl From<TagOptionProperties> for TagOption {
    fn from(properties: TagOptionProperties) -> TagOption {
        TagOption { properties }
    }
}

/// The [`AWS::ServiceCatalog::TagOptionAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-tagoptionassociation.html) resource type.
#[derive(Debug, Default)]
pub struct TagOptionAssociation {
    properties: TagOptionAssociationProperties
}

/// Properties for the `TagOptionAssociation` resource.
#[derive(Debug, Default)]
pub struct TagOptionAssociationProperties {
    /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-tagoptionassociation.html#cfn-servicecatalog-tagoptionassociation-resourceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_id: ::Value<String>,
    /// Property [`TagOptionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalog-tagoptionassociation.html#cfn-servicecatalog-tagoptionassociation-tagoptionid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tag_option_id: ::Value<String>,
}

impl ::serde::Serialize for TagOptionAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", &self.resource_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagOptionId", &self.tag_option_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TagOptionAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TagOptionAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TagOptionAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TagOptionAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resource_id: Option<::Value<String>> = None;
                let mut tag_option_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceId" => {
                            resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TagOptionId" => {
                            tag_option_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TagOptionAssociationProperties {
                    resource_id: resource_id.ok_or(::serde::de::Error::missing_field("ResourceId"))?,
                    tag_option_id: tag_option_id.ok_or(::serde::de::Error::missing_field("TagOptionId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TagOptionAssociation {
    type Properties = TagOptionAssociationProperties;
    const TYPE: &'static str = "AWS::ServiceCatalog::TagOptionAssociation";
    fn properties(&self) -> &TagOptionAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TagOptionAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TagOptionAssociation {}

impl From<TagOptionAssociationProperties> for TagOptionAssociation {
    fn from(properties: TagOptionAssociationProperties) -> TagOptionAssociation {
        TagOptionAssociation { properties }
    }
}

pub mod cloud_formation_product {
    //! Property types for the `CloudFormationProduct` resource.

    /// The [`AWS::ServiceCatalog::CloudFormationProduct.ProvisioningArtifactProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationproduct-provisioningartifactproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisioningArtifactProperties {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationproduct-provisioningartifactproperties.html#cfn-servicecatalog-cloudformationproduct-provisioningartifactproperties-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`DisableTemplateValidation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationproduct-provisioningartifactproperties.html#cfn-servicecatalog-cloudformationproduct-provisioningartifactproperties-disabletemplatevalidation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_template_validation: Option<::Value<bool>>,
        /// Property [`Info`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationproduct-provisioningartifactproperties.html#cfn-servicecatalog-cloudformationproduct-provisioningartifactproperties-info).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub info: ::Value<::json::Value>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationproduct-provisioningartifactproperties.html#cfn-servicecatalog-cloudformationproduct-provisioningartifactproperties-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ProvisioningArtifactProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref disable_template_validation) = self.disable_template_validation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableTemplateValidation", disable_template_validation)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Info", &self.info)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProvisioningArtifactProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisioningArtifactProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisioningArtifactProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisioningArtifactProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut disable_template_validation: Option<::Value<bool>> = None;
                    let mut info: Option<::Value<::json::Value>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisableTemplateValidation" => {
                                disable_template_validation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Info" => {
                                info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisioningArtifactProperties {
                        description: description,
                        disable_template_validation: disable_template_validation,
                        info: info.ok_or(::serde::de::Error::missing_field("Info"))?,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod cloud_formation_provisioned_product {
    //! Property types for the `CloudFormationProvisionedProduct` resource.

    /// The [`AWS::ServiceCatalog::CloudFormationProvisionedProduct.ProvisioningParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisioningParameter {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningparameter.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningparameter-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningparameter.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningparameter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ProvisioningParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProvisioningParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisioningParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisioningParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisioningParameter")
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

                    Ok(ProvisioningParameter {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ServiceCatalog::CloudFormationProvisionedProduct.ProvisioningPreferences`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisioningPreferences {
        /// Property [`StackSetAccounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences-stacksetaccounts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stack_set_accounts: Option<::ValueList<String>>,
        /// Property [`StackSetFailureToleranceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences-stacksetfailuretolerancecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stack_set_failure_tolerance_count: Option<::Value<u32>>,
        /// Property [`StackSetFailureTolerancePercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences-stacksetfailuretolerancepercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stack_set_failure_tolerance_percentage: Option<::Value<u32>>,
        /// Property [`StackSetMaxConcurrencyCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences-stacksetmaxconcurrencycount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stack_set_max_concurrency_count: Option<::Value<u32>>,
        /// Property [`StackSetMaxConcurrencyPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences-stacksetmaxconcurrencypercentage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stack_set_max_concurrency_percentage: Option<::Value<u32>>,
        /// Property [`StackSetOperationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences-stacksetoperationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stack_set_operation_type: Option<::Value<String>>,
        /// Property [`StackSetRegions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences.html#cfn-servicecatalog-cloudformationprovisionedproduct-provisioningpreferences-stacksetregions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stack_set_regions: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ProvisioningPreferences {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref stack_set_accounts) = self.stack_set_accounts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackSetAccounts", stack_set_accounts)?;
            }
            if let Some(ref stack_set_failure_tolerance_count) = self.stack_set_failure_tolerance_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackSetFailureToleranceCount", stack_set_failure_tolerance_count)?;
            }
            if let Some(ref stack_set_failure_tolerance_percentage) = self.stack_set_failure_tolerance_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackSetFailureTolerancePercentage", stack_set_failure_tolerance_percentage)?;
            }
            if let Some(ref stack_set_max_concurrency_count) = self.stack_set_max_concurrency_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackSetMaxConcurrencyCount", stack_set_max_concurrency_count)?;
            }
            if let Some(ref stack_set_max_concurrency_percentage) = self.stack_set_max_concurrency_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackSetMaxConcurrencyPercentage", stack_set_max_concurrency_percentage)?;
            }
            if let Some(ref stack_set_operation_type) = self.stack_set_operation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackSetOperationType", stack_set_operation_type)?;
            }
            if let Some(ref stack_set_regions) = self.stack_set_regions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackSetRegions", stack_set_regions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProvisioningPreferences {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisioningPreferences, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisioningPreferences;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisioningPreferences")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut stack_set_accounts: Option<::ValueList<String>> = None;
                    let mut stack_set_failure_tolerance_count: Option<::Value<u32>> = None;
                    let mut stack_set_failure_tolerance_percentage: Option<::Value<u32>> = None;
                    let mut stack_set_max_concurrency_count: Option<::Value<u32>> = None;
                    let mut stack_set_max_concurrency_percentage: Option<::Value<u32>> = None;
                    let mut stack_set_operation_type: Option<::Value<String>> = None;
                    let mut stack_set_regions: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StackSetAccounts" => {
                                stack_set_accounts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StackSetFailureToleranceCount" => {
                                stack_set_failure_tolerance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StackSetFailureTolerancePercentage" => {
                                stack_set_failure_tolerance_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StackSetMaxConcurrencyCount" => {
                                stack_set_max_concurrency_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StackSetMaxConcurrencyPercentage" => {
                                stack_set_max_concurrency_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StackSetOperationType" => {
                                stack_set_operation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StackSetRegions" => {
                                stack_set_regions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisioningPreferences {
                        stack_set_accounts: stack_set_accounts,
                        stack_set_failure_tolerance_count: stack_set_failure_tolerance_count,
                        stack_set_failure_tolerance_percentage: stack_set_failure_tolerance_percentage,
                        stack_set_max_concurrency_count: stack_set_max_concurrency_count,
                        stack_set_max_concurrency_percentage: stack_set_max_concurrency_percentage,
                        stack_set_operation_type: stack_set_operation_type,
                        stack_set_regions: stack_set_regions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod service_action {
    //! Property types for the `ServiceAction` resource.

    /// The [`AWS::ServiceCatalog::ServiceAction.DefinitionParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-serviceaction-definitionparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct DefinitionParameter {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-serviceaction-definitionparameter.html#cfn-servicecatalog-serviceaction-definitionparameter-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicecatalog-serviceaction-definitionparameter.html#cfn-servicecatalog-serviceaction-definitionparameter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for DefinitionParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefinitionParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefinitionParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefinitionParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefinitionParameter")
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

                    Ok(DefinitionParameter {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
