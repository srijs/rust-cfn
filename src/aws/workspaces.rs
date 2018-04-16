//! Types for the `WorkSpaces` service.

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
}

impl ::serde::Serialize for WorkspaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BundleId", &self.bundle_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryId", &self.directory_id)?;
        if let Some(ref root_volume_encryption_enabled) = self.root_volume_encryption_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootVolumeEncryptionEnabled", root_volume_encryption_enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", &self.user_name)?;
        if let Some(ref user_volume_encryption_enabled) = self.user_volume_encryption_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserVolumeEncryptionEnabled", user_volume_encryption_enabled)?;
        }
        if let Some(ref volume_encryption_key) = self.volume_encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeEncryptionKey", volume_encryption_key)?;
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
                let mut user_name: Option<::Value<String>> = None;
                let mut user_volume_encryption_enabled: Option<::Value<bool>> = None;
                let mut volume_encryption_key: Option<::Value<String>> = None;

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
                        "UserName" => {
                            user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserVolumeEncryptionEnabled" => {
                            user_volume_encryption_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VolumeEncryptionKey" => {
                            volume_encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkspaceProperties {
                    bundle_id: bundle_id.ok_or(::serde::de::Error::missing_field("BundleId"))?,
                    directory_id: directory_id.ok_or(::serde::de::Error::missing_field("DirectoryId"))?,
                    root_volume_encryption_enabled: root_volume_encryption_enabled,
                    user_name: user_name.ok_or(::serde::de::Error::missing_field("UserName"))?,
                    user_volume_encryption_enabled: user_volume_encryption_enabled,
                    volume_encryption_key: volume_encryption_key,
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
