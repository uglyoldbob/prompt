use proc_macro::TokenStream;
use syn::{DeriveInput, Ident};

#[proc_macro_derive(Prompting)]
pub fn derive_prompting(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let sident = input.ident;
    let expanded: TokenStream;
    match &input.data {
        syn::Data::Struct(s) => {
            let fields = &s.fields;
            let mut field_stuff: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            if let syn::Fields::Named(n) = fields {
                for (i, n) in n.named.iter().enumerate() {
                    println!("Field {:?}", n.ident);
                    let ftype = &n.ty;
                    if let Some(ident) = &n.ident {
                        println!("Field2 {}", ident.to_string());
                        let name = Ident::new(&format!("a_{}", i), proc_macro2::Span::call_site());
                        let text = ident.to_string();
                        let q: proc_macro2::TokenStream = quote::quote! {
                            let #name = <#ftype as prompt::Prompting>::prompt(Some(#text))?;
                        };
                        field_stuff.extend(q);
                    }
                }

                let mut q2s: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

                for (i, n) in n.named.iter().enumerate() {
                    if let Some(ident) = &n.ident {
                        let name = Ident::new(&format!("a_{}", i), proc_macro2::Span::call_site());
                        let q2: proc_macro2::TokenStream = quote::quote! {
                            #ident: #name,
                        };
                        q2s.extend(q2);
                    }
                }

                let q: proc_macro2::TokenStream = quote::quote! {
                    Ok(Self {
                        #q2s
                    })
                };
                field_stuff.extend(q);
            }
            expanded = quote::quote! {
                impl prompt::Prompting for #sident {
                    fn prompt(name: Option<&str>) -> Result<Self, prompt::Error> {
                        #field_stuff
                    }
                }
            }
            .into();
        }
        _ => panic!("Unhandled object type"),
    };

    println!("Token stream is {:?}", expanded.to_string());
    TokenStream::from(expanded)
}
