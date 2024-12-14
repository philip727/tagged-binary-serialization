use core::panic;

use proc_macro2::TokenStream;
use tagged_binary_serialization::type_specification::TypeTag;

use syn::{
    parenthesized, parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Fields,
    GenericParam, Generics, LitInt,
};

use quote::{quote, quote_spanned};

#[proc_macro_derive(TagDecode, attributes(encode))]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let generics = add_decode_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut tag: Option<u16> = None;
    for attr in input.attrs.iter() {
        if !attr.path().is_ident("encode") {
            continue;
        }

        if let Err(e) = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("tag") {
                let content;
                parenthesized!(content in meta.input);
                let lit: LitInt = content.parse()?;
                let n: u16 = lit.base10_parse()?;

                tag = Some(n);

                return Ok(());
            }

            Err(meta.error("Unrecognised encode"))
        }) {
            panic!("{}", e);
        }
    }

    let tag = tag.expect("Attribute `encode(tag(value))` must specify a tag.");

    if tag <= TypeTag::STD_MAX {
        panic!(
            "Attribute `encode(tag(value))` must be higher than {}",
            TypeTag::STD_MAX
        );
    }

    let struct_decode = decode_struct(&input.data);

    let output = quote! {
        impl #impl_generics tagged_binary_serialization::decode::TagDecode for #name #ty_generics #where_clause {
            const DECODE_TAG: u16 = #tag;

            fn decode(bytes: &[u8]) -> Result<(Self, usize), tagged_binary_serialization::errors::SerializationError>
            where
                Self: Sized,
            {
                let tag_bytes: [u8; 2] = (&bytes[0..2])
                    .try_into()
                    .map_err(|_| tagged_binary_serialization::errors::SerializationError::NoTag)?;

                let tag = u16::from_le_bytes(tag_bytes);

                if tag != Self::DECODE_TAG {
                    return Err(tagged_binary_serialization::errors::SerializationError::MismatchedTag(Self::DECODE_TAG, tag));
                }

                let mut offset = 2;

                #struct_decode

                Ok((value, offset))
            }
        }
    };

    output.into()
}

fn add_decode_trait_bounds(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(tagged_binary_serialization::decode::TagDecode));
        }
    }

    generics
}

fn decode_struct(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;

                    quote_spanned! {f.span() =>
                        let (#name, size): (#ty, usize) = tagged_binary_serialization::decode::TagDecode::decode(&bytes[offset..])?;
                        offset += size;
                    }
                });

                let field_names = fields.named.iter().map(|f| &f.ident);

                quote! {
                    #(#recurse)*

                    let value = Self {
                        #(#field_names),*
                    };
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[proc_macro_derive(TagEncode, attributes(encode))]
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let generics = add_encode_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut tag: Option<u16> = None;
    for attr in input.attrs.iter() {
        if !attr.path().is_ident("encode") {
            continue;
        }

        if let Err(e) = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("tag") {
                let content;
                parenthesized!(content in meta.input);
                let lit: LitInt = content.parse()?;
                let n: u16 = lit.base10_parse()?;

                tag = Some(n);

                return Ok(());
            }

            Err(meta.error("Unrecognised encode"))
        }) {
            panic!("{}", e);
        }
    }

    let tag = tag.expect("Attribute `encode(tag(value))` must specify a tag.");
    let field_encode = encode_fields(&input.data);

    if tag <= TypeTag::STD_MAX {
        panic!(
            "Attribute `encode(tag(value))` must be higher than {}",
            TypeTag::STD_MAX
        );
    }

    let output = quote! {
        impl #impl_generics tagged_binary_serialization::encode::TagEncode for #name #ty_generics #where_clause {
            const ENCODE_TAG: u16 = #tag;

            fn encode(&self) -> Vec<u8> {
                let mut bytes = Self::ENCODE_TAG.to_le_bytes().to_vec();
                #field_encode
                bytes.extend(field_bytes);

                bytes
            }
        }
    };

    output.into()
}

fn add_encode_trait_bounds(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(tagged_binary_serialization::encode::TagEncode));
        }
    }

    generics
}

fn encode_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;

                        quote_spanned! {f.span() => field_bytes.extend(tagged_binary_serialization::encode::TagEncode::encode(&self.#name));}
                    });

                quote! {
                    let mut field_bytes: Vec<u8> = Vec::new();
                    #(#recurse)*
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {}
