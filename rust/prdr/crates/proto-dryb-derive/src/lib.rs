extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = match ast.data {
        syn::Data::Struct(s) => impl_serialize_struct(name, s),
        syn::Data::Enum(e) => impl_serialize_enum(name, e),
        _ => panic!("Serialize only works with structs and enums"),
    };

    TokenStream::from(expanded)
}

fn impl_serialize_struct(name: &syn::Ident, s: syn::DataStruct) -> proc_macro2::TokenStream {
    match s.fields {
        Fields::Named(fields) => {
            let field_serializations = fields.named.iter().map(|f| {
                let field_name = &f.ident;
                quote! {
                    offset += self.#field_name.serialize(&mut buffer[offset..], endian)?;
                }
            });

            quote! {
                impl Serialize for #name {
                    fn serialize(&self, buffer: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
                        let mut offset = 0;
                        #(#field_serializations)*
                        Ok(offset)
                    }
                }
            }
        }
        _ => panic!("Serialize only works with structs that have named fields"),
    }
}

fn impl_serialize_enum(name: &syn::Ident, e: syn::DataEnum) -> proc_macro2::TokenStream {
    let variant_arms = e.variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        let (enum_fields, variant_pattern) = match &variant.fields {
            Fields::Named(fields) => {
                let field_names = fields
                    .named
                    .iter()
                    .map(|i| i.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();

                let field_calculations = fields
                    .named
                    .iter()
                    .map(|field| {
                        let field_name = &field.ident;
                        quote! {
                            offset += #field_name.serialize(&mut buf[offset..], endian)?;
                        }
                    })
                    .collect::<Vec<_>>();

                let pattern = quote! { #name::#variant_name { #(#field_names),* } };

                (field_calculations, pattern)
            }
            Fields::Unnamed(fields) => {
                let field_names: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| format_ident!("field{}", i))
                    .collect();

                let field_calculations = field_names
                    .iter()
                    .map(|field_name| {
                        quote! {
                            offset += #field_name.serialize(&mut buf[offset..], endian)?;
                        }
                    })
                    .collect::<Vec<_>>();

                let pattern = quote! { #name::#variant_name(#(#field_names),*) };

                (field_calculations, pattern)
            }
            Fields::Unit => (Vec::default(), quote! { #name::#variant_name }),
        };

        quote! {
            #variant_pattern => {
                #(#enum_fields)*
                #index as u8
            }
        }
    });

    quote! {
        impl Serialize for #name {
            fn serialize(&self, buf: &mut [u8], endian: Endianness) -> Result<usize, SerializeError> {
                if buf.len() < 1 {
                    return Err(SerializeError::BufferOverflow);
                }

                let mut offset = 1;

                buf[0] = match self {
                    #(#variant_arms,)*
                };

                Ok(offset)
            }
        }
    }
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = match ast.data {
        syn::Data::Struct(s) => impl_deserialize_struct(name, s),
        syn::Data::Enum(e) => impl_deserialize_enum(name, e),
        _ => panic!("Deserialize only works with structs and enums"),
    };

    TokenStream::from(expanded)
}

fn impl_deserialize_struct(name: &syn::Ident, s: syn::DataStruct) -> proc_macro2::TokenStream {
    match s.fields {
        Fields::Named(fields) => {
            let field_deserializations = fields.named.iter().map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty;
                quote! {
                    let (#field_name, size) = #field_type::deserialize(&buf[offset..], endian)?;
                    offset += size;
                }
            });

            let field_names = fields.named.iter().map(|f| {
                let field_name = &f.ident;
                quote! { #field_name }
            });

            quote! {
                impl Deserialize for #name {
                    fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
                        let mut offset = 0;
                        #(#field_deserializations)*
                        Ok((Self { #(#field_names),* }, offset))
                    }
                }
            }
        }
        _ => panic!("Deserialize only works with structs that have named fields"),
    }
}

fn impl_deserialize_enum(name: &syn::Ident, e: syn::DataEnum) -> proc_macro2::TokenStream {
    let variant_arms = e.variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        let (enum_fields, variant_pattern) = match &variant.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter()
                    .map(|i| i.ident.as_ref().unwrap())
                    .collect::<Vec<_>>();

                let field_calculations = fields
                    .named
                    .iter()
                    .map(|field| {
                        let field_name = &field.ident;
                        let field_type = &field.ty;

                        return quote! {
                            let (#field_name, size) = #field_type::deserialize(&buf[offset..], endian)?;
                            offset += size;
                        };
                    })
                    .collect::<Vec<_>>();

                let pattern = quote! { #name::#variant_name { #(#field_names),* } };

                (field_calculations, pattern)
            }
            Fields::Unnamed(fields) => {
                let count = fields.unnamed.len();
                let field_names = (0..count)
                    .map(|_| format_ident!("x"))
                    .collect::<Vec<_>>();

                let field_calculations = fields
                    .unnamed
                    .iter()
                    .map(|field| {
                        let field_type = &field.ty;

                        return quote! {
                            let (x, size) = #field_type::deserialize(&buf[offset..], endian)?;
                            offset += size;
                        };
                    })
                    .collect::<Vec<_>>();

                let pattern = quote! { #name::#variant_name(#(#field_names),*) };

                (field_calculations, pattern)
            }
            Fields::Unit => (Vec::default(), quote! { #name::#variant_name }),
        };
        let index_u8 = index as u8;

        quote! {
            #index_u8 => {
                #(#enum_fields)*
                #variant_pattern
            }
        }
    });

    quote! {
        impl Deserialize for #name {
            fn deserialize(buf: &[u8], endian: Endianness) -> Result<(Self, usize), DeserializeError> {
                if buf.len() < 1 {
                    return Err(DeserializeError::Invalid);
                }

                let mut offset = 1;

                Ok((match buf[0] {
                    #(#variant_arms,)*
                    _ => return Err(DeserializeError::Invalid),
                }, offset))
            }
        }
    }
}
