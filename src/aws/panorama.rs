//! Types for the `Panorama` service.

/// The [`AWS::Panorama::ApplicationInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html) resource type.
#[derive(Debug, Default)]
pub struct ApplicationInstance {
    properties: ApplicationInstanceProperties
}

/// Properties for the `ApplicationInstance` resource.
#[derive(Debug, Default)]
pub struct ApplicationInstanceProperties {
    /// Property [`ApplicationInstanceIdToReplace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html#cfn-panorama-applicationinstance-applicationinstanceidtoreplace).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_instance_id_to_replace: Option<::Value<String>>,
    /// Property [`DefaultRuntimeContextDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html#cfn-panorama-applicationinstance-defaultruntimecontextdevice).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub default_runtime_context_device: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html#cfn-panorama-applicationinstance-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DeviceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html#cfn-panorama-applicationinstance-deviceid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub device_id: Option<::Value<String>>,
    /// Property [`ManifestOverridesPayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html#cfn-panorama-applicationinstance-manifestoverridespayload).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub manifest_overrides_payload: Option<::Value<self::application_instance::ManifestOverridesPayload>>,
    /// Property [`ManifestPayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html#cfn-panorama-applicationinstance-manifestpayload).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub manifest_payload: ::Value<self::application_instance::ManifestPayload>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html#cfn-panorama-applicationinstance-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`RuntimeRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html#cfn-panorama-applicationinstance-runtimerolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub runtime_role_arn: Option<::Value<String>>,
    /// Property [`StatusFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html#cfn-panorama-applicationinstance-statusfilter).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status_filter: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-applicationinstance.html#cfn-panorama-applicationinstance-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ApplicationInstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref application_instance_id_to_replace) = self.application_instance_id_to_replace {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationInstanceIdToReplace", application_instance_id_to_replace)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultRuntimeContextDevice", &self.default_runtime_context_device)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref device_id) = self.device_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceId", device_id)?;
        }
        if let Some(ref manifest_overrides_payload) = self.manifest_overrides_payload {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestOverridesPayload", manifest_overrides_payload)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestPayload", &self.manifest_payload)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref runtime_role_arn) = self.runtime_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimeRoleArn", runtime_role_arn)?;
        }
        if let Some(ref status_filter) = self.status_filter {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatusFilter", status_filter)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationInstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationInstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationInstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationInstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_instance_id_to_replace: Option<::Value<String>> = None;
                let mut default_runtime_context_device: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut device_id: Option<::Value<String>> = None;
                let mut manifest_overrides_payload: Option<::Value<self::application_instance::ManifestOverridesPayload>> = None;
                let mut manifest_payload: Option<::Value<self::application_instance::ManifestPayload>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut runtime_role_arn: Option<::Value<String>> = None;
                let mut status_filter: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationInstanceIdToReplace" => {
                            application_instance_id_to_replace = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultRuntimeContextDevice" => {
                            default_runtime_context_device = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeviceId" => {
                            device_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManifestOverridesPayload" => {
                            manifest_overrides_payload = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManifestPayload" => {
                            manifest_payload = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuntimeRoleArn" => {
                            runtime_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StatusFilter" => {
                            status_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationInstanceProperties {
                    application_instance_id_to_replace: application_instance_id_to_replace,
                    default_runtime_context_device: default_runtime_context_device.ok_or(::serde::de::Error::missing_field("DefaultRuntimeContextDevice"))?,
                    description: description,
                    device_id: device_id,
                    manifest_overrides_payload: manifest_overrides_payload,
                    manifest_payload: manifest_payload.ok_or(::serde::de::Error::missing_field("ManifestPayload"))?,
                    name: name,
                    runtime_role_arn: runtime_role_arn,
                    status_filter: status_filter,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApplicationInstance {
    type Properties = ApplicationInstanceProperties;
    const TYPE: &'static str = "AWS::Panorama::ApplicationInstance";
    fn properties(&self) -> &ApplicationInstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationInstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApplicationInstance {}

impl From<ApplicationInstanceProperties> for ApplicationInstance {
    fn from(properties: ApplicationInstanceProperties) -> ApplicationInstance {
        ApplicationInstance { properties }
    }
}

/// The [`AWS::Panorama::Package`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-package.html) resource type.
#[derive(Debug, Default)]
pub struct Package {
    properties: PackageProperties
}

/// Properties for the `Package` resource.
#[derive(Debug, Default)]
pub struct PackageProperties {
    /// Property [`PackageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-package.html#cfn-panorama-package-packagename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub package_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-package.html#cfn-panorama-package-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PackageProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackageName", &self.package_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PackageProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PackageProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PackageProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PackageProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut package_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PackageName" => {
                            package_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PackageProperties {
                    package_name: package_name.ok_or(::serde::de::Error::missing_field("PackageName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Package {
    type Properties = PackageProperties;
    const TYPE: &'static str = "AWS::Panorama::Package";
    fn properties(&self) -> &PackageProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PackageProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Package {}

impl From<PackageProperties> for Package {
    fn from(properties: PackageProperties) -> Package {
        Package { properties }
    }
}

/// The [`AWS::Panorama::PackageVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-packageversion.html) resource type.
#[derive(Debug, Default)]
pub struct PackageVersion {
    properties: PackageVersionProperties
}

/// Properties for the `PackageVersion` resource.
#[derive(Debug, Default)]
pub struct PackageVersionProperties {
    /// Property [`MarkLatest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-packageversion.html#cfn-panorama-packageversion-marklatest).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mark_latest: Option<::Value<bool>>,
    /// Property [`OwnerAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-packageversion.html#cfn-panorama-packageversion-owneraccount).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub owner_account: Option<::Value<String>>,
    /// Property [`PackageId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-packageversion.html#cfn-panorama-packageversion-packageid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub package_id: ::Value<String>,
    /// Property [`PackageVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-packageversion.html#cfn-panorama-packageversion-packageversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub package_version: ::Value<String>,
    /// Property [`PatchVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-packageversion.html#cfn-panorama-packageversion-patchversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub patch_version: ::Value<String>,
    /// Property [`UpdatedLatestPatchVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-panorama-packageversion.html#cfn-panorama-packageversion-updatedlatestpatchversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub updated_latest_patch_version: Option<::Value<String>>,
}

impl ::serde::Serialize for PackageVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref mark_latest) = self.mark_latest {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MarkLatest", mark_latest)?;
        }
        if let Some(ref owner_account) = self.owner_account {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerAccount", owner_account)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackageId", &self.package_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackageVersion", &self.package_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatchVersion", &self.patch_version)?;
        if let Some(ref updated_latest_patch_version) = self.updated_latest_patch_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdatedLatestPatchVersion", updated_latest_patch_version)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PackageVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PackageVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PackageVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PackageVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut mark_latest: Option<::Value<bool>> = None;
                let mut owner_account: Option<::Value<String>> = None;
                let mut package_id: Option<::Value<String>> = None;
                let mut package_version: Option<::Value<String>> = None;
                let mut patch_version: Option<::Value<String>> = None;
                let mut updated_latest_patch_version: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "MarkLatest" => {
                            mark_latest = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OwnerAccount" => {
                            owner_account = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PackageId" => {
                            package_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PackageVersion" => {
                            package_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PatchVersion" => {
                            patch_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UpdatedLatestPatchVersion" => {
                            updated_latest_patch_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PackageVersionProperties {
                    mark_latest: mark_latest,
                    owner_account: owner_account,
                    package_id: package_id.ok_or(::serde::de::Error::missing_field("PackageId"))?,
                    package_version: package_version.ok_or(::serde::de::Error::missing_field("PackageVersion"))?,
                    patch_version: patch_version.ok_or(::serde::de::Error::missing_field("PatchVersion"))?,
                    updated_latest_patch_version: updated_latest_patch_version,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PackageVersion {
    type Properties = PackageVersionProperties;
    const TYPE: &'static str = "AWS::Panorama::PackageVersion";
    fn properties(&self) -> &PackageVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PackageVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PackageVersion {}

impl From<PackageVersionProperties> for PackageVersion {
    fn from(properties: PackageVersionProperties) -> PackageVersion {
        PackageVersion { properties }
    }
}

pub mod application_instance {
    //! Property types for the `ApplicationInstance` resource.

    /// The [`AWS::Panorama::ApplicationInstance.ManifestOverridesPayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-panorama-applicationinstance-manifestoverridespayload.html) property type.
    #[derive(Debug, Default)]
    pub struct ManifestOverridesPayload {
        /// Property [`PayloadData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-panorama-applicationinstance-manifestoverridespayload.html#cfn-panorama-applicationinstance-manifestoverridespayload-payloaddata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub payload_data: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ManifestOverridesPayload {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref payload_data) = self.payload_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadData", payload_data)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ManifestOverridesPayload {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ManifestOverridesPayload, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ManifestOverridesPayload;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ManifestOverridesPayload")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut payload_data: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PayloadData" => {
                                payload_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ManifestOverridesPayload {
                        payload_data: payload_data,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Panorama::ApplicationInstance.ManifestPayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-panorama-applicationinstance-manifestpayload.html) property type.
    #[derive(Debug, Default)]
    pub struct ManifestPayload {
        /// Property [`PayloadData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-panorama-applicationinstance-manifestpayload.html#cfn-panorama-applicationinstance-manifestpayload-payloaddata).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub payload_data: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ManifestPayload {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref payload_data) = self.payload_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadData", payload_data)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ManifestPayload {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ManifestPayload, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ManifestPayload;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ManifestPayload")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut payload_data: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PayloadData" => {
                                payload_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ManifestPayload {
                        payload_data: payload_data,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
