use std::io;
use std::collections::BTreeMap;

use ::model::*;
use super::{Printer, mutate_field_name};

pub(super) fn generate_serialize(trait_name: &str, name: &str, props: &BTreeMap<String, PropertySpecification>, p: &mut Printer) -> io::Result<()> {
    p.format_line(format_args!("impl {} for {} {{", trait_name, name))?;
    p.format_line(format_args!("    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {{"))?;
    p.format_line(format_args!("        #[allow(unused_mut)]"))?;
    p.format_line(format_args!("        let mut map = ::serde::Serializer::serialize_map(s, None)?;"))?;
    for (prop_name, _prop_spec) in props {
        let field_name = mutate_field_name(prop_name);
        p.format_line(format_args!("        ::serde::ser::SerializeMap::serialize_entry(&mut map, \"{}\", &self.{})?;", prop_name, field_name))?;
    }
    p.format_line(format_args!("        ::serde::ser::SerializeMap::end(map)"))?;
    p.format_line(format_args!("    }}"))?;
    p.format_line(format_args!("}}"))
}

pub(super) fn generate_deserialize(name: &str, props: &BTreeMap<String, PropertySpecification>, p: &mut Printer) -> io::Result<()> {
    p.format_line(format_args!("impl<'de> ::serde::Deserialize<'de> for {} {{", name))?;
    p.indent();
    p.format_line(format_args!("fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<{}, D::Error> {{", name))?;
    p.indent();
    generate_deserialize_body(name, props, p)?;
    p.unindent();
    p.format_line(format_args!("}}"))?;
    p.unindent();
    p.format_line(format_args!("}}"))
}

pub(super) fn generate_deserialize_value(name: &str, props: &BTreeMap<String, PropertySpecification>, p: &mut Printer) -> io::Result<()> {
    p.format_line(format_args!("impl ::codec::DeserializeValue for {} {{", name))?;
    p.indent();
    p.format_line(format_args!("fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<{}, D::Error> {{", name))?;
    p.indent();
    generate_deserialize_body(name, props, p)?;
    p.unindent();
    p.format_line(format_args!("}}"))?;
    p.unindent();
    p.format_line(format_args!("}}"))
}

fn generate_deserialize_body(name: &str, props: &BTreeMap<String, PropertySpecification>, p: &mut Printer) -> io::Result<()> {
    p.format_line(format_args!("struct Visitor;"))?;
    p.hard_break()?;
    p.format_line(format_args!("impl<'de> ::serde::de::Visitor<'de> for Visitor {{"))?;
    p.indent();
    p.format_line(format_args!("type Value = {};", name))?;
    p.hard_break()?;
    p.format_line(format_args!("fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{"))?;
    p.format_line(format_args!("    write!(f, \"a struct of type {}\")", name))?;
    p.format_line(format_args!("}}"))?;
    p.hard_break()?;
    if props.len() > 0 {
        generate_deserialize_visit_map_non_empty(name, props, p)?;
    } else {
        generate_deserialize_visit_map_empty(name, p)?;
    }
    p.unindent();
    p.format_line(format_args!("}}"))?;
    p.hard_break()?;
    p.format_line(format_args!("d.deserialize_map(Visitor)"))
}

fn generate_deserialize_visit_map_empty(name: &str, p: &mut Printer) -> io::Result<()> {
    p.format_line(format_args!("fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {{"))?;
    p.format_line(format_args!("    Ok({} {{}})", name))?;
    p.format_line(format_args!("}}"))
}

fn generate_deserialize_visit_map_non_empty(name: &str, props: &BTreeMap<String, PropertySpecification>, p: &mut Printer) -> io::Result<()> {
    p.format_line(format_args!("fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {{"))?;
    p.indent();
    for (prop_name, _prop_spec) in props {
        let field_name = mutate_field_name(prop_name);
        p.format_line(format_args!("let mut {} = None;", field_name))?;
    }
    p.hard_break()?;
    p.format_line(format_args!("while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {{"))?;
    p.format_line(format_args!("    match __cfn_key.as_ref() {{"))?;
    for (prop_name, _prop_spec) in props {
        let field_name = mutate_field_name(prop_name);
        p.format_line(format_args!("        \"{}\" => {{", prop_name))?;
        p.format_line(format_args!("            {} = Some(::serde::de::MapAccess::next_value(&mut map)?);", field_name))?;
        p.format_line(format_args!("        }}"))?;
    }
    p.format_line(format_args!("        _ => {{}}"))?;
    p.format_line(format_args!("    }}"))?;
    p.format_line(format_args!("}}"))?;
    p.hard_break()?;
    p.format_line(format_args!("Ok({} {{", name))?;
    for (prop_name, prop_spec) in props {
        let field_name = mutate_field_name(prop_name);
        if prop_spec.required.unwrap_or(true) {
            p.format_line(format_args!("    {}: {}.ok_or(::serde::de::Error::missing_field(\"{}\"))?,", field_name, field_name, prop_name))?;
        } else {
            p.format_line(format_args!("    {}: {},", field_name, field_name))?;
        }
    }
    p.format_line(format_args!("}})"))?;
    p.unindent();
    p.format_line(format_args!("}}"))
}
