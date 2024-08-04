use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ImplItem, ItemImpl, Type, TypeReference};


#[proc_macro_attribute]
pub fn call_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let self_ty = &input.self_ty;

    let methods = input.items.iter().filter_map(|item| {
        if let ImplItem::Fn(m) = item {
            let name = &m.sig.ident;
            let name_str = name.to_string();

            let args_processing = m.sig.inputs.iter().enumerate().filter_map(|(mut i, arg)| {
                if let FnArg::Typed(pat) = arg {
                    i -= 1;
                    let var_name = Ident::new(&format!("arg{}", i), m.sig.ident.span());
                    let typ = &*pat.ty;
                    if let Type::Reference(TypeReference { elem, .. }) = typ {
                        Some(quote! {
                            // let #var_name = args.get(#i)
                            //     .map(|arg| arg.downcast_ref::<#elem>())
                            //     .expect("Invalid argument type");

                            let #var_name = args[#i].downcast_ref::<#elem>().expect("Invalid argument type");
                        })
                    } else {
                        Some(quote! {
                            let #var_name = args[#i].downcast_ref::<#typ>().expect("Invalid argument type");
                            let #var_name = #var_name.clone();
                        })
                    }
                } else {
                    None
                }
            });

            let call_args = if m.sig.inputs.len() >= 2 {
                (0..m.sig.inputs.len()-1).filter_map(|i| {
                    let var_name = Ident::new(&format!("arg{}", i), m.sig.ident.span());
                    Some(quote! { #var_name })
                }).collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            

            Some(quote! {
                map.insert(
                    #name_str.to_string(),
                    Box::new(move |args: &[Box<dyn std::any::Any>]| {
                        #(#args_processing)*
                        let result = self.#name(#(#call_args),*);
                        Box::new(result) as Box<dyn std::any::Any>
                    })
                );
            })
        } else {
            None
        }
    });

    let output = quote! {
        #input

        impl #self_ty {
            pub fn get_function_map(&'static self) -> std::collections::HashMap<String, Box<dyn Fn(&[Box<dyn std::any::Any>]) -> Box<dyn std::any::Any>>> {
                let mut map: std::collections::HashMap<String, Box<dyn Fn(&[Box<dyn std::any::Any>]) -> Box<dyn std::any::Any>>> = std::collections::HashMap::new();
                #(#methods)*
                map
            }
        }
    };

    TokenStream::from(output)
}

