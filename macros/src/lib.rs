use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn derive_component(tokens: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, .. } = syn::parse_macro_input!(tokens);
    quote::quote! {
        impl Component for #ident {}
    }
    .into()
}
