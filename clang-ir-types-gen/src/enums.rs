use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;
use tblgen::{Record, RecordKeeper};

use crate::common::*;

pub(crate) fn enum_param_type_tokens(
    rec: Record<'_>,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let enum_rec = rec
        .def_value("enum")
        .or_else(|_| rec.def_value("baseAttr"))?;
    enum_type_tokens(enum_rec)
}

pub(crate) fn enum_type_tokens(rec: Record<'_>) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let name = enum_type_name(rec)?;
    let ident = enum_type_ident(&name);
    Ok(quote!(crate::enums::#ident))
}

pub(crate) fn enum_type_name(rec: Record<'_>) -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(name) = rec.str_value("className")
        && !name.is_empty() {
            return Ok(name.to_string());
        }
    if let Ok(ret) = rec.str_value("returnType")
        && let Some(last) = ret.rsplit("::").next()
            && !last.is_empty()
        {
            return Ok(last.to_string());
        }
    variant_base_name(rec, "")
}

pub(crate) fn enum_type_ident(name: &str) -> Ident {
    if name == "FPClassTestEnum" {
        return format_ident!("FpClassFlags");
    }
    pascal_ident(name)
}

fn enum_variant_ident(class_name: &str, symbol: &str) -> Ident {
    let symbol = match (class_name, symbol) {
        ("GlobalLinkageKind", s) => s.strip_suffix("Linkage").unwrap_or(s),
        _ => symbol,
    };
    safe_variant_ident(symbol)
}

pub(crate) fn generate_enums(
    keeper: &RecordKeeper<'_>,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let mut enums = BTreeMap::new();
    for class in [
        "CIR_I32EnumAttr",
        "CIR_I64EnumAttr",
        "CIR_I32BitEnumAttr",
        "I32EnumAttr",
        "I64EnumAttr",
        "I32BitEnumAttr",
    ] {
        for rec in keeper.all_derived_definitions_if_defined(class) {
            let name = rec.name()?;
            if name.starts_with("CIR_") || name == "FPClassTestEnum" {
                enums.insert(name.to_string(), rec);
            }
        }
    }

    let mut defs = Vec::new();
    for rec in enums.values().copied() {
        defs.push(generate_enum_def(rec)?);
    }

    Ok(quote! {
        #![allow(non_camel_case_types)]
        #![allow(non_upper_case_globals)]

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct ParseEnumError {
            type_name: &'static str,
            input: String,
        }

        impl ParseEnumError {
            pub fn new(type_name: &'static str, input: impl Into<String>) -> Self {
                Self { type_name, input: input.into() }
            }
        }

        impl std::fmt::Display for ParseEnumError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "`{}` is not a valid {}", self.input, self.type_name)
            }
        }

        impl std::error::Error for ParseEnumError {}

        #(#defs)*
    })
}

fn generate_enum_def(rec: Record<'_>) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let class_name = enum_type_name(rec)?;
    let enum_ident = enum_type_ident(&class_name);
    let doc = record_doc(rec, None)?;
    let enumerants = rec.list_of_defs_value("enumerants")?;

    let mut cases = Vec::new();
    let mut seen = BTreeSet::new();
    for case in enumerants {
        let symbol = case.str_value("symbol")?;
        if !seen.insert(symbol.to_string()) {
            continue;
        }
        let value = case.int_value("value")?;
        let keyword = case.str_value("str").unwrap_or(symbol);
        let variant = enum_variant_ident(&class_name, symbol);
        cases.push((variant, value, keyword.to_string()));
    }

    let is_bit_enum = record_classes(rec)?
        .iter()
        .any(|c| c == "CIR_I32BitEnumAttr" || c == "I32BitEnumAttr");

    if is_bit_enum {
        let variants = cases.iter().map(|(v, value, _)| {
            quote! {
                pub const #v: u64 = #value as u64;
            }
        });
        Ok(quote! {
            #doc
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #enum_ident(pub u64);

            impl #enum_ident {
                #(#variants)*
            }
        })
    } else {
        let variant_idents = cases.iter().map(|(v, _, _)| v.clone()).collect::<Vec<_>>();
        let values = cases.iter().map(|(_, value, _)| *value).collect::<Vec<_>>();
        let keywords = cases
            .iter()
            .map(|(_, _, keyword)| keyword.clone())
            .collect::<Vec<_>>();
        let enum_name = enum_ident.to_string();

        let enum_def = quote! {
            pub enum #enum_ident {
                #(#variant_idents = #values),*
            }
        };

        let try_arms = variant_idents
            .iter()
            .zip(values.iter())
            .map(|(v, value)| quote!(#value => Ok(Self::#v)));
        let into_arms = variant_idents
            .iter()
            .zip(values.iter())
            .map(|(v, value)| quote!(#enum_ident::#v => i128::from(#value)));
        let from_str_arms = variant_idents
            .iter()
            .zip(keywords.iter())
            .map(|(v, keyword)| quote!(#keyword => Ok(Self::#v)));
        let display_arms = variant_idents
            .iter()
            .zip(keywords.iter())
            .map(|(v, keyword)| quote!(Self::#v => #keyword));

        Ok(quote! {
            #doc
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[repr(i64)]
            #enum_def

            impl TryFrom<i128> for #enum_ident {
                type Error = ParseEnumError;

                fn try_from(value: i128) -> Result<Self, Self::Error> {
                    let value = value as i64;
                    match value {
                        #(#try_arms),*,
                        _ => Err(ParseEnumError::new(#enum_name, value.to_string())),
                    }
                }
            }

            impl From<#enum_ident> for i128 {
                fn from(value: #enum_ident) -> i128 {
                    match value {
                        #(#into_arms),*
                    }
                }
            }

            impl std::str::FromStr for #enum_ident {
                type Err = ParseEnumError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        #(#from_str_arms),*,
                        _ => Err(ParseEnumError::new(#enum_name, s.to_string())),
                    }
                }
            }

            impl std::fmt::Display for #enum_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let keyword = match self {
                        #(#display_arms),*
                    };
                    write!(f, "{keyword}")
                }
            }

            impl TryFrom<&crate::attrs::Attribute> for #enum_ident {
                type Error = ParseEnumError;

                fn try_from(attr: &crate::attrs::Attribute) -> Result<Self, Self::Error> {
                    match attr {
                        crate::attrs::Attribute::Int { value, .. } => {
                            Self::try_from(*value as i128)
                        }
                        crate::attrs::Attribute::CirInt { value, .. } => {
                            value
                                .parse::<i128>()
                                .map_err(|_| ParseEnumError::new(#enum_name, value.clone()))
                                .and_then(Self::try_from)
                        }
                        crate::attrs::Attribute::Dialect { raw: Some(raw), .. } => {
                            raw.trim().parse()
                        }
                        other => Err(ParseEnumError::new(
                            #enum_name,
                            format!("{other:?}"),
                        )),
                    }
                }
            }
        })
    }
}
