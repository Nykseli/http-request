use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::parse::{self, Parser};
use syn::token::Brace;
use syn::{parse_macro_input, DeriveInput, FieldsNamed, ItemStruct};

#[derive(FromDeriveInput)]
#[darling(attributes(syscall), forward_attrs(allow, doc, cfg))]
struct Opts {
    num: String,
}

#[proc_macro_derive(SysCall, attributes(syscall,))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let num = format_ident!("{}", opts.num);

    let output = quote! {
        impl SysCall for #ident {
            fn num() -> SysCallNum {
                SysCallNum::#num
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn syscall_response(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    let named = [
        syn::Field::parse_named
            .parse2(quote! { pub http_status: u64 })
            .unwrap(),
        syn::Field::parse_named
            .parse2(quote! { pub ret_value: i64 })
            .unwrap(),
    ];

    // Push the values to the list if it already exsists
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for name in named {
            fields.named.push(name);
        }
    } else {
        // Or create a new list
        let mut new = syn::punctuated::Punctuated::new();
        for name in named {
            new.push(name);
        }

        item_struct.fields = syn::Fields::Named(FieldsNamed {
            brace_token: Brace {
                span: Span::call_site(),
            },
            named: new,
        });
    }

    return quote! {
        #item_struct
    }
    .into();
}
