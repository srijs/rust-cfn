//! Types for the `RAM` service.

/// The [`AWS::RAM::ResourceShare`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ram-resourceshare.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceShare {
    properties: ResourceShareProperties
}

/// Properties for the `ResourceShare` resource.
#[derive(Debug, Default)]
pub struct ResourceShareProperties {
    /// Property [`AllowExternalPrincipals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ram-resourceshare.html#cfn-ram-resourceshare-allowexternalprincipals).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_external_principals: Option<::Value<bool>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ram-resourceshare.html#cfn-ram-resourceshare-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`PermissionArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ram-resourceshare.html#cfn-ram-resourceshare-permissionarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permission_arns: Option<::ValueList<String>>,
    /// Property [`Principals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ram-resourceshare.html#cfn-ram-resourceshare-principals).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub principals: Option<::ValueList<String>>,
    /// Property [`ResourceArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ram-resourceshare.html#cfn-ram-resourceshare-resourcearns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_arns: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ram-resourceshare.html#cfn-ram-resourceshare-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ResourceShareProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allow_external_principals) = self.allow_external_principals {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowExternalPrincipals", allow_external_principals)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref permission_arns) = self.permission_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PermissionArns", permission_arns)?;
        }
        if let Some(ref principals) = self.principals {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principals", principals)?;
        }
        if let Some(ref resource_arns) = self.resource_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArns", resource_arns)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceShareProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceShareProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceShareProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceShareProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allow_external_principals: Option<::Value<bool>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut permission_arns: Option<::ValueList<String>> = None;
                let mut principals: Option<::ValueList<String>> = None;
                let mut resource_arns: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowExternalPrincipals" => {
                            allow_external_principals = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PermissionArns" => {
                            permission_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principals" => {
                            principals = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceArns" => {
                            resource_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceShareProperties {
                    allow_external_principals: allow_external_principals,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    permission_arns: permission_arns,
                    principals: principals,
                    resource_arns: resource_arns,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceShare {
    type Properties = ResourceShareProperties;
    const TYPE: &'static str = "AWS::RAM::ResourceShare";
    fn properties(&self) -> &ResourceShareProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceShareProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceShare {}

impl From<ResourceShareProperties> for ResourceShare {
    fn from(properties: ResourceShareProperties) -> ResourceShare {
        ResourceShare { properties }
    }
}
