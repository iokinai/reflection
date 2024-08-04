use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Reflection)]
pub fn reflection_derive(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);

    let struct_name = macro_input.ident;
    let struct_name_str = struct_name.to_string();

    let fields = if let syn::Data::Struct(data) = macro_input.data {
        data.fields
    } else {
        return TokenStream::from(quote! {
            compile_error!("ParseFields can only be used with structs");
        });
    };

    let field_names = fields.iter().filter_map(|f| f.ident.as_ref());
    let field_types = fields.iter().map(|f| &f.ty);

    let field_infos = field_names.zip(field_types).map(|(name, ty)| {
        let name_str = name.to_string();
        let ty_str = quote!(#ty).to_string();
        quote! {
            Field {
                name: #name_str.to_string(),
                rtype: #ty_str.to_string(),
            }
        }
    });

    let output = quote! {
        impl #struct_name {
            pub fn struct_name() -> String {
                String::from(#struct_name_str)
            }

            pub fn fields() -> Vec<Field> {
                vec![#(#field_infos),*]
            }
        }
    };

    TokenStream::from(output)
}