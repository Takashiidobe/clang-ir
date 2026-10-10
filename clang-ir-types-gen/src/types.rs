use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use tblgen::{Record, RecordKeeper, init::TypedInit};

use crate::common::*;

pub(crate) fn generate_types(
    keeper: &RecordKeeper<'_>,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let type_records: Vec<Record<'_>> = keeper.all_derived_definitions("CIR_Type").collect();
    let variants = type_records
        .iter()
        .copied()
        .map(generate_type_variant)
        .collect::<Result<Vec<_>, _>>()?;
    let projection_impl = type_projection_impl();
    let display_impl = type_display_impl(&type_records)?;

    Ok(quote! {
        #![allow(non_camel_case_types)]

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Type {
            #(#variants,)*
            /// A named type alias.
            Named(String),
            /// A builtin signless integer type.
            Integer(u32),
            /// The builtin `index` type.
            Index,
            /// A builtin function type.
            FunctionType { inputs: Vec<Type>, results: Vec<Type> },
            /// A type outside the CIR-specific variants.
            Dialect { dialect: String, mnemonic: String, raw: Option<String> },
        }

        #projection_impl
        #display_impl
    })
}

fn type_projection_impl() -> TokenStream {
    quote! {
        impl Type {
            pub fn as_array(&self) -> Option<(&Type, u64)> {
                match self {
                    Self::Array { element_type, size } => Some((element_type, *size)),
                    _ => None,
                }
            }

            pub fn as_vector(&self) -> Option<(&Type, u64, bool)> {
                match self {
                    Self::Vector { element_type, size, is_scalable } => {
                        Some((element_type, *size, is_scalable.unwrap_or(false)))
                    }
                    _ => None,
                }
            }

            pub fn as_integer(&self) -> Option<(bool, u32, bool)> {
                match self {
                    Self::Int { width, is_signed, is_bit_int } => {
                        Some((*is_signed, *width, *is_bit_int))
                    }
                    _ => None,
                }
            }

            pub fn pointee(&self) -> Option<&Type> {
                match self {
                    Self::Pointer { pointee, .. } => Some(pointee),
                    _ => None,
                }
            }

            pub fn as_function(&self) -> Option<(&[Type], Option<&Type>, bool)> {
                match self {
                    Self::Func { inputs, optional_return_type, var_arg } => {
                        Some((inputs, optional_return_type.as_deref(), *var_arg))
                    }
                    _ => None,
                }
            }

            pub fn record_name(&self) -> Option<&str> {
                match self {
                    Self::Struct { name, .. } | Self::Union { name, .. } => name.as_deref(),
                    _ => None,
                }
            }
        }
    }
}

const CUSTOM_DISPLAY_VARIANTS: &[&str] = &[
    "Int",
    "LongDouble",
    "Pointer",
    "Array",
    "Vector",
    "Func",
    "Struct",
    "Union",
    "Complex",
    "DataMember",
    "Method",
];

fn type_variant_shape(
    rec: Record<'_>,
) -> Result<(Ident, String, bool), Box<dyn std::error::Error>> {
    let name = rec.name()?;
    let mnemonic = rec.str_value("mnemonic").unwrap_or(name).to_string();
    let variant = variant_ident(rec, "Type")?;
    let has_fields = if rec.has_field("parameters") {
        rec.dag_value("parameters")?
            .args()
            .any(|(arg_name, _)| arg_name.is_some())
    } else {
        false
    };
    Ok((variant, mnemonic, has_fields))
}

fn type_display_fallback_arms(
    type_records: &[Record<'_>],
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let mut arms = Vec::new();
    for rec in type_records {
        let (variant, mnemonic, has_fields) = type_variant_shape(*rec)?;
        if CUSTOM_DISPLAY_VARIANTS.contains(&variant.to_string().as_str()) {
            continue;
        }
        arms.push(if has_fields {
            quote! { Self::#variant { .. } => write!(f, #mnemonic), }
        } else {
            quote! { Self::#variant => write!(f, #mnemonic), }
        });
    }
    Ok(quote! { #(#arms)* })
}

fn type_display_impl(
    type_records: &[Record<'_>],
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let fallback_arms = type_display_fallback_arms(type_records)?;
    Ok(quote! {
        impl std::fmt::Display for Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                fn write_list(
                    f: &mut std::fmt::Formatter<'_>,
                    values: &[Type],
                ) -> std::fmt::Result {
                    for (i, value) in values.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{value}")?;
                    }
                    Ok(())
                }

                match self {
                    Self::Named(name) => write!(f, "{name}"),
                    Self::Integer(width) => write!(f, "i{width}"),
                    Self::Index => write!(f, "index"),
                    Self::FunctionType { inputs, results } => {
                        write!(f, "(")?;
                        write_list(f, inputs)?;
                        write!(f, ") -> ")?;
                        match results.as_slice() {
                            [one] => write!(f, "{one}"),
                            many => {
                                write!(f, "(")?;
                                write_list(f, many)?;
                                write!(f, ")")
                            }
                        }
                    }
                    Self::Int { width, is_signed, is_bit_int } => {
                        write!(f, "{}{width}", if *is_signed { "s" } else { "u" })?;
                        if *is_bit_int {
                            write!(f, "_bitint")?;
                        }
                        Ok(())
                    }
                    Self::LongDouble { underlying } => write!(f, "long_double<{underlying}>"),
                    Self::Pointer { pointee, addr_space } => {
                        write!(f, "{pointee}*")?;
                        if let Some(raw) = addr_space {
                            write!(f, " {raw}")?;
                        }
                        Ok(())
                    }
                    Self::Array { element_type, size } => write!(f, "{element_type}[{size}]"),
                    Self::Vector { element_type, size, .. } => {
                        write!(f, "vector<{size} x {element_type}>")
                    }
                    Self::Func { inputs, optional_return_type, var_arg } => {
                        write!(f, "(")?;
                        write_list(f, inputs)?;
                        if *var_arg {
                            if !inputs.is_empty() {
                                write!(f, ", ")?;
                            }
                            write!(f, "...")?;
                        }
                        write!(f, ") -> ")?;
                        match optional_return_type {
                            Some(ty) => write!(f, "{ty}"),
                            None => write!(f, "void"),
                        }
                    }
                    Self::Struct { name, .. } => {
                        write!(f, "struct {}", name.as_deref().unwrap_or("<anon>"))
                    }
                    Self::Union { name, .. } => {
                        write!(f, "union {}", name.as_deref().unwrap_or("<anon>"))
                    }
                    Self::Complex { element_type } => write!(f, "complex<{element_type}>"),
                    Self::DataMember { member_ty, class_ty } => {
                        write!(f, "data_member<{member_ty} in {class_ty}>")
                    }
                    Self::Method { member_func_ty, class_ty } => {
                        write!(f, "method<{member_func_ty} in {class_ty}>")
                    }
                    Self::Dialect { dialect, mnemonic, raw } => {
                        write!(f, "{dialect}.{mnemonic}")?;
                        if let Some(raw) = raw {
                            write!(f, "<{raw}>")?;
                        }
                        Ok(())
                    }
                    #fallback_arms
                }
            }
        }
    })
}

fn generate_type_variant(rec: Record<'_>) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let name = rec.name()?;
    let mnemonic = rec.str_value("mnemonic").unwrap_or(name);
    let variant = variant_ident(rec, "Type")?;
    let doc = record_doc(rec, Some(&format!("`!cir.{mnemonic}`")))?;

    let mut fields = Vec::new();
    if rec.has_field("parameters") {
        let dag = rec.dag_value("parameters")?;
        for (arg_name, init) in dag.args() {
            let Some(name) = arg_name else { continue };
            let field = safe_field_ident(name);
            let ty = type_param_type_tokens(init)?;
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

fn type_param_type_tokens(init: TypedInit<'_>) -> Result<TokenStream, Box<dyn std::error::Error>> {
    match init {
        TypedInit::String(cpp) => Ok(cpp_type_tokens(Module::Types, cpp.to_str()?, false, false)),
        TypedInit::Def(def) => {
            let rec: Record = def.into();
            let classes = record_classes(rec)?;
            if classes.iter().any(|c| c == "OptionalParameter") {
                let cpp = parameter_cpp_type(rec).unwrap_or_else(|| "mlir::Type".into());
                Ok(cpp_type_tokens(Module::Types, &cpp, false, true))
            } else if classes.iter().any(|c| c == "OptionalArrayRefParameter") {
                let cpp = parameter_cpp_type(rec).unwrap_or_else(|| "mlir::Type".into());
                Ok(cpp_type_tokens(Module::Types, &cpp, false, true))
            } else if classes.iter().any(|c| c == "ArrayRefParameter") {
                let cpp = parameter_cpp_type(rec).unwrap_or_else(|| "mlir::Type".into());
                Ok(cpp_type_tokens(Module::Types, &cpp, false, false))
            } else if classes.iter().any(|c| c == "DefaultValuedParameter") {
                let cpp = parameter_cpp_type(rec).unwrap_or_else(|| "bool".into());
                Ok(cpp_type_tokens(Module::Types, &cpp, false, false))
            } else {
                Ok(quote!(Box<Type>))
            }
        }
        _ => Ok(type_token(Module::Types, "String")),
    }
}
