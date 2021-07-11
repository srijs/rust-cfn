//! Types for the `DataBrew` service.

/// The [`AWS::DataBrew::Dataset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-dataset.html) resource type.
#[derive(Debug, Default)]
pub struct Dataset {
    properties: DatasetProperties
}

/// Properties for the `Dataset` resource.
#[derive(Debug, Default)]
pub struct DatasetProperties {
    /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-dataset.html#cfn-databrew-dataset-format).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub format: Option<::Value<String>>,
    /// Property [`FormatOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-dataset.html#cfn-databrew-dataset-formatoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub format_options: Option<::Value<self::dataset::FormatOptions>>,
    /// Property [`Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-dataset.html#cfn-databrew-dataset-input).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub input: ::Value<self::dataset::Input>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-dataset.html#cfn-databrew-dataset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`PathOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-dataset.html#cfn-databrew-dataset-pathoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub path_options: Option<::Value<self::dataset::PathOptions>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-dataset.html#cfn-databrew-dataset-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DatasetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref format) = self.format {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", format)?;
        }
        if let Some(ref format_options) = self.format_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FormatOptions", format_options)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Input", &self.input)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref path_options) = self.path_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathOptions", path_options)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DatasetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DatasetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DatasetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DatasetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut format: Option<::Value<String>> = None;
                let mut format_options: Option<::Value<self::dataset::FormatOptions>> = None;
                let mut input: Option<::Value<self::dataset::Input>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut path_options: Option<::Value<self::dataset::PathOptions>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Format" => {
                            format = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FormatOptions" => {
                            format_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Input" => {
                            input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PathOptions" => {
                            path_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DatasetProperties {
                    format: format,
                    format_options: format_options,
                    input: input.ok_or(::serde::de::Error::missing_field("Input"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    path_options: path_options,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Dataset {
    type Properties = DatasetProperties;
    const TYPE: &'static str = "AWS::DataBrew::Dataset";
    fn properties(&self) -> &DatasetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DatasetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Dataset {}

impl From<DatasetProperties> for Dataset {
    fn from(properties: DatasetProperties) -> Dataset {
        Dataset { properties }
    }
}

/// The [`AWS::DataBrew::Job`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html) resource type.
#[derive(Debug, Default)]
pub struct Job {
    properties: JobProperties
}

/// Properties for the `Job` resource.
#[derive(Debug, Default)]
pub struct JobProperties {
    /// Property [`DataCatalogOutputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-datacatalogoutputs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_catalog_outputs: Option<::ValueList<self::job::DataCatalogOutput>>,
    /// Property [`DatasetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-datasetname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dataset_name: Option<::Value<String>>,
    /// Property [`EncryptionKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-encryptionkeyarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_key_arn: Option<::Value<String>>,
    /// Property [`EncryptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-encryptionmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_mode: Option<::Value<String>>,
    /// Property [`JobSample`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-jobsample).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub job_sample: Option<::Value<self::job::JobSample>>,
    /// Property [`LogSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-logsubscription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_subscription: Option<::Value<String>>,
    /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-maxcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_capacity: Option<::Value<u32>>,
    /// Property [`MaxRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-maxretries).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_retries: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OutputLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-outputlocation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub output_location: Option<::Value<self::job::OutputLocation>>,
    /// Property [`Outputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-outputs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub outputs: Option<::ValueList<self::job::Output>>,
    /// Property [`ProjectName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-projectname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub project_name: Option<::Value<String>>,
    /// Property [`Recipe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-recipe).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub recipe: Option<::Value<self::job::Recipe>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-timeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub timeout: Option<::Value<u32>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html#cfn-databrew-job-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for JobProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref data_catalog_outputs) = self.data_catalog_outputs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataCatalogOutputs", data_catalog_outputs)?;
        }
        if let Some(ref dataset_name) = self.dataset_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetName", dataset_name)?;
        }
        if let Some(ref encryption_key_arn) = self.encryption_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKeyArn", encryption_key_arn)?;
        }
        if let Some(ref encryption_mode) = self.encryption_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionMode", encryption_mode)?;
        }
        if let Some(ref job_sample) = self.job_sample {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobSample", job_sample)?;
        }
        if let Some(ref log_subscription) = self.log_subscription {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogSubscription", log_subscription)?;
        }
        if let Some(ref max_capacity) = self.max_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", max_capacity)?;
        }
        if let Some(ref max_retries) = self.max_retries {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRetries", max_retries)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref output_location) = self.output_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputLocation", output_location)?;
        }
        if let Some(ref outputs) = self.outputs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Outputs", outputs)?;
        }
        if let Some(ref project_name) = self.project_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectName", project_name)?;
        }
        if let Some(ref recipe) = self.recipe {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Recipe", recipe)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timeout) = self.timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for JobProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<JobProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JobProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type JobProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_catalog_outputs: Option<::ValueList<self::job::DataCatalogOutput>> = None;
                let mut dataset_name: Option<::Value<String>> = None;
                let mut encryption_key_arn: Option<::Value<String>> = None;
                let mut encryption_mode: Option<::Value<String>> = None;
                let mut job_sample: Option<::Value<self::job::JobSample>> = None;
                let mut log_subscription: Option<::Value<String>> = None;
                let mut max_capacity: Option<::Value<u32>> = None;
                let mut max_retries: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut output_location: Option<::Value<self::job::OutputLocation>> = None;
                let mut outputs: Option<::ValueList<self::job::Output>> = None;
                let mut project_name: Option<::Value<String>> = None;
                let mut recipe: Option<::Value<self::job::Recipe>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut timeout: Option<::Value<u32>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataCatalogOutputs" => {
                            data_catalog_outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatasetName" => {
                            dataset_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionKeyArn" => {
                            encryption_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionMode" => {
                            encryption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobSample" => {
                            job_sample = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogSubscription" => {
                            log_subscription = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxCapacity" => {
                            max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxRetries" => {
                            max_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutputLocation" => {
                            output_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Outputs" => {
                            outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProjectName" => {
                            project_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Recipe" => {
                            recipe = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Timeout" => {
                            timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(JobProperties {
                    data_catalog_outputs: data_catalog_outputs,
                    dataset_name: dataset_name,
                    encryption_key_arn: encryption_key_arn,
                    encryption_mode: encryption_mode,
                    job_sample: job_sample,
                    log_subscription: log_subscription,
                    max_capacity: max_capacity,
                    max_retries: max_retries,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    output_location: output_location,
                    outputs: outputs,
                    project_name: project_name,
                    recipe: recipe,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                    timeout: timeout,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Job {
    type Properties = JobProperties;
    const TYPE: &'static str = "AWS::DataBrew::Job";
    fn properties(&self) -> &JobProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Job {}

impl From<JobProperties> for Job {
    fn from(properties: JobProperties) -> Job {
        Job { properties }
    }
}

/// The [`AWS::DataBrew::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-project.html) resource type.
#[derive(Debug, Default)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug, Default)]
pub struct ProjectProperties {
    /// Property [`DatasetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-project.html#cfn-databrew-project-datasetname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dataset_name: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-project.html#cfn-databrew-project-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RecipeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-project.html#cfn-databrew-project-recipename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub recipe_name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-project.html#cfn-databrew-project-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Sample`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-project.html#cfn-databrew-project-sample).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sample: Option<::Value<self::project::Sample>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-project.html#cfn-databrew-project-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetName", &self.dataset_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecipeName", &self.recipe_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref sample) = self.sample {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sample", sample)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProjectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProjectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProjectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dataset_name: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut recipe_name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut sample: Option<::Value<self::project::Sample>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DatasetName" => {
                            dataset_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RecipeName" => {
                            recipe_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Sample" => {
                            sample = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProjectProperties {
                    dataset_name: dataset_name.ok_or(::serde::de::Error::missing_field("DatasetName"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    recipe_name: recipe_name.ok_or(::serde::de::Error::missing_field("RecipeName"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    sample: sample,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Project {
    type Properties = ProjectProperties;
    const TYPE: &'static str = "AWS::DataBrew::Project";
    fn properties(&self) -> &ProjectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProjectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Project {}

impl From<ProjectProperties> for Project {
    fn from(properties: ProjectProperties) -> Project {
        Project { properties }
    }
}

/// The [`AWS::DataBrew::Recipe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-recipe.html) resource type.
#[derive(Debug, Default)]
pub struct Recipe {
    properties: RecipeProperties
}

/// Properties for the `Recipe` resource.
#[derive(Debug, Default)]
pub struct RecipeProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-recipe.html#cfn-databrew-recipe-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-recipe.html#cfn-databrew-recipe-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Steps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-recipe.html#cfn-databrew-recipe-steps).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub steps: ::ValueList<self::recipe::RecipeStep>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-recipe.html#cfn-databrew-recipe-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for RecipeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Steps", &self.steps)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RecipeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RecipeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RecipeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RecipeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut steps: Option<::ValueList<self::recipe::RecipeStep>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Steps" => {
                            steps = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RecipeProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    steps: steps.ok_or(::serde::de::Error::missing_field("Steps"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Recipe {
    type Properties = RecipeProperties;
    const TYPE: &'static str = "AWS::DataBrew::Recipe";
    fn properties(&self) -> &RecipeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RecipeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Recipe {}

impl From<RecipeProperties> for Recipe {
    fn from(properties: RecipeProperties) -> Recipe {
        Recipe { properties }
    }
}

/// The [`AWS::DataBrew::Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-schedule.html) resource type.
#[derive(Debug, Default)]
pub struct Schedule {
    properties: ScheduleProperties
}

/// Properties for the `Schedule` resource.
#[derive(Debug, Default)]
pub struct ScheduleProperties {
    /// Property [`CronExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-schedule.html#cfn-databrew-schedule-cronexpression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cron_expression: ::Value<String>,
    /// Property [`JobNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-schedule.html#cfn-databrew-schedule-jobnames).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub job_names: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-schedule.html#cfn-databrew-schedule-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-schedule.html#cfn-databrew-schedule-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ScheduleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CronExpression", &self.cron_expression)?;
        if let Some(ref job_names) = self.job_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobNames", job_names)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScheduleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScheduleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScheduleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cron_expression: Option<::Value<String>> = None;
                let mut job_names: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CronExpression" => {
                            cron_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobNames" => {
                            job_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScheduleProperties {
                    cron_expression: cron_expression.ok_or(::serde::de::Error::missing_field("CronExpression"))?,
                    job_names: job_names,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Schedule {
    type Properties = ScheduleProperties;
    const TYPE: &'static str = "AWS::DataBrew::Schedule";
    fn properties(&self) -> &ScheduleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScheduleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Schedule {}

impl From<ScheduleProperties> for Schedule {
    fn from(properties: ScheduleProperties) -> Schedule {
        Schedule { properties }
    }
}

pub mod dataset {
    //! Property types for the `Dataset` resource.

    /// The [`AWS::DataBrew::Dataset.CsvOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-csvoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct CsvOptions {
        /// Property [`Delimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-csvoptions.html#cfn-databrew-dataset-csvoptions-delimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delimiter: Option<::Value<String>>,
        /// Property [`HeaderRow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-csvoptions.html#cfn-databrew-dataset-csvoptions-headerrow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_row: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for CsvOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delimiter) = self.delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Delimiter", delimiter)?;
            }
            if let Some(ref header_row) = self.header_row {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderRow", header_row)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CsvOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CsvOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CsvOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CsvOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delimiter: Option<::Value<String>> = None;
                    let mut header_row: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Delimiter" => {
                                delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HeaderRow" => {
                                header_row = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CsvOptions {
                        delimiter: delimiter,
                        header_row: header_row,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.DataCatalogInputDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datacataloginputdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct DataCatalogInputDefinition {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datacataloginputdefinition.html#cfn-databrew-dataset-datacataloginputdefinition-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datacataloginputdefinition.html#cfn-databrew-dataset-datacataloginputdefinition-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datacataloginputdefinition.html#cfn-databrew-dataset-datacataloginputdefinition-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: Option<::Value<String>>,
        /// Property [`TempDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datacataloginputdefinition.html#cfn-databrew-dataset-datacataloginputdefinition-tempdirectory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub temp_directory: Option<::Value<S3Location>>,
    }

    impl ::codec::SerializeValue for DataCatalogInputDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref table_name) = self.table_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", table_name)?;
            }
            if let Some(ref temp_directory) = self.temp_directory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TempDirectory", temp_directory)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataCatalogInputDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataCatalogInputDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataCatalogInputDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataCatalogInputDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;
                    let mut temp_directory: Option<::Value<S3Location>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TempDirectory" => {
                                temp_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataCatalogInputDefinition {
                        catalog_id: catalog_id,
                        database_name: database_name,
                        table_name: table_name,
                        temp_directory: temp_directory,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.DatabaseInputDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-databaseinputdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct DatabaseInputDefinition {
        /// Property [`DatabaseTableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-databaseinputdefinition.html#cfn-databrew-dataset-databaseinputdefinition-databasetablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_table_name: Option<::Value<String>>,
        /// Property [`GlueConnectionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-databaseinputdefinition.html#cfn-databrew-dataset-databaseinputdefinition-glueconnectionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub glue_connection_name: Option<::Value<String>>,
        /// Property [`TempDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-databaseinputdefinition.html#cfn-databrew-dataset-databaseinputdefinition-tempdirectory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub temp_directory: Option<::Value<S3Location>>,
    }

    impl ::codec::SerializeValue for DatabaseInputDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref database_table_name) = self.database_table_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseTableName", database_table_name)?;
            }
            if let Some(ref glue_connection_name) = self.glue_connection_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlueConnectionName", glue_connection_name)?;
            }
            if let Some(ref temp_directory) = self.temp_directory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TempDirectory", temp_directory)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatabaseInputDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseInputDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatabaseInputDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatabaseInputDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database_table_name: Option<::Value<String>> = None;
                    let mut glue_connection_name: Option<::Value<String>> = None;
                    let mut temp_directory: Option<::Value<S3Location>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatabaseTableName" => {
                                database_table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GlueConnectionName" => {
                                glue_connection_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TempDirectory" => {
                                temp_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatabaseInputDefinition {
                        database_table_name: database_table_name,
                        glue_connection_name: glue_connection_name,
                        temp_directory: temp_directory,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.DatasetParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datasetparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct DatasetParameter {
        /// Property [`CreateColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datasetparameter.html#cfn-databrew-dataset-datasetparameter-createcolumn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub create_column: Option<::Value<bool>>,
        /// Property [`DatetimeOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datasetparameter.html#cfn-databrew-dataset-datasetparameter-datetimeoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datetime_options: Option<::Value<DatetimeOptions>>,
        /// Property [`Filter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datasetparameter.html#cfn-databrew-dataset-datasetparameter-filter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter: Option<::Value<FilterExpression>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datasetparameter.html#cfn-databrew-dataset-datasetparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datasetparameter.html#cfn-databrew-dataset-datasetparameter-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for DatasetParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref create_column) = self.create_column {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateColumn", create_column)?;
            }
            if let Some(ref datetime_options) = self.datetime_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatetimeOptions", datetime_options)?;
            }
            if let Some(ref filter) = self.filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Filter", filter)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatasetParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatasetParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatasetParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatasetParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut create_column: Option<::Value<bool>> = None;
                    let mut datetime_options: Option<::Value<DatetimeOptions>> = None;
                    let mut filter: Option<::Value<FilterExpression>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CreateColumn" => {
                                create_column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatetimeOptions" => {
                                datetime_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Filter" => {
                                filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatasetParameter {
                        create_column: create_column,
                        datetime_options: datetime_options,
                        filter: filter,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.DatetimeOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datetimeoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct DatetimeOptions {
        /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datetimeoptions.html#cfn-databrew-dataset-datetimeoptions-format).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub format: ::Value<String>,
        /// Property [`LocaleCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datetimeoptions.html#cfn-databrew-dataset-datetimeoptions-localecode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub locale_code: Option<::Value<String>>,
        /// Property [`TimezoneOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datetimeoptions.html#cfn-databrew-dataset-datetimeoptions-timezoneoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timezone_offset: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DatetimeOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", &self.format)?;
            if let Some(ref locale_code) = self.locale_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocaleCode", locale_code)?;
            }
            if let Some(ref timezone_offset) = self.timezone_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimezoneOffset", timezone_offset)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatetimeOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatetimeOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatetimeOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatetimeOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut format: Option<::Value<String>> = None;
                    let mut locale_code: Option<::Value<String>> = None;
                    let mut timezone_offset: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Format" => {
                                format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocaleCode" => {
                                locale_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimezoneOffset" => {
                                timezone_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatetimeOptions {
                        format: format.ok_or(::serde::de::Error::missing_field("Format"))?,
                        locale_code: locale_code,
                        timezone_offset: timezone_offset,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.ExcelOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-exceloptions.html) property type.
    #[derive(Debug, Default)]
    pub struct ExcelOptions {
        /// Property [`HeaderRow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-exceloptions.html#cfn-databrew-dataset-exceloptions-headerrow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub header_row: Option<::Value<bool>>,
        /// Property [`SheetIndexes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-exceloptions.html#cfn-databrew-dataset-exceloptions-sheetindexes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sheet_indexes: Option<::ValueList<u32>>,
        /// Property [`SheetNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-exceloptions.html#cfn-databrew-dataset-exceloptions-sheetnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sheet_names: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ExcelOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref header_row) = self.header_row {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HeaderRow", header_row)?;
            }
            if let Some(ref sheet_indexes) = self.sheet_indexes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SheetIndexes", sheet_indexes)?;
            }
            if let Some(ref sheet_names) = self.sheet_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SheetNames", sheet_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExcelOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExcelOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExcelOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExcelOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut header_row: Option<::Value<bool>> = None;
                    let mut sheet_indexes: Option<::ValueList<u32>> = None;
                    let mut sheet_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HeaderRow" => {
                                header_row = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SheetIndexes" => {
                                sheet_indexes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SheetNames" => {
                                sheet_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExcelOptions {
                        header_row: header_row,
                        sheet_indexes: sheet_indexes,
                        sheet_names: sheet_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.FilesLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-fileslimit.html) property type.
    #[derive(Debug, Default)]
    pub struct FilesLimit {
        /// Property [`MaxFiles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-fileslimit.html#cfn-databrew-dataset-fileslimit-maxfiles).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_files: ::Value<u32>,
        /// Property [`Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-fileslimit.html#cfn-databrew-dataset-fileslimit-order).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub order: Option<::Value<String>>,
        /// Property [`OrderedBy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-fileslimit.html#cfn-databrew-dataset-fileslimit-orderedby).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ordered_by: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FilesLimit {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxFiles", &self.max_files)?;
            if let Some(ref order) = self.order {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Order", order)?;
            }
            if let Some(ref ordered_by) = self.ordered_by {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrderedBy", ordered_by)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilesLimit {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilesLimit, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilesLimit;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilesLimit")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_files: Option<::Value<u32>> = None;
                    let mut order: Option<::Value<String>> = None;
                    let mut ordered_by: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxFiles" => {
                                max_files = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Order" => {
                                order = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrderedBy" => {
                                ordered_by = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilesLimit {
                        max_files: max_files.ok_or(::serde::de::Error::missing_field("MaxFiles"))?,
                        order: order,
                        ordered_by: ordered_by,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.FilterExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-filterexpression.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterExpression {
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-filterexpression.html#cfn-databrew-dataset-filterexpression-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: ::Value<String>,
        /// Property [`ValuesMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-filterexpression.html#cfn-databrew-dataset-filterexpression-valuesmap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values_map: ::ValueList<FilterValue>,
    }

    impl ::codec::SerializeValue for FilterExpression {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValuesMap", &self.values_map)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterExpression {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterExpression, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterExpression;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterExpression")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expression: Option<::Value<String>> = None;
                    let mut values_map: Option<::ValueList<FilterValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValuesMap" => {
                                values_map = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilterExpression {
                        expression: expression.ok_or(::serde::de::Error::missing_field("Expression"))?,
                        values_map: values_map.ok_or(::serde::de::Error::missing_field("ValuesMap"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.FilterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-filtervalue.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterValue {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-filtervalue.html#cfn-databrew-dataset-filtervalue-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
        /// Property [`ValueReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-filtervalue.html#cfn-databrew-dataset-filtervalue-valuereference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value_reference: ::Value<String>,
    }

    impl ::codec::SerializeValue for FilterValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueReference", &self.value_reference)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut value: Option<::Value<String>> = None;
                    let mut value_reference: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueReference" => {
                                value_reference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilterValue {
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                        value_reference: value_reference.ok_or(::serde::de::Error::missing_field("ValueReference"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.FormatOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-formatoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct FormatOptions {
        /// Property [`Csv`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-formatoptions.html#cfn-databrew-dataset-formatoptions-csv).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub csv: Option<::Value<CsvOptions>>,
        /// Property [`Excel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-formatoptions.html#cfn-databrew-dataset-formatoptions-excel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excel: Option<::Value<ExcelOptions>>,
        /// Property [`Json`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-formatoptions.html#cfn-databrew-dataset-formatoptions-json).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json: Option<::Value<JsonOptions>>,
    }

    impl ::codec::SerializeValue for FormatOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref csv) = self.csv {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Csv", csv)?;
            }
            if let Some(ref excel) = self.excel {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Excel", excel)?;
            }
            if let Some(ref json) = self.json {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Json", json)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FormatOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FormatOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FormatOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FormatOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut csv: Option<::Value<CsvOptions>> = None;
                    let mut excel: Option<::Value<ExcelOptions>> = None;
                    let mut json: Option<::Value<JsonOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Csv" => {
                                csv = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Excel" => {
                                excel = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Json" => {
                                json = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FormatOptions {
                        csv: csv,
                        excel: excel,
                        json: json,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-input.html) property type.
    #[derive(Debug, Default)]
    pub struct Input {
        /// Property [`DataCatalogInputDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-input.html#cfn-databrew-dataset-input-datacataloginputdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_catalog_input_definition: Option<::Value<DataCatalogInputDefinition>>,
        /// Property [`DatabaseInputDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-input.html#cfn-databrew-dataset-input-databaseinputdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_input_definition: Option<::Value<DatabaseInputDefinition>>,
        /// Property [`S3InputDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-input.html#cfn-databrew-dataset-input-s3inputdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_input_definition: Option<::Value<S3Location>>,
    }

    impl ::codec::SerializeValue for Input {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_catalog_input_definition) = self.data_catalog_input_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataCatalogInputDefinition", data_catalog_input_definition)?;
            }
            if let Some(ref database_input_definition) = self.database_input_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseInputDefinition", database_input_definition)?;
            }
            if let Some(ref s3_input_definition) = self.s3_input_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3InputDefinition", s3_input_definition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Input {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Input, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Input;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Input")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_catalog_input_definition: Option<::Value<DataCatalogInputDefinition>> = None;
                    let mut database_input_definition: Option<::Value<DatabaseInputDefinition>> = None;
                    let mut s3_input_definition: Option<::Value<S3Location>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataCatalogInputDefinition" => {
                                data_catalog_input_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseInputDefinition" => {
                                database_input_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3InputDefinition" => {
                                s3_input_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Input {
                        data_catalog_input_definition: data_catalog_input_definition,
                        database_input_definition: database_input_definition,
                        s3_input_definition: s3_input_definition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.JsonOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-jsonoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct JsonOptions {
        /// Property [`MultiLine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-jsonoptions.html#cfn-databrew-dataset-jsonoptions-multiline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multi_line: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for JsonOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref multi_line) = self.multi_line {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiLine", multi_line)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JsonOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JsonOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JsonOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JsonOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut multi_line: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MultiLine" => {
                                multi_line = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JsonOptions {
                        multi_line: multi_line,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.PathOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-pathoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct PathOptions {
        /// Property [`FilesLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-pathoptions.html#cfn-databrew-dataset-pathoptions-fileslimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub files_limit: Option<::Value<FilesLimit>>,
        /// Property [`LastModifiedDateCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-pathoptions.html#cfn-databrew-dataset-pathoptions-lastmodifieddatecondition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_modified_date_condition: Option<::Value<FilterExpression>>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-pathoptions.html#cfn-databrew-dataset-pathoptions-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::ValueList<PathParameter>>,
    }

    impl ::codec::SerializeValue for PathOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref files_limit) = self.files_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilesLimit", files_limit)?;
            }
            if let Some(ref last_modified_date_condition) = self.last_modified_date_condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastModifiedDateCondition", last_modified_date_condition)?;
            }
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PathOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PathOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PathOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PathOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut files_limit: Option<::Value<FilesLimit>> = None;
                    let mut last_modified_date_condition: Option<::Value<FilterExpression>> = None;
                    let mut parameters: Option<::ValueList<PathParameter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FilesLimit" => {
                                files_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastModifiedDateCondition" => {
                                last_modified_date_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PathOptions {
                        files_limit: files_limit,
                        last_modified_date_condition: last_modified_date_condition,
                        parameters: parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.PathParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-pathparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct PathParameter {
        /// Property [`DatasetParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-pathparameter.html#cfn-databrew-dataset-pathparameter-datasetparameter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dataset_parameter: ::Value<DatasetParameter>,
        /// Property [`PathParameterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-pathparameter.html#cfn-databrew-dataset-pathparameter-pathparametername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path_parameter_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for PathParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetParameter", &self.dataset_parameter)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathParameterName", &self.path_parameter_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PathParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PathParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PathParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PathParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dataset_parameter: Option<::Value<DatasetParameter>> = None;
                    let mut path_parameter_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatasetParameter" => {
                                dataset_parameter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PathParameterName" => {
                                path_parameter_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PathParameter {
                        dataset_parameter: dataset_parameter.ok_or(::serde::de::Error::missing_field("DatasetParameter"))?,
                        path_parameter_name: path_parameter_name.ok_or(::serde::de::Error::missing_field("PathParameterName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Dataset.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-s3location.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-s3location.html#cfn-databrew-dataset-s3location-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-s3location.html#cfn-databrew-dataset-s3location-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key: key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod job {
    //! Property types for the `Job` resource.

    /// The [`AWS::DataBrew::Job.CsvOutputOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-csvoutputoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct CsvOutputOptions {
        /// Property [`Delimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-csvoutputoptions.html#cfn-databrew-job-csvoutputoptions-delimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delimiter: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CsvOutputOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delimiter) = self.delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Delimiter", delimiter)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CsvOutputOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CsvOutputOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CsvOutputOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CsvOutputOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delimiter: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Delimiter" => {
                                delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CsvOutputOptions {
                        delimiter: delimiter,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Job.DataCatalogOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-datacatalogoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct DataCatalogOutput {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-datacatalogoutput.html#cfn-databrew-job-datacatalogoutput-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-datacatalogoutput.html#cfn-databrew-job-datacatalogoutput-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`DatabaseOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-datacatalogoutput.html#cfn-databrew-job-datacatalogoutput-databaseoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_options: Option<::Value<DatabaseTableOutputOptions>>,
        /// Property [`Overwrite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-datacatalogoutput.html#cfn-databrew-job-datacatalogoutput-overwrite).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub overwrite: Option<::Value<bool>>,
        /// Property [`S3Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-datacatalogoutput.html#cfn-databrew-job-datacatalogoutput-s3options).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_options: Option<::Value<S3TableOutputOptions>>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-datacatalogoutput.html#cfn-databrew-job-datacatalogoutput-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataCatalogOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            if let Some(ref database_options) = self.database_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseOptions", database_options)?;
            }
            if let Some(ref overwrite) = self.overwrite {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Overwrite", overwrite)?;
            }
            if let Some(ref s3_options) = self.s3_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Options", s3_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataCatalogOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataCatalogOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataCatalogOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataCatalogOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut database_options: Option<::Value<DatabaseTableOutputOptions>> = None;
                    let mut overwrite: Option<::Value<bool>> = None;
                    let mut s3_options: Option<::Value<S3TableOutputOptions>> = None;
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseOptions" => {
                                database_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Overwrite" => {
                                overwrite = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Options" => {
                                s3_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataCatalogOutput {
                        catalog_id: catalog_id,
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        database_options: database_options,
                        overwrite: overwrite,
                        s3_options: s3_options,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Job.DatabaseTableOutputOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-databasetableoutputoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct DatabaseTableOutputOptions {
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-databasetableoutputoptions.html#cfn-databrew-job-databasetableoutputoptions-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
        /// Property [`TempDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-databasetableoutputoptions.html#cfn-databrew-job-databasetableoutputoptions-tempdirectory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub temp_directory: Option<::Value<S3Location>>,
    }

    impl ::codec::SerializeValue for DatabaseTableOutputOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            if let Some(ref temp_directory) = self.temp_directory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TempDirectory", temp_directory)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatabaseTableOutputOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseTableOutputOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatabaseTableOutputOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatabaseTableOutputOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut table_name: Option<::Value<String>> = None;
                    let mut temp_directory: Option<::Value<S3Location>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TempDirectory" => {
                                temp_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatabaseTableOutputOptions {
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                        temp_directory: temp_directory,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Job.JobSample`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-jobsample.html) property type.
    #[derive(Debug, Default)]
    pub struct JobSample {
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-jobsample.html#cfn-databrew-job-jobsample-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: Option<::Value<String>>,
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-jobsample.html#cfn-databrew-job-jobsample-size).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for JobSample {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mode) = self.mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", mode)?;
            }
            if let Some(ref size) = self.size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", size)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JobSample {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JobSample, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JobSample;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JobSample")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mode: Option<::Value<String>> = None;
                    let mut size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Size" => {
                                size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JobSample {
                        mode: mode,
                        size: size,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Job.Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-output.html) property type.
    #[derive(Debug, Default)]
    pub struct Output {
        /// Property [`CompressionFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-output.html#cfn-databrew-job-output-compressionformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub compression_format: Option<::Value<String>>,
        /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-output.html#cfn-databrew-job-output-format).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub format: Option<::Value<String>>,
        /// Property [`FormatOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-output.html#cfn-databrew-job-output-formatoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub format_options: Option<::Value<OutputFormatOptions>>,
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-output.html#cfn-databrew-job-output-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: ::Value<S3Location>,
        /// Property [`Overwrite`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-output.html#cfn-databrew-job-output-overwrite).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub overwrite: Option<::Value<bool>>,
        /// Property [`PartitionColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-output.html#cfn-databrew-job-output-partitioncolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition_columns: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Output {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref compression_format) = self.compression_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionFormat", compression_format)?;
            }
            if let Some(ref format) = self.format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", format)?;
            }
            if let Some(ref format_options) = self.format_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FormatOptions", format_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
            if let Some(ref overwrite) = self.overwrite {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Overwrite", overwrite)?;
            }
            if let Some(ref partition_columns) = self.partition_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionColumns", partition_columns)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Output {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Output, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Output;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Output")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compression_format: Option<::Value<String>> = None;
                    let mut format: Option<::Value<String>> = None;
                    let mut format_options: Option<::Value<OutputFormatOptions>> = None;
                    let mut location: Option<::Value<S3Location>> = None;
                    let mut overwrite: Option<::Value<bool>> = None;
                    let mut partition_columns: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CompressionFormat" => {
                                compression_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Format" => {
                                format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FormatOptions" => {
                                format_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Overwrite" => {
                                overwrite = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PartitionColumns" => {
                                partition_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Output {
                        compression_format: compression_format,
                        format: format,
                        format_options: format_options,
                        location: location.ok_or(::serde::de::Error::missing_field("Location"))?,
                        overwrite: overwrite,
                        partition_columns: partition_columns,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Job.OutputFormatOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-outputformatoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputFormatOptions {
        /// Property [`Csv`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-outputformatoptions.html#cfn-databrew-job-outputformatoptions-csv).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub csv: Option<::Value<CsvOutputOptions>>,
    }

    impl ::codec::SerializeValue for OutputFormatOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref csv) = self.csv {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Csv", csv)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputFormatOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputFormatOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputFormatOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputFormatOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut csv: Option<::Value<CsvOutputOptions>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Csv" => {
                                csv = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputFormatOptions {
                        csv: csv,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Job.OutputLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-outputlocation.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputLocation {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-outputlocation.html#cfn-databrew-job-outputlocation-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-outputlocation.html#cfn-databrew-job-outputlocation-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OutputLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputLocation {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key: key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Job.Recipe`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-recipe.html) property type.
    #[derive(Debug, Default)]
    pub struct Recipe {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-recipe.html#cfn-databrew-job-recipe-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-recipe.html#cfn-databrew-job-recipe-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Recipe {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Recipe {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Recipe, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Recipe;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Recipe")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Recipe {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Job.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-s3location.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-s3location.html#cfn-databrew-job-s3location-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-s3location.html#cfn-databrew-job-s3location-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key: key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Job.S3TableOutputOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-s3tableoutputoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct S3TableOutputOptions {
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-s3tableoutputoptions.html#cfn-databrew-job-s3tableoutputoptions-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: ::Value<S3Location>,
    }

    impl ::codec::SerializeValue for S3TableOutputOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", &self.location)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3TableOutputOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3TableOutputOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3TableOutputOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3TableOutputOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut location: Option<::Value<S3Location>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3TableOutputOptions {
                        location: location.ok_or(::serde::de::Error::missing_field("Location"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod project {
    //! Property types for the `Project` resource.

    /// The [`AWS::DataBrew::Project.Sample`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-project-sample.html) property type.
    #[derive(Debug, Default)]
    pub struct Sample {
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-project-sample.html#cfn-databrew-project-sample-size).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size: Option<::Value<u32>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-project-sample.html#cfn-databrew-project-sample-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Sample {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref size) = self.size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", size)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Sample {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Sample, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Sample;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Sample")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut size: Option<::Value<u32>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Size" => {
                                size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Sample {
                        size: size,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod recipe {
    //! Property types for the `Recipe` resource.

    /// The [`AWS::DataBrew::Recipe.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`Operation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-action.html#cfn-databrew-recipe-action-operation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operation: ::Value<String>,
        /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-action.html#cfn-databrew-recipe-action-parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameters: Option<::Value<ParameterMap>>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Operation", &self.operation)?;
            if let Some(ref parameters) = self.parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut operation: Option<::Value<String>> = None;
                    let mut parameters: Option<::Value<ParameterMap>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Operation" => {
                                operation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parameters" => {
                                parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        operation: operation.ok_or(::serde::de::Error::missing_field("Operation"))?,
                        parameters: parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Recipe.ConditionExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-conditionexpression.html) property type.
    #[derive(Debug, Default)]
    pub struct ConditionExpression {
        /// Property [`Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-conditionexpression.html#cfn-databrew-recipe-conditionexpression-condition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition: ::Value<String>,
        /// Property [`TargetColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-conditionexpression.html#cfn-databrew-recipe-conditionexpression-targetcolumn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_column: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-conditionexpression.html#cfn-databrew-recipe-conditionexpression-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConditionExpression {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Condition", &self.condition)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetColumn", &self.target_column)?;
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConditionExpression {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConditionExpression, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConditionExpression;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConditionExpression")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition: Option<::Value<String>> = None;
                    let mut target_column: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Condition" => {
                                condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetColumn" => {
                                target_column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConditionExpression {
                        condition: condition.ok_or(::serde::de::Error::missing_field("Condition"))?,
                        target_column: target_column.ok_or(::serde::de::Error::missing_field("TargetColumn"))?,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Recipe.DataCatalogInputDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-datacataloginputdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct DataCatalogInputDefinition {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-datacataloginputdefinition.html#cfn-databrew-recipe-datacataloginputdefinition-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-datacataloginputdefinition.html#cfn-databrew-recipe-datacataloginputdefinition-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-datacataloginputdefinition.html#cfn-databrew-recipe-datacataloginputdefinition-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: Option<::Value<String>>,
        /// Property [`TempDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-datacataloginputdefinition.html#cfn-databrew-recipe-datacataloginputdefinition-tempdirectory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub temp_directory: Option<::Value<S3Location>>,
    }

    impl ::codec::SerializeValue for DataCatalogInputDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref table_name) = self.table_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", table_name)?;
            }
            if let Some(ref temp_directory) = self.temp_directory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TempDirectory", temp_directory)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataCatalogInputDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataCatalogInputDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataCatalogInputDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataCatalogInputDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;
                    let mut temp_directory: Option<::Value<S3Location>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TempDirectory" => {
                                temp_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataCatalogInputDefinition {
                        catalog_id: catalog_id,
                        database_name: database_name,
                        table_name: table_name,
                        temp_directory: temp_directory,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Recipe.ParameterMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-parametermap.html) property type.
    #[derive(Debug, Default)]
    pub struct ParameterMap {
    }

    impl ::codec::SerializeValue for ParameterMap {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParameterMap {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterMap, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParameterMap;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParameterMap")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ParameterMap {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Recipe.RecipeParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct RecipeParameters {
        /// Property [`AggregateFunction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-aggregatefunction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aggregate_function: Option<::Value<String>>,
        /// Property [`Base`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-base).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base: Option<::Value<String>>,
        /// Property [`CaseStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-casestatement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub case_statement: Option<::Value<String>>,
        /// Property [`CategoryMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-categorymap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub category_map: Option<::Value<String>>,
        /// Property [`CharsToRemove`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-charstoremove).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub chars_to_remove: Option<::Value<String>>,
        /// Property [`CollapseConsecutiveWhitespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-collapseconsecutivewhitespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub collapse_consecutive_whitespace: Option<::Value<String>>,
        /// Property [`ColumnDataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-columndatatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_data_type: Option<::Value<String>>,
        /// Property [`ColumnRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-columnrange).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_range: Option<::Value<String>>,
        /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-count).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub count: Option<::Value<String>>,
        /// Property [`CustomCharacters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-customcharacters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_characters: Option<::Value<String>>,
        /// Property [`CustomStopWords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-customstopwords).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_stop_words: Option<::Value<String>>,
        /// Property [`CustomValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-customvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_value: Option<::Value<String>>,
        /// Property [`DatasetsColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-datasetscolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datasets_columns: Option<::Value<String>>,
        /// Property [`DateAddValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-dateaddvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_add_value: Option<::Value<String>>,
        /// Property [`DateTimeFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-datetimeformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_time_format: Option<::Value<String>>,
        /// Property [`DateTimeParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-datetimeparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_time_parameters: Option<::Value<String>>,
        /// Property [`DeleteOtherRows`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-deleteotherrows).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delete_other_rows: Option<::Value<String>>,
        /// Property [`Delimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-delimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delimiter: Option<::Value<String>>,
        /// Property [`EndPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-endpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_pattern: Option<::Value<String>>,
        /// Property [`EndPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-endposition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_position: Option<::Value<String>>,
        /// Property [`EndValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-endvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_value: Option<::Value<String>>,
        /// Property [`ExpandContractions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-expandcontractions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expand_contractions: Option<::Value<String>>,
        /// Property [`Exponent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-exponent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub exponent: Option<::Value<String>>,
        /// Property [`FalseString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-falsestring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub false_string: Option<::Value<String>>,
        /// Property [`GroupByAggFunctionOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-groupbyaggfunctionoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_by_agg_function_options: Option<::Value<String>>,
        /// Property [`GroupByColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-groupbycolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_by_columns: Option<::Value<String>>,
        /// Property [`HiddenColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-hiddencolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hidden_columns: Option<::Value<String>>,
        /// Property [`IgnoreCase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-ignorecase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ignore_case: Option<::Value<String>>,
        /// Property [`IncludeInSplit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-includeinsplit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_in_split: Option<::Value<String>>,
        /// Property [`Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-input).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input: Option<::Value<::json::Value>>,
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-interval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interval: Option<::Value<String>>,
        /// Property [`IsText`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-istext).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_text: Option<::Value<String>>,
        /// Property [`JoinKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-joinkeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub join_keys: Option<::Value<String>>,
        /// Property [`JoinType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-jointype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub join_type: Option<::Value<String>>,
        /// Property [`LeftColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-leftcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub left_columns: Option<::Value<String>>,
        /// Property [`Limit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-limit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub limit: Option<::Value<String>>,
        /// Property [`LowerBound`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-lowerbound).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lower_bound: Option<::Value<String>>,
        /// Property [`MapType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-maptype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub map_type: Option<::Value<String>>,
        /// Property [`ModeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-modetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode_type: Option<::Value<String>>,
        /// Property [`MultiLine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-multiline).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multi_line: Option<::Value<bool>>,
        /// Property [`NumRows`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-numrows).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_rows: Option<::Value<String>>,
        /// Property [`NumRowsAfter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-numrowsafter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_rows_after: Option<::Value<String>>,
        /// Property [`NumRowsBefore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-numrowsbefore).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub num_rows_before: Option<::Value<String>>,
        /// Property [`OrderByColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-orderbycolumn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub order_by_column: Option<::Value<String>>,
        /// Property [`OrderByColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-orderbycolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub order_by_columns: Option<::Value<String>>,
        /// Property [`Other`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-other).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub other: Option<::Value<String>>,
        /// Property [`Pattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-pattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern: Option<::Value<String>>,
        /// Property [`PatternOption1`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-patternoption1).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern_option1: Option<::Value<String>>,
        /// Property [`PatternOption2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-patternoption2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern_option2: Option<::Value<String>>,
        /// Property [`PatternOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-patternoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern_options: Option<::Value<String>>,
        /// Property [`Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-period).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub period: Option<::Value<String>>,
        /// Property [`Position`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-position).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub position: Option<::Value<String>>,
        /// Property [`RemoveAllPunctuation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removeallpunctuation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_all_punctuation: Option<::Value<String>>,
        /// Property [`RemoveAllQuotes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removeallquotes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_all_quotes: Option<::Value<String>>,
        /// Property [`RemoveAllWhitespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removeallwhitespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_all_whitespace: Option<::Value<String>>,
        /// Property [`RemoveCustomCharacters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removecustomcharacters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_custom_characters: Option<::Value<String>>,
        /// Property [`RemoveCustomValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removecustomvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_custom_value: Option<::Value<String>>,
        /// Property [`RemoveLeadingAndTrailingPunctuation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removeleadingandtrailingpunctuation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_leading_and_trailing_punctuation: Option<::Value<String>>,
        /// Property [`RemoveLeadingAndTrailingQuotes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removeleadingandtrailingquotes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_leading_and_trailing_quotes: Option<::Value<String>>,
        /// Property [`RemoveLeadingAndTrailingWhitespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removeleadingandtrailingwhitespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_leading_and_trailing_whitespace: Option<::Value<String>>,
        /// Property [`RemoveLetters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removeletters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_letters: Option<::Value<String>>,
        /// Property [`RemoveNumbers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removenumbers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_numbers: Option<::Value<String>>,
        /// Property [`RemoveSourceColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removesourcecolumn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_source_column: Option<::Value<String>>,
        /// Property [`RemoveSpecialCharacters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-removespecialcharacters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub remove_special_characters: Option<::Value<String>>,
        /// Property [`RightColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-rightcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub right_columns: Option<::Value<String>>,
        /// Property [`SampleSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-samplesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sample_size: Option<::Value<String>>,
        /// Property [`SampleType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-sampletype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sample_type: Option<::Value<String>>,
        /// Property [`SecondInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-secondinput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub second_input: Option<::Value<String>>,
        /// Property [`SecondaryInputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-secondaryinputs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secondary_inputs: Option<::ValueList<SecondaryInput>>,
        /// Property [`SheetIndexes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-sheetindexes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sheet_indexes: Option<::ValueList<u32>>,
        /// Property [`SheetNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-sheetnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sheet_names: Option<::ValueList<String>>,
        /// Property [`SourceColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-sourcecolumn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_column: Option<::Value<String>>,
        /// Property [`SourceColumn1`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-sourcecolumn1).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_column1: Option<::Value<String>>,
        /// Property [`SourceColumn2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-sourcecolumn2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_column2: Option<::Value<String>>,
        /// Property [`SourceColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-sourcecolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_columns: Option<::Value<String>>,
        /// Property [`StartColumnIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-startcolumnindex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_column_index: Option<::Value<String>>,
        /// Property [`StartPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-startpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_pattern: Option<::Value<String>>,
        /// Property [`StartPosition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-startposition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_position: Option<::Value<String>>,
        /// Property [`StartValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-startvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_value: Option<::Value<String>>,
        /// Property [`StemmingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-stemmingmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stemming_mode: Option<::Value<String>>,
        /// Property [`StepCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-stepcount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub step_count: Option<::Value<String>>,
        /// Property [`StepIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-stepindex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub step_index: Option<::Value<String>>,
        /// Property [`StopWordsMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-stopwordsmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stop_words_mode: Option<::Value<String>>,
        /// Property [`Strategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-strategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub strategy: Option<::Value<String>>,
        /// Property [`TargetColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-targetcolumn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_column: Option<::Value<String>>,
        /// Property [`TargetColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-targetcolumnnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_column_names: Option<::Value<String>>,
        /// Property [`TargetDateFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-targetdateformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_date_format: Option<::Value<String>>,
        /// Property [`TargetIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-targetindex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_index: Option<::Value<String>>,
        /// Property [`TimeZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-timezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_zone: Option<::Value<String>>,
        /// Property [`TokenizerPattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-tokenizerpattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tokenizer_pattern: Option<::Value<String>>,
        /// Property [`TrueString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-truestring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub true_string: Option<::Value<String>>,
        /// Property [`UdfLang`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-udflang).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub udf_lang: Option<::Value<String>>,
        /// Property [`Units`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-units).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub units: Option<::Value<String>>,
        /// Property [`UnpivotColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-unpivotcolumn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unpivot_column: Option<::Value<String>>,
        /// Property [`UpperBound`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-upperbound).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub upper_bound: Option<::Value<String>>,
        /// Property [`UseNewDataFrame`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-usenewdataframe).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_new_data_frame: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
        /// Property [`Value1`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-value1).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value1: Option<::Value<String>>,
        /// Property [`Value2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-value2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value2: Option<::Value<String>>,
        /// Property [`ValueColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-valuecolumn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value_column: Option<::Value<String>>,
        /// Property [`ViewFrame`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html#cfn-databrew-recipe-recipeparameters-viewframe).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub view_frame: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RecipeParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aggregate_function) = self.aggregate_function {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregateFunction", aggregate_function)?;
            }
            if let Some(ref base) = self.base {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Base", base)?;
            }
            if let Some(ref case_statement) = self.case_statement {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaseStatement", case_statement)?;
            }
            if let Some(ref category_map) = self.category_map {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CategoryMap", category_map)?;
            }
            if let Some(ref chars_to_remove) = self.chars_to_remove {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CharsToRemove", chars_to_remove)?;
            }
            if let Some(ref collapse_consecutive_whitespace) = self.collapse_consecutive_whitespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollapseConsecutiveWhitespace", collapse_consecutive_whitespace)?;
            }
            if let Some(ref column_data_type) = self.column_data_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnDataType", column_data_type)?;
            }
            if let Some(ref column_range) = self.column_range {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnRange", column_range)?;
            }
            if let Some(ref count) = self.count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
            }
            if let Some(ref custom_characters) = self.custom_characters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomCharacters", custom_characters)?;
            }
            if let Some(ref custom_stop_words) = self.custom_stop_words {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomStopWords", custom_stop_words)?;
            }
            if let Some(ref custom_value) = self.custom_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomValue", custom_value)?;
            }
            if let Some(ref datasets_columns) = self.datasets_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetsColumns", datasets_columns)?;
            }
            if let Some(ref date_add_value) = self.date_add_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateAddValue", date_add_value)?;
            }
            if let Some(ref date_time_format) = self.date_time_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateTimeFormat", date_time_format)?;
            }
            if let Some(ref date_time_parameters) = self.date_time_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateTimeParameters", date_time_parameters)?;
            }
            if let Some(ref delete_other_rows) = self.delete_other_rows {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOtherRows", delete_other_rows)?;
            }
            if let Some(ref delimiter) = self.delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Delimiter", delimiter)?;
            }
            if let Some(ref end_pattern) = self.end_pattern {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndPattern", end_pattern)?;
            }
            if let Some(ref end_position) = self.end_position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndPosition", end_position)?;
            }
            if let Some(ref end_value) = self.end_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndValue", end_value)?;
            }
            if let Some(ref expand_contractions) = self.expand_contractions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpandContractions", expand_contractions)?;
            }
            if let Some(ref exponent) = self.exponent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Exponent", exponent)?;
            }
            if let Some(ref false_string) = self.false_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FalseString", false_string)?;
            }
            if let Some(ref group_by_agg_function_options) = self.group_by_agg_function_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupByAggFunctionOptions", group_by_agg_function_options)?;
            }
            if let Some(ref group_by_columns) = self.group_by_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupByColumns", group_by_columns)?;
            }
            if let Some(ref hidden_columns) = self.hidden_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HiddenColumns", hidden_columns)?;
            }
            if let Some(ref ignore_case) = self.ignore_case {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IgnoreCase", ignore_case)?;
            }
            if let Some(ref include_in_split) = self.include_in_split {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeInSplit", include_in_split)?;
            }
            if let Some(ref input) = self.input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Input", input)?;
            }
            if let Some(ref interval) = self.interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", interval)?;
            }
            if let Some(ref is_text) = self.is_text {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsText", is_text)?;
            }
            if let Some(ref join_keys) = self.join_keys {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JoinKeys", join_keys)?;
            }
            if let Some(ref join_type) = self.join_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JoinType", join_type)?;
            }
            if let Some(ref left_columns) = self.left_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LeftColumns", left_columns)?;
            }
            if let Some(ref limit) = self.limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Limit", limit)?;
            }
            if let Some(ref lower_bound) = self.lower_bound {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LowerBound", lower_bound)?;
            }
            if let Some(ref map_type) = self.map_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MapType", map_type)?;
            }
            if let Some(ref mode_type) = self.mode_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModeType", mode_type)?;
            }
            if let Some(ref multi_line) = self.multi_line {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiLine", multi_line)?;
            }
            if let Some(ref num_rows) = self.num_rows {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumRows", num_rows)?;
            }
            if let Some(ref num_rows_after) = self.num_rows_after {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumRowsAfter", num_rows_after)?;
            }
            if let Some(ref num_rows_before) = self.num_rows_before {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumRowsBefore", num_rows_before)?;
            }
            if let Some(ref order_by_column) = self.order_by_column {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrderByColumn", order_by_column)?;
            }
            if let Some(ref order_by_columns) = self.order_by_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrderByColumns", order_by_columns)?;
            }
            if let Some(ref other) = self.other {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Other", other)?;
            }
            if let Some(ref pattern) = self.pattern {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pattern", pattern)?;
            }
            if let Some(ref pattern_option1) = self.pattern_option1 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatternOption1", pattern_option1)?;
            }
            if let Some(ref pattern_option2) = self.pattern_option2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatternOption2", pattern_option2)?;
            }
            if let Some(ref pattern_options) = self.pattern_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatternOptions", pattern_options)?;
            }
            if let Some(ref period) = self.period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", period)?;
            }
            if let Some(ref position) = self.position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Position", position)?;
            }
            if let Some(ref remove_all_punctuation) = self.remove_all_punctuation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveAllPunctuation", remove_all_punctuation)?;
            }
            if let Some(ref remove_all_quotes) = self.remove_all_quotes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveAllQuotes", remove_all_quotes)?;
            }
            if let Some(ref remove_all_whitespace) = self.remove_all_whitespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveAllWhitespace", remove_all_whitespace)?;
            }
            if let Some(ref remove_custom_characters) = self.remove_custom_characters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveCustomCharacters", remove_custom_characters)?;
            }
            if let Some(ref remove_custom_value) = self.remove_custom_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveCustomValue", remove_custom_value)?;
            }
            if let Some(ref remove_leading_and_trailing_punctuation) = self.remove_leading_and_trailing_punctuation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveLeadingAndTrailingPunctuation", remove_leading_and_trailing_punctuation)?;
            }
            if let Some(ref remove_leading_and_trailing_quotes) = self.remove_leading_and_trailing_quotes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveLeadingAndTrailingQuotes", remove_leading_and_trailing_quotes)?;
            }
            if let Some(ref remove_leading_and_trailing_whitespace) = self.remove_leading_and_trailing_whitespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveLeadingAndTrailingWhitespace", remove_leading_and_trailing_whitespace)?;
            }
            if let Some(ref remove_letters) = self.remove_letters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveLetters", remove_letters)?;
            }
            if let Some(ref remove_numbers) = self.remove_numbers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveNumbers", remove_numbers)?;
            }
            if let Some(ref remove_source_column) = self.remove_source_column {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveSourceColumn", remove_source_column)?;
            }
            if let Some(ref remove_special_characters) = self.remove_special_characters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveSpecialCharacters", remove_special_characters)?;
            }
            if let Some(ref right_columns) = self.right_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RightColumns", right_columns)?;
            }
            if let Some(ref sample_size) = self.sample_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampleSize", sample_size)?;
            }
            if let Some(ref sample_type) = self.sample_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampleType", sample_type)?;
            }
            if let Some(ref second_input) = self.second_input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondInput", second_input)?;
            }
            if let Some(ref secondary_inputs) = self.secondary_inputs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryInputs", secondary_inputs)?;
            }
            if let Some(ref sheet_indexes) = self.sheet_indexes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SheetIndexes", sheet_indexes)?;
            }
            if let Some(ref sheet_names) = self.sheet_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SheetNames", sheet_names)?;
            }
            if let Some(ref source_column) = self.source_column {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceColumn", source_column)?;
            }
            if let Some(ref source_column1) = self.source_column1 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceColumn1", source_column1)?;
            }
            if let Some(ref source_column2) = self.source_column2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceColumn2", source_column2)?;
            }
            if let Some(ref source_columns) = self.source_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceColumns", source_columns)?;
            }
            if let Some(ref start_column_index) = self.start_column_index {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartColumnIndex", start_column_index)?;
            }
            if let Some(ref start_pattern) = self.start_pattern {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartPattern", start_pattern)?;
            }
            if let Some(ref start_position) = self.start_position {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartPosition", start_position)?;
            }
            if let Some(ref start_value) = self.start_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartValue", start_value)?;
            }
            if let Some(ref stemming_mode) = self.stemming_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StemmingMode", stemming_mode)?;
            }
            if let Some(ref step_count) = self.step_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StepCount", step_count)?;
            }
            if let Some(ref step_index) = self.step_index {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StepIndex", step_index)?;
            }
            if let Some(ref stop_words_mode) = self.stop_words_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StopWordsMode", stop_words_mode)?;
            }
            if let Some(ref strategy) = self.strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Strategy", strategy)?;
            }
            if let Some(ref target_column) = self.target_column {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetColumn", target_column)?;
            }
            if let Some(ref target_column_names) = self.target_column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetColumnNames", target_column_names)?;
            }
            if let Some(ref target_date_format) = self.target_date_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetDateFormat", target_date_format)?;
            }
            if let Some(ref target_index) = self.target_index {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetIndex", target_index)?;
            }
            if let Some(ref time_zone) = self.time_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeZone", time_zone)?;
            }
            if let Some(ref tokenizer_pattern) = self.tokenizer_pattern {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenizerPattern", tokenizer_pattern)?;
            }
            if let Some(ref true_string) = self.true_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrueString", true_string)?;
            }
            if let Some(ref udf_lang) = self.udf_lang {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UdfLang", udf_lang)?;
            }
            if let Some(ref units) = self.units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Units", units)?;
            }
            if let Some(ref unpivot_column) = self.unpivot_column {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnpivotColumn", unpivot_column)?;
            }
            if let Some(ref upper_bound) = self.upper_bound {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpperBound", upper_bound)?;
            }
            if let Some(ref use_new_data_frame) = self.use_new_data_frame {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseNewDataFrame", use_new_data_frame)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            if let Some(ref value1) = self.value1 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value1", value1)?;
            }
            if let Some(ref value2) = self.value2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value2", value2)?;
            }
            if let Some(ref value_column) = self.value_column {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueColumn", value_column)?;
            }
            if let Some(ref view_frame) = self.view_frame {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ViewFrame", view_frame)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecipeParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecipeParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecipeParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecipeParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aggregate_function: Option<::Value<String>> = None;
                    let mut base: Option<::Value<String>> = None;
                    let mut case_statement: Option<::Value<String>> = None;
                    let mut category_map: Option<::Value<String>> = None;
                    let mut chars_to_remove: Option<::Value<String>> = None;
                    let mut collapse_consecutive_whitespace: Option<::Value<String>> = None;
                    let mut column_data_type: Option<::Value<String>> = None;
                    let mut column_range: Option<::Value<String>> = None;
                    let mut count: Option<::Value<String>> = None;
                    let mut custom_characters: Option<::Value<String>> = None;
                    let mut custom_stop_words: Option<::Value<String>> = None;
                    let mut custom_value: Option<::Value<String>> = None;
                    let mut datasets_columns: Option<::Value<String>> = None;
                    let mut date_add_value: Option<::Value<String>> = None;
                    let mut date_time_format: Option<::Value<String>> = None;
                    let mut date_time_parameters: Option<::Value<String>> = None;
                    let mut delete_other_rows: Option<::Value<String>> = None;
                    let mut delimiter: Option<::Value<String>> = None;
                    let mut end_pattern: Option<::Value<String>> = None;
                    let mut end_position: Option<::Value<String>> = None;
                    let mut end_value: Option<::Value<String>> = None;
                    let mut expand_contractions: Option<::Value<String>> = None;
                    let mut exponent: Option<::Value<String>> = None;
                    let mut false_string: Option<::Value<String>> = None;
                    let mut group_by_agg_function_options: Option<::Value<String>> = None;
                    let mut group_by_columns: Option<::Value<String>> = None;
                    let mut hidden_columns: Option<::Value<String>> = None;
                    let mut ignore_case: Option<::Value<String>> = None;
                    let mut include_in_split: Option<::Value<String>> = None;
                    let mut input: Option<::Value<::json::Value>> = None;
                    let mut interval: Option<::Value<String>> = None;
                    let mut is_text: Option<::Value<String>> = None;
                    let mut join_keys: Option<::Value<String>> = None;
                    let mut join_type: Option<::Value<String>> = None;
                    let mut left_columns: Option<::Value<String>> = None;
                    let mut limit: Option<::Value<String>> = None;
                    let mut lower_bound: Option<::Value<String>> = None;
                    let mut map_type: Option<::Value<String>> = None;
                    let mut mode_type: Option<::Value<String>> = None;
                    let mut multi_line: Option<::Value<bool>> = None;
                    let mut num_rows: Option<::Value<String>> = None;
                    let mut num_rows_after: Option<::Value<String>> = None;
                    let mut num_rows_before: Option<::Value<String>> = None;
                    let mut order_by_column: Option<::Value<String>> = None;
                    let mut order_by_columns: Option<::Value<String>> = None;
                    let mut other: Option<::Value<String>> = None;
                    let mut pattern: Option<::Value<String>> = None;
                    let mut pattern_option1: Option<::Value<String>> = None;
                    let mut pattern_option2: Option<::Value<String>> = None;
                    let mut pattern_options: Option<::Value<String>> = None;
                    let mut period: Option<::Value<String>> = None;
                    let mut position: Option<::Value<String>> = None;
                    let mut remove_all_punctuation: Option<::Value<String>> = None;
                    let mut remove_all_quotes: Option<::Value<String>> = None;
                    let mut remove_all_whitespace: Option<::Value<String>> = None;
                    let mut remove_custom_characters: Option<::Value<String>> = None;
                    let mut remove_custom_value: Option<::Value<String>> = None;
                    let mut remove_leading_and_trailing_punctuation: Option<::Value<String>> = None;
                    let mut remove_leading_and_trailing_quotes: Option<::Value<String>> = None;
                    let mut remove_leading_and_trailing_whitespace: Option<::Value<String>> = None;
                    let mut remove_letters: Option<::Value<String>> = None;
                    let mut remove_numbers: Option<::Value<String>> = None;
                    let mut remove_source_column: Option<::Value<String>> = None;
                    let mut remove_special_characters: Option<::Value<String>> = None;
                    let mut right_columns: Option<::Value<String>> = None;
                    let mut sample_size: Option<::Value<String>> = None;
                    let mut sample_type: Option<::Value<String>> = None;
                    let mut second_input: Option<::Value<String>> = None;
                    let mut secondary_inputs: Option<::ValueList<SecondaryInput>> = None;
                    let mut sheet_indexes: Option<::ValueList<u32>> = None;
                    let mut sheet_names: Option<::ValueList<String>> = None;
                    let mut source_column: Option<::Value<String>> = None;
                    let mut source_column1: Option<::Value<String>> = None;
                    let mut source_column2: Option<::Value<String>> = None;
                    let mut source_columns: Option<::Value<String>> = None;
                    let mut start_column_index: Option<::Value<String>> = None;
                    let mut start_pattern: Option<::Value<String>> = None;
                    let mut start_position: Option<::Value<String>> = None;
                    let mut start_value: Option<::Value<String>> = None;
                    let mut stemming_mode: Option<::Value<String>> = None;
                    let mut step_count: Option<::Value<String>> = None;
                    let mut step_index: Option<::Value<String>> = None;
                    let mut stop_words_mode: Option<::Value<String>> = None;
                    let mut strategy: Option<::Value<String>> = None;
                    let mut target_column: Option<::Value<String>> = None;
                    let mut target_column_names: Option<::Value<String>> = None;
                    let mut target_date_format: Option<::Value<String>> = None;
                    let mut target_index: Option<::Value<String>> = None;
                    let mut time_zone: Option<::Value<String>> = None;
                    let mut tokenizer_pattern: Option<::Value<String>> = None;
                    let mut true_string: Option<::Value<String>> = None;
                    let mut udf_lang: Option<::Value<String>> = None;
                    let mut units: Option<::Value<String>> = None;
                    let mut unpivot_column: Option<::Value<String>> = None;
                    let mut upper_bound: Option<::Value<String>> = None;
                    let mut use_new_data_frame: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;
                    let mut value1: Option<::Value<String>> = None;
                    let mut value2: Option<::Value<String>> = None;
                    let mut value_column: Option<::Value<String>> = None;
                    let mut view_frame: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AggregateFunction" => {
                                aggregate_function = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Base" => {
                                base = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaseStatement" => {
                                case_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CategoryMap" => {
                                category_map = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CharsToRemove" => {
                                chars_to_remove = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CollapseConsecutiveWhitespace" => {
                                collapse_consecutive_whitespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnDataType" => {
                                column_data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnRange" => {
                                column_range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Count" => {
                                count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomCharacters" => {
                                custom_characters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomStopWords" => {
                                custom_stop_words = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomValue" => {
                                custom_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatasetsColumns" => {
                                datasets_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DateAddValue" => {
                                date_add_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DateTimeFormat" => {
                                date_time_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DateTimeParameters" => {
                                date_time_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeleteOtherRows" => {
                                delete_other_rows = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Delimiter" => {
                                delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndPattern" => {
                                end_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndPosition" => {
                                end_position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndValue" => {
                                end_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExpandContractions" => {
                                expand_contractions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Exponent" => {
                                exponent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FalseString" => {
                                false_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupByAggFunctionOptions" => {
                                group_by_agg_function_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupByColumns" => {
                                group_by_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HiddenColumns" => {
                                hidden_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IgnoreCase" => {
                                ignore_case = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeInSplit" => {
                                include_in_split = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Input" => {
                                input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsText" => {
                                is_text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JoinKeys" => {
                                join_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JoinType" => {
                                join_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LeftColumns" => {
                                left_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Limit" => {
                                limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LowerBound" => {
                                lower_bound = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MapType" => {
                                map_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModeType" => {
                                mode_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MultiLine" => {
                                multi_line = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumRows" => {
                                num_rows = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumRowsAfter" => {
                                num_rows_after = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumRowsBefore" => {
                                num_rows_before = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrderByColumn" => {
                                order_by_column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrderByColumns" => {
                                order_by_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Other" => {
                                other = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pattern" => {
                                pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PatternOption1" => {
                                pattern_option1 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PatternOption2" => {
                                pattern_option2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PatternOptions" => {
                                pattern_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Period" => {
                                period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Position" => {
                                position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveAllPunctuation" => {
                                remove_all_punctuation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveAllQuotes" => {
                                remove_all_quotes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveAllWhitespace" => {
                                remove_all_whitespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveCustomCharacters" => {
                                remove_custom_characters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveCustomValue" => {
                                remove_custom_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveLeadingAndTrailingPunctuation" => {
                                remove_leading_and_trailing_punctuation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveLeadingAndTrailingQuotes" => {
                                remove_leading_and_trailing_quotes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveLeadingAndTrailingWhitespace" => {
                                remove_leading_and_trailing_whitespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveLetters" => {
                                remove_letters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveNumbers" => {
                                remove_numbers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveSourceColumn" => {
                                remove_source_column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RemoveSpecialCharacters" => {
                                remove_special_characters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RightColumns" => {
                                right_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SampleSize" => {
                                sample_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SampleType" => {
                                sample_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondInput" => {
                                second_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondaryInputs" => {
                                secondary_inputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SheetIndexes" => {
                                sheet_indexes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SheetNames" => {
                                sheet_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceColumn" => {
                                source_column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceColumn1" => {
                                source_column1 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceColumn2" => {
                                source_column2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceColumns" => {
                                source_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartColumnIndex" => {
                                start_column_index = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartPattern" => {
                                start_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartPosition" => {
                                start_position = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartValue" => {
                                start_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StemmingMode" => {
                                stemming_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StepCount" => {
                                step_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StepIndex" => {
                                step_index = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StopWordsMode" => {
                                stop_words_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Strategy" => {
                                strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetColumn" => {
                                target_column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetColumnNames" => {
                                target_column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetDateFormat" => {
                                target_date_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetIndex" => {
                                target_index = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeZone" => {
                                time_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TokenizerPattern" => {
                                tokenizer_pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrueString" => {
                                true_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UdfLang" => {
                                udf_lang = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Units" => {
                                units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnpivotColumn" => {
                                unpivot_column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpperBound" => {
                                upper_bound = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseNewDataFrame" => {
                                use_new_data_frame = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value1" => {
                                value1 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value2" => {
                                value2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueColumn" => {
                                value_column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ViewFrame" => {
                                view_frame = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecipeParameters {
                        aggregate_function: aggregate_function,
                        base: base,
                        case_statement: case_statement,
                        category_map: category_map,
                        chars_to_remove: chars_to_remove,
                        collapse_consecutive_whitespace: collapse_consecutive_whitespace,
                        column_data_type: column_data_type,
                        column_range: column_range,
                        count: count,
                        custom_characters: custom_characters,
                        custom_stop_words: custom_stop_words,
                        custom_value: custom_value,
                        datasets_columns: datasets_columns,
                        date_add_value: date_add_value,
                        date_time_format: date_time_format,
                        date_time_parameters: date_time_parameters,
                        delete_other_rows: delete_other_rows,
                        delimiter: delimiter,
                        end_pattern: end_pattern,
                        end_position: end_position,
                        end_value: end_value,
                        expand_contractions: expand_contractions,
                        exponent: exponent,
                        false_string: false_string,
                        group_by_agg_function_options: group_by_agg_function_options,
                        group_by_columns: group_by_columns,
                        hidden_columns: hidden_columns,
                        ignore_case: ignore_case,
                        include_in_split: include_in_split,
                        input: input,
                        interval: interval,
                        is_text: is_text,
                        join_keys: join_keys,
                        join_type: join_type,
                        left_columns: left_columns,
                        limit: limit,
                        lower_bound: lower_bound,
                        map_type: map_type,
                        mode_type: mode_type,
                        multi_line: multi_line,
                        num_rows: num_rows,
                        num_rows_after: num_rows_after,
                        num_rows_before: num_rows_before,
                        order_by_column: order_by_column,
                        order_by_columns: order_by_columns,
                        other: other,
                        pattern: pattern,
                        pattern_option1: pattern_option1,
                        pattern_option2: pattern_option2,
                        pattern_options: pattern_options,
                        period: period,
                        position: position,
                        remove_all_punctuation: remove_all_punctuation,
                        remove_all_quotes: remove_all_quotes,
                        remove_all_whitespace: remove_all_whitespace,
                        remove_custom_characters: remove_custom_characters,
                        remove_custom_value: remove_custom_value,
                        remove_leading_and_trailing_punctuation: remove_leading_and_trailing_punctuation,
                        remove_leading_and_trailing_quotes: remove_leading_and_trailing_quotes,
                        remove_leading_and_trailing_whitespace: remove_leading_and_trailing_whitespace,
                        remove_letters: remove_letters,
                        remove_numbers: remove_numbers,
                        remove_source_column: remove_source_column,
                        remove_special_characters: remove_special_characters,
                        right_columns: right_columns,
                        sample_size: sample_size,
                        sample_type: sample_type,
                        second_input: second_input,
                        secondary_inputs: secondary_inputs,
                        sheet_indexes: sheet_indexes,
                        sheet_names: sheet_names,
                        source_column: source_column,
                        source_column1: source_column1,
                        source_column2: source_column2,
                        source_columns: source_columns,
                        start_column_index: start_column_index,
                        start_pattern: start_pattern,
                        start_position: start_position,
                        start_value: start_value,
                        stemming_mode: stemming_mode,
                        step_count: step_count,
                        step_index: step_index,
                        stop_words_mode: stop_words_mode,
                        strategy: strategy,
                        target_column: target_column,
                        target_column_names: target_column_names,
                        target_date_format: target_date_format,
                        target_index: target_index,
                        time_zone: time_zone,
                        tokenizer_pattern: tokenizer_pattern,
                        true_string: true_string,
                        udf_lang: udf_lang,
                        units: units,
                        unpivot_column: unpivot_column,
                        upper_bound: upper_bound,
                        use_new_data_frame: use_new_data_frame,
                        value: value,
                        value1: value1,
                        value2: value2,
                        value_column: value_column,
                        view_frame: view_frame,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Recipe.RecipeStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipestep.html) property type.
    #[derive(Debug, Default)]
    pub struct RecipeStep {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipestep.html#cfn-databrew-recipe-recipestep-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<Action>,
        /// Property [`ConditionExpressions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipestep.html#cfn-databrew-recipe-recipestep-conditionexpressions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition_expressions: Option<::ValueList<ConditionExpression>>,
    }

    impl ::codec::SerializeValue for RecipeStep {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            if let Some(ref condition_expressions) = self.condition_expressions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionExpressions", condition_expressions)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecipeStep {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecipeStep, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecipeStep;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecipeStep")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<Action>> = None;
                    let mut condition_expressions: Option<::ValueList<ConditionExpression>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConditionExpressions" => {
                                condition_expressions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecipeStep {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        condition_expressions: condition_expressions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Recipe.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-s3location.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-s3location.html#cfn-databrew-recipe-s3location-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-s3location.html#cfn-databrew-recipe-s3location-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key: key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataBrew::Recipe.SecondaryInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-secondaryinput.html) property type.
    #[derive(Debug, Default)]
    pub struct SecondaryInput {
        /// Property [`DataCatalogInputDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-secondaryinput.html#cfn-databrew-recipe-secondaryinput-datacataloginputdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_catalog_input_definition: Option<::Value<DataCatalogInputDefinition>>,
        /// Property [`S3InputDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-secondaryinput.html#cfn-databrew-recipe-secondaryinput-s3inputdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_input_definition: Option<::Value<S3Location>>,
    }

    impl ::codec::SerializeValue for SecondaryInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_catalog_input_definition) = self.data_catalog_input_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataCatalogInputDefinition", data_catalog_input_definition)?;
            }
            if let Some(ref s3_input_definition) = self.s3_input_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3InputDefinition", s3_input_definition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SecondaryInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SecondaryInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SecondaryInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SecondaryInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_catalog_input_definition: Option<::Value<DataCatalogInputDefinition>> = None;
                    let mut s3_input_definition: Option<::Value<S3Location>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataCatalogInputDefinition" => {
                                data_catalog_input_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3InputDefinition" => {
                                s3_input_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SecondaryInput {
                        data_catalog_input_definition: data_catalog_input_definition,
                        s3_input_definition: s3_input_definition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
