use std::fs::File;
use std::fmt;
use std::io::{self, Write};
use std::path::Path;

use heck::SnakeCase;
use itertools::{Itertools, EitherOrBoth, merge_join_by};

use model::*;

mod serde;
use self::serde::{
    generate_serialize,
    generate_deserialize,
    generate_deserialize_value
};

struct Printer {
    inner: Box<Write>,
    indent: usize
}

impl Printer {
    fn new<W: Write + 'static>(w: W) -> Printer {
        Printer { inner: Box::new(w), indent: 0 }
    }

    fn format_line(&mut self, args: fmt::Arguments) -> io::Result<()> {
        static PADDING: &str = "                    ";
        writeln!(self.inner, "{}{}", &PADDING[0..self.indent], args)
    }

    fn hard_break(&mut self) -> io::Result<()> {
        self.inner.write_all(b"\n")
    }

    fn indent(&mut self) {
        self.indent += 4;
    }

    fn unindent(&mut self) {
        self.indent -= 4;
   }
}

pub fn generate<P: AsRef<Path>>(spec: Specification, base_path: P) -> io::Result<()> {
    let resource_groups = spec.resource_types.into_iter().map(|(res_name, res_spec)| {
        assert!(res_name.starts_with("AWS::"));
        let split = res_name[5..].split("::").collect::<Vec<_>>();
        assert!(split.len() == 2);
        (split[0].to_owned(), split[1].to_owned(), res_spec)
    }).group_by(|&(ref service_name, _, _)| service_name.to_owned());

    let property_groups = spec.property_types.into_iter().flatten().filter_map(|(prop_name, prop_spec)| {
        if prop_name.starts_with("AWS::") {
            let split = prop_name[5..].split("::").collect::<Vec<_>>();
            assert!(split.len() == 2);
            let split2 = split[1].split(".").collect::<Vec<_>>();
            assert!(split2.len() == 2);
            Some((split[0].to_owned(), split2[0].to_owned(), split2[1].to_owned(), prop_spec))
        } else {
            None
        }
    }).group_by(|&(ref service_name, _, _, _)| service_name.to_owned());

    let groups = merge_join_by(resource_groups.into_iter(), property_groups.into_iter(),
        |&(ref service_name_left, _), &(ref service_name_right, _)| service_name_left.cmp(service_name_right));

    let mut services = Vec::new();
    let mut resources = Vec::new();
    for joined_specs in groups {
        let (service_name, resource_specs_opt, property_specs_opt) = factor_joined_specs(joined_specs);
        services.push(service_name.to_owned());
        let file_path = base_path.as_ref().join(format!("{}.rs", service_name.to_lowercase()));
        let file = File::create(file_path)?;
        let mut printer = Printer::new(file);

        printer.format_line(format_args!("//! Types for the `{}` service.", service_name))?;

        if let Some(resource_specs) = resource_specs_opt {
            for (_, resource_name, resource_spec) in resource_specs {
                resources.push((service_name.to_owned(), resource_name.to_owned()));
                generate_resource_declaration(&service_name, &resource_name, &resource_spec, &mut printer)?;
            }
        }

        if let Some(property_specs) = property_specs_opt {
            let properties_by_resource = property_specs.group_by(|&(_, ref resource_name, _, _)| resource_name.to_owned());
            for (resource_name, resource_property_specs) in properties_by_resource.into_iter() {
                printer.hard_break()?;
                printer.format_line(format_args!("pub mod {} {{", resource_name.to_snake_case()))?;
                printer.indent();
                printer.format_line(format_args!("//! Property types for the `{}` resource.", resource_name))?;
                for (_, resource_name, property_name, property_spec) in resource_property_specs {
                    generate_property_declaration(&service_name, &resource_name, &property_name, &property_spec, &mut printer)?;
                }
                printer.unindent();
                printer.format_line(format_args!("}}"))?;
            }
        }
    }

    {
        let mod_file_path = base_path.as_ref().join("mod.rs");
        let mut mod_file = File::create(mod_file_path)?;
        write!(mod_file, "//! Types for CloudFormation resources and their properties.\n")?;
        for service_name in services {
            writeln!(mod_file, "pub mod {};", service_name.to_lowercase())?;
        }
    }

    Ok(())
}

fn factor_joined_specs<X, A, B>(either: EitherOrBoth<(X, A), (X, B)>) -> (X, Option<A>, Option<B>) {
    match either {
        EitherOrBoth::Left((key, left)) => (key, Some(left), None),
        EitherOrBoth::Right((key, right)) => (key, None, Some(right)),
        EitherOrBoth::Both((key, left), (_, right)) => (key, Some(left), Some(right))
    }
}

fn generate_property_declaration(service: &str, resource_name: &str, name: &str, spec: &PropertyType, p: &mut Printer) -> io::Result<()> {
    p.hard_break()?;
    p.format_line(format_args!("/// The [`AWS::{}::{}.{}`]({}) property type.", service, resource_name, name, spec.documentation))?;
    p.format_line(format_args!("#[derive(Debug)]"))?;
    p.format_line(format_args!("pub struct {} {{", name))?;
    p.indent();
    for (ref property_name, ref property_spec) in spec.properties.iter() {
        generate_field(None, property_name, property_spec, p)?;
    }
    p.unindent();
    p.format_line(format_args!("}}"))?;

    p.hard_break()?;
    generate_serialize("::codec::SerializeValue", name, &spec.properties, p)?;

    p.hard_break()?;
    generate_deserialize_value(name, &spec.properties, p)?;

    Ok(())
}

fn generate_resource_declaration(service: &str, name: &str, spec: &ResourceType, p: &mut Printer) -> io::Result<()> {
    p.hard_break()?;
    p.format_line(format_args!("/// The [`AWS::{}::{}`]({}) resource type.", service, name, spec.documentation))?;
    p.format_line(format_args!("#[derive(Debug)]"))?;
    p.format_line(format_args!("pub struct {} {{", name))?;
    p.format_line(format_args!("    properties: {}Properties", name))?;
    p.format_line(format_args!("}}"))?;

    p.hard_break()?;
    p.format_line(format_args!("/// Properties for the `{}` resource.", name))?;
    p.format_line(format_args!("#[derive(Debug)]"))?;
    p.format_line(format_args!("pub struct {}Properties {{", name))?;
    let namespace = name.to_snake_case();
    p.indent();
    for (ref property_name, ref property_spec) in spec.properties.iter() {
        generate_field(Some(&namespace), property_name, property_spec, p)?;
    }
    p.unindent();
    p.format_line(format_args!("}}"))?;

    p.hard_break()?;
    generate_serialize("::serde::Serialize", &format!("{}Properties", name), &spec.properties, p)?;

    p.hard_break()?;
    generate_deserialize(&format!("{}Properties", name), &spec.properties, p)?;

    p.hard_break()?;
    p.format_line(format_args!("impl<'a> ::Resource<'a> for {} {{", name))?;
    p.format_line(format_args!("    type Properties = {}Properties;", name))?;
    p.format_line(format_args!("    const TYPE: &'static str = \"AWS::{}::{}\";", service, name))?;
    p.format_line(format_args!("    fn properties(&self) -> &{}Properties {{", name))?;
    p.format_line(format_args!("        &self.properties"))?;
    p.format_line(format_args!("    }}"))?;
    p.format_line(format_args!("    fn properties_mut(&mut self) -> &mut {}Properties {{", name))?;
    p.format_line(format_args!("        &mut self.properties"))?;
    p.format_line(format_args!("    }}"))?;
    p.format_line(format_args!("}}"))?;

    p.hard_break()?;
    p.format_line(format_args!("impl ::private::Sealed for {} {{}}", name))?;

    p.hard_break()?;
    p.format_line(format_args!("impl From<{}Properties> for {} {{", name, name))?;
    p.format_line(format_args!("    fn from(properties: {}Properties) -> {} {{", name, name))?;
    p.format_line(format_args!("        {} {{ properties }}", name))?;
    p.format_line(format_args!("    }}"))?;
    p.format_line(format_args!("}}"))?;

    Ok(())
}

fn generate_field(namespace_opt: Option<&str>, name: &str, spec: &PropertySpecification, p: &mut Printer) -> io::Result<()> {
    let field_name = mutate_field_name(name);

    p.format_line(format_args!("/// Property `{}`.", name))?;
    if spec.required.unwrap_or(true) {
        p.format_line(format_args!("pub {}: {},", field_name, generate_field_type(namespace_opt, spec)))?;
    } else {
        p.format_line(format_args!("pub {}: Option<{}>,", field_name, generate_field_type(namespace_opt, spec)))?;
    }

    Ok(())
}

fn generate_field_type(namespace_opt: Option<&str>, spec: &PropertySpecification) -> String {
    if let Some(ref type_name) = spec.type_ {
        if type_name == "List" {
            if let Some(ref item_type) = spec.item_type {
                format!("::ValueList<{}>", generate_type(namespace_opt, item_type))
            } else {
                format!("::ValueList<{}>",
                    generate_primitive_type(spec.primitive_item_type.as_ref().unwrap()))
            }
        } else if type_name == "Map" {
            if let Some(ref item_type) = spec.item_type {
                format!("::ValueMap<{}>", generate_type(namespace_opt, item_type))
            } else {
                format!("::ValueMap<{}>",
                    generate_primitive_type(spec.primitive_item_type.as_ref().unwrap()))
            }
        } else {
            format!("::Value<{}>", generate_type(namespace_opt, type_name))
        }
    } else {
        format!("::Value<{}>", generate_primitive_type(spec.primitive_type.as_ref().unwrap()))
    }
}

fn mutate_field_name(name: &str) -> String {
    let mut field_name = name.to_snake_case();

    if field_name == "type" {
        field_name = "type_".into();
    }

    field_name
}

fn generate_type(namespace_opt: Option<&str>, name: &str) -> String {
    if name == "Tag" {
        "::Tag".to_owned()
    } else if let Some(namespace) = namespace_opt {
        format!("self::{}::{}", namespace, name)
    } else {
        name.to_owned()
    }
}

fn generate_primitive_type(primitive_type: &PrimitiveType) -> &str {
    match primitive_type {
        &PrimitiveType::String => "String",
        &PrimitiveType::Long => "u64",
        &PrimitiveType::Integer => "u32",
        &PrimitiveType::Double => "f64",
        &PrimitiveType::Boolean => "bool",
        &PrimitiveType::Timestamp => "String",
        &PrimitiveType::Json => "::json::Value"
    }
}
