//! Types for the `Route53RecoveryReadiness` service.

/// The [`AWS::Route53RecoveryReadiness::Cell`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-cell.html) resource type.
#[derive(Debug, Default)]
pub struct Cell {
    properties: CellProperties
}

/// Properties for the `Cell` resource.
#[derive(Debug, Default)]
pub struct CellProperties {
    /// Property [`CellName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-cell.html#cfn-route53recoveryreadiness-cell-cellname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cell_name: ::Value<String>,
    /// Property [`Cells`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-cell.html#cfn-route53recoveryreadiness-cell-cells).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cells: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-cell.html#cfn-route53recoveryreadiness-cell-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CellProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CellName", &self.cell_name)?;
        if let Some(ref cells) = self.cells {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cells", cells)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CellProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CellProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CellProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CellProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cell_name: Option<::Value<String>> = None;
                let mut cells: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CellName" => {
                            cell_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cells" => {
                            cells = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CellProperties {
                    cell_name: cell_name.ok_or(::serde::de::Error::missing_field("CellName"))?,
                    cells: cells,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Cell {
    type Properties = CellProperties;
    const TYPE: &'static str = "AWS::Route53RecoveryReadiness::Cell";
    fn properties(&self) -> &CellProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CellProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Cell {}

impl From<CellProperties> for Cell {
    fn from(properties: CellProperties) -> Cell {
        Cell { properties }
    }
}

/// The [`AWS::Route53RecoveryReadiness::ReadinessCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-readinesscheck.html) resource type.
#[derive(Debug, Default)]
pub struct ReadinessCheck {
    properties: ReadinessCheckProperties
}

/// Properties for the `ReadinessCheck` resource.
#[derive(Debug, Default)]
pub struct ReadinessCheckProperties {
    /// Property [`ReadinessCheckName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-readinesscheck.html#cfn-route53recoveryreadiness-readinesscheck-readinesscheckname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub readiness_check_name: ::Value<String>,
    /// Property [`ResourceSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-readinesscheck.html#cfn-route53recoveryreadiness-readinesscheck-resourcesetname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_set_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-readinesscheck.html#cfn-route53recoveryreadiness-readinesscheck-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ReadinessCheckProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadinessCheckName", &self.readiness_check_name)?;
        if let Some(ref resource_set_name) = self.resource_set_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceSetName", resource_set_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ReadinessCheckProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ReadinessCheckProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ReadinessCheckProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ReadinessCheckProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut readiness_check_name: Option<::Value<String>> = None;
                let mut resource_set_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ReadinessCheckName" => {
                            readiness_check_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceSetName" => {
                            resource_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ReadinessCheckProperties {
                    readiness_check_name: readiness_check_name.ok_or(::serde::de::Error::missing_field("ReadinessCheckName"))?,
                    resource_set_name: resource_set_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ReadinessCheck {
    type Properties = ReadinessCheckProperties;
    const TYPE: &'static str = "AWS::Route53RecoveryReadiness::ReadinessCheck";
    fn properties(&self) -> &ReadinessCheckProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ReadinessCheckProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ReadinessCheck {}

impl From<ReadinessCheckProperties> for ReadinessCheck {
    fn from(properties: ReadinessCheckProperties) -> ReadinessCheck {
        ReadinessCheck { properties }
    }
}

/// The [`AWS::Route53RecoveryReadiness::RecoveryGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-recoverygroup.html) resource type.
#[derive(Debug, Default)]
pub struct RecoveryGroup {
    properties: RecoveryGroupProperties
}

/// Properties for the `RecoveryGroup` resource.
#[derive(Debug, Default)]
pub struct RecoveryGroupProperties {
    /// Property [`Cells`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-recoverygroup.html#cfn-route53recoveryreadiness-recoverygroup-cells).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cells: Option<::ValueList<String>>,
    /// Property [`RecoveryGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-recoverygroup.html#cfn-route53recoveryreadiness-recoverygroup-recoverygroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub recovery_group_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-recoverygroup.html#cfn-route53recoveryreadiness-recoverygroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for RecoveryGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cells) = self.cells {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cells", cells)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecoveryGroupName", &self.recovery_group_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RecoveryGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RecoveryGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RecoveryGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RecoveryGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cells: Option<::ValueList<String>> = None;
                let mut recovery_group_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Cells" => {
                            cells = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RecoveryGroupName" => {
                            recovery_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RecoveryGroupProperties {
                    cells: cells,
                    recovery_group_name: recovery_group_name.ok_or(::serde::de::Error::missing_field("RecoveryGroupName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RecoveryGroup {
    type Properties = RecoveryGroupProperties;
    const TYPE: &'static str = "AWS::Route53RecoveryReadiness::RecoveryGroup";
    fn properties(&self) -> &RecoveryGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RecoveryGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RecoveryGroup {}

impl From<RecoveryGroupProperties> for RecoveryGroup {
    fn from(properties: RecoveryGroupProperties) -> RecoveryGroup {
        RecoveryGroup { properties }
    }
}

/// The [`AWS::Route53RecoveryReadiness::ResourceSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-resourceset.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceSet {
    properties: ResourceSetProperties
}

/// Properties for the `ResourceSet` resource.
#[derive(Debug, Default)]
pub struct ResourceSetProperties {
    /// Property [`ResourceSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-resourceset.html#cfn-route53recoveryreadiness-resourceset-resourcesetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_set_name: ::Value<String>,
    /// Property [`ResourceSetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-resourceset.html#cfn-route53recoveryreadiness-resourceset-resourcesettype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_set_type: ::Value<String>,
    /// Property [`Resources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-resourceset.html#cfn-route53recoveryreadiness-resourceset-resources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resources: ::ValueList<self::resource_set::Resource>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-resourceset.html#cfn-route53recoveryreadiness-resourceset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ResourceSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceSetName", &self.resource_set_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceSetType", &self.resource_set_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resources", &self.resources)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resource_set_name: Option<::Value<String>> = None;
                let mut resource_set_type: Option<::Value<String>> = None;
                let mut resources: Option<::ValueList<self::resource_set::Resource>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceSetName" => {
                            resource_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceSetType" => {
                            resource_set_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Resources" => {
                            resources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceSetProperties {
                    resource_set_name: resource_set_name.ok_or(::serde::de::Error::missing_field("ResourceSetName"))?,
                    resource_set_type: resource_set_type.ok_or(::serde::de::Error::missing_field("ResourceSetType"))?,
                    resources: resources.ok_or(::serde::de::Error::missing_field("Resources"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceSet {
    type Properties = ResourceSetProperties;
    const TYPE: &'static str = "AWS::Route53RecoveryReadiness::ResourceSet";
    fn properties(&self) -> &ResourceSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceSet {}

impl From<ResourceSetProperties> for ResourceSet {
    fn from(properties: ResourceSetProperties) -> ResourceSet {
        ResourceSet { properties }
    }
}

pub mod resource_set {
    //! Property types for the `ResourceSet` resource.

    /// The [`AWS::Route53RecoveryReadiness::ResourceSet.DNSTargetResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-dnstargetresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DNSTargetResource {
        /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-dnstargetresource.html#cfn-route53recoveryreadiness-resourceset-dnstargetresource-domainname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_name: Option<::Value<String>>,
        /// Property [`HostedZoneArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-dnstargetresource.html#cfn-route53recoveryreadiness-resourceset-dnstargetresource-hostedzonearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hosted_zone_arn: Option<::Value<String>>,
        /// Property [`RecordSetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-dnstargetresource.html#cfn-route53recoveryreadiness-resourceset-dnstargetresource-recordsetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_set_id: Option<::Value<String>>,
        /// Property [`RecordType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-dnstargetresource.html#cfn-route53recoveryreadiness-resourceset-dnstargetresource-recordtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_type: Option<::Value<String>>,
        /// Property [`TargetResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-dnstargetresource.html#cfn-route53recoveryreadiness-resourceset-dnstargetresource-targetresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_resource: Option<::Value<TargetResource>>,
    }

    impl ::codec::SerializeValue for DNSTargetResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref domain_name) = self.domain_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", domain_name)?;
            }
            if let Some(ref hosted_zone_arn) = self.hosted_zone_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneArn", hosted_zone_arn)?;
            }
            if let Some(ref record_set_id) = self.record_set_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordSetId", record_set_id)?;
            }
            if let Some(ref record_type) = self.record_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordType", record_type)?;
            }
            if let Some(ref target_resource) = self.target_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetResource", target_resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DNSTargetResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DNSTargetResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DNSTargetResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DNSTargetResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut domain_name: Option<::Value<String>> = None;
                    let mut hosted_zone_arn: Option<::Value<String>> = None;
                    let mut record_set_id: Option<::Value<String>> = None;
                    let mut record_type: Option<::Value<String>> = None;
                    let mut target_resource: Option<::Value<TargetResource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DomainName" => {
                                domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostedZoneArn" => {
                                hosted_zone_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordSetId" => {
                                record_set_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordType" => {
                                record_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetResource" => {
                                target_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DNSTargetResource {
                        domain_name: domain_name,
                        hosted_zone_arn: hosted_zone_arn,
                        record_set_id: record_set_id,
                        record_type: record_type,
                        target_resource: target_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53RecoveryReadiness::ResourceSet.NLBResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-nlbresource.html) property type.
    #[derive(Debug, Default)]
    pub struct NLBResource {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-nlbresource.html#cfn-route53recoveryreadiness-resourceset-nlbresource-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NLBResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NLBResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NLBResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NLBResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NLBResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NLBResource {
                        arn: arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53RecoveryReadiness::ResourceSet.R53ResourceRecord`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-r53resourcerecord.html) property type.
    #[derive(Debug, Default)]
    pub struct R53ResourceRecord {
        /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-r53resourcerecord.html#cfn-route53recoveryreadiness-resourceset-r53resourcerecord-domainname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_name: Option<::Value<String>>,
        /// Property [`RecordSetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-r53resourcerecord.html#cfn-route53recoveryreadiness-resourceset-r53resourcerecord-recordsetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_set_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for R53ResourceRecord {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref domain_name) = self.domain_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", domain_name)?;
            }
            if let Some(ref record_set_id) = self.record_set_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordSetId", record_set_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for R53ResourceRecord {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<R53ResourceRecord, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = R53ResourceRecord;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type R53ResourceRecord")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut domain_name: Option<::Value<String>> = None;
                    let mut record_set_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DomainName" => {
                                domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordSetId" => {
                                record_set_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(R53ResourceRecord {
                        domain_name: domain_name,
                        record_set_id: record_set_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53RecoveryReadiness::ResourceSet.Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-resource.html) property type.
    #[derive(Debug, Default)]
    pub struct Resource {
        /// Property [`ComponentId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-resource.html#cfn-route53recoveryreadiness-resourceset-resource-componentid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_id: Option<::Value<String>>,
        /// Property [`DnsTargetResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-resource.html#cfn-route53recoveryreadiness-resourceset-resource-dnstargetresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_target_resource: Option<::Value<DNSTargetResource>>,
        /// Property [`ReadinessScopes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-resource.html#cfn-route53recoveryreadiness-resourceset-resource-readinessscopes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub readiness_scopes: Option<::ValueList<String>>,
        /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-resource.html#cfn-route53recoveryreadiness-resourceset-resource-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Resource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref component_id) = self.component_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentId", component_id)?;
            }
            if let Some(ref dns_target_resource) = self.dns_target_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsTargetResource", dns_target_resource)?;
            }
            if let Some(ref readiness_scopes) = self.readiness_scopes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadinessScopes", readiness_scopes)?;
            }
            if let Some(ref resource_arn) = self.resource_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", resource_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Resource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Resource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Resource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Resource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_id: Option<::Value<String>> = None;
                    let mut dns_target_resource: Option<::Value<DNSTargetResource>> = None;
                    let mut readiness_scopes: Option<::ValueList<String>> = None;
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentId" => {
                                component_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DnsTargetResource" => {
                                dns_target_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadinessScopes" => {
                                readiness_scopes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceArn" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Resource {
                        component_id: component_id,
                        dns_target_resource: dns_target_resource,
                        readiness_scopes: readiness_scopes,
                        resource_arn: resource_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Route53RecoveryReadiness::ResourceSet.TargetResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-targetresource.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetResource {
        /// Property [`NLBResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-targetresource.html#cfn-route53recoveryreadiness-resourceset-targetresource-nlbresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nlb_resource: Option<::Value<NLBResource>>,
        /// Property [`R53Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-targetresource.html#cfn-route53recoveryreadiness-resourceset-targetresource-r53resource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r53_resource: Option<::Value<R53ResourceRecord>>,
    }

    impl ::codec::SerializeValue for TargetResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref nlb_resource) = self.nlb_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NLBResource", nlb_resource)?;
            }
            if let Some(ref r53_resource) = self.r53_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "R53Resource", r53_resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut nlb_resource: Option<::Value<NLBResource>> = None;
                    let mut r53_resource: Option<::Value<R53ResourceRecord>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NLBResource" => {
                                nlb_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "R53Resource" => {
                                r53_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetResource {
                        nlb_resource: nlb_resource,
                        r53_resource: r53_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
