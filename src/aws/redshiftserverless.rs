//! Types for the `RedshiftServerless` service.

/// The [`AWS::RedshiftServerless::Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html) resource type.
#[derive(Debug, Default)]
pub struct Namespace {
    properties: NamespaceProperties
}

/// Properties for the `Namespace` resource.
#[derive(Debug, Default)]
pub struct NamespaceProperties {
    /// Property [`AdminUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-adminuserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub admin_user_password: Option<::Value<String>>,
    /// Property [`AdminUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-adminusername).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub admin_username: Option<::Value<String>>,
    /// Property [`DbName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-dbname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_name: Option<::Value<String>>,
    /// Property [`DefaultIamRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-defaultiamrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_iam_role_arn: Option<::Value<String>>,
    /// Property [`FinalSnapshotName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-finalsnapshotname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub final_snapshot_name: Option<::Value<String>>,
    /// Property [`FinalSnapshotRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-finalsnapshotretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub final_snapshot_retention_period: Option<::Value<u32>>,
    /// Property [`IamRoles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-iamroles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_roles: Option<::ValueList<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-kmskeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`LogExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-logexports).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_exports: Option<::ValueList<String>>,
    /// Property [`NamespaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-namespacename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub namespace_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for NamespaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref admin_user_password) = self.admin_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminUserPassword", admin_user_password)?;
        }
        if let Some(ref admin_username) = self.admin_username {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminUsername", admin_username)?;
        }
        if let Some(ref db_name) = self.db_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DbName", db_name)?;
        }
        if let Some(ref default_iam_role_arn) = self.default_iam_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultIamRoleArn", default_iam_role_arn)?;
        }
        if let Some(ref final_snapshot_name) = self.final_snapshot_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FinalSnapshotName", final_snapshot_name)?;
        }
        if let Some(ref final_snapshot_retention_period) = self.final_snapshot_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FinalSnapshotRetentionPeriod", final_snapshot_retention_period)?;
        }
        if let Some(ref iam_roles) = self.iam_roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoles", iam_roles)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref log_exports) = self.log_exports {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogExports", log_exports)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceName", &self.namespace_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NamespaceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NamespaceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NamespaceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NamespaceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut admin_user_password: Option<::Value<String>> = None;
                let mut admin_username: Option<::Value<String>> = None;
                let mut db_name: Option<::Value<String>> = None;
                let mut default_iam_role_arn: Option<::Value<String>> = None;
                let mut final_snapshot_name: Option<::Value<String>> = None;
                let mut final_snapshot_retention_period: Option<::Value<u32>> = None;
                let mut iam_roles: Option<::ValueList<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut log_exports: Option<::ValueList<String>> = None;
                let mut namespace_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdminUserPassword" => {
                            admin_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdminUsername" => {
                            admin_username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DbName" => {
                            db_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultIamRoleArn" => {
                            default_iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FinalSnapshotName" => {
                            final_snapshot_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FinalSnapshotRetentionPeriod" => {
                            final_snapshot_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamRoles" => {
                            iam_roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogExports" => {
                            log_exports = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NamespaceName" => {
                            namespace_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NamespaceProperties {
                    admin_user_password: admin_user_password,
                    admin_username: admin_username,
                    db_name: db_name,
                    default_iam_role_arn: default_iam_role_arn,
                    final_snapshot_name: final_snapshot_name,
                    final_snapshot_retention_period: final_snapshot_retention_period,
                    iam_roles: iam_roles,
                    kms_key_id: kms_key_id,
                    log_exports: log_exports,
                    namespace_name: namespace_name.ok_or(::serde::de::Error::missing_field("NamespaceName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Namespace {
    type Properties = NamespaceProperties;
    const TYPE: &'static str = "AWS::RedshiftServerless::Namespace";
    fn properties(&self) -> &NamespaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NamespaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Namespace {}

impl From<NamespaceProperties> for Namespace {
    fn from(properties: NamespaceProperties) -> Namespace {
        Namespace { properties }
    }
}

/// The [`AWS::RedshiftServerless::Workgroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html) resource type.
#[derive(Debug, Default)]
pub struct Workgroup {
    properties: WorkgroupProperties
}

/// Properties for the `Workgroup` resource.
#[derive(Debug, Default)]
pub struct WorkgroupProperties {
    /// Property [`BaseCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-basecapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub base_capacity: Option<::Value<u32>>,
    /// Property [`ConfigParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-configparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub config_parameters: Option<::ValueList<self::workgroup::ConfigParameter>>,
    /// Property [`EnhancedVpcRouting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-enhancedvpcrouting).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enhanced_vpc_routing: Option<::Value<bool>>,
    /// Property [`NamespaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-namespacename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub namespace_name: Option<::Value<String>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-port).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub port: Option<::Value<u32>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-publiclyaccessible).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-securitygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-subnetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_ids: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`WorkgroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-workgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workgroup_name: ::Value<String>,
}

impl ::serde::Serialize for WorkgroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref base_capacity) = self.base_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseCapacity", base_capacity)?;
        }
        if let Some(ref config_parameters) = self.config_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigParameters", config_parameters)?;
        }
        if let Some(ref enhanced_vpc_routing) = self.enhanced_vpc_routing {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnhancedVpcRouting", enhanced_vpc_routing)?;
        }
        if let Some(ref namespace_name) = self.namespace_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceName", namespace_name)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref publicly_accessible) = self.publicly_accessible {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
        }
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref subnet_ids) = self.subnet_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkgroupName", &self.workgroup_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkgroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkgroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WorkgroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WorkgroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut base_capacity: Option<::Value<u32>> = None;
                let mut config_parameters: Option<::ValueList<self::workgroup::ConfigParameter>> = None;
                let mut enhanced_vpc_routing: Option<::Value<bool>> = None;
                let mut namespace_name: Option<::Value<String>> = None;
                let mut port: Option<::Value<u32>> = None;
                let mut publicly_accessible: Option<::Value<bool>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut workgroup_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BaseCapacity" => {
                            base_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConfigParameters" => {
                            config_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnhancedVpcRouting" => {
                            enhanced_vpc_routing = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NamespaceName" => {
                            namespace_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkgroupName" => {
                            workgroup_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkgroupProperties {
                    base_capacity: base_capacity,
                    config_parameters: config_parameters,
                    enhanced_vpc_routing: enhanced_vpc_routing,
                    namespace_name: namespace_name,
                    port: port,
                    publicly_accessible: publicly_accessible,
                    security_group_ids: security_group_ids,
                    subnet_ids: subnet_ids,
                    tags: tags,
                    workgroup_name: workgroup_name.ok_or(::serde::de::Error::missing_field("WorkgroupName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Workgroup {
    type Properties = WorkgroupProperties;
    const TYPE: &'static str = "AWS::RedshiftServerless::Workgroup";
    fn properties(&self) -> &WorkgroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkgroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Workgroup {}

impl From<WorkgroupProperties> for Workgroup {
    fn from(properties: WorkgroupProperties) -> Workgroup {
        Workgroup { properties }
    }
}

pub mod namespace {
    //! Property types for the `Namespace` resource.

    /// The [`AWS::RedshiftServerless::Namespace.Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html) property type.
    #[derive(Debug, Default)]
    pub struct Namespace {
        /// Property [`AdminUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-adminusername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub admin_username: Option<::Value<String>>,
        /// Property [`CreationDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-creationdate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub creation_date: Option<::Value<String>>,
        /// Property [`DbName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-dbname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub db_name: Option<::Value<String>>,
        /// Property [`DefaultIamRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-defaultiamrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_iam_role_arn: Option<::Value<String>>,
        /// Property [`IamRoles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-iamroles).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_roles: Option<::ValueList<String>>,
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`LogExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-logexports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_exports: Option<::ValueList<String>>,
        /// Property [`NamespaceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-namespacearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace_arn: Option<::Value<String>>,
        /// Property [`NamespaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-namespaceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace_id: Option<::Value<String>>,
        /// Property [`NamespaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-namespacename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace_name: Option<::Value<String>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html#cfn-redshiftserverless-namespace-namespace-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Namespace {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref admin_username) = self.admin_username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminUsername", admin_username)?;
            }
            if let Some(ref creation_date) = self.creation_date {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreationDate", creation_date)?;
            }
            if let Some(ref db_name) = self.db_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DbName", db_name)?;
            }
            if let Some(ref default_iam_role_arn) = self.default_iam_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultIamRoleArn", default_iam_role_arn)?;
            }
            if let Some(ref iam_roles) = self.iam_roles {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoles", iam_roles)?;
            }
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            if let Some(ref log_exports) = self.log_exports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogExports", log_exports)?;
            }
            if let Some(ref namespace_arn) = self.namespace_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceArn", namespace_arn)?;
            }
            if let Some(ref namespace_id) = self.namespace_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceId", namespace_id)?;
            }
            if let Some(ref namespace_name) = self.namespace_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceName", namespace_name)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Namespace {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Namespace, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Namespace;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Namespace")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut admin_username: Option<::Value<String>> = None;
                    let mut creation_date: Option<::Value<String>> = None;
                    let mut db_name: Option<::Value<String>> = None;
                    let mut default_iam_role_arn: Option<::Value<String>> = None;
                    let mut iam_roles: Option<::ValueList<String>> = None;
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut log_exports: Option<::ValueList<String>> = None;
                    let mut namespace_arn: Option<::Value<String>> = None;
                    let mut namespace_id: Option<::Value<String>> = None;
                    let mut namespace_name: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdminUsername" => {
                                admin_username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreationDate" => {
                                creation_date = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DbName" => {
                                db_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultIamRoleArn" => {
                                default_iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IamRoles" => {
                                iam_roles = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogExports" => {
                                log_exports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NamespaceArn" => {
                                namespace_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NamespaceId" => {
                                namespace_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NamespaceName" => {
                                namespace_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Namespace {
                        admin_username: admin_username,
                        creation_date: creation_date,
                        db_name: db_name,
                        default_iam_role_arn: default_iam_role_arn,
                        iam_roles: iam_roles,
                        kms_key_id: kms_key_id,
                        log_exports: log_exports,
                        namespace_arn: namespace_arn,
                        namespace_id: namespace_id,
                        namespace_name: namespace_name,
                        status: status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod workgroup {
    //! Property types for the `Workgroup` resource.

    /// The [`AWS::RedshiftServerless::Workgroup.ConfigParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-configparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfigParameter {
        /// Property [`ParameterKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-configparameter.html#cfn-redshiftserverless-workgroup-configparameter-parameterkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_key: Option<::Value<String>>,
        /// Property [`ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-configparameter.html#cfn-redshiftserverless-workgroup-configparameter-parametervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConfigParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref parameter_key) = self.parameter_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterKey", parameter_key)?;
            }
            if let Some(ref parameter_value) = self.parameter_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValue", parameter_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameter_key: Option<::Value<String>> = None;
                    let mut parameter_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterKey" => {
                                parameter_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterValue" => {
                                parameter_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigParameter {
                        parameter_key: parameter_key,
                        parameter_value: parameter_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RedshiftServerless::Workgroup.Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-endpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct Endpoint {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-endpoint.html#cfn-redshiftserverless-workgroup-endpoint-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-endpoint.html#cfn-redshiftserverless-workgroup-endpoint-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`VpcEndpoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-endpoint.html#cfn-redshiftserverless-workgroup-endpoint-vpcendpoints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_endpoints: Option<::ValueList<VpcEndpoint>>,
    }

    impl ::codec::SerializeValue for Endpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address) = self.address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", address)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref vpc_endpoints) = self.vpc_endpoints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcEndpoints", vpc_endpoints)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Endpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Endpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Endpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Endpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut vpc_endpoints: Option<::ValueList<VpcEndpoint>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcEndpoints" => {
                                vpc_endpoints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Endpoint {
                        address: address,
                        port: port,
                        vpc_endpoints: vpc_endpoints,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RedshiftServerless::Workgroup.NetworkInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-networkinterface.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkInterface {
        /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-networkinterface.html#cfn-redshiftserverless-workgroup-networkinterface-availabilityzone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_zone: Option<::Value<String>>,
        /// Property [`NetworkInterfaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-networkinterface.html#cfn-redshiftserverless-workgroup-networkinterface-networkinterfaceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_interface_id: Option<::Value<String>>,
        /// Property [`PrivateIpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-networkinterface.html#cfn-redshiftserverless-workgroup-networkinterface-privateipaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_ip_address: Option<::Value<String>>,
        /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-networkinterface.html#cfn-redshiftserverless-workgroup-networkinterface-subnetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NetworkInterface {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_zone) = self.availability_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
            }
            if let Some(ref network_interface_id) = self.network_interface_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaceId", network_interface_id)?;
            }
            if let Some(ref private_ip_address) = self.private_ip_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddress", private_ip_address)?;
            }
            if let Some(ref subnet_id) = self.subnet_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", subnet_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkInterface {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkInterface, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkInterface;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkInterface")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone: Option<::Value<String>> = None;
                    let mut network_interface_id: Option<::Value<String>> = None;
                    let mut private_ip_address: Option<::Value<String>> = None;
                    let mut subnet_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkInterfaceId" => {
                                network_interface_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateIpAddress" => {
                                private_ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetId" => {
                                subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkInterface {
                        availability_zone: availability_zone,
                        network_interface_id: network_interface_id,
                        private_ip_address: private_ip_address,
                        subnet_id: subnet_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RedshiftServerless::Workgroup.VpcEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-vpcendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcEndpoint {
        /// Property [`NetworkInterfaces`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-vpcendpoint.html#cfn-redshiftserverless-workgroup-vpcendpoint-networkinterfaces).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_interfaces: Option<::ValueList<NetworkInterface>>,
        /// Property [`VpcEndpointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-vpcendpoint.html#cfn-redshiftserverless-workgroup-vpcendpoint-vpcendpointid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_endpoint_id: Option<::Value<String>>,
        /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-vpcendpoint.html#cfn-redshiftserverless-workgroup-vpcendpoint-vpcid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VpcEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref network_interfaces) = self.network_interfaces {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaces", network_interfaces)?;
            }
            if let Some(ref vpc_endpoint_id) = self.vpc_endpoint_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcEndpointId", vpc_endpoint_id)?;
            }
            if let Some(ref vpc_id) = self.vpc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut network_interfaces: Option<::ValueList<NetworkInterface>> = None;
                    let mut vpc_endpoint_id: Option<::Value<String>> = None;
                    let mut vpc_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NetworkInterfaces" => {
                                network_interfaces = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcEndpointId" => {
                                vpc_endpoint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcEndpoint {
                        network_interfaces: network_interfaces,
                        vpc_endpoint_id: vpc_endpoint_id,
                        vpc_id: vpc_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RedshiftServerless::Workgroup.Workgroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html) property type.
    #[derive(Debug, Default)]
    pub struct Workgroup {
        /// Property [`BaseCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-basecapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_capacity: Option<::Value<u32>>,
        /// Property [`ConfigParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-configparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub config_parameters: Option<::ValueList<ConfigParameter>>,
        /// Property [`CreationDate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-creationdate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub creation_date: Option<::Value<String>>,
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: Option<::Value<Endpoint>>,
        /// Property [`EnhancedVpcRouting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-enhancedvpcrouting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enhanced_vpc_routing: Option<::Value<bool>>,
        /// Property [`NamespaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-namespacename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace_name: Option<::Value<String>>,
        /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-publiclyaccessible).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub publicly_accessible: Option<::Value<bool>>,
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
        /// Property [`WorkgroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-workgrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workgroup_arn: Option<::Value<String>>,
        /// Property [`WorkgroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-workgroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workgroup_id: Option<::Value<String>>,
        /// Property [`WorkgroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html#cfn-redshiftserverless-workgroup-workgroup-workgroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workgroup_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Workgroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref base_capacity) = self.base_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseCapacity", base_capacity)?;
            }
            if let Some(ref config_parameters) = self.config_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigParameters", config_parameters)?;
            }
            if let Some(ref creation_date) = self.creation_date {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreationDate", creation_date)?;
            }
            if let Some(ref endpoint) = self.endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", endpoint)?;
            }
            if let Some(ref enhanced_vpc_routing) = self.enhanced_vpc_routing {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnhancedVpcRouting", enhanced_vpc_routing)?;
            }
            if let Some(ref namespace_name) = self.namespace_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceName", namespace_name)?;
            }
            if let Some(ref publicly_accessible) = self.publicly_accessible {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
            }
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            if let Some(ref workgroup_arn) = self.workgroup_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkgroupArn", workgroup_arn)?;
            }
            if let Some(ref workgroup_id) = self.workgroup_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkgroupId", workgroup_id)?;
            }
            if let Some(ref workgroup_name) = self.workgroup_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkgroupName", workgroup_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Workgroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Workgroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Workgroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Workgroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base_capacity: Option<::Value<u32>> = None;
                    let mut config_parameters: Option<::ValueList<ConfigParameter>> = None;
                    let mut creation_date: Option<::Value<String>> = None;
                    let mut endpoint: Option<::Value<Endpoint>> = None;
                    let mut enhanced_vpc_routing: Option<::Value<bool>> = None;
                    let mut namespace_name: Option<::Value<String>> = None;
                    let mut publicly_accessible: Option<::Value<bool>> = None;
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut status: Option<::Value<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;
                    let mut workgroup_arn: Option<::Value<String>> = None;
                    let mut workgroup_id: Option<::Value<String>> = None;
                    let mut workgroup_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseCapacity" => {
                                base_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfigParameters" => {
                                config_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreationDate" => {
                                creation_date = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnhancedVpcRouting" => {
                                enhanced_vpc_routing = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NamespaceName" => {
                                namespace_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PubliclyAccessible" => {
                                publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkgroupArn" => {
                                workgroup_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkgroupId" => {
                                workgroup_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkgroupName" => {
                                workgroup_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Workgroup {
                        base_capacity: base_capacity,
                        config_parameters: config_parameters,
                        creation_date: creation_date,
                        endpoint: endpoint,
                        enhanced_vpc_routing: enhanced_vpc_routing,
                        namespace_name: namespace_name,
                        publicly_accessible: publicly_accessible,
                        security_group_ids: security_group_ids,
                        status: status,
                        subnet_ids: subnet_ids,
                        workgroup_arn: workgroup_arn,
                        workgroup_id: workgroup_id,
                        workgroup_name: workgroup_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
