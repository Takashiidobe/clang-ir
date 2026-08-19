use proc_macro2::TokenStream;
use quote::quote;
use tblgen::{Record, RecordKeeper, init::TypedInit};

use crate::common::*;

pub(crate) fn generate_types(
    keeper: &RecordKeeper<'_>,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let variants = keeper
        .all_derived_definitions("CIR_Type")
        .map(generate_type_variant)
        .collect::<Result<Vec<_>, _>>()?;
    let display_impl = type_display_impl();

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

        #display_impl
    })
}

fn type_display_impl() -> TokenStream {
    quote! {
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
                    Self::Bool => write!(f, "bool"),
                    Self::Void => write!(f, "void"),
                    Self::Single => write!(f, "float"),
                    Self::Double => write!(f, "double"),
                    Self::Fp16 => write!(f, "f16"),
                    Self::Bf16 => write!(f, "bf16"),
                    Self::Fp80 => write!(f, "f80"),
                    Self::Fp128 => write!(f, "f128"),
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
                    Self::VPtr => write!(f, "vptr"),
                    Self::EhToken => write!(f, "eh_token"),
                    Self::CleanupToken => write!(f, "cleanup_token"),
                    Self::CatchToken => write!(f, "catch_token"),
                    Self::Dialect { dialect, mnemonic, raw } => {
                        write!(f, "{dialect}.{mnemonic}")?;
                        if let Some(raw) = raw {
                            write!(f, "<{raw}>")?;
                        }
                        Ok(())
                    }
                }
            }
        }
    }
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
