extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;


fn is_all_uppercase(ident: &str) -> bool {
    ident.chars().all(|c| !c.is_lowercase())
}

#[proc_macro]
pub fn print_each_token(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();
    let mut output_string = String::new();
    let mut params = Vec::new();
    let mut token_iter = input2.into_iter().peekable();
    let mut prev_was_punct = false;

    while let Some(token) = token_iter.next() {
        match &token {
            TokenTree::Ident(i) => {
                if is_all_uppercase(&i.to_string()) {
                    output_string.push_str(" ");
                    output_string.push_str(&i.to_string());
                    output_string.push_str(" ");
                } else {
                    let modified_ident = format!(" \"{}\" ", i);
                    output_string.push_str(&modified_ident);
                }
                prev_was_punct = false;
            },
            TokenTree::Literal(l) => {
                let content = l.to_string();
                let content = content.trim_matches('"');
                let escaped_content = content.replace("'", "\\'");
                let modified_literal = format!(" '{}' '", escaped_content);
                output_string.push_str(&modified_literal);
                prev_was_punct = false;
            },
            TokenTree::Punct(p) => {
                if !prev_was_punct {
                    output_string.push_str(" ");
                }
                output_string.push(p.as_char());
                if p.as_char() == '$' {
                    if let Some(TokenTree::Ident(i)) = token_iter.peek() {
                        let param_ident = format!("${}", i);
                        output_string.push_str(&param_ident);
                        params.push(param_ident.clone());
                        let _ = token_iter.next();  // Move past the identifier
                    } else {
                        panic!("Expected an identifier after '$' but found none.");
                    }
                }
                prev_was_punct = true;
            },
            TokenTree::Group(g) => {
                let content = print_each_token(g.stream().into());
                output_string.push_str(" ( ");
                output_string.push_str(&content.to_string());
                output_string.push_str(" ) ");
                prev_was_punct = false;
            },
        }
    }

    println!("SQL: {}", output_string.trim());
    println!("Params: {:?}", params);

    quote! { () }.into()
}



#[proc_macro]
pub fn create_inline_struct_foo(_input: TokenStream) -> TokenStream {
    let output = quote! {
        #[derive(Debug)]
        struct Foo {
            a: i32,
        }
    };
    output.into()
}