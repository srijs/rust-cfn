//! Types for the `InspectorV2` service.

/// The [`AWS::InspectorV2::Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspectorv2-filter.html) resource type.
#[derive(Debug, Default)]
pub struct Filter {
    properties: FilterProperties
}

/// Properties for the `Filter` resource.
#[derive(Debug, Default)]
pub struct FilterProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspectorv2-filter.html#cfn-inspectorv2-filter-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`FilterAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspectorv2-filter.html#cfn-inspectorv2-filter-filteraction).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub filter_action: ::Value<String>,
    /// Property [`FilterCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspectorv2-filter.html#cfn-inspectorv2-filter-filtercriteria).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub filter_criteria: ::Value<self::filter::FilterCriteria>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-inspectorv2-filter.html#cfn-inspectorv2-filter-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for FilterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterAction", &self.filter_action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterCriteria", &self.filter_criteria)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FilterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FilterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FilterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut filter_action: Option<::Value<String>> = None;
                let mut filter_criteria: Option<::Value<self::filter::FilterCriteria>> = None;
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FilterAction" => {
                            filter_action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FilterCriteria" => {
                            filter_criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FilterProperties {
                    description: description,
                    filter_action: filter_action.ok_or(::serde::de::Error::missing_field("FilterAction"))?,
                    filter_criteria: filter_criteria.ok_or(::serde::de::Error::missing_field("FilterCriteria"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Filter {
    type Properties = FilterProperties;
    const TYPE: &'static str = "AWS::InspectorV2::Filter";
    fn properties(&self) -> &FilterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FilterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Filter {}

impl From<FilterProperties> for Filter {
    fn from(properties: FilterProperties) -> Filter {
        Filter { properties }
    }
}

pub mod filter {
    //! Property types for the `Filter` resource.

    /// The [`AWS::InspectorV2::Filter.DateFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-datefilter.html) property type.
    #[derive(Debug, Default)]
    pub struct DateFilter {
        /// Property [`EndInclusive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-datefilter.html#cfn-inspectorv2-filter-datefilter-endinclusive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_inclusive: Option<::Value<u32>>,
        /// Property [`StartInclusive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-datefilter.html#cfn-inspectorv2-filter-datefilter-startinclusive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_inclusive: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for DateFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref end_inclusive) = self.end_inclusive {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndInclusive", end_inclusive)?;
            }
            if let Some(ref start_inclusive) = self.start_inclusive {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartInclusive", start_inclusive)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DateFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DateFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DateFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DateFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end_inclusive: Option<::Value<u32>> = None;
                    let mut start_inclusive: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndInclusive" => {
                                end_inclusive = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartInclusive" => {
                                start_inclusive = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DateFilter {
                        end_inclusive: end_inclusive,
                        start_inclusive: start_inclusive,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::InspectorV2::Filter.FilterCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterCriteria {
        /// Property [`AwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-awsaccountid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_account_id: Option<::ValueList<StringFilter>>,
        /// Property [`ComponentId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-componentid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_id: Option<::ValueList<StringFilter>>,
        /// Property [`ComponentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-componenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_type: Option<::ValueList<StringFilter>>,
        /// Property [`Ec2InstanceImageId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-ec2instanceimageid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ec2_instance_image_id: Option<::ValueList<StringFilter>>,
        /// Property [`Ec2InstanceSubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-ec2instancesubnetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ec2_instance_subnet_id: Option<::ValueList<StringFilter>>,
        /// Property [`Ec2InstanceVpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-ec2instancevpcid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ec2_instance_vpc_id: Option<::ValueList<StringFilter>>,
        /// Property [`EcrImageArchitecture`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-ecrimagearchitecture).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecr_image_architecture: Option<::ValueList<StringFilter>>,
        /// Property [`EcrImageHash`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-ecrimagehash).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecr_image_hash: Option<::ValueList<StringFilter>>,
        /// Property [`EcrImagePushedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-ecrimagepushedat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecr_image_pushed_at: Option<::ValueList<DateFilter>>,
        /// Property [`EcrImageRegistry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-ecrimageregistry).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecr_image_registry: Option<::ValueList<StringFilter>>,
        /// Property [`EcrImageRepositoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-ecrimagerepositoryname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecr_image_repository_name: Option<::ValueList<StringFilter>>,
        /// Property [`EcrImageTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-ecrimagetags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ecr_image_tags: Option<::ValueList<StringFilter>>,
        /// Property [`FindingArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-findingarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub finding_arn: Option<::ValueList<StringFilter>>,
        /// Property [`FindingStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-findingstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub finding_status: Option<::ValueList<StringFilter>>,
        /// Property [`FindingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-findingtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub finding_type: Option<::ValueList<StringFilter>>,
        /// Property [`FirstObservedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-firstobservedat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub first_observed_at: Option<::ValueList<DateFilter>>,
        /// Property [`InspectorScore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-inspectorscore).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inspector_score: Option<::ValueList<NumberFilter>>,
        /// Property [`LastObservedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-lastobservedat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_observed_at: Option<::ValueList<DateFilter>>,
        /// Property [`NetworkProtocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-networkprotocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_protocol: Option<::ValueList<StringFilter>>,
        /// Property [`PortRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-portrange).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port_range: Option<::ValueList<PortRangeFilter>>,
        /// Property [`RelatedVulnerabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-relatedvulnerabilities).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub related_vulnerabilities: Option<::ValueList<StringFilter>>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-resourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id: Option<::ValueList<StringFilter>>,
        /// Property [`ResourceTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-resourcetags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_tags: Option<::ValueList<MapFilter>>,
        /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-resourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_type: Option<::ValueList<StringFilter>>,
        /// Property [`Severity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-severity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub severity: Option<::ValueList<StringFilter>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: Option<::ValueList<StringFilter>>,
        /// Property [`UpdatedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-updatedat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub updated_at: Option<::ValueList<DateFilter>>,
        /// Property [`VendorSeverity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-vendorseverity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vendor_severity: Option<::ValueList<StringFilter>>,
        /// Property [`VulnerabilityId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-vulnerabilityid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vulnerability_id: Option<::ValueList<StringFilter>>,
        /// Property [`VulnerabilitySource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-vulnerabilitysource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vulnerability_source: Option<::ValueList<StringFilter>>,
        /// Property [`VulnerablePackages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-filtercriteria.html#cfn-inspectorv2-filter-filtercriteria-vulnerablepackages).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vulnerable_packages: Option<::ValueList<PackageFilter>>,
    }

    impl ::codec::SerializeValue for FilterCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_account_id) = self.aws_account_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", aws_account_id)?;
            }
            if let Some(ref component_id) = self.component_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentId", component_id)?;
            }
            if let Some(ref component_type) = self.component_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentType", component_type)?;
            }
            if let Some(ref ec2_instance_image_id) = self.ec2_instance_image_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2InstanceImageId", ec2_instance_image_id)?;
            }
            if let Some(ref ec2_instance_subnet_id) = self.ec2_instance_subnet_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2InstanceSubnetId", ec2_instance_subnet_id)?;
            }
            if let Some(ref ec2_instance_vpc_id) = self.ec2_instance_vpc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2InstanceVpcId", ec2_instance_vpc_id)?;
            }
            if let Some(ref ecr_image_architecture) = self.ecr_image_architecture {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcrImageArchitecture", ecr_image_architecture)?;
            }
            if let Some(ref ecr_image_hash) = self.ecr_image_hash {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcrImageHash", ecr_image_hash)?;
            }
            if let Some(ref ecr_image_pushed_at) = self.ecr_image_pushed_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcrImagePushedAt", ecr_image_pushed_at)?;
            }
            if let Some(ref ecr_image_registry) = self.ecr_image_registry {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcrImageRegistry", ecr_image_registry)?;
            }
            if let Some(ref ecr_image_repository_name) = self.ecr_image_repository_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcrImageRepositoryName", ecr_image_repository_name)?;
            }
            if let Some(ref ecr_image_tags) = self.ecr_image_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcrImageTags", ecr_image_tags)?;
            }
            if let Some(ref finding_arn) = self.finding_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FindingArn", finding_arn)?;
            }
            if let Some(ref finding_status) = self.finding_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FindingStatus", finding_status)?;
            }
            if let Some(ref finding_type) = self.finding_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FindingType", finding_type)?;
            }
            if let Some(ref first_observed_at) = self.first_observed_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirstObservedAt", first_observed_at)?;
            }
            if let Some(ref inspector_score) = self.inspector_score {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InspectorScore", inspector_score)?;
            }
            if let Some(ref last_observed_at) = self.last_observed_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastObservedAt", last_observed_at)?;
            }
            if let Some(ref network_protocol) = self.network_protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkProtocol", network_protocol)?;
            }
            if let Some(ref port_range) = self.port_range {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortRange", port_range)?;
            }
            if let Some(ref related_vulnerabilities) = self.related_vulnerabilities {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelatedVulnerabilities", related_vulnerabilities)?;
            }
            if let Some(ref resource_id) = self.resource_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", resource_id)?;
            }
            if let Some(ref resource_tags) = self.resource_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTags", resource_tags)?;
            }
            if let Some(ref resource_type) = self.resource_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", resource_type)?;
            }
            if let Some(ref severity) = self.severity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Severity", severity)?;
            }
            if let Some(ref title) = self.title {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", title)?;
            }
            if let Some(ref updated_at) = self.updated_at {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdatedAt", updated_at)?;
            }
            if let Some(ref vendor_severity) = self.vendor_severity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VendorSeverity", vendor_severity)?;
            }
            if let Some(ref vulnerability_id) = self.vulnerability_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VulnerabilityId", vulnerability_id)?;
            }
            if let Some(ref vulnerability_source) = self.vulnerability_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VulnerabilitySource", vulnerability_source)?;
            }
            if let Some(ref vulnerable_packages) = self.vulnerable_packages {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VulnerablePackages", vulnerable_packages)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_account_id: Option<::ValueList<StringFilter>> = None;
                    let mut component_id: Option<::ValueList<StringFilter>> = None;
                    let mut component_type: Option<::ValueList<StringFilter>> = None;
                    let mut ec2_instance_image_id: Option<::ValueList<StringFilter>> = None;
                    let mut ec2_instance_subnet_id: Option<::ValueList<StringFilter>> = None;
                    let mut ec2_instance_vpc_id: Option<::ValueList<StringFilter>> = None;
                    let mut ecr_image_architecture: Option<::ValueList<StringFilter>> = None;
                    let mut ecr_image_hash: Option<::ValueList<StringFilter>> = None;
                    let mut ecr_image_pushed_at: Option<::ValueList<DateFilter>> = None;
                    let mut ecr_image_registry: Option<::ValueList<StringFilter>> = None;
                    let mut ecr_image_repository_name: Option<::ValueList<StringFilter>> = None;
                    let mut ecr_image_tags: Option<::ValueList<StringFilter>> = None;
                    let mut finding_arn: Option<::ValueList<StringFilter>> = None;
                    let mut finding_status: Option<::ValueList<StringFilter>> = None;
                    let mut finding_type: Option<::ValueList<StringFilter>> = None;
                    let mut first_observed_at: Option<::ValueList<DateFilter>> = None;
                    let mut inspector_score: Option<::ValueList<NumberFilter>> = None;
                    let mut last_observed_at: Option<::ValueList<DateFilter>> = None;
                    let mut network_protocol: Option<::ValueList<StringFilter>> = None;
                    let mut port_range: Option<::ValueList<PortRangeFilter>> = None;
                    let mut related_vulnerabilities: Option<::ValueList<StringFilter>> = None;
                    let mut resource_id: Option<::ValueList<StringFilter>> = None;
                    let mut resource_tags: Option<::ValueList<MapFilter>> = None;
                    let mut resource_type: Option<::ValueList<StringFilter>> = None;
                    let mut severity: Option<::ValueList<StringFilter>> = None;
                    let mut title: Option<::ValueList<StringFilter>> = None;
                    let mut updated_at: Option<::ValueList<DateFilter>> = None;
                    let mut vendor_severity: Option<::ValueList<StringFilter>> = None;
                    let mut vulnerability_id: Option<::ValueList<StringFilter>> = None;
                    let mut vulnerability_source: Option<::ValueList<StringFilter>> = None;
                    let mut vulnerable_packages: Option<::ValueList<PackageFilter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsAccountId" => {
                                aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentId" => {
                                component_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ComponentType" => {
                                component_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ec2InstanceImageId" => {
                                ec2_instance_image_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ec2InstanceSubnetId" => {
                                ec2_instance_subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ec2InstanceVpcId" => {
                                ec2_instance_vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcrImageArchitecture" => {
                                ecr_image_architecture = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcrImageHash" => {
                                ecr_image_hash = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcrImagePushedAt" => {
                                ecr_image_pushed_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcrImageRegistry" => {
                                ecr_image_registry = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcrImageRepositoryName" => {
                                ecr_image_repository_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EcrImageTags" => {
                                ecr_image_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FindingArn" => {
                                finding_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FindingStatus" => {
                                finding_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FindingType" => {
                                finding_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirstObservedAt" => {
                                first_observed_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InspectorScore" => {
                                inspector_score = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastObservedAt" => {
                                last_observed_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkProtocol" => {
                                network_protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PortRange" => {
                                port_range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RelatedVulnerabilities" => {
                                related_vulnerabilities = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceTags" => {
                                resource_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceType" => {
                                resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Severity" => {
                                severity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdatedAt" => {
                                updated_at = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VendorSeverity" => {
                                vendor_severity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VulnerabilityId" => {
                                vulnerability_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VulnerabilitySource" => {
                                vulnerability_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VulnerablePackages" => {
                                vulnerable_packages = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilterCriteria {
                        aws_account_id: aws_account_id,
                        component_id: component_id,
                        component_type: component_type,
                        ec2_instance_image_id: ec2_instance_image_id,
                        ec2_instance_subnet_id: ec2_instance_subnet_id,
                        ec2_instance_vpc_id: ec2_instance_vpc_id,
                        ecr_image_architecture: ecr_image_architecture,
                        ecr_image_hash: ecr_image_hash,
                        ecr_image_pushed_at: ecr_image_pushed_at,
                        ecr_image_registry: ecr_image_registry,
                        ecr_image_repository_name: ecr_image_repository_name,
                        ecr_image_tags: ecr_image_tags,
                        finding_arn: finding_arn,
                        finding_status: finding_status,
                        finding_type: finding_type,
                        first_observed_at: first_observed_at,
                        inspector_score: inspector_score,
                        last_observed_at: last_observed_at,
                        network_protocol: network_protocol,
                        port_range: port_range,
                        related_vulnerabilities: related_vulnerabilities,
                        resource_id: resource_id,
                        resource_tags: resource_tags,
                        resource_type: resource_type,
                        severity: severity,
                        title: title,
                        updated_at: updated_at,
                        vendor_severity: vendor_severity,
                        vulnerability_id: vulnerability_id,
                        vulnerability_source: vulnerability_source,
                        vulnerable_packages: vulnerable_packages,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::InspectorV2::Filter.MapFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-mapfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct MapFilter {
        /// Property [`Comparison`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-mapfilter.html#cfn-inspectorv2-filter-mapfilter-comparison).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-mapfilter.html#cfn-inspectorv2-filter-mapfilter-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-mapfilter.html#cfn-inspectorv2-filter-mapfilter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MapFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comparison", &self.comparison)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MapFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MapFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MapFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MapFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comparison" => {
                                comparison = ::serde::de::MapAccess::next_value(&mut map)?;
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

                    Ok(MapFilter {
                        comparison: comparison.ok_or(::serde::de::Error::missing_field("Comparison"))?,
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::InspectorV2::Filter.NumberFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-numberfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct NumberFilter {
        /// Property [`LowerInclusive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-numberfilter.html#cfn-inspectorv2-filter-numberfilter-lowerinclusive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lower_inclusive: Option<::Value<f64>>,
        /// Property [`UpperInclusive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-numberfilter.html#cfn-inspectorv2-filter-numberfilter-upperinclusive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub upper_inclusive: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for NumberFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lower_inclusive) = self.lower_inclusive {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LowerInclusive", lower_inclusive)?;
            }
            if let Some(ref upper_inclusive) = self.upper_inclusive {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpperInclusive", upper_inclusive)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NumberFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NumberFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NumberFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NumberFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lower_inclusive: Option<::Value<f64>> = None;
                    let mut upper_inclusive: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LowerInclusive" => {
                                lower_inclusive = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpperInclusive" => {
                                upper_inclusive = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NumberFilter {
                        lower_inclusive: lower_inclusive,
                        upper_inclusive: upper_inclusive,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::InspectorV2::Filter.PackageFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-packagefilter.html) property type.
    #[derive(Debug, Default)]
    pub struct PackageFilter {
        /// Property [`Architecture`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-packagefilter.html#cfn-inspectorv2-filter-packagefilter-architecture).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub architecture: Option<::Value<StringFilter>>,
        /// Property [`Epoch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-packagefilter.html#cfn-inspectorv2-filter-packagefilter-epoch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub epoch: Option<::Value<NumberFilter>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-packagefilter.html#cfn-inspectorv2-filter-packagefilter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<StringFilter>>,
        /// Property [`Release`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-packagefilter.html#cfn-inspectorv2-filter-packagefilter-release).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub release: Option<::Value<StringFilter>>,
        /// Property [`SourceLayerHash`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-packagefilter.html#cfn-inspectorv2-filter-packagefilter-sourcelayerhash).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_layer_hash: Option<::Value<StringFilter>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-packagefilter.html#cfn-inspectorv2-filter-packagefilter-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<StringFilter>>,
    }

    impl ::codec::SerializeValue for PackageFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref architecture) = self.architecture {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Architecture", architecture)?;
            }
            if let Some(ref epoch) = self.epoch {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Epoch", epoch)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref release) = self.release {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Release", release)?;
            }
            if let Some(ref source_layer_hash) = self.source_layer_hash {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceLayerHash", source_layer_hash)?;
            }
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PackageFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PackageFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PackageFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PackageFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut architecture: Option<::Value<StringFilter>> = None;
                    let mut epoch: Option<::Value<NumberFilter>> = None;
                    let mut name: Option<::Value<StringFilter>> = None;
                    let mut release: Option<::Value<StringFilter>> = None;
                    let mut source_layer_hash: Option<::Value<StringFilter>> = None;
                    let mut version: Option<::Value<StringFilter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Architecture" => {
                                architecture = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Epoch" => {
                                epoch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Release" => {
                                release = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceLayerHash" => {
                                source_layer_hash = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PackageFilter {
                        architecture: architecture,
                        epoch: epoch,
                        name: name,
                        release: release,
                        source_layer_hash: source_layer_hash,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::InspectorV2::Filter.PortRangeFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-portrangefilter.html) property type.
    #[derive(Debug, Default)]
    pub struct PortRangeFilter {
        /// Property [`BeginInclusive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-portrangefilter.html#cfn-inspectorv2-filter-portrangefilter-begininclusive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub begin_inclusive: Option<::Value<u32>>,
        /// Property [`EndInclusive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-portrangefilter.html#cfn-inspectorv2-filter-portrangefilter-endinclusive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_inclusive: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for PortRangeFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref begin_inclusive) = self.begin_inclusive {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BeginInclusive", begin_inclusive)?;
            }
            if let Some(ref end_inclusive) = self.end_inclusive {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndInclusive", end_inclusive)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortRangeFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortRangeFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortRangeFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortRangeFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut begin_inclusive: Option<::Value<u32>> = None;
                    let mut end_inclusive: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BeginInclusive" => {
                                begin_inclusive = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndInclusive" => {
                                end_inclusive = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PortRangeFilter {
                        begin_inclusive: begin_inclusive,
                        end_inclusive: end_inclusive,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::InspectorV2::Filter.StringFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-stringfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct StringFilter {
        /// Property [`Comparison`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-stringfilter.html#cfn-inspectorv2-filter-stringfilter-comparison).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-inspectorv2-filter-stringfilter.html#cfn-inspectorv2-filter-stringfilter-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for StringFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comparison", &self.comparison)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StringFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StringFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StringFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StringFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comparison" => {
                                comparison = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StringFilter {
                        comparison: comparison.ok_or(::serde::de::Error::missing_field("Comparison"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
