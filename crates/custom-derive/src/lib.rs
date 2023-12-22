use darling::{util::parse_expr, FromDeriveInput};
use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_derive(RonAsset, attributes(ron_asset))]
pub fn ron_asset_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  let parsed = Receiver::from_derive_input(&ast).unwrap();
  let name = &parsed.ident;
  let assets = &parsed.assets;
  let extension = &parsed.extension;

  let gen = quote! {
    impl AssetLoader for #name
    {
      type Asset = #name;
      type Settings = ();
      type Error = RonAssetLoaderError;
      fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        ctx: &'a mut LoadContext,
      ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
          let mut bytes = Vec::new();
          reader.read_to_end(&mut bytes).await?;
          let ron_asset = from_bytes::<#name>(&bytes)?;
          let linked_assets = ron_asset.construct_nested_assets(ctx);
          // TODO: how to access linked_assets
          Ok(ron_asset)
        })
      }

      fn extensions(&self) -> &[&str] {
        &[#extension]
      }
    }

  };
  gen.into()
}

#[derive(FromDeriveInput)]
#[darling(attributes(ron_asset))]
struct Receiver {
  ident: Ident,
  extension: String,
  #[darling(with = parse_expr::preserve_str_literal)]
  assets: Expr,
}

// #[derive(Default, FromMeta)]
// #[darling(default)]
// struct RonAssetParams(syn::LitStr, Option<syn::Ident>);
// impl Parse for RonAssetParams {
//   fn parse(input: ParseStream) -> Result<Self> {
//     let content;
//     syn::parenthesized!(content in input);
//     let extension = content.parse()?;
//     content.parse::<Token![,]>()?;
//     let Ok(asset_type) = content.parse() else {
//       return Ok(RonAssetParams(extension, None));
//     };
//     Ok(RonAssetParams(extension, Some(asset_type)))
//   }
// }

// fn impl_ron_asset(ast: &syn::DeriveInput) -> TokenStream {
//   let attrib = ast
//     .attrs
//     .iter()
//     .filter(|a| a.path().is_ident("ron_asset"))
//     .nth(0)
//     .expect("Must have ron asset attrib");
//   attrib.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated);
//   let params = syn::parse2(attrib.tts.clone()).expect("Invalid ron_asset attribute");

//   let name = &ast.ident;
//   let assetName = &ast.ident;
//   let gen = quote! {
//       impl RonAsset for #name {
//           fn hello_macro() {
//               println!("Hello, Macro! My name is {}!", stringify!(#name));
//           }
//       }
//   };
//   gen.into()
// }
