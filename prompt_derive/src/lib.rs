use proc_macro::TokenStream;
use syn::{DeriveInput, Ident};

#[cfg(feature = "egui")]
fn build_enum_variant_builder(v: &syn::Variant) -> proc_macro2::TokenStream {
    let sident = proc_macro2::Ident::new("Self", proc_macro2::Span::call_site());
    match &v.fields {
        syn::Fields::Named(f) => {
            let mut def: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            for f in f.named.iter() {
                tokens.extend([proc_macro2::TokenTree::Ident(
                    f.ident.as_ref().unwrap().clone(),
                )]);
                let ftype = &f.ty;
                let val =
                    quote::quote!(<#ftype as core::default::Default>::default());
                tokens.extend([proc_macro2::TokenTree::Punct(
                    proc_macro2::Punct::new(':', proc_macro2::Spacing::Alone),
                )]);
                tokens.extend(val);
                tokens.extend([proc_macro2::TokenTree::Punct(
                    proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
                )]);
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
            tokens
        }
        syn::Fields::Unnamed(f) => {
            let mut def: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            for (i, f) in f.unnamed.iter().enumerate() {
                let ty = &f.ty;
                tokens.extend(quote::quote!(<#ty as core::default::Default>::default()));
                tokens.extend([proc_macro2::TokenTree::Punct(
                        proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
                    )]);
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
                proc_macro2::Delimiter::Parenthesis,
                def,
            ))]);
            tokens
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
            tokens
        }
    }
}

/// Builds a match pair for a given enum variant to help convert an enum to a string
#[cfg(feature = "egui")]
fn build_enum_variant_to_fields(v: &syn::Variant) -> (proc_macro2::TokenStream, Vec<&syn::Field>) {
    let sident = proc_macro2::Ident::new("Self", proc_macro2::Span::call_site());
    let mut fields = Vec::new();
    let q: proc_macro2::TokenStream = match &v.fields {
        syn::Fields::Named(f) => {
            let mut def: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            for f in f.named.iter() {
                println!("IDENT IS {:?}", f.ident);
                tokens.extend([proc_macro2::TokenTree::Ident(
                    f.ident.as_ref().unwrap().clone(),
                )]);
                tokens.extend([proc_macro2::TokenTree::Punct(
                    proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
                )]);
                fields.push(f);
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
            tokens
        }
        syn::Fields::Unnamed(f) => {
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
            let mut tokens2: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            for (i, f) in f.unnamed.iter().enumerate() {
                let varname = quote::format_ident!("a_{}", i);
                tokens2.extend([proc_macro2::TokenTree::Ident(
                    varname,
                )]);
                tokens2.extend([proc_macro2::TokenTree::Punct(
                    proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
                )]);
                fields.push(f);
            }
            quote::quote!{#tokens (#tokens2)}
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
            tokens
        }
    };
    (q, fields)
}

/// Builds a match pair for a given enum variant to help convert an enum to a string
#[cfg(feature = "egui")]
fn build_enum_variant_to_string(v: &syn::Variant) -> (proc_macro2::TokenStream, String) {
    let sident = proc_macro2::Ident::new("Self", proc_macro2::Span::call_site());
    let text2 = v.ident.to_string();
    let q: proc_macro2::TokenStream = match &v.fields {
        syn::Fields::Named(f) => {
            let mut def: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            for f in f.named.iter() {
                tokens.extend([proc_macro2::TokenTree::Ident(
                    f.ident.as_ref().unwrap().clone(),
                )]);
                tokens.extend([proc_macro2::TokenTree::Punct(
                    proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
                )]);
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
            tokens
        }
        syn::Fields::Unnamed(f) => {
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
            let mut tokens2: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            for (i, _f) in f.unnamed.iter().enumerate() {
                let varname = quote::format_ident!("a_{}", i);
                tokens2.extend([proc_macro2::TokenTree::Ident(
                    varname,
                )]);
                tokens2.extend([proc_macro2::TokenTree::Punct(
                    proc_macro2::Punct::new(',', proc_macro2::Spacing::Alone),
                )]);
            }
            quote::quote!{#tokens (#tokens2)}
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
            tokens
        }
    };
    (q, text2)
}

#[cfg(feature = "egui")]
#[proc_macro_derive(EguiPrompting)]
pub fn derive_egui_prompting(input: TokenStream) -> TokenStream {
    use std::any::Any;

    let input = syn::parse_macro_input!(input as DeriveInput);
    let sident = input.ident;
    let expanded: TokenStream;

    match &input.data {
        syn::Data::Enum(e) => {
            let mut field_stuff: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            let q: proc_macro2::TokenStream = quote::quote! {
                let combobox = if let Some(name) = name {
                    let mut s = "Select a ".to_string();
                    s.push_str(&name);
                    egui::ComboBox::from_label(s)
                } else {
                    egui::ComboBox::from_label("Select")
                };
            };
            field_stuff.extend(q);

            let mut match_stuff: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            for v in &e.variants {
                let (q, t) = build_enum_variant_to_string(v);
                match_stuff.extend(
                    quote::quote! {
                        #q => #t,
                    });
            }

            let q_start: proc_macro2::TokenStream = quote::quote! {
                let val = match self {
                    #match_stuff
                };
            };
            field_stuff.extend(q_start);

            let mut combo_stuff: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            for v in &e.variants {
                let text = v.ident.to_string();
                let assign = build_enum_variant_builder(v);
                let t = v.type_id();
                let q: proc_macro2::TokenStream = quote::quote! {
                    if ui.selectable_label(false, #text).clicked() {
                        *self = #assign;
                    }
                };
                combo_stuff.extend(q);
            }

            let mut option_prompt: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
            for v in &e.variants {
                let (q, f) = build_enum_variant_to_fields(v);
                let mut option_code: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
                if !f.is_empty() {
                    for (i, f) in f.iter().enumerate() {
                        if let Some(ident) = &f.ident {
                            let varname = quote::format_ident!("{}", ident);
                            let text = ident.to_string();
                            let q: proc_macro2::TokenStream = quote::quote! {
                                let subname = format!("{}/{}", name.unwrap_or(""), #text);
                                #varname.build_gui(ui, Some(&subname))?;
                            };
                            option_code.extend(q);
                        }
                        else {
                            let varname = quote::format_ident!("a_{}", i);
                            let text = format!("{}", i);
                            let q: proc_macro2::TokenStream = quote::quote! {
                                let subname = format!("{}/{}", name.unwrap_or(""), #text);
                                #varname.build_gui(ui, Some(&subname))?;
                            };
                            option_code.extend(q);
                        }
                    }
                    option_prompt.extend(quote::quote! {
                        #q => { #option_code },
                    });
                }
            }

            expanded = quote::quote! {
                impl prompt::EguiPrompting for #sident {
                    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
                        #field_stuff
                        combobox.selected_text(val)
                            .show_ui(ui, |ui| { #combo_stuff });
                        match self {
                            #option_prompt
                            _ => {}
                        }
                        Ok(())
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
                    ui.label(name);
                }
            };
            field_stuff.extend(q);

            if let syn::Fields::Named(n) = fields {
                for n in n.named.iter() {
                    if let Some(ident) = &n.ident {
                        let text = ident.to_string();
                        let varname = quote::format_ident!("{}", ident);
                        let q: proc_macro2::TokenStream = quote::quote! {
                            let subname = format!("{}/{}", name.unwrap_or(""), #text);
                            self.#varname.build_gui(ui, Some(&subname))?;
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
                    Ok(())
                };
                field_stuff.extend(q);
            }
            expanded = quote::quote! {
                impl prompt::EguiPrompting for #sident {
                    fn build_gui(&mut self, ui: &mut egui::Ui, name: Option<&str>) -> Result<(), String> {
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

            let match_else: proc_macro2::TokenStream =
                quote::quote!(_ => println!("Invalid option"),);

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
