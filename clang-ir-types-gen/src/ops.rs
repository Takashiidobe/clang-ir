use std::{collections::BTreeMap, fs, path::Path};

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Index};
use tblgen::{Record, RecordKeeper, init::TypedInit};

use crate::common::*;
use crate::enums::{enum_type_ident, enum_type_name, enum_type_tokens};
use crate::io::write_rust_file;

pub(crate) struct OpFieldInit {
    field: Ident,
    value: TokenStream,
}

pub(crate) struct OpEnumEntry {
    mnemonic: String,
    doc: TokenStream,
    variant: Ident,
    module: &'static str,
    struct_ident: Ident,
    field_inits: Vec<OpFieldInit>,
    display_arm: TokenStream,
}

pub(crate) fn collect_ops(
    keeper: &RecordKeeper<'_>,
) -> Result<(BTreeMap<&'static str, Vec<TokenStream>>, Vec<OpEnumEntry>), Box<dyn std::error::Error>>
{
    let mut ops = keeper.all_derived_definitions("CIR_Op").collect::<Vec<_>>();
    ops.sort_by_key(|rec| {
        rec.str_value("opName")
            .unwrap_or_else(|_| rec.name().unwrap_or(""))
            .to_string()
    });

    let mut modules = BTreeMap::new();
    let mut variants = Vec::new();

    for rec in ops {
        let name = rec.name()?;
        let mnemonic = rec.str_value("opName").unwrap_or(name).to_string();
        let module = op_module(&mnemonic);
        let struct_ident = op_struct_ident(&mnemonic);
        let (struct_tokens, field_inits) = generate_op_struct(rec, &mnemonic, &struct_ident)?;
        let doc = record_doc(rec, Some(&format!("`cir.{mnemonic}`")))?;
        let display_arm = generate_op_display_arm(rec, &mnemonic, &struct_ident, module)?;

        modules
            .entry(module)
            .or_insert_with(Vec::new)
            .push(struct_tokens);
        variants.push(OpEnumEntry {
            mnemonic,
            doc,
            variant: struct_ident.clone(),
            module,
            struct_ident,
            field_inits,
            display_arm,
        });
    }

    variants.sort_by(|a, b| a.mnemonic.cmp(&b.mnemonic));
    Ok((modules, variants))
}

pub(crate) fn write_ops(
    ops_dir: &Path,
    modules: BTreeMap<&'static str, Vec<TokenStream>>,
    variants: Vec<OpEnumEntry>,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(ops_dir)?;

    let mod_tokens = generate_ops_mod(&modules, &variants);
    write_rust_file(
        &ops_dir.join("mod.rs"),
        mod_tokens,
        "Typed CIR operations, split by operation category.",
    )?;

    for (module, tokens) in modules {
        let file_tokens = quote! {
            #(#tokens)*
        };
        write_rust_file(
            &ops_dir.join(format!("{module}.rs")),
            file_tokens,
            module_file_doc(module),
        )?;
    }

    Ok(())
}

fn generate_ops_mod(
    modules: &BTreeMap<&'static str, Vec<TokenStream>>,
    variants: &[OpEnumEntry],
) -> TokenStream {
    let module_decls = modules
        .keys()
        .map(|name| format_ident!("{name}"))
        .collect::<Vec<_>>();
    let module_uses = module_decls.clone();
    let enum_variants = variants.iter().map(|entry| {
        let doc = &entry.doc;
        let variant = &entry.variant;
        let module = format_ident!("{}", entry.module);
        let struct_ident = &entry.struct_ident;
        quote! {
            #doc
            #variant(#module::#struct_ident),
        }
    });
    let lowering_arms = variants.iter().map(|entry| {
        let mnemonic = &entry.mnemonic;
        let lower_fn = format_ident!("lower_{}", mnemonic.to_snake_case());
        quote! {
            #mnemonic => #lower_fn(op),
        }
    });
    let lowering_functions = variants.iter().map(|entry| {
        let variant = &entry.variant;
        let module = format_ident!("{}", entry.module);
        let struct_ident = &entry.struct_ident;
        let lower_fn = format_ident!("lower_{}", entry.mnemonic.to_snake_case());
        let fields = entry.field_inits.iter().map(|field| {
            let name = &field.field;
            let value = &field.value;
            quote! { #name: #value }
        });
        quote! {
            fn #lower_fn(op: &crate::ast::Operation) -> Option<Op> {
                let mut __operand_index = 0usize;
                Some(Op::#variant(#module::#struct_ident {
                    #(#fields,)*
                }))
            }
        }
    });
    let display_arms = variants.iter().map(|entry| &entry.display_arm);
    let helpers = lowering_helpers();

    quote! {
        #![allow(non_camel_case_types)]

        pub type ValueId = String;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct Region {
            pub blocks: Vec<Block>,
        }

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct Block {
            pub label: Option<String>,
            pub args: Vec<(ValueId, crate::types::Type)>,
            pub ops: Vec<Op>,
        }

        #(pub mod #module_decls;)*
        #(pub use #module_uses::*;)*

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum Op {
            #(#enum_variants)*
            Other(crate::ast::Operation),
        }

        impl Op {
            /// Builds the typed operation for `op.mnemonic()`, or returns
            /// `None` when this op is not a known CIR op or its generic form
            /// doesn't match the generated schema.
            pub fn from_operation(op: &crate::ast::Operation) -> Option<Self> {
                match op.mnemonic() {
                    #(#lowering_arms)*
                    _ => None,
                }
            }
        }

        #(#lowering_functions)*

        pub fn lower_op(op: &crate::ast::Operation) -> Op {
            Op::from_operation(op).unwrap_or_else(|| Op::Other(op.clone()))
        }

        pub fn lower_block(block: &crate::ast::Block) -> Block {
            Block {
                label: block.label.clone(),
                args: block.args.clone(),
                ops: block.ops.iter().map(lower_op).collect(),
            }
        }

        pub fn lower_region(region: &crate::ast::Region) -> Region {
            Region {
                blocks: region.blocks.iter().map(lower_block).collect(),
            }
        }

        pub fn write_indent(
            f: &mut std::fmt::Formatter<'_>,
            level: usize,
        ) -> std::fmt::Result {
            for _ in 0..level {
                write!(f, "    ")?;
            }
            Ok(())
        }

        pub fn write_value_list(
            f: &mut std::fmt::Formatter<'_>,
            ids: &[ValueId],
        ) -> std::fmt::Result {
            for (i, id) in ids.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "%{id}")?;
            }
            Ok(())
        }

        pub fn write_flags(
            f: &mut std::fmt::Formatter<'_>,
            flags: &[(&str, bool)],
        ) -> std::fmt::Result {
            let set: Vec<&str> = flags
                .iter()
                .filter(|(_, v)| *v)
                .map(|(name, _)| *name)
                .collect();
            if !set.is_empty() {
                write!(f, " [{}]", set.join(", "))?;
            }
            Ok(())
        }

        pub fn write_block(
            f: &mut std::fmt::Formatter<'_>,
            block: &Block,
            level: usize,
        ) -> std::fmt::Result {
            if let Some(label) = &block.label {
                write_indent(f, level)?;
                write!(f, "^{label}")?;
                if !block.args.is_empty() {
                    write!(f, "(")?;
                    for (i, (id, ty)) in block.args.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "%{id}: {ty}")?;
                    }
                    write!(f, ")")?;
                }
                writeln!(f, ":")?;
            }
            for op in &block.ops {
                write_op(f, op, level + 1)?;
            }
            Ok(())
        }

        pub fn write_region(
            f: &mut std::fmt::Formatter<'_>,
            region: &Region,
            level: usize,
        ) -> std::fmt::Result {
            for block in &region.blocks {
                write_block(f, block, level)?;
            }
            Ok(())
        }

        pub fn write_op(
            f: &mut std::fmt::Formatter<'_>,
            op: &Op,
            level: usize,
        ) -> std::fmt::Result {
            match op {
                #(#display_arms)*
                Op::Other(raw) => {
                    write_indent(f, level)?;
                    writeln!(f, "<unmodeled: {}>", raw.name)
                }
            }
        }

        #helpers

        impl std::fmt::Display for Op {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write_op(f, self, 0)
            }
        }
    }
}

fn lowering_helpers() -> TokenStream {
    quote! {
        fn op_attr<'a>(
            op: &'a crate::ast::Operation,
            key: &str,
        ) -> Option<&'a crate::attrs::Attribute> {
            op.attr(key)
        }

        fn unit_attr(op: &crate::ast::Operation, key: &str) -> bool {
            op.attr(key)
                .is_some_and(|a| !matches!(a, crate::attrs::Attribute::Bool(false)))
        }

        fn attr_str(op: &crate::ast::Operation, key: &str) -> Option<String> {
            op.attr(key).and_then(|a| a.as_str().map(str::to_string))
        }

        fn attr_type(op: &crate::ast::Operation, key: &str) -> Option<crate::types::Type> {
            op.attr(key).and_then(|a| a.as_type().cloned())
        }

        #[allow(dead_code)]
        fn attr_i32(op: &crate::ast::Operation, key: &str) -> Option<i32> {
            op.attr(key).and_then(|a| a.as_int()).and_then(|v| i32::try_from(v).ok())
        }

        #[allow(dead_code)]
        fn attr_i64(op: &crate::ast::Operation, key: &str) -> Option<i64> {
            op.attr(key).and_then(|a| a.as_int()).and_then(|v| i64::try_from(v).ok())
        }

        fn attr_u64(op: &crate::ast::Operation, key: &str) -> Option<u64> {
            op.attr(key).and_then(|a| a.as_int()).and_then(|v| u64::try_from(v).ok())
        }

        fn dense_i32_array(attr: &crate::attrs::Attribute) -> Option<Vec<usize>> {
            match attr {
                crate::attrs::Attribute::Dialect {
                    dialect,
                    mnemonic,
                    raw: Some(raw),
                    ..
                } if dialect == "builtin" && mnemonic == "array" => {
                    let (_elem_ty, list) = raw.split_once(':')?;
                    list.split(',')
                        .map(|v| v.trim().parse::<usize>().ok())
                        .collect()
                }
                crate::attrs::Attribute::Array(items) => items
                    .iter()
                    .map(|a| a.as_int().and_then(|v| usize::try_from(v).ok()))
                    .collect(),
                _ => None,
            }
        }

        fn operand_segment_sizes(op: &crate::ast::Operation) -> Option<Vec<usize>> {
            op.attr("operandSegmentSizes").and_then(dense_i32_array)
        }

        fn take_operand_group(
            op: &crate::ast::Operation,
            index: &mut usize,
            group_index: usize,
        ) -> Option<Vec<ValueId>> {
            if let Some(sizes) = operand_segment_sizes(op) {
                let size = *sizes.get(group_index)?;
                let end = index.checked_add(size)?;
                let values = op.operands.get(*index..end)?.to_vec();
                *index = end;
                Some(values)
            } else {
                let value = op.operands.get(*index)?.clone();
                *index += 1;
                Some(vec![value])
            }
        }

        fn take_single_operand(
            op: &crate::ast::Operation,
            index: &mut usize,
            group_index: usize,
        ) -> Option<ValueId> {
            let mut values = take_operand_group(op, index, group_index)?;
            (values.len() == 1).then(|| values.remove(0))
        }

        fn take_optional_operand(
            op: &crate::ast::Operation,
            index: &mut usize,
            group_index: usize,
        ) -> Option<ValueId> {
            if let Some(sizes) = operand_segment_sizes(op) {
                let size = *sizes.get(group_index)?;
                match size {
                    0 => None,
                    1 => {
                        let value = op.operands.get(*index)?.clone();
                        *index += 1;
                        Some(value)
                    }
                    _ => None,
                }
            } else if *index < op.operands.len() {
                let value = op.operands.get(*index)?.clone();
                *index += 1;
                Some(value)
            } else {
                None
            }
        }

        fn take_variadic_operand(
            op: &crate::ast::Operation,
            index: &mut usize,
            group_index: usize,
        ) -> Option<Vec<ValueId>> {
            if operand_segment_sizes(op).is_some() {
                take_operand_group(op, index, group_index)
            } else {
                let values = op.operands.get(*index..)?.to_vec();
                *index = op.operands.len();
                Some(values)
            }
        }

        fn take_variadic_of_variadic(
            op: &crate::ast::Operation,
            index: &mut usize,
            segments_key: &str,
        ) -> Option<Vec<Vec<ValueId>>> {
            let sizes = dense_i32_array(op.attr(segments_key)?)?;
            let mut groups = Vec::with_capacity(sizes.len());
            for size in sizes {
                let end = index.checked_add(size)?;
                groups.push(op.operands.get(*index..end)?.to_vec());
                *index = end;
            }
            Some(groups)
        }
    }
}

fn generate_op_struct(
    rec: Record<'_>,
    mnemonic: &str,
    struct_ident: &Ident,
) -> Result<(TokenStream, Vec<OpFieldInit>), Box<dyn std::error::Error>> {
    let doc = record_doc(rec, Some(&format!("`cir.{mnemonic}`")))?;
    let mut fields = Vec::new();
    let mut field_inits = Vec::new();

    if rec.has_field("results") {
        let results = rec.dag_value("results")?;
        for (i, (arg_name, init)) in results.args().enumerate() {
            let Some(name) = arg_name else { continue };
            let is_variadic = init_is_class(init, &["Variadic"]);
            let is_optional = init_is_class(init, &["Optional"]);
            let (value_field, ty_field) = if results.num_args() == 1 && name == "result" {
                (safe_field_ident("result"), safe_field_ident("result_ty"))
            } else {
                (
                    safe_field_ident(name),
                    safe_field_ident(&format!("{name}_ty")),
                )
            };
            let value_ty = if is_variadic {
                quote!(Vec<super::ValueId>)
            } else if is_optional {
                quote!(Option<super::ValueId>)
            } else {
                quote!(super::ValueId)
            };
            let ty_ty = if is_variadic {
                quote!(Vec<crate::types::Type>)
            } else if is_optional {
                quote!(Option<crate::types::Type>)
            } else {
                quote!(crate::types::Type)
            };
            fields.push(quote! {
                pub #value_field: #value_ty,
                pub #ty_field: #ty_ty,
            });
            let index = Index::from(i);
            if is_variadic {
                field_inits.push(OpFieldInit {
                    field: value_field,
                    value: quote! { op.results.iter().map(|(id, _)| id.clone()).collect() },
                });
                field_inits.push(OpFieldInit {
                    field: ty_field,
                    value: quote! { op.results.iter().map(|(_, ty)| ty.clone()).collect() },
                });
            } else if is_optional {
                field_inits.push(OpFieldInit {
                    field: value_field,
                    value: quote! { op.results.get(#index).map(|(id, _)| id.clone()) },
                });
                field_inits.push(OpFieldInit {
                    field: ty_field,
                    value: quote! { op.results.get(#index).map(|(_, ty)| ty.clone()) },
                });
            } else {
                field_inits.push(OpFieldInit {
                    field: value_field,
                    value: quote! { op.results.get(#index)?.0.clone() },
                });
                field_inits.push(OpFieldInit {
                    field: ty_field,
                    value: quote! { op.results.get(#index)?.1.clone() },
                });
            }
        }
    }

    if rec.has_field("arguments") {
        let args = rec.dag_value("arguments")?;
        let mut operand_group = 0usize;
        for (arg_name, init) in args.args() {
            let Some(name) = arg_name else { continue };
            let field = safe_field_ident(name);
            let ty = op_arg_type_tokens(init)?;
            let doc = init_doc(init);
            let (value, is_operand) = op_arg_initializer(init, name, mnemonic, operand_group)?;
            if is_operand {
                operand_group += 1;
            }
            fields.push(quote! {
                #doc
                pub #field: #ty,
            });
            field_inits.push(OpFieldInit { field, value });
        }
    }

    if rec.has_field("regions") {
        let regions = rec.dag_value("regions")?;
        for (i, (arg_name, init)) in regions.args().enumerate() {
            let Some(name) = arg_name else { continue };
            let field = safe_field_ident(name);
            let is_variadic = init_is_class(init, &["Variadic", "VariadicRegion"]);
            let ty = if is_variadic {
                quote!(Vec<super::Region>)
            } else {
                quote!(super::Region)
            };
            fields.push(quote! {
                pub #field: #ty,
            });
            let index = Index::from(i);
            let value = if is_variadic {
                quote! { op.regions.iter().skip(#index).map(lower_region).collect() }
            } else {
                quote! { lower_region(op.regions.get(#index)?) }
            };
            field_inits.push(OpFieldInit { field, value });
        }
    }

    if rec
        .dag_value("successors")
        .is_ok_and(|successors| successors.num_args() > 0)
    {
        fields.push(quote! {
            pub successors: Vec<String>,
        });
        field_inits.push(OpFieldInit {
            field: safe_field_ident("successors"),
            value: quote! { op.successors.clone() },
        });
    }

    fields.push(quote! {
        pub loc: Option<crate::ast::SourceLocation>,
    });
    field_inits.push(OpFieldInit {
        field: safe_field_ident("loc"),
        value: quote! { op.loc.clone() },
    });

    Ok((
        quote! {
            #doc
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #struct_ident {
                #(#fields)*
            }
        },
        field_inits,
    ))
}

fn generate_op_display_arm(
    _rec: Record<'_>,
    mnemonic: &str,
    struct_ident: &Ident,
    module: &'static str,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let variant = struct_ident;
    let module_ident = format_ident!("{module}");
    let mnemonic_lit = mnemonic.to_string();

    let arm = match mnemonic {
        "alloca" => quote! {
            Op::#variant(#module_ident::#struct_ident {
                addr,
                addr_ty,
                dyn_alloc_size,
                name,
                alignment,
                ..
            }) => {
                write_indent(f, level)?;
                write!(f, "%{addr} = alloca ")?;
                if let crate::types::Type::Pointer { pointee, .. } = addr_ty {
                    write!(f, "{pointee}")?;
                } else {
                    write!(f, "{addr_ty}")?;
                }
                write!(f, ", {name:?}")?;
                if let Some(size) = dyn_alloc_size {
                    write!(f, ", size(%{size})")?;
                }
                write!(f, ", align {alignment}")?;
                writeln!(f)
            }
        },
        "store" => quote! {
            Op::#variant(#module_ident::#struct_ident {
                value,
                addr,
                alignment,
                ..
            }) => {
                write_indent(f, level)?;
                write!(f, "store %{value}, %{addr}")?;
                if let Some(a) = alignment {
                    if let Some(v) = a.as_int() {
                        write!(f, ", align {v}")?;
                    }
                }
                writeln!(f)
            }
        },
        "load" => quote! {
            Op::#variant(#module_ident::#struct_ident {
                result,
                result_ty,
                addr,
                alignment,
                ..
            }) => {
                write_indent(f, level)?;
                write!(f, "%{result} = load %{addr} : {result_ty}")?;
                if let Some(a) = alignment {
                    if let Some(v) = a.as_int() {
                        write!(f, ", align {v}")?;
                    }
                }
                writeln!(f)
            }
        },
        "add" => quote! {
            Op::#variant(#module_ident::#struct_ident {
                result,
                result_ty,
                lhs,
                rhs,
                no_signed_wrap,
                no_unsigned_wrap,
                saturated,
                ..
            }) => {
                write_indent(f, level)?;
                write!(f, "%{result} = add %{lhs}, %{rhs} : {result_ty}")?;
                write_flags(
                    f,
                    &[
                        ("nsw", *no_signed_wrap),
                        ("nuw", *no_unsigned_wrap),
                        ("sat", *saturated),
                    ],
                )?;
                writeln!(f)
            }
        },
        "const" => quote! {
            Op::#variant(#module_ident::#struct_ident { res, res_ty, value, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "%{res} = const {value} : {res_ty}")
            }
        },
        "get_global" => quote! {
            Op::#variant(#module_ident::#struct_ident { addr, addr_ty, name, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "%{addr} = get_global {name} : {addr_ty}")
            }
        },
        "cast" => quote! {
            Op::#variant(#module_ident::#struct_ident { result, result_ty, kind, src, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "%{result} = cast({kind}) %{src} : {result_ty}")
            }
        },
        "call" => quote! {
            Op::#variant(#module_ident::#struct_ident {
                result,
                result_ty,
                callee,
                args,
                side_effect,
                ..
            }) => {
                write_indent(f, level)?;
                if let Some(result) = result {
                    write!(f, "%{result} = ")?;
                }
                write!(f, "call ")?;
                if let Some(callee) = callee {
                    write!(f, "{callee}")?;
                }
                write!(f, "(")?;
                write_value_list(f, args)?;
                write!(f, ")")?;
                if let Some(ty) = result_ty {
                    write!(f, " : {ty}")?;
                }
                if let Ok(se) = crate::enums::SideEffect::try_from(side_effect) {
                    write!(f, " [{se}]")?;
                }
                writeln!(f)
            }
        },
        "return" => quote! {
            Op::#variant(#module_ident::#struct_ident { input, .. }) => {
                write_indent(f, level)?;
                write!(f, "return")?;
                if let Some(v) = input.first() {
                    write!(f, " %{v}")?;
                }
                writeln!(f)
            }
        },
        "yield" => quote! {
            Op::#variant(#module_ident::#struct_ident { args, .. }) => {
                write_indent(f, level)?;
                write!(f, "yield")?;
                if let Some(v) = args.first() {
                    write!(f, " %{v}")?;
                }
                writeln!(f)
            }
        },
        "br" => quote! {
            Op::#variant(#module_ident::#struct_ident { successors, .. }) => {
                write_indent(f, level)?;
                if let Some(dest) = successors.first() {
                    writeln!(f, "br ^{dest}")
                } else {
                    writeln!(f, "br")
                }
            }
        },
        "brcond" => quote! {
            Op::#variant(#module_ident::#struct_ident { cond, successors, .. }) => {
                write_indent(f, level)?;
                write!(f, "brcond %{cond}")?;
                if let Some(dest) = successors.first() {
                    write!(f, ", ^{dest}")?;
                }
                if let Some(dest) = successors.get(1) {
                    write!(f, ", ^{dest}")?;
                }
                writeln!(f)
            }
        },
        "goto" => quote! {
            Op::#variant(#module_ident::#struct_ident { label, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "goto {label}")
            }
        },
        "label" => quote! {
            Op::#variant(#module_ident::#struct_ident { label, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "label {label}:")
            }
        },
        "indirect_goto" => quote! {
            Op::#variant(#module_ident::#struct_ident { addr, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "indirect_goto %{addr}")
            }
        },
        "unreachable" => quote! {
            Op::#variant(#module_ident::#struct_ident { .. }) => {
                write_indent(f, level)?;
                writeln!(f, "unreachable")
            }
        },
        "trap" => quote! {
            Op::#variant(#module_ident::#struct_ident { .. }) => {
                write_indent(f, level)?;
                writeln!(f, "trap")
            }
        },
        "condition" => quote! {
            Op::#variant(#module_ident::#struct_ident { condition, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "condition %{condition}")
            }
        },
        "break" => quote! {
            Op::#variant(#module_ident::#struct_ident { .. }) => {
                write_indent(f, level)?;
                writeln!(f, "break")
            }
        },
        "continue" => quote! {
            Op::#variant(#module_ident::#struct_ident { .. }) => {
                write_indent(f, level)?;
                writeln!(f, "continue")
            }
        },
        "scope" => quote! {
            Op::#variant(#module_ident::#struct_ident { results, scope_region, .. }) => {
                write_indent(f, level)?;
                if let Some(results) = results {
                    write!(f, "%{results} = ")?;
                }
                writeln!(f, "scope {{")?;
                write_region(f, scope_region, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}}")
            }
        },
        "cleanup.scope" => quote! {
            Op::#variant(#module_ident::#struct_ident {
                cleanup_kind,
                body_region,
                cleanup_region,
                ..
            }) => {
                write_indent(f, level)?;
                writeln!(f, "cleanup.scope {{")?;
                write_region(f, body_region, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}} cleanup {cleanup_kind} {{")?;
                write_region(f, cleanup_region, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}}")
            }
        },
        "if" => quote! {
            Op::#variant(#module_ident::#struct_ident {
                condition,
                then_region,
                else_region,
                ..
            }) => {
                write_indent(f, level)?;
                writeln!(f, "if %{condition} {{")?;
                write_region(f, then_region, level + 1)?;
                if !else_region.blocks.iter().all(|b| b.ops.is_empty()) {
                    write_indent(f, level)?;
                    writeln!(f, "}} else {{")?;
                    write_region(f, else_region, level + 1)?;
                }
                write_indent(f, level)?;
                writeln!(f, "}}")
            }
        },
        "while" => quote! {
            Op::#variant(#module_ident::#struct_ident { cond, body, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "while {{")?;
                write_region(f, cond, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}} do {{")?;
                write_region(f, body, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}}")
            }
        },
        "do" => quote! {
            Op::#variant(#module_ident::#struct_ident { body, cond, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "do {{")?;
                write_region(f, body, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}} while {{")?;
                write_region(f, cond, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}}")
            }
        },
        "for" => quote! {
            Op::#variant(#module_ident::#struct_ident { cond, body, step, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "for cond {{")?;
                write_region(f, cond, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}} step {{")?;
                write_region(f, step, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}} body {{")?;
                write_region(f, body, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}}")
            }
        },
        "switch" => quote! {
            Op::#variant(#module_ident::#struct_ident { condition, body, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "switch %{condition} {{")?;
                write_region(f, body, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}}")
            }
        },
        "case" => quote! {
            Op::#variant(#module_ident::#struct_ident { value, kind, case_region, .. }) => {
                write_indent(f, level)?;
                write!(f, "case {kind} {value} {{")?;
                writeln!(f)?;
                write_region(f, case_region, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}}")
            }
        },
        "ternary" => quote! {
            Op::#variant(#module_ident::#struct_ident {
                result,
                result_ty,
                cond,
                true_region,
                false_region,
                ..
            }) => {
                write_indent(f, level)?;
                if let Some(result) = result {
                    write!(f, "%{result} = ")?;
                }
                write!(f, "ternary %{cond} ? {{")?;
                if let Some(ty) = result_ty {
                    write!(f, " : {ty}")?;
                }
                writeln!(f)?;
                write_region(f, true_region, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}} : {{")?;
                write_region(f, false_region, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}}")
            }
        },
        "try" => quote! {
            Op::#variant(#module_ident::#struct_ident {
                cleanup,
                try_region,
                handler_regions,
                ..
            }) => {
                write_indent(f, level)?;
                write!(f, "try")?;
                write_flags(f, &[("cleanup", *cleanup)])?;
                writeln!(f, " {{")?;
                write_region(f, try_region, level + 1)?;
                write_indent(f, level)?;
                writeln!(f, "}}")?;
                for handler in handler_regions {
                    write_indent(f, level)?;
                    writeln!(f, "catch {{")?;
                    write_region(f, handler, level + 1)?;
                    write_indent(f, level)?;
                    writeln!(f, "}}")?;
                }
                Ok(())
            }
        },
        "begin_catch" => quote! {
            Op::#variant(#module_ident::#struct_ident {
                catch_token,
                exn_ptr,
                exn_ptr_ty,
                eh_token,
                ..
            }) => {
                write_indent(f, level)?;
                writeln!(f, "%{catch_token}, %{exn_ptr} = begin_catch %{eh_token} : {exn_ptr_ty}")
            }
        },
        "end_catch" => quote! {
            Op::#variant(#module_ident::#struct_ident { catch_token, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "end_catch %{catch_token}")
            }
        },
        "init_catch_param" => quote! {
            Op::#variant(#module_ident::#struct_ident { exn_ptr, param_addr, kind, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "init_catch_param {kind} %{exn_ptr} to %{param_addr}")
            }
        },
        "resume" => quote! {
            Op::#variant(#module_ident::#struct_ident { eh_token, .. }) => {
                write_indent(f, level)?;
                writeln!(f, "resume %{eh_token}")
            }
        },
        _ => quote! {
            Op::#variant(_) => {
                write_indent(f, level)?;
                writeln!(f, #mnemonic_lit)
            }
        },
    };

    Ok(arm)
}

fn resolve_op_arg(init: TypedInit<'_>) -> Result<TypedInit<'_>, Box<dyn std::error::Error>> {
    let TypedInit::Def(def) = init else {
        return Ok(init);
    };
    let rec: Record = def.into();
    let classes = record_classes(rec)?;
    if classes.iter().any(|c| c == "Arg")
        && let Ok(constraint) = rec.def_value("constraint")
    {
        return resolve_op_arg(TypedInit::Def(constraint.def_init()));
    }
    Ok(TypedInit::Def(rec.def_init()))
}

fn op_arg_initializer(
    init: TypedInit<'_>,
    arg_name: &str,
    mnemonic: &str,
    group_index: usize,
) -> Result<(TokenStream, bool), Box<dyn std::error::Error>> {
    let resolved = resolve_op_arg(init)?;
    let TypedInit::Def(def) = resolved else {
        return Ok((quote! { crate::attrs::Attribute::Unit }, false));
    };
    let rec: Record = def.into();
    let classes = record_classes(rec)?;
    let group_index_tokens = Index::from(group_index);

    if classes.iter().any(|c| c == "VariadicOfVariadic") {
        let segments_key = match (mnemonic, arg_name) {
            ("asm", "asm_operands") => "operands_segments",
            ("switch.flat", "case_operands") => "case_operand_segments",
            ("indirectbr", "succ_operands") => "operand_segments",
            _ => "operand_segments",
        };
        return Ok((
            quote! { take_variadic_of_variadic(op, &mut __operand_index, #segments_key)? },
            true,
        ));
    }

    if classes.iter().any(|c| c == "Variadic") {
        if mnemonic == "call" && arg_name == "args" {
            return Ok((
                quote! {
                    if op.attr("callee").is_some() {
                        take_variadic_operand(op, &mut __operand_index, #group_index_tokens)?
                    } else {
                        let args = op.operands.get(1..).unwrap_or(&[]).to_vec();
                        __operand_index = op.operands.len();
                        args
                    }
                },
                true,
            ));
        }
        return Ok((
            quote! { take_variadic_operand(op, &mut __operand_index, #group_index_tokens)? },
            true,
        ));
    }

    if classes.iter().any(|c| c == "Optional") {
        return Ok((
            quote! { take_optional_operand(op, &mut __operand_index, #group_index_tokens) },
            true,
        ));
    }

    if is_type_constraint(&classes) {
        return Ok((
            quote! { take_single_operand(op, &mut __operand_index, #group_index_tokens)? },
            true,
        ));
    }

    Ok((op_attr_initializer(rec, arg_name)?, false))
}

enum AttrInitKind {
    Unit,
    String,
    I32,
    I64,
    U64,
    Type,
    Attribute,
    Enum(Ident),
    BitEnum(Ident),
}

fn attr_init_kind(rec: Record<'_>) -> Result<AttrInitKind, Box<dyn std::error::Error>> {
    let classes = record_classes(rec)?;

    if is_unit_attr_record(rec) {
        return Ok(AttrInitKind::Unit);
    }

    if classes
        .iter()
        .any(|c| c == "CIR_I32BitEnumAttr" || c == "I32BitEnumAttr")
    {
        return Ok(AttrInitKind::BitEnum(enum_type_ident(&enum_type_name(
            rec,
        )?)));
    }

    if is_raw_enum(&classes)
        || classes
            .iter()
            .any(|c| c == "CIR_EnumAttr" || c == "EnumAttr")
    {
        return Ok(AttrInitKind::Enum(enum_type_ident(&enum_type_name(rec)?)));
    }

    if classes.iter().any(|c| c == "ConfinedAttr") {
        if let Some(base) = rec.optional_def_value("baseAttr")
            && let Ok(base_name) = base.name()
            && (base_name == "I64Attr" || base_name == "IndexAttr")
        {
            return Ok(AttrInitKind::U64);
        }
        return Ok(AttrInitKind::Attribute);
    }

    if classes.iter().any(|c| {
        c == "StrAttr"
            || c == "StringBasedAttr"
            || c == "FlatSymbolRefAttr"
            || c == "SymbolNameAttr"
    }) {
        return Ok(AttrInitKind::String);
    }

    if classes.iter().any(|c| c == "IndexAttr") {
        return Ok(AttrInitKind::I64);
    }
    if classes.iter().any(|c| c == "I64Attr") {
        return Ok(AttrInitKind::I64);
    }
    if classes.iter().any(|c| c == "I32Attr") {
        return Ok(AttrInitKind::I32);
    }

    if classes
        .iter()
        .any(|c| c == "TypeAttr" || c == "TypeAttrBase" || c == "TypeAttrOf")
        || is_type_constraint(&classes)
    {
        return Ok(AttrInitKind::Type);
    }

    Ok(AttrInitKind::Attribute)
}

fn op_attr_initializer(
    rec: Record<'_>,
    arg_name: &str,
) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let classes = record_classes(rec)?;
    let key = arg_name.to_string();

    if classes.iter().any(|c| c == "OptionalAttr") {
        let inner = rec.optional_def_value("baseAttr");
        let kind = match inner {
            Some(inner) => attr_init_kind(inner)?,
            None => AttrInitKind::Attribute,
        };
        return Ok(optional_attr_initializer(kind, &key));
    }

    let kind = attr_init_kind(rec)?;
    Ok(match kind {
        AttrInitKind::Unit => quote! { unit_attr(op, #key) },
        AttrInitKind::String => quote! { attr_str(op, #key)? },
        AttrInitKind::I32 => quote! { attr_i32(op, #key)? },
        AttrInitKind::I64 => quote! { attr_i64(op, #key)? },
        AttrInitKind::U64 => quote! { attr_u64(op, #key)? },
        AttrInitKind::Type => quote! { attr_type(op, #key)? },
        AttrInitKind::Attribute => quote! { op_attr(op, #key)?.clone() },
        AttrInitKind::Enum(ident) => {
            quote! { crate::enums::#ident::try_from(op_attr(op, #key)?).ok()? }
        }
        AttrInitKind::BitEnum(ident) => {
            quote! { crate::enums::#ident(attr_u64(op, #key)?) }
        }
    })
}

fn optional_attr_initializer(kind: AttrInitKind, key: &str) -> TokenStream {
    let key = key.to_string();
    match kind {
        AttrInitKind::Unit => quote! { op.attr(#key).is_some() },
        AttrInitKind::String => {
            quote! { op.attr(#key).and_then(|a| a.as_str().map(str::to_string)) }
        }
        AttrInitKind::I32 => quote! {
            op.attr(#key).and_then(|a| a.as_int()).and_then(|v| i32::try_from(v).ok())
        },
        AttrInitKind::I64 => quote! {
            op.attr(#key).and_then(|a| a.as_int()).and_then(|v| i64::try_from(v).ok())
        },
        AttrInitKind::U64 => quote! {
            op.attr(#key).and_then(|a| a.as_int()).and_then(|v| u64::try_from(v).ok())
        },
        AttrInitKind::Type => quote! {
            op.attr(#key).and_then(|a| a.as_type().cloned())
        },
        AttrInitKind::Attribute => quote! { op.attr(#key).cloned() },
        AttrInitKind::Enum(ident) => quote! {
            op.attr(#key).and_then(|a| crate::enums::#ident::try_from(a).ok())
        },
        AttrInitKind::BitEnum(ident) => quote! {
            op.attr(#key).and_then(|a| a.as_int()).and_then(|v| u64::try_from(v).ok()).map(crate::enums::#ident)
        },
    }
}

fn op_arg_type_tokens(init: TypedInit<'_>) -> Result<TokenStream, Box<dyn std::error::Error>> {
    match init {
        TypedInit::Def(def) => {
            let rec: Record = def.into();
            let classes = record_classes(rec)?;
            if classes.iter().any(|c| c == "Arg")
                && let Ok(constraint) = rec.def_value("constraint")
            {
                return op_arg_type_tokens(TypedInit::Def(constraint.def_init()));
            }
            if classes.iter().any(|c| c == "VariadicOfVariadic") {
                return Ok(quote!(Vec<Vec<super::ValueId>>));
            }
            if classes.iter().any(|c| c == "Variadic") {
                return Ok(quote!(Vec<super::ValueId>));
            }
            if classes.iter().any(|c| c == "Optional") {
                return Ok(quote!(Option<super::ValueId>));
            }
            if is_unit_attr_record(rec) {
                return Ok(quote!(bool));
            }
            Ok(op_attr_type_tokens(rec)?)
        }
        TypedInit::String(cpp) => Ok(cpp_type_tokens(Module::Ops, cpp.to_str()?, false, false)),
        _ => Ok(quote!(crate::attrs::Attribute)),
    }
}

fn op_attr_type_tokens(rec: Record<'_>) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let classes = record_classes(rec)?;

    if classes.iter().any(|c| c == "OptionalAttr") {
        let inner = if let Some(base) = rec.optional_def_value("baseAttr") {
            op_attr_type_tokens(base)?
        } else {
            quote!(crate::attrs::Attribute)
        };
        return Ok(quote!(Option<#inner>));
    }

    if is_raw_enum(&classes) {
        return enum_type_tokens(rec);
    }

    if classes
        .iter()
        .any(|c| c == "CIR_EnumAttr" || c == "EnumAttr")
    {
        return enum_type_tokens(rec);
    }

    if classes.iter().any(|c| c == "ConfinedAttr") {
        if let Some(base) = rec.optional_def_value("baseAttr")
            && let Ok(base_name) = base.name()
            && (base_name == "I64Attr" || base_name == "IndexAttr")
        {
            return Ok(quote!(u64));
        }
        return Ok(quote!(crate::attrs::Attribute));
    }

    if classes.iter().any(|c| {
        c == "StrAttr"
            || c == "StringBasedAttr"
            || c == "FlatSymbolRefAttr"
            || c == "SymbolNameAttr"
    }) {
        return Ok(quote!(String));
    }

    if classes.iter().any(|c| c == "IndexAttr") {
        return Ok(quote!(i64));
    }
    if classes.iter().any(|c| c == "I64Attr") {
        return Ok(quote!(i64));
    }
    if classes.iter().any(|c| c == "I32Attr") {
        return Ok(quote!(i32));
    }

    if classes
        .iter()
        .any(|c| c == "TypeAttr" || c == "TypeAttrBase" || c == "TypeAttrOf")
    {
        return Ok(quote!(crate::types::Type));
    }

    if is_type_constraint(&classes) {
        return Ok(quote!(super::ValueId));
    }

    Ok(quote!(crate::attrs::Attribute))
}

fn op_module(mnemonic: &str) -> &'static str {
    if mnemonic.starts_with("atomic.") {
        return "atomics";
    }
    if mnemonic.starts_with("complex.") {
        return "complex";
    }
    if mnemonic.starts_with("vec.") {
        return "vectors";
    }
    if mnemonic.starts_with("vtable.") || mnemonic.starts_with("vtt.") {
        return "vtables";
    }
    if mnemonic.starts_with("va_") {
        return "varargs";
    }
    if mnemonic.starts_with("std.") || mnemonic.starts_with("libc.") {
        return "stdlib";
    }
    if mnemonic.starts_with("call") || mnemonic == "call_llvm_intrinsic" {
        return "calls";
    }
    if mnemonic.starts_with("eh.")
        || matches!(
            mnemonic,
            "try"
                | "throw"
                | "try_throw"
                | "alloc.exception"
                | "begin_catch"
                | "end_catch"
                | "init_catch_param"
                | "construct_catch_param"
                | "catch_param"
                | "begin_cleanup"
                | "end_cleanup"
                | "resume"
                | "resume.flat"
                | "token.none"
        )
    {
        return "exceptions";
    }
    if matches!(
        mnemonic,
        "if" | "while"
            | "do"
            | "for"
            | "switch"
            | "switch.flat"
            | "case"
            | "br"
            | "brcond"
            | "goto"
            | "indirect_goto"
            | "label"
            | "yield"
            | "condition"
            | "break"
            | "continue"
            | "return"
            | "unreachable"
            | "trap"
            | "scope"
            | "cleanup.scope"
            | "ternary"
    ) {
        return "control_flow";
    }
    if matches!(
        mnemonic,
        "alloca"
            | "load"
            | "store"
            | "copy"
            | "set_bitfield"
            | "get_bitfield"
            | "get_member"
            | "extract_member"
            | "insert_member"
            | "get_element"
            | "ptr_stride"
            | "ptr_diff"
            | "clear_padding"
            | "lifetime.start"
            | "lifetime.end"
            | "clear_cache"
            | "base_class_addr"
            | "derived_class_addr"
            | "base_data_member"
            | "derived_data_member"
            | "base_method"
            | "derived_method"
            | "get_runtime_member"
            | "get_method"
    ) {
        return "memory";
    }
    if matches!(
        mnemonic,
        "const" | "get_global" | "global" | "func" | "local_init"
    ) {
        return "globals";
    }
    if mnemonic.starts_with("array.") {
        return "arrays";
    }
    if is_arithmetic_mnemonic(mnemonic) {
        return "arithmetic";
    }
    "misc"
}

fn is_arithmetic_mnemonic(mnemonic: &str) -> bool {
    matches!(
        mnemonic,
        "add"
            | "sub"
            | "mul"
            | "div"
            | "rem"
            | "and"
            | "or"
            | "xor"
            | "shift"
            | "rotate"
            | "cmp"
            | "cmp3way"
            | "select"
            | "freeze"
            | "fneg"
            | "fadd"
            | "fsub"
            | "fmul"
            | "fdiv"
            | "frem"
            | "min"
            | "max"
            | "abs"
            | "sqrt"
            | "acos"
            | "asin"
            | "atan"
            | "atan2"
            | "ceil"
            | "cos"
            | "cosh"
            | "exp"
            | "exp10"
            | "exp2"
            | "log"
            | "log10"
            | "log2"
            | "nearbyint"
            | "rint"
            | "round"
            | "roundeven"
            | "sin"
            | "sinh"
            | "tan"
            | "tanh"
            | "trunc"
            | "fabs"
            | "floor"
            | "lround"
            | "llround"
            | "lrint"
            | "llrint"
            | "copysign"
            | "fmaxnum"
            | "fmaximum"
            | "fminnum"
            | "fminimum"
            | "fmod"
            | "pow"
            | "frexp"
            | "modf"
            | "fma"
            | "fmuladd"
            | "is_fp_class"
            | "is_constant"
            | "objsize"
    ) || mnemonic.starts_with("bit.")
        || mnemonic == "byte_swap"
}

fn module_file_doc(module: &str) -> &'static str {
    match module {
        "arithmetic" => "Arithmetic, comparison, and bit-manipulation operations.",
        "atomics" => "Atomic operations.",
        "arrays" => "Array construction and destruction operations.",
        "calls" => "Call-like operations.",
        "complex" => "Complex-number operations.",
        "control_flow" => "Structured and block-level control-flow operations.",
        "exceptions" => "Exception and cleanup operations.",
        "globals" => "Global, function, and constant declaration operations.",
        "memory" => "Memory and aggregate access operations.",
        "misc" => "Operations without a more specific category.",
        "stdlib" => "Standard-library operations.",
        "varargs" => "C varargs operations.",
        "vectors" => "Vector operations.",
        "vtables" => "Vtable and RTTI operations.",
        _ => "CIR operations.",
    }
}
