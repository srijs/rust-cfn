//! Types for the `LakeFormation` service.

/// The [`AWS::LakeFormation::DataLakeSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html) resource type.
#[derive(Debug, Default)]
pub struct DataLakeSettings {
    properties: DataLakeSettingsProperties
}

/// Properties for the `DataLakeSettings` resource.
#[derive(Debug, Default)]
pub struct DataLakeSettingsProperties {
    /// Property [`Admins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-admins).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub admins: Option<::Value<self::data_lake_settings::Admins>>,
    /// Property [`TrustedResourceOwners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-trustedresourceowners).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub trusted_resource_owners: Option<::ValueList<String>>,
}

impl ::serde::Serialize for DataLakeSettingsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref admins) = self.admins {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Admins", admins)?;
        }
        if let Some(ref trusted_resource_owners) = self.trusted_resource_owners {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedResourceOwners", trusted_resource_owners)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataLakeSettingsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLakeSettingsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataLakeSettingsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataLakeSettingsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut admins: Option<::Value<self::data_lake_settings::Admins>> = None;
                let mut trusted_resource_owners: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Admins" => {
                            admins = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrustedResourceOwners" => {
                            trusted_resource_owners = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataLakeSettingsProperties {
                    admins: admins,
                    trusted_resource_owners: trusted_resource_owners,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataLakeSettings {
    type Properties = DataLakeSettingsProperties;
    const TYPE: &'static str = "AWS::LakeFormation::DataLakeSettings";
    fn properties(&self) -> &DataLakeSettingsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataLakeSettingsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataLakeSettings {}

impl From<DataLakeSettingsProperties> for DataLakeSettings {
    fn from(properties: DataLakeSettingsProperties) -> DataLakeSettings {
        DataLakeSettings { properties }
    }
}

/// The [`AWS::LakeFormation::Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html) resource type.
#[derive(Debug, Default)]
pub struct Permissions {
    properties: PermissionsProperties
}

/// Properties for the `Permissions` resource.
#[derive(Debug, Default)]
pub struct PermissionsProperties {
    /// Property [`DataLakePrincipal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html#cfn-lakeformation-permissions-datalakeprincipal).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_lake_principal: ::Value<self::permissions::DataLakePrincipal>,
    /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html#cfn-lakeformation-permissions-permissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions: Option<::ValueList<String>>,
    /// Property [`PermissionsWithGrantOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html#cfn-lakeformation-permissions-permissionswithgrantoption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions_with_grant_option: Option<::ValueList<String>>,
    /// Property [`Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html#cfn-lakeformation-permissions-resource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource: ::Value<self::permissions::Resource>,
}

impl ::serde::Serialize for PermissionsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLakePrincipal", &self.data_lake_principal)?;
        if let Some(ref permissions) = self.permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
        }
        if let Some(ref permissions_with_grant_option) = self.permissions_with_grant_option {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PermissionsWithGrantOption", permissions_with_grant_option)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resource", &self.resource)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PermissionsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PermissionsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PermissionsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PermissionsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_lake_principal: Option<::Value<self::permissions::DataLakePrincipal>> = None;
                let mut permissions: Option<::ValueList<String>> = None;
                let mut permissions_with_grant_option: Option<::ValueList<String>> = None;
                let mut resource: Option<::Value<self::permissions::Resource>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataLakePrincipal" => {
                            data_lake_principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Permissions" => {
                            permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PermissionsWithGrantOption" => {
                            permissions_with_grant_option = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Resource" => {
                            resource = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PermissionsProperties {
                    data_lake_principal: data_lake_principal.ok_or(::serde::de::Error::missing_field("DataLakePrincipal"))?,
                    permissions: permissions,
                    permissions_with_grant_option: permissions_with_grant_option,
                    resource: resource.ok_or(::serde::de::Error::missing_field("Resource"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Permissions {
    type Properties = PermissionsProperties;
    const TYPE: &'static str = "AWS::LakeFormation::Permissions";
    fn properties(&self) -> &PermissionsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PermissionsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Permissions {}

impl From<PermissionsProperties> for Permissions {
    fn from(properties: PermissionsProperties) -> Permissions {
        Permissions { properties }
    }
}

/// The [`AWS::LakeFormation::Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html) resource type.
#[derive(Debug, Default)]
pub struct Resource {
    properties: ResourceProperties
}

/// Properties for the `Resource` resource.
#[derive(Debug, Default)]
pub struct ResourceProperties {
    /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html#cfn-lakeformation-resource-resourcearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_arn: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html#cfn-lakeformation-resource-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`UseServiceLinkedRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html#cfn-lakeformation-resource-useservicelinkedrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub use_service_linked_role: ::Value<bool>,
}

impl ::serde::Serialize for ResourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseServiceLinkedRole", &self.use_service_linked_role)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut resource_arn: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut use_service_linked_role: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ResourceArn" => {
                            resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UseServiceLinkedRole" => {
                            use_service_linked_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceProperties {
                    resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                    role_arn: role_arn,
                    use_service_linked_role: use_service_linked_role.ok_or(::serde::de::Error::missing_field("UseServiceLinkedRole"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Resource {
    type Properties = ResourceProperties;
    const TYPE: &'static str = "AWS::LakeFormation::Resource";
    fn properties(&self) -> &ResourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Resource {}

impl From<ResourceProperties> for Resource {
    fn from(properties: ResourceProperties) -> Resource {
        Resource { properties }
    }
}

pub mod data_lake_settings {
    //! Property types for the `DataLakeSettings` resource.

    /// The [`AWS::LakeFormation::DataLakeSettings.Admins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-admins.html) property type.
    #[derive(Debug, Default)]
    pub struct Admins {
    }

    impl ::codec::SerializeValue for Admins {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Admins {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Admins, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Admins;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Admins")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Admins {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::DataLakeSettings.DataLakePrincipal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-datalakeprincipal.html) property type.
    #[derive(Debug, Default)]
    pub struct DataLakePrincipal {
        /// Property [`DataLakePrincipalIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-datalakeprincipal.html#cfn-lakeformation-datalakesettings-datalakeprincipal-datalakeprincipalidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_lake_principal_identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataLakePrincipal {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_lake_principal_identifier) = self.data_lake_principal_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLakePrincipalIdentifier", data_lake_principal_identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataLakePrincipal {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLakePrincipal, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataLakePrincipal;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataLakePrincipal")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_lake_principal_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataLakePrincipalIdentifier" => {
                                data_lake_principal_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataLakePrincipal {
                        data_lake_principal_identifier: data_lake_principal_identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod permissions {
    //! Property types for the `Permissions` resource.

    /// The [`AWS::LakeFormation::Permissions.ColumnWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-columnwildcard.html) property type.
    #[derive(Debug, Default)]
    pub struct ColumnWildcard {
        /// Property [`ExcludedColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-columnwildcard.html#cfn-lakeformation-permissions-columnwildcard-excludedcolumnnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded_column_names: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ColumnWildcard {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref excluded_column_names) = self.excluded_column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedColumnNames", excluded_column_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColumnWildcard {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColumnWildcard, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColumnWildcard;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColumnWildcard")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut excluded_column_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExcludedColumnNames" => {
                                excluded_column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ColumnWildcard {
                        excluded_column_names: excluded_column_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.DataLakePrincipal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalakeprincipal.html) property type.
    #[derive(Debug, Default)]
    pub struct DataLakePrincipal {
        /// Property [`DataLakePrincipalIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalakeprincipal.html#cfn-lakeformation-permissions-datalakeprincipal-datalakeprincipalidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_lake_principal_identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataLakePrincipal {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_lake_principal_identifier) = self.data_lake_principal_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLakePrincipalIdentifier", data_lake_principal_identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataLakePrincipal {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLakePrincipal, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataLakePrincipal;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataLakePrincipal")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_lake_principal_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataLakePrincipalIdentifier" => {
                                data_lake_principal_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataLakePrincipal {
                        data_lake_principal_identifier: data_lake_principal_identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.DataLocationResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalocationresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DataLocationResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalocationresource.html#cfn-lakeformation-permissions-datalocationresource-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`S3Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalocationresource.html#cfn-lakeformation-permissions-datalocationresource-s3resource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_resource: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataLocationResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref s3_resource) = self.s3_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Resource", s3_resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataLocationResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLocationResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataLocationResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataLocationResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut s3_resource: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Resource" => {
                                s3_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataLocationResource {
                        catalog_id: catalog_id,
                        s3_resource: s3_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.DatabaseResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-databaseresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DatabaseResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-databaseresource.html#cfn-lakeformation-permissions-databaseresource-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-databaseresource.html#cfn-lakeformation-permissions-databaseresource-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DatabaseResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatabaseResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatabaseResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatabaseResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatabaseResource {
                        catalog_id: catalog_id,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html) property type.
    #[derive(Debug, Default)]
    pub struct Resource {
        /// Property [`DataLocationResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html#cfn-lakeformation-permissions-resource-datalocationresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_location_resource: Option<::Value<DataLocationResource>>,
        /// Property [`DatabaseResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html#cfn-lakeformation-permissions-resource-databaseresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_resource: Option<::Value<DatabaseResource>>,
        /// Property [`TableResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html#cfn-lakeformation-permissions-resource-tableresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_resource: Option<::Value<TableResource>>,
        /// Property [`TableWithColumnsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html#cfn-lakeformation-permissions-resource-tablewithcolumnsresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_with_columns_resource: Option<::Value<TableWithColumnsResource>>,
    }

    impl ::codec::SerializeValue for Resource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_location_resource) = self.data_location_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLocationResource", data_location_resource)?;
            }
            if let Some(ref database_resource) = self.database_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseResource", database_resource)?;
            }
            if let Some(ref table_resource) = self.table_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableResource", table_resource)?;
            }
            if let Some(ref table_with_columns_resource) = self.table_with_columns_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableWithColumnsResource", table_with_columns_resource)?;
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
                    let mut data_location_resource: Option<::Value<DataLocationResource>> = None;
                    let mut database_resource: Option<::Value<DatabaseResource>> = None;
                    let mut table_resource: Option<::Value<TableResource>> = None;
                    let mut table_with_columns_resource: Option<::Value<TableWithColumnsResource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataLocationResource" => {
                                data_location_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseResource" => {
                                database_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableResource" => {
                                table_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableWithColumnsResource" => {
                                table_with_columns_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Resource {
                        data_location_resource: data_location_resource,
                        database_resource: database_resource,
                        table_resource: table_resource,
                        table_with_columns_resource: table_with_columns_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.TableResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html) property type.
    #[derive(Debug, Default)]
    pub struct TableResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html#cfn-lakeformation-permissions-tableresource-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html#cfn-lakeformation-permissions-tableresource-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html#cfn-lakeformation-permissions-tableresource-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`TableWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html#cfn-lakeformation-permissions-tableresource-tablewildcard).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_wildcard: Option<::Value<TableWildcard>>,
    }

    impl ::codec::SerializeValue for TableResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref table_wildcard) = self.table_wildcard {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableWildcard", table_wildcard)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut table_wildcard: Option<::Value<TableWildcard>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableWildcard" => {
                                table_wildcard = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableResource {
                        catalog_id: catalog_id,
                        database_name: database_name,
                        name: name,
                        table_wildcard: table_wildcard,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.TableWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewildcard.html) property type.
    #[derive(Debug, Default)]
    pub struct TableWildcard {
    }

    impl ::codec::SerializeValue for TableWildcard {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableWildcard {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableWildcard, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableWildcard;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableWildcard")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(TableWildcard {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.TableWithColumnsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct TableWithColumnsResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html#cfn-lakeformation-permissions-tablewithcolumnsresource-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`ColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html#cfn-lakeformation-permissions-tablewithcolumnsresource-columnnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_names: Option<::ValueList<String>>,
        /// Property [`ColumnWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html#cfn-lakeformation-permissions-tablewithcolumnsresource-columnwildcard).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_wildcard: Option<::Value<ColumnWildcard>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html#cfn-lakeformation-permissions-tablewithcolumnsresource-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html#cfn-lakeformation-permissions-tablewithcolumnsresource-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TableWithColumnsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref column_names) = self.column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnNames", column_names)?;
            }
            if let Some(ref column_wildcard) = self.column_wildcard {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnWildcard", column_wildcard)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableWithColumnsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableWithColumnsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableWithColumnsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableWithColumnsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut column_names: Option<::ValueList<String>> = None;
                    let mut column_wildcard: Option<::Value<ColumnWildcard>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnNames" => {
                                column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnWildcard" => {
                                column_wildcard = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableWithColumnsResource {
                        catalog_id: catalog_id,
                        column_names: column_names,
                        column_wildcard: column_wildcard,
                        database_name: database_name,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
