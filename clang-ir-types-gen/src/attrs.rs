use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use tblgen::{Record, RecordKeeper, init::TypedInit};

use crate::common::*;
use crate::enums::enum_param_type_tokens;

pub(crate) fn generate_attrs(
    keeper: &RecordKeeper<'_>,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let mut attrs = BTreeMap::new();
    for rec in keeper.all_derived_definitions("CIR_Attr") {
        attrs.insert(rec.name()?.to_string(), rec);
    }
    for rec in keeper.all_derived_definitions_if_defined("CIR_EnumAttr") {
        let name = rec.name()?;
        if name.starts_with("CIR_") {
            attrs.insert(name.to_string(), rec);
        }
    }

    let variants = attrs
        .values()
        .copied()
        .map(generate_attr_variant)
        .collect::<Result<Vec<_>, _>>()?;
    let attribute_display_impl = attribute_display_impl();

    Ok(quote! {
        #![allow(non_camel_case_types)]

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Attribute {
            #(#variants,)*
            Unit,
            Bool(bool),
            Int { value: i128, ty: Option<crate::types::Type> },
            Float { text: String, ty: Option<crate::types::Type> },
            Str(String),
            Array(Vec<Attribute>),
            Dict(Vec<(String, Attribute)>),
            SymbolRef(String),
            Type(crate::types::Type),
            Named(String),
            Dialect {
                dialect: String,
                mnemonic: String,
                raw: Option<String>,
                ty: Option<crate::types::Type>,
            },
        }

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum ConstArrayData {
            Str(Vec<u8>),
            Elements(Vec<Attribute>),
        }

        #attribute_display_impl

        impl Attribute {
            pub fn as_str(&self) -> Option<&str> {
                match self {
                    Self::Str(s) | Self::SymbolRef(s) => Some(s),
                    _ => None,
                }
            }

            pub fn as_type(&self) -> Option<&crate::types::Type> {
                match self {
                    Self::Type(ty) => Some(ty),
                    _ => None,
                }
            }

            pub fn as_bool(&self) -> Option<bool> {
                match self {
                    Self::Bool(b) | Self::CirBool { value: b, .. } => Some(*b),
                    _ => None,
                }
            }

            pub fn as_int(&self) -> Option<i128> {
                match self {
                    Self::Int { value, .. } => Some(*value),
                    Self::CirInt { value, .. } => value.parse().ok(),
                    _ => None,
                }
            }

            pub fn as_dense_array_ints(&self) -> Option<Vec<i128>> {
                let Self::Dialect { dialect, mnemonic, raw: Some(raw), .. } = self else {
                    return None;
                };
                if dialect != "builtin" || mnemonic != "array" {
                    return None;
                }
                let digits = raw.split_once(':').map_or(raw.as_str(), |(_, rest)| rest);
                digits
                    .split(',')
                    .map(|part| part.trim().parse::<i128>().ok())
                    .collect()
            }
        }
    })
}

fn attribute_display_impl() -> TokenStream {
    quote! {
        impl std::fmt::Display for Attribute {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Unit => write!(f, "unit"),
                    Self::Bool(b) => write!(f, "{b}"),
                    Self::Int { value, .. } => write!(f, "{value}"),
                    Self::Float { text, .. } => write!(f, "{text}"),
                    Self::Str(s) => write!(f, "{s:?}"),
                    Self::Array(items) => {
                        write!(f, "[")?;
                        for (i, item) in items.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "{item}")?;
                        }
                        write!(f, "]")
                    }
                    Self::Dict(entries) => {
                        write!(f, "{{")?;
                        for (i, (k, v)) in entries.iter().enumerate() {
                            if i > 0 {
                                write!(f, ", ")?;
                            }
                            write!(f, "{k} = {v}")?;
                        }
                        write!(f, "}}")
                    }
                    Self::SymbolRef(s) => write!(f, "@{s}"),
                    Self::Type(t) => write!(f, "{t}"),
                    Self::Named(n) => write!(f, "#{n}"),
                    Self::CirInt { value, .. } | Self::CirFloat { value, .. } => write!(f, "{value}"),
                    Self::CirBool { value, .. } => write!(f, "{value}"),
                    Self::ConstArray { elts, .. } => write!(f, "[{elts}]"),
                    Self::ConstVector { elts, .. } => write!(f, "[{elts}]"),
                    Self::ConstRecord { members, .. } => write!(f, "{{{members}}}"),
                    Self::ConstComplex { real, imag, .. } => write!(f, "({real}, {imag})"),
                    Self::GlobalView { symbol, indices, .. } => {
                        write!(f, "@{symbol}")?;
                        if let Some(indices) = indices {
                            write!(f, "[{indices}]")?;
                        }
                        Ok(())
                    }
                    Self::Zero { .. } => write!(f, "zero"),
                    Self::Poison { .. } => write!(f, "poison"),
                    Self::Dialect { dialect, mnemonic, raw, .. } => {
                        write!(f, "#{dialect}.{mnemonic}")?;
                        if let Some(raw) = raw {
                            write!(f, "<{raw}>")?;
                        }
                        Ok(())
                    }
                    other => write!(f, "{other:?}"),
                }
            }
        }
    }
}

fn generate_attr_variant(rec: Record<'_>) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let name = rec.name()?;
    let mnemonic = rec.str_value("mnemonic").unwrap_or(name);
    let variant = attr_variant_ident(rec)?;
    let doc = record_doc(rec, Some(&format!("`#cir.{mnemonic}`")))?;

    let mut fields = Vec::new();
    if rec.has_field("parameters") {
        let dag = rec.dag_value("parameters")?;
        for (arg_name, init) in dag.args() {
            let Some(name) = arg_name else { continue };
            let field = safe_field_ident(name);
            let ty = attr_param_type_tokens(init)?;
            let doc = init_doc(init);
            fields.push(quote! {
                #doc
                #field: #ty,
            });
        }
    }

    let body = if fields.is_empty() {
        quote!()
    } else {
        quote! { { #(#fields)* } }
    };

    Ok(quote! {
        #doc
        #variant #body
    })
}

fn attr_variant_ident(rec: Record<'_>) -> Result<Ident, Box<dyn std::error::Error>> {
    let name = rec.name()?;
    let special = match name {
        "CIR_IntAttr" => Some("CirInt"),
        "CIR_BoolAttr" => Some("CirBool"),
        "CIR_FPAttr" => Some("CirFloat"),
        _ => None,
    };
    Ok(safe_variant_ident(
        special.unwrap_or(&variant_base_name(rec, "Attr")?),
    ))
}

fn attr_param_type_tokens(init: TypedInit<'_>) -> Result<TokenStream, Box<dyn std::error::Error>> {
    match init {
        TypedInit::String(cpp) => Ok(cpp_type_tokens(Module::Attrs, cpp.to_str()?, false, false)),
        TypedInit::Def(def) => {
            let rec: Record = def.into();
            let classes = record_classes(rec)?;
            if classes.iter().any(|c| c == "AttributeSelfTypeParameter") {
                Ok(type_token(Module::Attrs, "Type"))
            } else if classes
                .iter()
                .any(|c| c == "APIntParameter" || c == "APFloatParameter")
            {
                Ok(string_token())
            } else if classes.iter().any(|c| c == "EnumParameter") {
                Ok(enum_param_type_tokens(rec)?)
            } else if classes.iter().any(|c| c == "OptionalParameter") {
                let cpp = parameter_cpp_type(rec).unwrap_or_else(|| "mlir::Attribute".into());
                Ok(cpp_type_tokens(Module::Attrs, &cpp, false, true))
            } else if classes.iter().any(|c| c == "DefaultValuedParameter") {
                let cpp = parameter_cpp_type(rec).unwrap_or_else(|| "bool".into());
                Ok(cpp_type_tokens(Module::Attrs, &cpp, false, false))
            } else if is_type_constraint(&classes) {
                Ok(type_token(Module::Attrs, "Type"))
            } else {
                Ok(quote!(Box<Attribute>))
            }
        }
        _ => Ok(attr_token(Module::Attrs, "Attribute")),
    }
}
