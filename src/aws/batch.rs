//! Types for the `Batch` service.

/// The [`AWS::Batch::ComputeEnvironment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html) resource type.
#[derive(Debug)]
pub struct ComputeEnvironment {
    properties: ComputeEnvironmentProperties
}

/// Properties for the `ComputeEnvironment` resource.
#[derive(Debug)]
pub struct ComputeEnvironmentProperties {
    /// Property `ComputeEnvironmentName`.
    pub compute_environment_name: Option<::Value<String>>,
    /// Property `ComputeResources`.
    pub compute_resources: ::Value<self::compute_environment::ComputeResources>,
    /// Property `ServiceRole`.
    pub service_role: ::Value<String>,
    /// Property `State`.
    pub state: Option<::Value<String>>,
    /// Property `Type`.
    pub type_: ::Value<String>,
}

impl ::serde::Serialize for ComputeEnvironmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref compute_environment_name) = self.compute_environment_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeEnvironmentName", compute_environment_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeResources", &self.compute_resources)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRole", &self.service_role)?;
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ComputeEnvironmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ComputeEnvironmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ComputeEnvironmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ComputeEnvironmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut compute_environment_name = None;
                let mut compute_resources = None;
                let mut service_role = None;
                let mut state = None;
                let mut type_ = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ComputeEnvironmentName" => {
                            compute_environment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ComputeResources" => {
                            compute_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRole" => {
                            service_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ComputeEnvironmentProperties {
                    compute_environment_name: compute_environment_name,
                    compute_resources: compute_resources.ok_or(::serde::de::Error::missing_field("ComputeResources"))?,
                    service_role: service_role.ok_or(::serde::de::Error::missing_field("ServiceRole"))?,
                    state: state,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ComputeEnvironment {
    type Properties = ComputeEnvironmentProperties;
    const TYPE: &'static str = "AWS::Batch::ComputeEnvironment";
    fn properties(&self) -> &ComputeEnvironmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ComputeEnvironmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ComputeEnvironment {}

impl From<ComputeEnvironmentProperties> for ComputeEnvironment {
    fn from(properties: ComputeEnvironmentProperties) -> ComputeEnvironment {
        ComputeEnvironment { properties }
    }
}

/// The [`AWS::Batch::JobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html) resource type.
#[derive(Debug)]
pub struct JobDefinition {
    properties: JobDefinitionProperties
}

/// Properties for the `JobDefinition` resource.
#[derive(Debug)]
pub struct JobDefinitionProperties {
    /// Property `ContainerProperties`.
    pub container_properties: ::Value<self::job_definition::ContainerProperties>,
    /// Property `JobDefinitionName`.
    pub job_definition_name: Option<::Value<String>>,
    /// Property `Parameters`.
    pub parameters: Option<::Value<::json::Value>>,
    /// Property `RetryStrategy`.
    pub retry_strategy: Option<::Value<self::job_definition::RetryStrategy>>,
    /// Property `Type`.
    pub type_: ::Value<String>,
}

impl ::serde::Serialize for JobDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerProperties", &self.container_properties)?;
        if let Some(ref job_definition_name) = self.job_definition_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobDefinitionName", job_definition_name)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref retry_strategy) = self.retry_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryStrategy", retry_strategy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for JobDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<JobDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JobDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type JobDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut container_properties = None;
                let mut job_definition_name = None;
                let mut parameters = None;
                let mut retry_strategy = None;
                let mut type_ = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContainerProperties" => {
                            container_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobDefinitionName" => {
                            job_definition_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetryStrategy" => {
                            retry_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(JobDefinitionProperties {
                    container_properties: container_properties.ok_or(::serde::de::Error::missing_field("ContainerProperties"))?,
                    job_definition_name: job_definition_name,
                    parameters: parameters,
                    retry_strategy: retry_strategy,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for JobDefinition {
    type Properties = JobDefinitionProperties;
    const TYPE: &'static str = "AWS::Batch::JobDefinition";
    fn properties(&self) -> &JobDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for JobDefinition {}

impl From<JobDefinitionProperties> for JobDefinition {
    fn from(properties: JobDefinitionProperties) -> JobDefinition {
        JobDefinition { properties }
    }
}

/// The [`AWS::Batch::JobQueue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html) resource type.
#[derive(Debug)]
pub struct JobQueue {
    properties: JobQueueProperties
}

/// Properties for the `JobQueue` resource.
#[derive(Debug)]
pub struct JobQueueProperties {
    /// Property `ComputeEnvironmentOrder`.
    pub compute_environment_order: ::ValueList<self::job_queue::ComputeEnvironmentOrder>,
    /// Property `JobQueueName`.
    pub job_queue_name: Option<::Value<String>>,
    /// Property `Priority`.
    pub priority: ::Value<u32>,
    /// Property `State`.
    pub state: Option<::Value<String>>,
}

impl ::serde::Serialize for JobQueueProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeEnvironmentOrder", &self.compute_environment_order)?;
        if let Some(ref job_queue_name) = self.job_queue_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobQueueName", job_queue_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for JobQueueProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<JobQueueProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JobQueueProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type JobQueueProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut compute_environment_order = None;
                let mut job_queue_name = None;
                let mut priority = None;
                let mut state = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ComputeEnvironmentOrder" => {
                            compute_environment_order = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobQueueName" => {
                            job_queue_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Priority" => {
                            priority = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(JobQueueProperties {
                    compute_environment_order: compute_environment_order.ok_or(::serde::de::Error::missing_field("ComputeEnvironmentOrder"))?,
                    job_queue_name: job_queue_name,
                    priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                    state: state,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for JobQueue {
    type Properties = JobQueueProperties;
    const TYPE: &'static str = "AWS::Batch::JobQueue";
    fn properties(&self) -> &JobQueueProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobQueueProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for JobQueue {}

impl From<JobQueueProperties> for JobQueue {
    fn from(properties: JobQueueProperties) -> JobQueue {
        JobQueue { properties }
    }
}

pub mod compute_environment {
    //! Property types for the `ComputeEnvironment` resource.

    /// The [`AWS::Batch::ComputeEnvironment.ComputeResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html) property type.
    #[derive(Debug)]
    pub struct ComputeResources {
        /// Property `BidPercentage`.
        pub bid_percentage: Option<::Value<u32>>,
        /// Property `DesiredvCpus`.
        pub desiredv_cpus: Option<::Value<u32>>,
        /// Property `Ec2KeyPair`.
        pub ec2_key_pair: Option<::Value<String>>,
        /// Property `ImageId`.
        pub image_id: Option<::Value<String>>,
        /// Property `InstanceRole`.
        pub instance_role: ::Value<String>,
        /// Property `InstanceTypes`.
        pub instance_types: ::ValueList<String>,
        /// Property `MaxvCpus`.
        pub maxv_cpus: ::Value<u32>,
        /// Property `MinvCpus`.
        pub minv_cpus: ::Value<u32>,
        /// Property `SecurityGroupIds`.
        pub security_group_ids: ::ValueList<String>,
        /// Property `SpotIamFleetRole`.
        pub spot_iam_fleet_role: Option<::Value<String>>,
        /// Property `Subnets`.
        pub subnets: ::ValueList<String>,
        /// Property `Tags`.
        pub tags: Option<::Value<::json::Value>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for ComputeResources {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bid_percentage) = self.bid_percentage {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BidPercentage", bid_percentage)?;
            }
            if let Some(ref desiredv_cpus) = self.desiredv_cpus {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredvCpus", desiredv_cpus)?;
            }
            if let Some(ref ec2_key_pair) = self.ec2_key_pair {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2KeyPair", ec2_key_pair)?;
            }
            if let Some(ref image_id) = self.image_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageId", image_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceRole", &self.instance_role)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceTypes", &self.instance_types)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxvCpus", &self.maxv_cpus)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinvCpus", &self.minv_cpus)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            if let Some(ref spot_iam_fleet_role) = self.spot_iam_fleet_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpotIamFleetRole", spot_iam_fleet_role)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComputeResources {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComputeResources, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComputeResources;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComputeResources")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bid_percentage = None;
                    let mut desiredv_cpus = None;
                    let mut ec2_key_pair = None;
                    let mut image_id = None;
                    let mut instance_role = None;
                    let mut instance_types = None;
                    let mut maxv_cpus = None;
                    let mut minv_cpus = None;
                    let mut security_group_ids = None;
                    let mut spot_iam_fleet_role = None;
                    let mut subnets = None;
                    let mut tags = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BidPercentage" => {
                                bid_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DesiredvCpus" => {
                                desiredv_cpus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ec2KeyPair" => {
                                ec2_key_pair = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageId" => {
                                image_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceRole" => {
                                instance_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceTypes" => {
                                instance_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxvCpus" => {
                                maxv_cpus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinvCpus" => {
                                minv_cpus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpotIamFleetRole" => {
                                spot_iam_fleet_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComputeResources {
                        bid_percentage: bid_percentage,
                        desiredv_cpus: desiredv_cpus,
                        ec2_key_pair: ec2_key_pair,
                        image_id: image_id,
                        instance_role: instance_role.ok_or(::serde::de::Error::missing_field("InstanceRole"))?,
                        instance_types: instance_types.ok_or(::serde::de::Error::missing_field("InstanceTypes"))?,
                        maxv_cpus: maxv_cpus.ok_or(::serde::de::Error::missing_field("MaxvCpus"))?,
                        minv_cpus: minv_cpus.ok_or(::serde::de::Error::missing_field("MinvCpus"))?,
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        spot_iam_fleet_role: spot_iam_fleet_role,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                        tags: tags,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod job_definition {
    //! Property types for the `JobDefinition` resource.

    /// The [`AWS::Batch::JobDefinition.ContainerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html) property type.
    #[derive(Debug)]
    pub struct ContainerProperties {
        /// Property `Command`.
        pub command: Option<::ValueList<String>>,
        /// Property `Environment`.
        pub environment: Option<::ValueList<Environment>>,
        /// Property `Image`.
        pub image: ::Value<String>,
        /// Property `JobRoleArn`.
        pub job_role_arn: Option<::Value<String>>,
        /// Property `Memory`.
        pub memory: ::Value<u32>,
        /// Property `MountPoints`.
        pub mount_points: Option<::ValueList<MountPoints>>,
        /// Property `Privileged`.
        pub privileged: Option<::Value<bool>>,
        /// Property `ReadonlyRootFilesystem`.
        pub readonly_root_filesystem: Option<::Value<bool>>,
        /// Property `Ulimits`.
        pub ulimits: Option<::ValueList<Ulimit>>,
        /// Property `User`.
        pub user: Option<::Value<String>>,
        /// Property `Vcpus`.
        pub vcpus: ::Value<u32>,
        /// Property `Volumes`.
        pub volumes: Option<::ValueList<Volumes>>,
    }

    impl ::codec::SerializeValue for ContainerProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref command) = self.command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", command)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", &self.image)?;
            if let Some(ref job_role_arn) = self.job_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobRoleArn", job_role_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", &self.memory)?;
            if let Some(ref mount_points) = self.mount_points {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountPoints", mount_points)?;
            }
            if let Some(ref privileged) = self.privileged {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Privileged", privileged)?;
            }
            if let Some(ref readonly_root_filesystem) = self.readonly_root_filesystem {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadonlyRootFilesystem", readonly_root_filesystem)?;
            }
            if let Some(ref ulimits) = self.ulimits {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ulimits", ulimits)?;
            }
            if let Some(ref user) = self.user {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "User", user)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Vcpus", &self.vcpus)?;
            if let Some(ref volumes) = self.volumes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Volumes", volumes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContainerProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContainerProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContainerProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContainerProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut command = None;
                    let mut environment = None;
                    let mut image = None;
                    let mut job_role_arn = None;
                    let mut memory = None;
                    let mut mount_points = None;
                    let mut privileged = None;
                    let mut readonly_root_filesystem = None;
                    let mut ulimits = None;
                    let mut user = None;
                    let mut vcpus = None;
                    let mut volumes = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Command" => {
                                command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Image" => {
                                image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobRoleArn" => {
                                job_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Memory" => {
                                memory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountPoints" => {
                                mount_points = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Privileged" => {
                                privileged = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadonlyRootFilesystem" => {
                                readonly_root_filesystem = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ulimits" => {
                                ulimits = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "User" => {
                                user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Vcpus" => {
                                vcpus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Volumes" => {
                                volumes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContainerProperties {
                        command: command,
                        environment: environment,
                        image: image.ok_or(::serde::de::Error::missing_field("Image"))?,
                        job_role_arn: job_role_arn,
                        memory: memory.ok_or(::serde::de::Error::missing_field("Memory"))?,
                        mount_points: mount_points,
                        privileged: privileged,
                        readonly_root_filesystem: readonly_root_filesystem,
                        ulimits: ulimits,
                        user: user,
                        vcpus: vcpus.ok_or(::serde::de::Error::missing_field("Vcpus"))?,
                        volumes: volumes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-environment.html) property type.
    #[derive(Debug)]
    pub struct Environment {
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `Value`.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Environment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Environment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Environment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Environment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Environment {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.MountPoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-mountpoints.html) property type.
    #[derive(Debug)]
    pub struct MountPoints {
        /// Property `ContainerPath`.
        pub container_path: Option<::Value<String>>,
        /// Property `ReadOnly`.
        pub read_only: Option<::Value<bool>>,
        /// Property `SourceVolume`.
        pub source_volume: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MountPoints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_path) = self.container_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPath", container_path)?;
            }
            if let Some(ref read_only) = self.read_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnly", read_only)?;
            }
            if let Some(ref source_volume) = self.source_volume {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceVolume", source_volume)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MountPoints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MountPoints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MountPoints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MountPoints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_path = None;
                    let mut read_only = None;
                    let mut source_volume = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerPath" => {
                                container_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadOnly" => {
                                read_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceVolume" => {
                                source_volume = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MountPoints {
                        container_path: container_path,
                        read_only: read_only,
                        source_volume: source_volume,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.RetryStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-retrystrategy.html) property type.
    #[derive(Debug)]
    pub struct RetryStrategy {
        /// Property `Attempts`.
        pub attempts: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RetryStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attempts) = self.attempts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attempts", attempts)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RetryStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RetryStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RetryStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RetryStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attempts = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attempts" => {
                                attempts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RetryStrategy {
                        attempts: attempts,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.Ulimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ulimit.html) property type.
    #[derive(Debug)]
    pub struct Ulimit {
        /// Property `HardLimit`.
        pub hard_limit: ::Value<u32>,
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `SoftLimit`.
        pub soft_limit: ::Value<u32>,
    }

    impl ::codec::SerializeValue for Ulimit {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HardLimit", &self.hard_limit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SoftLimit", &self.soft_limit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Ulimit {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ulimit, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ulimit;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ulimit")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hard_limit = None;
                    let mut name = None;
                    let mut soft_limit = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HardLimit" => {
                                hard_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SoftLimit" => {
                                soft_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Ulimit {
                        hard_limit: hard_limit.ok_or(::serde::de::Error::missing_field("HardLimit"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        soft_limit: soft_limit.ok_or(::serde::de::Error::missing_field("SoftLimit"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.Volumes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumes.html) property type.
    #[derive(Debug)]
    pub struct Volumes {
        /// Property `Host`.
        pub host: Option<::Value<VolumesHost>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Volumes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref host) = self.host {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", host)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Volumes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Volumes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Volumes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Volumes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut host = None;
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Volumes {
                        host: host,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Batch::JobDefinition.VolumesHost`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumeshost.html) property type.
    #[derive(Debug)]
    pub struct VolumesHost {
        /// Property `SourcePath`.
        pub source_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VolumesHost {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source_path) = self.source_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePath", source_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VolumesHost {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumesHost, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VolumesHost;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VolumesHost")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_path = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourcePath" => {
                                source_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VolumesHost {
                        source_path: source_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod job_queue {
    //! Property types for the `JobQueue` resource.

    /// The [`AWS::Batch::JobQueue.ComputeEnvironmentOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobqueue-computeenvironmentorder.html) property type.
    #[derive(Debug)]
    pub struct ComputeEnvironmentOrder {
        /// Property `ComputeEnvironment`.
        pub compute_environment: ::Value<String>,
        /// Property `Order`.
        pub order: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ComputeEnvironmentOrder {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComputeEnvironment", &self.compute_environment)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Order", &self.order)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ComputeEnvironmentOrder {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ComputeEnvironmentOrder, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ComputeEnvironmentOrder;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ComputeEnvironmentOrder")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compute_environment = None;
                    let mut order = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComputeEnvironment" => {
                                compute_environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Order" => {
                                order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ComputeEnvironmentOrder {
                        compute_environment: compute_environment.ok_or(::serde::de::Error::missing_field("ComputeEnvironment"))?,
                        order: order.ok_or(::serde::de::Error::missing_field("Order"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
