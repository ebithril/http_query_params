use proc_macro::TokenStream;
use quote::quote;

fn is_option(ty: &syn::TypePath) -> bool {
    for segment in &ty.path.segments {
        if !segment.ident.to_string().starts_with("Option") {
            continue;
        }

        return true;
    }

    false
}

fn impl_query_params(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Struct(ref data) = ast.data {
        if let syn::Fields::Named(ref fields) = data.fields {
            let field_impl = fields.named.iter().map(|field| {
                let name = field.ident.as_ref().unwrap();
                let name_string = name.to_string();
                if let syn::Type::Path(ref path) = field.ty {
                    if is_option(path) {
                        return quote! {
                            if self.#name.is_some() {
                                params.insert(#name_string.to_string(), self.#name.as_ref().unwrap().to_string());
                            }
                        };
                    }
                    else {
                        return quote!(params.insert(#name_string.to_string(), self.#name.to_string()););
                    }
                }

                quote!()
            });

            let name = &ast.ident;
            let gen = quote! {
                impl #name {
                    fn as_map(&self) -> HashMap<String, String> {
                        let mut params: HashMap<String, String> = HashMap::new();
                        #(#field_impl)*
                        params
                    }
                }
            };

            return gen.into();
        }
    }

    TokenStream::from(
        syn::Error::new(ast.ident.span(), "Couldn't generate code").to_compile_error(),
    )
}

#[proc_macro_derive(HttpQueryParams)]
pub fn derive_http_query_params(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_query_params(&ast)
}
