use proc_macro::TokenStream;
use quote::TokenStreamExt;
use syn::{DeriveInput, Ident};

#[proc_macro_derive(Prompting)]
pub fn derive_prompting(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let sident = input.ident;
    let expanded: TokenStream;
    match &input.data {
        syn::Data::Enum(e) => {
            let mut field_stuff: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            let q: proc_macro2::TokenStream = quote::quote! {
                if let Some(name) = name {
                    println!("[{}]", name);
                }
            };
            field_stuff.extend(q);

            let q: proc_macro2::TokenStream = quote::quote! {
                println!("Enter the variant type, valid options are listed below");
            };
            field_stuff.extend(q);

            for v in &e.variants {
                let text = v.ident.to_string();
                let q: proc_macro2::TokenStream = quote::quote! {
                    println!("\t{}", #text);
                };
                field_stuff.extend(q);
            }

            let name = Ident::new(&format!("a"), proc_macro2::Span::call_site());
            let q: proc_macro2::TokenStream = quote::quote! {
                let #name = <String as prompt::Prompting>::prompt(None)?;
            };
            field_stuff.extend(q);

            let mut match_stuff: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            for v in &e.variants {
                let text2 = v.ident.to_string();
                let q: proc_macro2::TokenStream = match &v.fields {
                    syn::Fields::Named(f) => {
                        let mut def: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                        let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                        for (i, f) in f.named.iter().enumerate() {
                            if i != 0 {
                                tokens.extend([proc_macro2::TokenTree::Punct(
                                    proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
                                )]);
                            }
                            tokens.extend([proc_macro2::TokenTree::Ident(
                                f.ident.as_ref().unwrap().clone(),
                            )]);
                            let ftype = &f.ty;
                            let text = f.ident.as_ref().unwrap().to_string();
                            let val =
                                quote::quote!(<#ftype as prompt::Prompting>::prompt(Some(#text))?);
                            tokens.extend([proc_macro2::TokenTree::Punct(
                                proc_macro2::Punct::new(':', proc_macro2::Spacing::Alone),
                            )]);
                            tokens.extend(val);
                        }
                        def.extend(tokens);

                        let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                        tokens.extend([proc_macro2::TokenTree::Ident(sident.clone())]);
                        tokens.extend([
                            proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                                ':',
                                proc_macro2::Spacing::Joint,
                            )),
                            proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                                ':',
                                proc_macro2::Spacing::Alone,
                            )),
                        ]);
                        tokens.extend([proc_macro2::TokenTree::Ident(v.ident.clone())]);
                        tokens.extend([proc_macro2::TokenTree::Group(proc_macro2::Group::new(
                            proc_macro2::Delimiter::Brace,
                            def,
                        ))]);
                        quote::quote! {
                            #text2 => { return Ok(#tokens); }
                            _ => println!("Invalid option"),
                        }
                    }
                    syn::Fields::Unnamed(f) => {
                        let mut def: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                        let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                        for (i, f) in f.unnamed.iter().enumerate() {
                            if i != 0 {
                                tokens.extend([proc_macro2::TokenTree::Punct(
                                    proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
                                )]);
                            }
                            tokens.extend([proc_macro2::TokenTree::Literal(
                                proc_macro2::Literal::usize_unsuffixed(i),
                            )]);
                            let ftype = &f.ty;
                            let val = quote::quote!(<#ftype as prompt::Prompting>::prompt(Some(&format!("{}", #i)))?);
                            tokens.extend([proc_macro2::TokenTree::Punct(
                                proc_macro2::Punct::new(':', proc_macro2::Spacing::Alone),
                            )]);
                            tokens.extend(val);
                        }
                        def.extend(tokens);

                        let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                        tokens.extend([proc_macro2::TokenTree::Ident(sident.clone())]);
                        tokens.extend([
                            proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                                ':',
                                proc_macro2::Spacing::Joint,
                            )),
                            proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                                ':',
                                proc_macro2::Spacing::Alone,
                            )),
                        ]);
                        tokens.extend([proc_macro2::TokenTree::Ident(v.ident.clone())]);
                        tokens.extend([proc_macro2::TokenTree::Group(proc_macro2::Group::new(
                            proc_macro2::Delimiter::Brace,
                            def,
                        ))]);
                        quote::quote! {
                            #text2 => { return Ok(#tokens); }
                        }
                    }
                    syn::Fields::Unit => {
                        let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                        tokens.extend([proc_macro2::TokenTree::Ident(sident.clone())]);
                        tokens.extend([
                            proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                                ':',
                                proc_macro2::Spacing::Joint,
                            )),
                            proc_macro2::TokenTree::Punct(proc_macro2::Punct::new(
                                ':',
                                proc_macro2::Spacing::Alone,
                            )),
                        ]);
                        tokens.extend([proc_macro2::TokenTree::Ident(v.ident.clone())]);
                        quote::quote! {
                            #text2 => { return Ok(#tokens); }
                        }
                    }
                };
                match_stuff.extend(q);
            }

            let match_else: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

            let q_start: proc_macro2::TokenStream = quote::quote! {
                match a.as_str() {
                    #match_stuff
                    #match_else
                }
            };
            field_stuff.extend(q_start);

            expanded = quote::quote! {
                impl prompt::Prompting for #sident {
                    fn prompt(name: Option<&str>) -> Result<Self, prompt::Error> {
                        loop {
                            #field_stuff
                        }
                    }
                }
            }
            .into();
        }
        syn::Data::Struct(s) => {
            let fields = &s.fields;
            let mut field_stuff: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

            let q: proc_macro2::TokenStream = quote::quote! {
                if let Some(name) = name {
                    println!("[{}]", name);
                }
            };
            field_stuff.extend(q);

            if let syn::Fields::Named(n) = fields {
                for (i, n) in n.named.iter().enumerate() {
                    let ftype = &n.ty;
                    if let Some(ident) = &n.ident {
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
    TokenStream::from(expanded)
}
