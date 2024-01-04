#![allow(dead_code)]
#![allow(clippy::unit_arg)]

mod patterns;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Attribute, Block, Error, Expr, ExprClosure, ExprLit, ExprRange,
    Ident, Item, ItemFn, ItemMod, Lit, LitStr, Local, LocalInit, Meta, MetaList, MetaNameValue,
    Pat, PatIdent, PatType, RangeLimits, ReturnType, Signature, Stmt, Token, Type, Visibility,
};

macro_rules! err {
    ($tokens:expr, $message:expr) => {
        Err(Error::new($tokens.span(), $message))
    };
}

#[inline]
fn meta_is_empty(meta: &Meta) -> bool {
    matches!(meta, Meta::List(MetaList { tokens, .. }) if tokens.is_empty())
        || matches!(meta, Meta::Path(_))
}

//
// Code related to parsing starts here.
//

#[proc_macro_attribute]
pub fn sysfs(args: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let args = parse_macro_input!(args as SysfsAttrArgs);
    let item = parse_macro_input!(item as ItemSysfsAttrFn);

    match sysfs_attr(&args, item) {
        Ok(item) => item.into_token_stream().into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn sysfs_attrs(args: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let args = parse_macro_input!(args as SysfsModArgs);
    let mut item = parse_macro_input!(item as ItemMod);

    match args.apply(&mut item) {
        Ok(()) => item.to_token_stream().into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[derive(Clone, Default)]
struct SysfsAttrArgs {
    sysfs_dir: Option<LitStr>,
}

#[derive(Clone)]
struct ItemSysfsAttrFn {
    attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    let_read: Option<Local>,
    let_write: Option<Local>,
    dots: Token![..],
    block: Box<Block>,
}

#[derive(Clone)]
struct SysfsModArgs {
    sysfs_dir: LitStr,
}

impl SysfsModArgs {
    fn apply(&self, item: &mut ItemMod) -> syn::Result<()> {
        // Take a mutable reference to the items vector.
        let Some((_, items)) = &mut item.content else {
            return err!(item.span(), "this item must have braced content");
        };

        for item in items.iter_mut() {
            // We do not care about anything besides functions with the `sysfs`
            // attribute.
            let Item::Fn(ItemFn { attrs, .. }) = item else {
                continue;
            };

            // Now check for the attribute.
            for attr in attrs.iter_mut() {
                if attr.path().is_ident("sysfs") {
                    // We must avoid parsing the meta if there are no
                    // parenthesis. This causes syntax errors even though
                    // `SysfsAttrsArgs` already handles this case.
                    if meta_is_empty(&attr.meta) {
                        let args = SysfsAttrArgs::try_from(self.clone())?;
                        attr.meta = parse_quote! { sysfs(#args) };
                        break;
                    }

                    // Parse the existing `sysfs` arguments.
                    let mut attr_args: SysfsAttrArgs = attr.parse_args()?;

                    self.update_attr_args(&mut attr_args)?;

                    // Replace the original arguments with the modified ones.
                    if let Meta::List(meta) = &mut attr.meta {
                        meta.tokens = attr_args.to_token_stream();
                    } else {
                        unreachable!();
                    }

                    // Multiple `sysfs` attributes on the same item is UB.
                    break;
                }
            }
        }

        Ok(())
    }

    fn update_attr_args(&self, attr_args: &mut SysfsAttrArgs) -> syn::Result<()> {
        // Perform necessary mutations on `sysfs_dir`.
        if let Some(lit) = &mut attr_args.sysfs_dir {
            // There is a path, and it may need to be prefixed, so check.
            if let Some(path) = lit.value().strip_prefix("./") {
                // Prefix the path with the module's `sysfs_dir`.
                *lit = LitStr::new(&format!("{}/{}", self.sysfs_dir.value(), path), lit.span());
            } else {
                // Do nothing to the path.
            }
        } else {
            // The definition did not have a `sysfs_dir`, so we clone
            // the module's as a fallback.
            attr_args.sysfs_dir = Some(self.sysfs_dir.clone());
        }

        // No other mutations currently.
        Ok(())
    }
}

/// Discards the attributes.
fn expr_require_lit_str(expr: Expr) -> syn::Result<LitStr> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit), ..
        }) => Ok(lit),
        _ => err!(expr, "expected a literal string"),
    }
}

impl Parse for SysfsAttrArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Self::default())
        } else if input.peek(Token![in]) {
            // Special handling to extract the `in` token, and then re-parse
            // as a comma-punctuated list.
            let _in_token = <Token![in]>::parse(input)?;
            let sysfs_dir = expr_require_lit_str(Expr::parse(input)?)?;
            let mut args = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
            args.insert(0, parse_quote!(sysfs_dir = #sysfs_dir));
            Self::try_from(args)
        } else {
            // Parse as a comma-punctuated list of `Meta` items.
            let args = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
            Self::try_from(args)
        }
    }
}

impl TryFrom<Punctuated<Meta, Token![,]>> for SysfsAttrArgs {
    type Error = Error;

    fn try_from(args: Punctuated<Meta, Token![,]>) -> syn::Result<Self> {
        let args_span = args.span();

        let mut sysfs_dir = None;

        args.into_iter().try_for_each(|arg| match arg {
            Meta::NameValue(MetaNameValue { path, value, .. }) if path.is_ident("sysfs_dir") => {
                Ok(sysfs_dir = Some(expr_require_lit_str(value)?))
            }
            _ => err!(arg, "unknown meta argument"),
        })?;

        let sysfs_dir =
            sysfs_dir.ok_or_else(|| Error::new(args_span, "argument `sysfs_dir` is required"))?;

        Ok(Self {
            sysfs_dir: Some(sysfs_dir),
        })
    }
}

impl Parse for SysfsModArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            err!(input, "this attribute requires arguments")
        } else if input.peek(Token![in]) {
            let _in_token = <Token![in]>::parse(input)?;
            let sysfs_dir = expr_require_lit_str(Expr::parse(input)?)?;

            Ok(Self { sysfs_dir })
        } else {
            let mut sysfs_dir = None;

            let args = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
            args.into_iter().try_for_each(|arg| match arg {
                Meta::NameValue(MetaNameValue { path, value, .. })
                    if path.is_ident("sysfs_dir") =>
                {
                    Ok(sysfs_dir = Some(expr_require_lit_str(value)?))
                }
                _ => err!(arg, "unknown meta argument"),
            })?;

            let sysfs_dir = sysfs_dir
                .ok_or_else(|| Error::new(input.span(), "argument `sysfs_dir` is required"))?;

            Ok(Self { sysfs_dir })
        }
    }
}

impl Parse for ItemSysfsAttrFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        ItemFn::parse(input)?.try_into()
    }
}

impl TryFrom<ItemFn> for ItemSysfsAttrFn {
    type Error = Error;

    fn try_from(
        ItemFn {
            attrs,
            vis,
            sig,
            mut block,
        }: ItemFn,
    ) -> Result<Self, Self::Error> {
        // Expect a local `let read = #init`, where the init is expected to be a
        // function that infallibly transforms a string into the return type of
        // this function.
        let let_read = block
            .stmts
            .iter()
            .rposition(|stmt| {
                matches!(stmt, Stmt::Local(Local {
                    pat: Pat::Ident(PatIdent { ident, .. }),
                    init: Some(LocalInit { .. }),
                    ..
                }) if ident == "read")
            })
            .map(|index| match block.stmts.remove(index) {
                Stmt::Local(local) => local,
                _ => unreachable!(),
            });

        // Expect a local `let write = |#ident:#ty|` where init is a closure
        // that forms an arbitrary type as a string suitable for output to
        // the file.
        let let_write = block
            .stmts
            .iter()
            .rposition(|stmt| {
                matches!(stmt, Stmt::Local(Local {
                    pat: Pat::Ident(PatIdent { ident, .. }),
                    init: Some(LocalInit { .. }),
                    ..
                }) if ident == "write")
            })
            .map(|index| match block.stmts.remove(index) {
                Stmt::Local(local) => local,
                _ => unreachable!(),
            });

        // The dots at the end of the function indicate "et cetera",
        // where the generated content will be put. It is not allowed to have
        // code after the `..`, but you may before.
        // The `let_read` and `let_write` immediately precede this token,
        // so additional code is expected to be at the top of the block.
        let dots = match block.stmts.pop() {
            Some(Stmt::Expr(
                Expr::Range(ExprRange {
                    attrs,
                    start: None,
                    limits: RangeLimits::HalfOpen(dots),
                    end: None,
                }),
                None,
            )) if attrs.is_empty() => Ok(dots),
            _ => err!(block, "expected `..` to be the return expression"),
        }?;

        Ok(Self {
            attrs,
            vis,
            sig,
            let_read,
            let_write,
            dots,
            block,
        })
    }
}

//
// Code related to generating tokens starts here.
//

fn sysfs_attr(args: &SysfsAttrArgs, item: ItemSysfsAttrFn) -> syn::Result<TokenStream2> {
    let mut tokens = TokenStream2::new();
    if let Ok(mut getter) = GetterFunction::try_from(item.clone()) {
        if let (Some(sysfs_dir), None) = (&args.sysfs_dir, &getter.sysfs_dir) {
            getter.sysfs_dir = Some(sysfs_dir.clone())
        }
        tokens.extend(getter.to_token_stream());
    }
    if let Ok(mut setter) = SetterFunction::try_from(item.clone()) {
        if let (Some(sysfs_dir), None) = (&args.sysfs_dir, &setter.sysfs_dir) {
            setter.sysfs_dir = Some(sysfs_dir.clone())
        }
        tokens.extend(setter.to_token_stream());
    }
    Ok(tokens)
}

impl ToTokens for SysfsAttrArgs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { sysfs_dir } = self;
        let mut args = Punctuated::<Meta, Token![,]>::new();
        if let Some(sysfs_dir) = sysfs_dir {
            args.push(parse_quote!(sysfs_dir = #sysfs_dir));
        }
        args.to_tokens(tokens)
    }
}

impl ToTokens for SysfsModArgs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { sysfs_dir } = self;
        let mut args = Punctuated::<Meta, Token![,]>::new();
        args.push(parse_quote!(sysfs_dir = #sysfs_dir));
        args.to_tokens(tokens)
    }
}

impl TryFrom<SysfsModArgs> for SysfsAttrArgs {
    type Error = Error;

    fn try_from(other: SysfsModArgs) -> syn::Result<Self> {
        // This works for now since `sysfs_dir` is the only argument,
        // but this may eventually be a fallible operation.
        Ok(Self {
            sysfs_dir: Some(other.sysfs_dir),
        })
    }
}

struct GetterFunction {
    attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    into_type: Box<Type>,
    let_read: Local,
    stmts: Vec<Stmt>,
    sysfs_dir: Option<LitStr>,
    sysfs_file: String,
}

struct SetterFunction {
    attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    let_write: Local,
    from_ident: Ident,
    from_type: Box<Type>,
    stmts: Vec<Stmt>,
    sysfs_dir: Option<LitStr>,
    sysfs_file: String,
}

fn let_sysfs_path(sysfs_dir: &Option<LitStr>, sysfs_file: &str) -> Stmt {
    let literal = match sysfs_dir {
        Some(sysfs_dir) => format!("{}/{}", sysfs_dir.value(), sysfs_file),
        None => format!("{}/{}", "{SYSFS_DIR}", sysfs_file),
    };
    parse_quote! {
        let sysfs_path = format!(#literal);
    }
}

impl ToTokens for GetterFunction {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            attrs,
            vis,
            sig,
            let_read,
            into_type,
            stmts,
            sysfs_dir,
            sysfs_file,
        } = self;
        let let_sysfs_path = let_sysfs_path(sysfs_dir, sysfs_file);

        tokens.extend(quote! {
            #(#attrs)*
            #vis #sig {
                #(#stmts)*
                #let_sysfs_path
                #let_read
                unsafe {
                    ::sysfs_lib::sysfs_read::<#into_type>(&sysfs_path, read)
                }
            }
        });
    }
}

impl ToTokens for SetterFunction {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            attrs,
            vis,
            sig,
            let_write,
            from_ident,
            from_type: _,
            stmts,
            sysfs_dir,
            sysfs_file,
        } = self;
        let let_sysfs_path = let_sysfs_path(sysfs_dir, sysfs_file);

        tokens.extend(quote! {
            #(#attrs)*
            #vis #sig {
                #(#stmts)*
                #let_sysfs_path
                #let_write
                unsafe {
                    ::sysfs_lib::sysfs_write(&sysfs_path, write(#from_ident))
                }
            }
        });
    }
}

impl TryFrom<ItemSysfsAttrFn> for GetterFunction {
    type Error = Error;

    fn try_from(
        ItemSysfsAttrFn {
            mut attrs,
            vis,
            mut sig,
            let_read,
            block,
            ..
        }: ItemSysfsAttrFn,
    ) -> syn::Result<Self> {
        if let Some(mut local) = let_read {
            let sysfs_file = sig.ident.to_string();

            // Take all attributes from the local, and apply them to the function
            // instead. The local assignment will not retain attributes.
            attrs.append(&mut local.attrs);
            // Extract the original type from the signature,
            // and wrap the existing one with `::sysfs_lib::Result`.
            let into_type;
            (into_type, sig.output) = if let ReturnType::Type(_, ty) = sig.output {
                Ok((ty.clone(), parse_quote!(-> ::sysfs_lib::Result<#ty>)))
            } else {
                err!(
                    sig.output,
                    "explicit return type needed for getter function"
                )
            }?;

            Ok(Self {
                attrs,
                vis,
                sig,
                into_type,
                let_read: local,
                stmts: block.stmts,
                sysfs_dir: None,
                sysfs_file,
            })
        } else {
            err!(block, "expected to find `let read = ...`")
        }
    }
}

impl TryFrom<ItemSysfsAttrFn> for SetterFunction {
    type Error = Error;

    fn try_from(
        ItemSysfsAttrFn {
            mut attrs,
            vis,
            mut sig,
            let_write,
            block,
            ..
        }: ItemSysfsAttrFn,
    ) -> syn::Result<Self> {
        let sysfs_file = sig.ident.to_string();

        let mut local = let_write
            .ok_or_else(|| Error::new(block.span(), "expected to find `let write = ...`"))?;

        attrs.append(&mut local.attrs);

        let (from_ident, from_type) = match &local.init {
            Some(LocalInit {
                expr,
                diverge: None, // needs separate error
                ..
            }) => Ok(expr),
            _ => err!(local, "expected to be initialized"),
        }
        .and_then(|expr| match expr.as_ref() {
            Expr::Closure(ExprClosure { inputs, .. }) => Ok(inputs),
            _ => err!(local, "expected a closure"),
        })
        .and_then(|inputs| match inputs.first() {
            Some(Pat::Type(PatType { pat, ty, .. })) => Ok((pat, ty)),
            _ => err!(local, "expected a typed identifier"),
        })
        .and_then(|(pat, ty)| match pat.as_ref() {
            Pat::Ident(PatIdent { ident, .. }) => Ok((ident, ty)),
            _ => err!(local, "expected an identifier"),
        })
        .map(|(ident, ty)| (ident.clone(), ty.clone()))?;

        sig.ident = format_ident!("set_{}", sig.ident);
        sig.inputs.push(parse_quote!(#from_ident: #from_type));
        sig.output = parse_quote!(-> ::sysfs_lib::Result<()>);

        Ok(Self {
            attrs,
            vis,
            sig,
            let_write: local,
            from_ident,
            from_type,
            stmts: block.stmts,
            sysfs_dir: None,
            sysfs_file,
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[rustfmt::skip]
    macro_rules! test_parse {
        ({ $($input:tt)* } => $parse_ty:ty) => {{
            let result: syn::Result<$parse_ty> = syn::parse_str(&(quote::quote!{
                $($input)*
            }).to_string());
            match result {
                Ok(_) => (),
                Err(e) => panic!("{}", e.to_string()),
            }
        }};
    }
}
