use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::parse::{self, Parser};
use syn::token::Brace;
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, ItemStruct, Type,
};

fn field_to_ident<'a>(field: &'a Field) -> (&'a Ident, &'a Ident) {
    let name = if let Some(n) = &field.ident {
        n
    } else {
        unreachable!();
    };

    let ty = if let Type::Path(tp) = &field.ty {
        &tp.path.segments[0].ident
    } else {
        unreachable!();
    };

    (name, ty)
}

#[derive(FromDeriveInput)]
#[darling(attributes(syscall), forward_attrs(allow, doc, cfg))]
struct Opts {
    num: String,
}

// TODO: replace syscall attribute with value based on ident. OpenResp -> Open etc
#[proc_macro_derive(SysCall, attributes(syscall,))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, data, .. } = input;

    let mut ident_fields: Vec<(&Ident, &Ident)> = vec![];
    if let Data::Struct(s) = &data {
        if let Fields::Named(fields) = &s.fields {
            for named in fields.named.iter() {
                ident_fields.push(field_to_ident(&named));
            }
        }
    }

    let mut new_args = quote!();
    let mut set_args = quote!();
    for field in &ident_fields {
        let name = field.0;
        let ty = field.1;
        new_args.extend(quote!(#name: #ty,));
        set_args.extend(quote!(
            #name: #name,
        ));
    }

    let num = format_ident!("{}", opts.num);

    let output = quote! {
        impl #ident {
            pub fn new(#new_args) -> Self {
                Self {
                    #set_args
                }
            }
        }

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
            .parse2(quote! { pub ret_value: i64 })
            .unwrap(),
        syn::Field::parse_named
            .parse2(quote! { pub http_status: u64 })
            .unwrap(),
    ];

    // Push the values to the list if it already exsists
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for name in named {
            fields.named.insert(0, name);
        }
    } else {
        // Or create a new list
        let mut new = syn::punctuated::Punctuated::new();
        for name in named {
            new.insert(0, name);
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
