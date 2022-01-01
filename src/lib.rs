use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Attribute, Field, Fields, ItemStruct,
};

#[proc_macro_attribute]
pub fn subset(_args: TokenStream, item_ts: TokenStream) -> TokenStream {
    // let args = parse_macro_input!(args as AttributeArgs);
    let item = item_ts.clone();
    let mut item = parse_macro_input!(item as ItemStruct);

    // nothing to do for unit type
    if let Fields::Unit = item.fields {
        return item_ts;
    }

    // TODO support enums as well

    // TODO custom name, and other properties

    let mut subset_item = item.clone();

    let mut subset_name = subset_item.ident.to_string();
    subset_name.push_str("Subset");
    subset_item.ident = Ident::new(&subset_name, subset_item.ident.span());

    clean_non_subset(fields(&mut item), false);
    clean_non_subset(fields(&mut subset_item), true);

    // TODO just combine token streams?
    let expanded = quote! {
        #item
        #subset_item
    };
    TokenStream::from(expanded)
}

fn fields<'a>(item: &'a mut ItemStruct) -> &'a mut Punctuated<Field, Comma> {
    match &mut item.fields {
        Fields::Named(fields) => &mut fields.named,
        Fields::Unnamed(fields) => &mut fields.unnamed,
        Fields::Unit => unreachable!(),
    }
}

fn clean_non_subset(fields: &mut Punctuated<Field, Comma>, remove: bool) {
    *fields = fields
        .iter()
        .cloned()
        .filter(|f| !remove || f.attrs.iter().any(is_subset_attr))
        .map(|mut f| {
            f.attrs.retain(|a| !is_subset_attr(a));
            f
        })
        .collect();
}

fn is_subset_attr(attr: &Attribute) -> bool {
    if let Some(ident) = attr.path.get_ident() {
        return ident.to_string() == "in_subset";
    }
    false
}
