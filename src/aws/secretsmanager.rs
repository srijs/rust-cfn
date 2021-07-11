//! Types for the `SecretsManager` service.

/// The [`AWS::SecretsManager::ResourcePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-resourcepolicy.html) resource type.
#[derive(Debug, Default)]
pub struct ResourcePolicy {
    properties: ResourcePolicyProperties
}

/// Properties for the `ResourcePolicy` resource.
#[derive(Debug, Default)]
pub struct ResourcePolicyProperties {
    /// Property [`BlockPublicPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-resourcepolicy.html#cfn-secretsmanager-resourcepolicy-blockpublicpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub block_public_policy: Option<::Value<bool>>,
    /// Property [`ResourcePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-resourcepolicy.html#cfn-secretsmanager-resourcepolicy-resourcepolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_policy: ::Value<::json::Value>,
    /// Property [`SecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-resourcepolicy.html#cfn-secretsmanager-resourcepolicy-secretid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub secret_id: ::Value<String>,
}

impl ::serde::Serialize for ResourcePolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref block_public_policy) = self.block_public_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockPublicPolicy", block_public_policy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcePolicy", &self.resource_policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretId", &self.secret_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourcePolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourcePolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourcePolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourcePolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut block_public_policy: Option<::Value<bool>> = None;
                let mut resource_policy: Option<::Value<::json::Value>> = None;
                let mut secret_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BlockPublicPolicy" => {
                            block_public_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourcePolicy" => {
                            resource_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecretId" => {
                            secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourcePolicyProperties {
                    block_public_policy: block_public_policy,
                    resource_policy: resource_policy.ok_or(::serde::de::Error::missing_field("ResourcePolicy"))?,
                    secret_id: secret_id.ok_or(::serde::de::Error::missing_field("SecretId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourcePolicy {
    type Properties = ResourcePolicyProperties;
    const TYPE: &'static str = "AWS::SecretsManager::ResourcePolicy";
    fn properties(&self) -> &ResourcePolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourcePolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourcePolicy {}

impl From<ResourcePolicyProperties> for ResourcePolicy {
    fn from(properties: ResourcePolicyProperties) -> ResourcePolicy {
        ResourcePolicy { properties }
    }
}

/// The [`AWS::SecretsManager::RotationSchedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-rotationschedule.html) resource type.
#[derive(Debug, Default)]
pub struct RotationSchedule {
    properties: RotationScheduleProperties
}

/// Properties for the `RotationSchedule` resource.
#[derive(Debug, Default)]
pub struct RotationScheduleProperties {
    /// Property [`HostedRotationLambda`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-rotationschedule.html#cfn-secretsmanager-rotationschedule-hostedrotationlambda).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hosted_rotation_lambda: Option<::Value<self::rotation_schedule::HostedRotationLambda>>,
    /// Property [`RotationLambdaARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-rotationschedule.html#cfn-secretsmanager-rotationschedule-rotationlambdaarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rotation_lambda_arn: Option<::Value<String>>,
    /// Property [`RotationRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-rotationschedule.html#cfn-secretsmanager-rotationschedule-rotationrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rotation_rules: Option<::Value<self::rotation_schedule::RotationRules>>,
    /// Property [`SecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-rotationschedule.html#cfn-secretsmanager-rotationschedule-secretid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub secret_id: ::Value<String>,
}

impl ::serde::Serialize for RotationScheduleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref hosted_rotation_lambda) = self.hosted_rotation_lambda {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedRotationLambda", hosted_rotation_lambda)?;
        }
        if let Some(ref rotation_lambda_arn) = self.rotation_lambda_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RotationLambdaARN", rotation_lambda_arn)?;
        }
        if let Some(ref rotation_rules) = self.rotation_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RotationRules", rotation_rules)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretId", &self.secret_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RotationScheduleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RotationScheduleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RotationScheduleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RotationScheduleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut hosted_rotation_lambda: Option<::Value<self::rotation_schedule::HostedRotationLambda>> = None;
                let mut rotation_lambda_arn: Option<::Value<String>> = None;
                let mut rotation_rules: Option<::Value<self::rotation_schedule::RotationRules>> = None;
                let mut secret_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HostedRotationLambda" => {
                            hosted_rotation_lambda = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RotationLambdaARN" => {
                            rotation_lambda_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RotationRules" => {
                            rotation_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecretId" => {
                            secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RotationScheduleProperties {
                    hosted_rotation_lambda: hosted_rotation_lambda,
                    rotation_lambda_arn: rotation_lambda_arn,
                    rotation_rules: rotation_rules,
                    secret_id: secret_id.ok_or(::serde::de::Error::missing_field("SecretId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RotationSchedule {
    type Properties = RotationScheduleProperties;
    const TYPE: &'static str = "AWS::SecretsManager::RotationSchedule";
    fn properties(&self) -> &RotationScheduleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RotationScheduleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RotationSchedule {}

impl From<RotationScheduleProperties> for RotationSchedule {
    fn from(properties: RotationScheduleProperties) -> RotationSchedule {
        RotationSchedule { properties }
    }
}

/// The [`AWS::SecretsManager::Secret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secret.html) resource type.
#[derive(Debug, Default)]
pub struct Secret {
    properties: SecretProperties
}

/// Properties for the `Secret` resource.
#[derive(Debug, Default)]
pub struct SecretProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secret.html#cfn-secretsmanager-secret-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`GenerateSecretString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secret.html#cfn-secretsmanager-secret-generatesecretstring).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub generate_secret_string: Option<::Value<self::secret::GenerateSecretString>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secret.html#cfn-secretsmanager-secret-kmskeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secret.html#cfn-secretsmanager-secret-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`ReplicaRegions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secret.html#cfn-secretsmanager-secret-replicaregions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replica_regions: Option<::ValueList<self::secret::ReplicaRegion>>,
    /// Property [`SecretString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secret.html#cfn-secretsmanager-secret-secretstring).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub secret_string: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secret.html#cfn-secretsmanager-secret-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SecretProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref generate_secret_string) = self.generate_secret_string {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GenerateSecretString", generate_secret_string)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref replica_regions) = self.replica_regions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicaRegions", replica_regions)?;
        }
        if let Some(ref secret_string) = self.secret_string {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretString", secret_string)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecretProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecretProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecretProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecretProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut generate_secret_string: Option<::Value<self::secret::GenerateSecretString>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut replica_regions: Option<::ValueList<self::secret::ReplicaRegion>> = None;
                let mut secret_string: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GenerateSecretString" => {
                            generate_secret_string = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicaRegions" => {
                            replica_regions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecretString" => {
                            secret_string = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SecretProperties {
                    description: description,
                    generate_secret_string: generate_secret_string,
                    kms_key_id: kms_key_id,
                    name: name,
                    replica_regions: replica_regions,
                    secret_string: secret_string,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Secret {
    type Properties = SecretProperties;
    const TYPE: &'static str = "AWS::SecretsManager::Secret";
    fn properties(&self) -> &SecretProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecretProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Secret {}

impl From<SecretProperties> for Secret {
    fn from(properties: SecretProperties) -> Secret {
        Secret { properties }
    }
}

/// The [`AWS::SecretsManager::SecretTargetAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secrettargetattachment.html) resource type.
#[derive(Debug, Default)]
pub struct SecretTargetAttachment {
    properties: SecretTargetAttachmentProperties
}

/// Properties for the `SecretTargetAttachment` resource.
#[derive(Debug, Default)]
pub struct SecretTargetAttachmentProperties {
    /// Property [`SecretId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secrettargetattachment.html#cfn-secretsmanager-secrettargetattachment-secretid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub secret_id: ::Value<String>,
    /// Property [`TargetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secrettargetattachment.html#cfn-secretsmanager-secrettargetattachment-targetid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_id: ::Value<String>,
    /// Property [`TargetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-secretsmanager-secrettargetattachment.html#cfn-secretsmanager-secrettargetattachment-targettype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_type: ::Value<String>,
}

impl ::serde::Serialize for SecretTargetAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretId", &self.secret_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetId", &self.target_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetType", &self.target_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecretTargetAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecretTargetAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecretTargetAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecretTargetAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut secret_id: Option<::Value<String>> = None;
                let mut target_id: Option<::Value<String>> = None;
                let mut target_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "SecretId" => {
                            secret_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetId" => {
                            target_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetType" => {
                            target_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SecretTargetAttachmentProperties {
                    secret_id: secret_id.ok_or(::serde::de::Error::missing_field("SecretId"))?,
                    target_id: target_id.ok_or(::serde::de::Error::missing_field("TargetId"))?,
                    target_type: target_type.ok_or(::serde::de::Error::missing_field("TargetType"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SecretTargetAttachment {
    type Properties = SecretTargetAttachmentProperties;
    const TYPE: &'static str = "AWS::SecretsManager::SecretTargetAttachment";
    fn properties(&self) -> &SecretTargetAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecretTargetAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecretTargetAttachment {}

impl From<SecretTargetAttachmentProperties> for SecretTargetAttachment {
    fn from(properties: SecretTargetAttachmentProperties) -> SecretTargetAttachment {
        SecretTargetAttachment { properties }
    }
}

pub mod rotation_schedule {
    //! Property types for the `RotationSchedule` resource.

    /// The [`AWS::SecretsManager::RotationSchedule.HostedRotationLambda`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-hostedrotationlambda.html) property type.
    #[derive(Debug, Default)]
    pub struct HostedRotationLambda {
        /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-hostedrotationlambda.html#cfn-secretsmanager-rotationschedule-hostedrotationlambda-kmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_arn: Option<::Value<String>>,
        /// Property [`MasterSecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-hostedrotationlambda.html#cfn-secretsmanager-rotationschedule-hostedrotationlambda-mastersecretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_secret_arn: Option<::Value<String>>,
        /// Property [`MasterSecretKmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-hostedrotationlambda.html#cfn-secretsmanager-rotationschedule-hostedrotationlambda-mastersecretkmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub master_secret_kms_key_arn: Option<::Value<String>>,
        /// Property [`RotationLambdaName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-hostedrotationlambda.html#cfn-secretsmanager-rotationschedule-hostedrotationlambda-rotationlambdaname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rotation_lambda_name: Option<::Value<String>>,
        /// Property [`RotationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-hostedrotationlambda.html#cfn-secretsmanager-rotationschedule-hostedrotationlambda-rotationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rotation_type: ::Value<String>,
        /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-hostedrotationlambda.html#cfn-secretsmanager-rotationschedule-hostedrotationlambda-vpcsecuritygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_security_group_ids: Option<::Value<String>>,
        /// Property [`VpcSubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-hostedrotationlambda.html#cfn-secretsmanager-rotationschedule-hostedrotationlambda-vpcsubnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_subnet_ids: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HostedRotationLambda {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_arn) = self.kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
            }
            if let Some(ref master_secret_arn) = self.master_secret_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterSecretArn", master_secret_arn)?;
            }
            if let Some(ref master_secret_kms_key_arn) = self.master_secret_kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterSecretKmsKeyArn", master_secret_kms_key_arn)?;
            }
            if let Some(ref rotation_lambda_name) = self.rotation_lambda_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RotationLambdaName", rotation_lambda_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RotationType", &self.rotation_type)?;
            if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
            }
            if let Some(ref vpc_subnet_ids) = self.vpc_subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSubnetIds", vpc_subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HostedRotationLambda {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HostedRotationLambda, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HostedRotationLambda;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HostedRotationLambda")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_arn: Option<::Value<String>> = None;
                    let mut master_secret_arn: Option<::Value<String>> = None;
                    let mut master_secret_kms_key_arn: Option<::Value<String>> = None;
                    let mut rotation_lambda_name: Option<::Value<String>> = None;
                    let mut rotation_type: Option<::Value<String>> = None;
                    let mut vpc_security_group_ids: Option<::Value<String>> = None;
                    let mut vpc_subnet_ids: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MasterSecretArn" => {
                                master_secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MasterSecretKmsKeyArn" => {
                                master_secret_kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RotationLambdaName" => {
                                rotation_lambda_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RotationType" => {
                                rotation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcSecurityGroupIds" => {
                                vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcSubnetIds" => {
                                vpc_subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HostedRotationLambda {
                        kms_key_arn: kms_key_arn,
                        master_secret_arn: master_secret_arn,
                        master_secret_kms_key_arn: master_secret_kms_key_arn,
                        rotation_lambda_name: rotation_lambda_name,
                        rotation_type: rotation_type.ok_or(::serde::de::Error::missing_field("RotationType"))?,
                        vpc_security_group_ids: vpc_security_group_ids,
                        vpc_subnet_ids: vpc_subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecretsManager::RotationSchedule.RotationRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-rotationrules.html) property type.
    #[derive(Debug, Default)]
    pub struct RotationRules {
        /// Property [`AutomaticallyAfterDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-rotationschedule-rotationrules.html#cfn-secretsmanager-rotationschedule-rotationrules-automaticallyafterdays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automatically_after_days: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RotationRules {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref automatically_after_days) = self.automatically_after_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticallyAfterDays", automatically_after_days)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RotationRules {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RotationRules, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RotationRules;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RotationRules")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut automatically_after_days: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutomaticallyAfterDays" => {
                                automatically_after_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RotationRules {
                        automatically_after_days: automatically_after_days,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod secret {
    //! Property types for the `Secret` resource.

    /// The [`AWS::SecretsManager::Secret.GenerateSecretString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html) property type.
    #[derive(Debug, Default)]
    pub struct GenerateSecretString {
        /// Property [`ExcludeCharacters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html#cfn-secretsmanager-secret-generatesecretstring-excludecharacters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_characters: Option<::Value<String>>,
        /// Property [`ExcludeLowercase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html#cfn-secretsmanager-secret-generatesecretstring-excludelowercase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_lowercase: Option<::Value<bool>>,
        /// Property [`ExcludeNumbers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html#cfn-secretsmanager-secret-generatesecretstring-excludenumbers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_numbers: Option<::Value<bool>>,
        /// Property [`ExcludePunctuation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html#cfn-secretsmanager-secret-generatesecretstring-excludepunctuation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_punctuation: Option<::Value<bool>>,
        /// Property [`ExcludeUppercase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html#cfn-secretsmanager-secret-generatesecretstring-excludeuppercase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exclude_uppercase: Option<::Value<bool>>,
        /// Property [`GenerateStringKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html#cfn-secretsmanager-secret-generatesecretstring-generatestringkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub generate_string_key: Option<::Value<String>>,
        /// Property [`IncludeSpace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html#cfn-secretsmanager-secret-generatesecretstring-includespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_space: Option<::Value<bool>>,
        /// Property [`PasswordLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html#cfn-secretsmanager-secret-generatesecretstring-passwordlength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password_length: Option<::Value<u32>>,
        /// Property [`RequireEachIncludedType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html#cfn-secretsmanager-secret-generatesecretstring-requireeachincludedtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_each_included_type: Option<::Value<bool>>,
        /// Property [`SecretStringTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-generatesecretstring.html#cfn-secretsmanager-secret-generatesecretstring-secretstringtemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_string_template: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GenerateSecretString {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exclude_characters) = self.exclude_characters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeCharacters", exclude_characters)?;
            }
            if let Some(ref exclude_lowercase) = self.exclude_lowercase {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeLowercase", exclude_lowercase)?;
            }
            if let Some(ref exclude_numbers) = self.exclude_numbers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeNumbers", exclude_numbers)?;
            }
            if let Some(ref exclude_punctuation) = self.exclude_punctuation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludePunctuation", exclude_punctuation)?;
            }
            if let Some(ref exclude_uppercase) = self.exclude_uppercase {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeUppercase", exclude_uppercase)?;
            }
            if let Some(ref generate_string_key) = self.generate_string_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GenerateStringKey", generate_string_key)?;
            }
            if let Some(ref include_space) = self.include_space {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeSpace", include_space)?;
            }
            if let Some(ref password_length) = self.password_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordLength", password_length)?;
            }
            if let Some(ref require_each_included_type) = self.require_each_included_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireEachIncludedType", require_each_included_type)?;
            }
            if let Some(ref secret_string_template) = self.secret_string_template {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretStringTemplate", secret_string_template)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GenerateSecretString {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GenerateSecretString, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GenerateSecretString;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GenerateSecretString")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exclude_characters: Option<::Value<String>> = None;
                    let mut exclude_lowercase: Option<::Value<bool>> = None;
                    let mut exclude_numbers: Option<::Value<bool>> = None;
                    let mut exclude_punctuation: Option<::Value<bool>> = None;
                    let mut exclude_uppercase: Option<::Value<bool>> = None;
                    let mut generate_string_key: Option<::Value<String>> = None;
                    let mut include_space: Option<::Value<bool>> = None;
                    let mut password_length: Option<::Value<u32>> = None;
                    let mut require_each_included_type: Option<::Value<bool>> = None;
                    let mut secret_string_template: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExcludeCharacters" => {
                                exclude_characters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeLowercase" => {
                                exclude_lowercase = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeNumbers" => {
                                exclude_numbers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludePunctuation" => {
                                exclude_punctuation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExcludeUppercase" => {
                                exclude_uppercase = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GenerateStringKey" => {
                                generate_string_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeSpace" => {
                                include_space = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PasswordLength" => {
                                password_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireEachIncludedType" => {
                                require_each_included_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretStringTemplate" => {
                                secret_string_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GenerateSecretString {
                        exclude_characters: exclude_characters,
                        exclude_lowercase: exclude_lowercase,
                        exclude_numbers: exclude_numbers,
                        exclude_punctuation: exclude_punctuation,
                        exclude_uppercase: exclude_uppercase,
                        generate_string_key: generate_string_key,
                        include_space: include_space,
                        password_length: password_length,
                        require_each_included_type: require_each_included_type,
                        secret_string_template: secret_string_template,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SecretsManager::Secret.ReplicaRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-replicaregion.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplicaRegion {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-replicaregion.html#cfn-secretsmanager-secret-replicaregion-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-secretsmanager-secret-replicaregion.html#cfn-secretsmanager-secret-replicaregion-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: ::Value<String>,
    }

    impl ::codec::SerializeValue for ReplicaRegion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicaRegion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicaRegion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicaRegion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicaRegion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicaRegion {
                        kms_key_id: kms_key_id,
                        region: region.ok_or(::serde::de::Error::missing_field("Region"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
