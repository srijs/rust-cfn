//! Types for the `WorkSpaces` service.

/// The [`AWS::WorkSpaces::ConnectionAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-connectionalias.html) resource type.
#[derive(Debug, Default)]
pub struct ConnectionAlias {
    properties: ConnectionAliasProperties
}

/// Properties for the `ConnectionAlias` resource.
#[derive(Debug, Default)]
pub struct ConnectionAliasProperties {
    /// Property [`ConnectionString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-connectionalias.html#cfn-workspaces-connectionalias-connectionstring).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connection_string: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-connectionalias.html#cfn-workspaces-connectionalias-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ConnectionAliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionString", &self.connection_string)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectionAliasProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionAliasProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectionAliasProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectionAliasProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connection_string: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectionString" => {
                            connection_string = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectionAliasProperties {
                    connection_string: connection_string.ok_or(::serde::de::Error::missing_field("ConnectionString"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConnectionAlias {
    type Properties = ConnectionAliasProperties;
    const TYPE: &'static str = "AWS::WorkSpaces::ConnectionAlias";
    fn properties(&self) -> &ConnectionAliasProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectionAliasProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConnectionAlias {}

impl From<ConnectionAliasProperties> for ConnectionAlias {
    fn from(properties: ConnectionAliasProperties) -> ConnectionAlias {
        ConnectionAlias { properties }
    }
}

/// The [`AWS::WorkSpaces::Workspace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html) resource type.
#[derive(Debug, Default)]
pub struct Workspace {
    properties: WorkspaceProperties
}

/// Properties for the `Workspace` resource.
#[derive(Debug, Default)]
pub struct WorkspaceProperties {
    /// Property [`BundleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html#cfn-workspaces-workspace-bundleid).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub bundle_id: ::Value<String>,
    /// Property [`DirectoryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html#cfn-workspaces-workspace-directoryid).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub directory_id: ::Value<String>,
    /// Property [`RootVolumeEncryptionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html#cfn-workspaces-workspace-rootvolumeencryptionenabled).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub root_volume_encryption_enabled: Option<::Value<bool>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html#cfn-workspaces-workspace-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html#cfn-workspaces-workspace-username).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_name: ::Value<String>,
    /// Property [`UserVolumeEncryptionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html#cfn-workspaces-workspace-uservolumeencryptionenabled).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub user_volume_encryption_enabled: Option<::Value<bool>>,
    /// Property [`VolumeEncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html#cfn-workspaces-workspace-volumeencryptionkey).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub volume_encryption_key: Option<::Value<String>>,
    /// Property [`WorkspaceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html#cfn-workspaces-workspace-workspaceproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub workspace_properties: Option<::Value<self::workspace::WorkspaceProperties>>,
}

impl ::serde::Serialize for WorkspaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BundleId", &self.bundle_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryId", &self.directory_id)?;
        if let Some(ref root_volume_encryption_enabled) = self.root_volume_encryption_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootVolumeEncryptionEnabled", root_volume_encryption_enabled)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", &self.user_name)?;
        if let Some(ref user_volume_encryption_enabled) = self.user_volume_encryption_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserVolumeEncryptionEnabled", user_volume_encryption_enabled)?;
        }
        if let Some(ref volume_encryption_key) = self.volume_encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeEncryptionKey", volume_encryption_key)?;
        }
        if let Some(ref workspace_properties) = self.workspace_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkspaceProperties", workspace_properties)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkspaceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkspaceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WorkspaceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WorkspaceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bundle_id: Option<::Value<String>> = None;
                let mut directory_id: Option<::Value<String>> = None;
                let mut root_volume_encryption_enabled: Option<::Value<bool>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_name: Option<::Value<String>> = None;
                let mut user_volume_encryption_enabled: Option<::Value<bool>> = None;
                let mut volume_encryption_key: Option<::Value<String>> = None;
                let mut workspace_properties: Option<::Value<self::workspace::WorkspaceProperties>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BundleId" => {
                            bundle_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DirectoryId" => {
                            directory_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RootVolumeEncryptionEnabled" => {
                            root_volume_encryption_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserName" => {
                            user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserVolumeEncryptionEnabled" => {
                            user_volume_encryption_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VolumeEncryptionKey" => {
                            volume_encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkspaceProperties" => {
                            workspace_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkspaceProperties {
                    bundle_id: bundle_id.ok_or(::serde::de::Error::missing_field("BundleId"))?,
                    directory_id: directory_id.ok_or(::serde::de::Error::missing_field("DirectoryId"))?,
                    root_volume_encryption_enabled: root_volume_encryption_enabled,
                    tags: tags,
                    user_name: user_name.ok_or(::serde::de::Error::missing_field("UserName"))?,
                    user_volume_encryption_enabled: user_volume_encryption_enabled,
                    volume_encryption_key: volume_encryption_key,
                    workspace_properties: workspace_properties,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Workspace {
    type Properties = WorkspaceProperties;
    const TYPE: &'static str = "AWS::WorkSpaces::Workspace";
    fn properties(&self) -> &WorkspaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkspaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Workspace {}

impl From<WorkspaceProperties> for Workspace {
    fn from(properties: WorkspaceProperties) -> Workspace {
        Workspace { properties }
    }
}

pub mod connection_alias {
    //! Property types for the `ConnectionAlias` resource.

    /// The [`AWS::WorkSpaces::ConnectionAlias.ConnectionAliasAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-connectionalias-connectionaliasassociation.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectionAliasAssociation {
        /// Property [`AssociatedAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-connectionalias-connectionaliasassociation.html#cfn-workspaces-connectionalias-connectionaliasassociation-associatedaccountid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub associated_account_id: Option<::Value<String>>,
        /// Property [`AssociationStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-connectionalias-connectionaliasassociation.html#cfn-workspaces-connectionalias-connectionaliasassociation-associationstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub association_status: Option<::Value<String>>,
        /// Property [`ConnectionIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-connectionalias-connectionaliasassociation.html#cfn-workspaces-connectionalias-connectionaliasassociation-connectionidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_identifier: Option<::Value<String>>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-connectionalias-connectionaliasassociation.html#cfn-workspaces-connectionalias-connectionaliasassociation-resourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConnectionAliasAssociation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref associated_account_id) = self.associated_account_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatedAccountId", associated_account_id)?;
            }
            if let Some(ref association_status) = self.association_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociationStatus", association_status)?;
            }
            if let Some(ref connection_identifier) = self.connection_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionIdentifier", connection_identifier)?;
            }
            if let Some(ref resource_id) = self.resource_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", resource_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectionAliasAssociation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionAliasAssociation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionAliasAssociation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionAliasAssociation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut associated_account_id: Option<::Value<String>> = None;
                    let mut association_status: Option<::Value<String>> = None;
                    let mut connection_identifier: Option<::Value<String>> = None;
                    let mut resource_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssociatedAccountId" => {
                                associated_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AssociationStatus" => {
                                association_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectionIdentifier" => {
                                connection_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionAliasAssociation {
                        associated_account_id: associated_account_id,
                        association_status: association_status,
                        connection_identifier: connection_identifier,
                        resource_id: resource_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod workspace {
    //! Property types for the `Workspace` resource.

    /// The [`AWS::WorkSpaces::Workspace.WorkspaceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-workspace-workspaceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkspaceProperties {
        /// Property [`ComputeTypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-workspace-workspaceproperties.html#cfn-workspaces-workspace-workspaceproperties-computetypename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compute_type_name: Option<::Value<String>>,
        /// Property [`RootVolumeSizeGib`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-workspace-workspaceproperties.html#cfn-workspaces-workspace-workspaceproperties-rootvolumesizegib).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub root_volume_size_gib: Option<::Value<u32>>,
        /// Property [`RunningMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-workspace-workspaceproperties.html#cfn-workspaces-workspace-workspaceproperties-runningmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub running_mode: Option<::Value<String>>,
        /// Property [`RunningModeAutoStopTimeoutInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-workspace-workspaceproperties.html#cfn-workspaces-workspace-workspaceproperties-runningmodeautostoptimeoutinminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub running_mode_auto_stop_timeout_in_minutes: Option<::Value<u32>>,
        /// Property [`UserVolumeSizeGib`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspaces-workspace-workspaceproperties.html#cfn-workspaces-workspace-workspaceproperties-uservolumesizegib).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_volume_size_gib: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for WorkspaceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref compute_type_name) = self.compute_type_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeTypeName", compute_type_name)?;
            }
            if let Some(ref root_volume_size_gib) = self.root_volume_size_gib {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootVolumeSizeGib", root_volume_size_gib)?;
            }
            if let Some(ref running_mode) = self.running_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunningMode", running_mode)?;
            }
            if let Some(ref running_mode_auto_stop_timeout_in_minutes) = self.running_mode_auto_stop_timeout_in_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunningModeAutoStopTimeoutInMinutes", running_mode_auto_stop_timeout_in_minutes)?;
            }
            if let Some(ref user_volume_size_gib) = self.user_volume_size_gib {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserVolumeSizeGib", user_volume_size_gib)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkspaceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkspaceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkspaceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkspaceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compute_type_name: Option<::Value<String>> = None;
                    let mut root_volume_size_gib: Option<::Value<u32>> = None;
                    let mut running_mode: Option<::Value<String>> = None;
                    let mut running_mode_auto_stop_timeout_in_minutes: Option<::Value<u32>> = None;
                    let mut user_volume_size_gib: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComputeTypeName" => {
                                compute_type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RootVolumeSizeGib" => {
                                root_volume_size_gib = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RunningMode" => {
                                running_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RunningModeAutoStopTimeoutInMinutes" => {
                                running_mode_auto_stop_timeout_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserVolumeSizeGib" => {
                                user_volume_size_gib = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkspaceProperties {
                        compute_type_name: compute_type_name,
                        root_volume_size_gib: root_volume_size_gib,
                        running_mode: running_mode,
                        running_mode_auto_stop_timeout_in_minutes: running_mode_auto_stop_timeout_in_minutes,
                        user_volume_size_gib: user_volume_size_gib,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
