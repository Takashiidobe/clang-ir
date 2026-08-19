use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;
use tblgen::{Record, init::TypedInit};

use crate::enums::enum_type_ident;

#[derive(Clone, Copy)]
pub(crate) enum Module {
    Types,
    Attrs,
    Ops,
}
pub(crate) fn variant_ident(
    rec: Record<'_>,
    suffix: &str,
) -> Result<Ident, Box<dyn std::error::Error>> {
    Ok(safe_variant_ident(&variant_base_name(rec, suffix)?))
}

pub(crate) fn variant_base_name(
    rec: Record<'_>,
    suffix: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(cpp) = rec.str_value("cppClassName")
        && !cpp.is_empty()
    {
        let base = cpp.strip_suffix(suffix).unwrap_or(cpp);
        return Ok(pascal(base));
    }
    let name = rec.name()?;
    let base = name
        .strip_prefix("CIR_")
        .unwrap_or(name)
        .strip_suffix(suffix)
        .unwrap_or_else(|| name.strip_prefix("CIR_").unwrap_or(name));
    Ok(pascal(base))
}

pub(crate) fn op_struct_ident(mnemonic: &str) -> Ident {
    safe_variant_ident(&mnemonic.replace('.', "_"))
}

pub(crate) fn record_classes(rec: Record<'_>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut classes = Vec::new();
    for i in 0..rec.num_type_classes() {
        if let Some(class) = rec.type_class(i) {
            classes.push(class.name()?.to_string());
        }
    }
    Ok(classes)
}

pub(crate) fn init_is_class(init: TypedInit<'_>, class_names: &[&str]) -> bool {
    let TypedInit::Def(def) = init else {
        return false;
    };
    let rec: Record = def.into();
    record_classes(rec)
        .ok()
        .is_some_and(|classes| classes.iter().any(|c| class_names.contains(&c.as_str())))
}

pub(crate) fn is_type_constraint(classes: &[String]) -> bool {
    classes.iter().any(|c| {
        matches!(
            c.as_str(),
            "CIR_Type"
                | "CIR_TypeBase"
                | "AnyTypeOf"
                | "BuildableType"
                | "Type"
                | "CIR_ConfinedType"
                | "CIR_PtrTo"
                | "CIR_PtrToType"
                | "CIR_SInt"
                | "CIR_UInt"
                | "CIR_SIntOfWidths"
                | "CIR_UIntOfWidths"
                | "CIR_ScalarOrVectorOf"
                | "CIR_VectorTypeOf"
        )
    })
}

pub(crate) fn is_raw_enum(classes: &[String]) -> bool {
    classes.iter().any(|c| {
        matches!(
            c.as_str(),
            "CIR_I32EnumAttr"
                | "CIR_I64EnumAttr"
                | "CIR_I32BitEnumAttr"
                | "I32EnumAttr"
                | "I64EnumAttr"
                | "I32BitEnumAttr"
        )
    })
}

pub(crate) fn is_unit_attr_record(rec: Record<'_>) -> bool {
    let classes = record_classes(rec).unwrap_or_default();
    classes.iter().any(|c| {
        matches!(
            c.as_str(),
            "UnitAttr" | "UnitProp" | "_cls_UnitProp" | "DefaultValuedProp"
        )
    }) || rec.str_value("returnType").ok() == Some("bool")
}

pub(crate) fn parameter_cpp_type(rec: Record<'_>) -> Option<String> {
    rec.str_value("cppType").ok().map(str::to_string)
}

pub(crate) fn cpp_type_tokens(
    module: Module,
    cpp: &str,
    force_vec: bool,
    force_opt: bool,
) -> TokenStream {
    let (base, mut vec, mut opt) = normalize_cpp_type(cpp);
    vec |= force_vec;
    opt |= force_opt;

    let mut ty = rust_base_type_tokens(module, &base);
    if !vec
        && ((matches!(module, Module::Types) && is_base_type(&base))
            || (matches!(module, Module::Attrs) && is_base_attr(&base)))
    {
        ty = quote!(Box<#ty>);
    }
    if vec {
        ty = quote!(Vec<#ty>);
    }
    if opt {
        ty = quote!(Option<#ty>);
    }
    ty
}

pub(crate) fn is_base_type(base: &str) -> bool {
    matches!(
        base,
        "mlir::Type"
            | "cir::RecordType"
            | "cir::FuncType"
            | "cir::IntType"
            | "cir::BoolType"
            | "cir::PointerType"
            | "cir::ComplexType"
            | "cir::StructType"
            | "cir::UnionType"
            | "cir::DataMemberType"
            | "cir::MethodType"
            | "cir::VoidType"
            | "cir::ArrayType"
            | "cir::VectorType"
    )
}

pub(crate) fn is_base_attr(base: &str) -> bool {
    matches!(
        base,
        "mlir::Attribute" | "mlir::ArrayAttr" | "mlir::IntegerAttr"
    )
}

pub(crate) fn normalize_cpp_type(cpp: &str) -> (String, bool, bool) {
    let mut s = cpp.trim().to_string();
    let mut vec = false;
    let mut opt = false;
    loop {
        let before = s.len();
        if let Some(rest) = s.strip_prefix("::") {
            s = rest.trim().to_string();
        }
        if let Some(rest) = s.strip_prefix("const ") {
            s = rest.trim().to_string();
        }
        if let Some(rest) = s.strip_suffix('&') {
            s = rest.trim().to_string();
        }
        if let Some(inner) = strip_template("llvm::ArrayRef", &s)
            .or_else(|| strip_template("llvm::SmallVector", &s))
            .or_else(|| strip_template("ArrayRef", &s))
        {
            s = inner;
            vec = true;
        }
        if let Some(inner) =
            strip_template("std::optional", &s).or_else(|| strip_template("Optional", &s))
        {
            s = inner;
            opt = true;
        }
        if s.len() == before {
            break;
        }
    }
    (s, vec, opt)
}

pub(crate) fn strip_template(prefix: &str, s: &str) -> Option<String> {
    let rest = s.strip_prefix(prefix)?;
    let rest = rest.trim_start();
    if !rest.starts_with('<') {
        return None;
    }
    let mut depth = 0usize;
    let content_start = 1;
    for (idx, ch) in rest.char_indices().skip(1) {
        match ch {
            '<' => depth += 1,
            '>' => {
                if depth == 0 {
                    return Some(rest[content_start..idx].trim().to_string());
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    None
}

pub(crate) fn rust_base_type_tokens(module: Module, base: &str) -> TokenStream {
    let base = base.trim();
    match base {
        "bool" => quote!(bool),
        "unsigned" | "uint32_t" => quote!(u32),
        "uint64_t" => quote!(u64),
        "int" | "int32_t" => quote!(i32),
        "int64_t" => quote!(i64),
        "std::string"
        | "mlir::StringAttr"
        | "::mlir::StringAttr"
        | "mlir::FlatSymbolRefAttr"
        | "mlir::SymbolRefAttr" => quote!(String),
        "mlir::Type"
        | "cir::RecordType"
        | "cir::FuncType"
        | "cir::IntType"
        | "cir::BoolType"
        | "cir::PointerType"
        | "cir::ComplexType"
        | "cir::StructType"
        | "cir::UnionType"
        | "cir::DataMemberType"
        | "cir::MethodType"
        | "cir::VoidType"
        | "cir::ArrayType"
        | "cir::VectorType" => type_token(module, "Type"),
        "mlir::Attribute" | "mlir::ArrayAttr" | "mlir::IntegerAttr" => {
            if matches!(module, Module::Types) {
                quote!(String)
            } else {
                attr_token(module, "Attribute")
            }
        }
        "llvm::APInt" | "llvm::APFloat" | "const clang::VarDecl *" => quote!(String),
        "mlir::ptr::MemorySpaceAttrInterface" => quote!(String),
        "cir::RecordMemberKind" => {
            let ident = enum_type_ident("RecordMemberKind");
            quote!(crate::enums::#ident)
        }
        _ => {
            if let Some(last) = base.rsplit("::").next()
                && (last.ends_with("Kind")
                    || last.ends_with("KindAttr")
                    || last.ends_with("DeviceVarKind")
                    || last == "CUDADeviceVarKind")
            {
                let ident = enum_type_ident(last.trim_end_matches("Attr"));
                return quote!(crate::enums::#ident);
            }
            quote!(String)
        }
    }
}

pub(crate) fn type_token(module: Module, _name: &str) -> TokenStream {
    match module {
        Module::Types => quote!(Type),
        Module::Attrs | Module::Ops => quote!(crate::types::Type),
    }
}

pub(crate) fn attr_token(module: Module, _name: &str) -> TokenStream {
    match module {
        Module::Attrs => quote!(Attribute),
        Module::Ops => quote!(crate::attrs::Attribute),
        Module::Types => quote!(String),
    }
}

pub(crate) fn string_token() -> TokenStream {
    quote!(String)
}

pub(crate) fn safe_field_ident(name: &str) -> Ident {
    let name = name.to_snake_case();
    let name = if name == "type" {
        "ty".to_string()
    } else {
        name
    };
    if syn::parse_str::<syn::Ident>(&name).is_err() {
        format_ident!("{name}_")
    } else {
        format_ident!("{name}")
    }
}

pub(crate) fn safe_variant_ident(name: &str) -> Ident {
    let name = pascal(name);
    if syn::parse_str::<syn::Ident>(&name).is_err() {
        format_ident!("{name}_")
    } else {
        format_ident!("{name}")
    }
}

pub(crate) fn pascal_ident(name: &str) -> Ident {
    safe_variant_ident(name)
}

pub(crate) fn pascal(name: &str) -> String {
    name.to_upper_camel_case()
}

pub(crate) fn record_doc(
    rec: Record<'_>,
    mnemonic: Option<&str>,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let mut lines = Vec::new();
    if let Some(m) = mnemonic {
        lines.push(m.to_string());
    }
    if let Ok(summary) = rec.str_value("summary")
        && !summary.is_empty()
    {
        lines.push(summary.to_string());
    }
    if let Ok(description) = rec.str_value("description") {
        let description = dedent_description(description);
        for line in description.lines() {
            lines.push(line.to_string());
        }
    }
    Ok(doc_attrs(&lines))
}

pub(crate) fn dedent_description(description: &str) -> String {
    let lines = description.lines().collect::<Vec<_>>();
    let common = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    lines
        .iter()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                line.chars().skip(common).collect()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn init_doc(init: TypedInit<'_>) -> TokenStream {
    if let TypedInit::Def(def) = init {
        let rec: Record = def.into();
        if let Ok(summary) = rec.str_value("summary")
            && !summary.is_empty()
        {
            return doc_attrs(&[summary.to_string()]);
        }
    }
    quote!()
}

pub(crate) fn doc_attrs(lines: &[String]) -> TokenStream {
    let attrs = lines.iter().map(|line| {
        if line.is_empty() {
            quote!(#[doc = ""])
        } else {
            let padded = format!(" {line}");
            quote!(#[doc = #padded])
        }
    });
    quote!(#(#attrs)*)
}
