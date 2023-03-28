use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{Expr, parse::Parse, Token, parse_macro_input};

struct KeyValuePair {
    key: TokenTree,
    value: Expr
}

impl Parse for KeyValuePair {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: TokenTree = input.parse()?;
        input.parse::<Token![:]>()?;
        let value: Expr = input.parse()?;

        Ok(Self {
            key,
            value,
        })
    }
}

struct BtreeMapEntries {
    entries: Vec<KeyValuePair>
}

impl Parse for BtreeMapEntries {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut entries = vec![];
        
        while !input.is_empty() {
            let entry: KeyValuePair = input.parse()?;

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
    
            entries.push(entry);
        }

        Ok(Self { entries })
    }
}

#[proc_macro]
pub fn btreemap_procedural(input: TokenStream) -> TokenStream {
    let kv_pairs = parse_macro_input!(input as BtreeMapEntries);

    let (keys, values): (Vec<_>, Vec<_>) = kv_pairs.entries.into_iter().map(|entry| (entry.key, entry.value)).unzip();

    quote!(
        {
            let mut temp_map = std::collections::BTreeMap::new();
            #(
                temp_map.insert(#keys, #values);
            )*
            temp_map
        }
    ).into()
}