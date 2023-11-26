use convert_case::{Case, Casing};
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

fn impl_query_params(ast: &syn::DeriveInput, case: Case) -> TokenStream {
    if let syn::Data::Struct(ref data) = ast.data {
        if let syn::Fields::Named(ref fields) = data.fields {
            let field_impl = fields.named.iter().map(|field| {
                let name = field.ident.as_ref().unwrap();
                let name_string = name.to_string().to_case(case);
                if let syn::Type::Path(ref path) = field.ty {
                    if is_option(path) {
                        return quote! {
                            if let Some(value) = self.#name.as_ref() {
                                params.insert(#name_string.to_string(), value.to_string());
                            }
                        };
                    }

                    return quote!(params.insert(#name_string.to_string(), self.#name.to_string()););
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

#[proc_macro_derive(HttpQueryParams, attributes(case))]
pub fn derive_http_query_params(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let mut case = Case::Snake;
    for attr in &ast.attrs {
        if !attr.path().is_ident("case") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("camelCase") {
                case = Case::Camel;
                return Ok(());
            }

            Err(meta.error("Non supported case"))
        }).unwrap();
    }

    impl_query_params(&ast, case)
}
