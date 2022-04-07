use proc_macro::{TokenStream, TokenTree};
use quote::quote;

#[proc_macro]
pub fn game_mod(args: TokenStream) -> TokenStream {
    let mut name = None;
    let mut desc = None;
    let tokens: Vec<TokenTree> = args.into_iter().collect();
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        match token {
            TokenTree::Ident(ident) => {
                match ident.to_string().as_str() {
                    "name" => match tokens.get(i+1..=i+2) {
                        Some([TokenTree::Punct(punct), TokenTree::Literal(liter)])
                            if punct.to_string().as_str() == "=" && liter.to_string().contains("\"") => {
                                name = Some(liter.to_string());
                                i += 2;
                            }
                        _ => panic!("name improperly specified, should be 'name = \"mod_name\"'")
                    }
                    "desc" => match tokens.get(i+1..=i+2) {
                        Some([TokenTree::Punct(punct), TokenTree::Literal(liter)])
                            if punct.to_string().as_str() == "=" && liter.to_string().contains("\"") => {
                                desc = Some(liter.to_string());
                                i += 2;
                            }
                        _ => panic!("desc improperly specified, should be 'desc = \"mod_desc\"'")
                    }
                    _ => panic!("unrecognized identifier: '{}'", ident)
                }
            }
            TokenTree::Punct(punct) if punct.to_string().as_str().matches([',', ';']).count() > 0 => {
                // do nothing

                // this branch is here to allow users to put ',' or ';' at the end of their arguments
                // if they wish, this helper macro will simply ignore those tokens

                // technically this also allows someone to spam ,,,,, and ;;;;; arbitrarily, this
                // macro will simply filter that out
            }
            _ => panic!("unrecognized token: '{}'", token)
        }
        i += 1;
    }
    let name = name.expect("name unspecified, should be 'name = \"mod_name\"'").replace("\"", "");
    let desc = desc.expect("desc unspecified, should be 'desc = \"mod_desc\"'").replace("\"", "");
    let tokens = quote! {
        #[no_mangle]
        pub static __MOD_NAME: &str = #name;
        #[no_mangle]
        pub static __MOD_DESC: &str = #desc;
    };
    tokens.into()
}
