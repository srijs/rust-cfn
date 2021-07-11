use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct Specification {
    /// The version of the resource specification. The version format is
    /// `majorVersion.minorVersion.patch`, where each release increments the version number.
    /// All resources have the same version number regardless of whether the resource was updated.
    /// AWS CloudFormation increments the patch number when the service makes a backwards-compatible
    /// bug fix, such as fixing a broken documentation link. When AWS CloudFormation adds resources
    /// or properties that are backwards compatible, it increments the minor version number.
    /// For example, later versions of a specification might add additional resource properties
    /// to support new features of an AWS service. Backwards incompatible changes increment the
    /// major version number. A backwards incompatible change can result from a change in the
    /// resource specification, such as a name change to a field, or a change to a resource,
    /// such as the making an optional resource property required.
    #[serde(rename="ResourceSpecificationVersion")]
    pub version: String,
    /// For resources that have properties within a property (also known as subproperties),
    /// a list of subproperty specifications, such as which properties are required, the type
    /// of allowed value for each property, and their update behavior. For more information,
    /// see `PropertySpecification`. If a resource doesn't have subproperties, this section is omitted.
    #[serde(rename="PropertyTypes")]
    pub property_types: Option<BTreeMap<String, PropertyType>>,
    /// The list of resources and information about each resource's properties, such as its
    /// property names, which properties are requires, and their update behavior. For more
    /// information, see `ResourceSpecification`. 
    #[serde(rename="ResourceTypes")]
    pub resource_types: BTreeMap<String, ResourceType>
}

#[derive(Deserialize)]
pub struct PropertyType {
    #[serde(rename="Documentation", default)]
    pub documentation: String,
    #[serde(rename="Properties", default)]
    pub properties: BTreeMap<String, PropertySpecification>
}

#[derive(Deserialize)]
pub struct ResourceType {
    /// A list of resource attributes that you can use in an Fn::GetAtt function. For each
    /// attribute, this section provides the attribute name and the type of value that AWS
    /// CloudFormation returns. 
    #[serde(rename="Attributes")]
    pub attributes: Option<BTreeMap<String, AttributeSpecification>>,
    /// A link to the AWS CloudFormation User Guide for information about the resource. 
    #[serde(rename="Documentation", default)]
    pub documentation: String,
    /// A list of property specifications for the resource. For details, see `PropertySpecification`.
    #[serde(rename="Properties", default)]
    pub properties: BTreeMap<String, PropertySpecification>
}

#[derive(Deserialize)]
pub struct AttributeSpecification {
    /// If the value of the Type field is List, indicates the type of list that the Fn::GetAtt
    /// function returns for the attribute if the list contains non-primitive types. The valid
    /// type is a name of a property. 
    #[serde(rename="ItemType")]
    pub item_type: Option<String>,
    /// If the value of the Type field is List, indicates the type of list that the Fn::GetAtt
    /// function returns for the attribute if the list contains primitive types. For lists that
    /// contain non-primitive types, the ItemType property indicates the valid value type. The
    /// valid primitive types for lists are String, Long, Integer, Double, Boolean, or Timestamp.
    /// For example, if the type value is List and the primitive item type value is String, the
    /// Fn::GetAtt function returns a list of strings.
    #[serde(rename="PrimitiveItemType")]
    pub primitive_item_type: Option<PrimitiveType>,
    /// For primitive return values, the type of primitive value that the Fn::GetAtt function
    /// returns for the attribute. A primitive type is a basic data type for resource property
    /// values. The valid primitive types are String, Long, Integer, Double, Boolean, Timestamp
    /// or Json. 
    #[serde(rename="PrimitiveType")]
    pub primitive_type: Option<PrimitiveType>,
    /// For non-primitive return values, the type of value that the Fn::GetAtt function returns
    /// for the attribute. The valid types are a property name or List. A list is a comma-separated
    /// list of values. The value type for lists are indicated by the ItemType or PrimitiveItemType
    /// field.
    #[serde(rename="Type")]
    pub type_: Option<String>
}

#[derive(Deserialize)]
pub struct PropertySpecification {
    /// A link to the AWS CloudFormation User Guide that provides information about the property.
    #[serde(rename="Documentation")]
    pub documentation: String,
    /// If the value of the Type field is List, indicates whether AWS CloudFormation allows
    /// duplicate values.  If the value is true, AWS CloudFormation ignores duplicate values.
    /// If the value is false, AWS CloudFormation returns an error if you submit duplicate values. 
    #[serde(rename="DuplicatesAllowed")]
    pub duplicates_allowed: Option<bool>,
    /// If the value of the Type field is List or Map, indicates the type of list or map if they
    /// contain non-primitive types. Otherwise, this field is omitted. For lists or maps that
    /// contain primitive types, the PrimitiveItemType property indicates the valid value type.
    /// A subproperty name is a valid item type. For example, if the type value is List and the
    /// item type value is PortMapping, you can specify a list of port mapping properties.
    #[serde(rename="ItemType")]
    pub item_type: Option<String>,
    /// If the value of the Type field is List or Map, indicates the type of list or map if they
    /// contain primitive types. Otherwise, this field is omitted. For lists or maps that contain
    /// non-primitive types, the ItemType property indicates the valid value type.
    /// The valid primitive types for lists and maps are String, Long, Integer, Double, Boolean,
    /// or Timestamp. For example, if the type value is List and the item type value is String,
    /// you can specify a list of strings for the property. If the type value is Map and the item
    /// type value is Boolean, you can specify a string to Boolean mapping for the property.
    #[serde(rename="PrimitiveItemType")]
    pub primitive_item_type: Option<PrimitiveType>,
    /// For primitive values, the valid primitive type for the property. A primitive type is a
    /// basic data type for resource property values. The valid primitive types are String, Long,
    /// Integer, Double, Boolean, Timestamp or Json. If valid values are a non-primitive type,
    /// this field is omitted and the Type field indicates the valid value type. 
    #[serde(rename="PrimitiveType")]
    pub primitive_type: Option<PrimitiveType>,
    /// Indicates whether the property is required.
    #[serde(rename="Required")]
    pub required: Option<bool>,
    /// For non-primitive types, valid values for the property. The valid types are a subproperty
    /// name, List or Map. If valid values are a primitive type, this field is omitted and the
    /// PrimitiveType field indicates the valid value type. A list is a comma-separated list of
    /// values. A map is a set of key-value pairs, where the keys are always strings. The value
    /// type for lists and maps are indicated by the ItemType or PrimitiveItemType field.
    #[serde(rename="Type")]
    pub type_: Option<String>,
    /// During a stack update, the update behavior when you add, remove, or modify the property.
    /// AWS CloudFormation replaces the resource when you change Immutable properties.
    /// AWS CloudFormation doesn't replace the resource when you change mutable properties.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other
    /// properties you updated. For more information, see the relevant resource type documentation. 
    #[serde(rename="UpdateType")]
    pub update_type: Option<UpdateType>
}

#[derive(Deserialize)]
pub enum PrimitiveType {
    String,
    Long,
    Integer,
    Double,
    Boolean,
    Timestamp,
    Json
}

#[derive(Clone, Copy, Deserialize)]
pub enum UpdateType {
    Mutable,
    Immutable,
    Conditional
}
