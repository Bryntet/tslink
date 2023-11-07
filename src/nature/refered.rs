use crate::{
    context::Context,
    error::E,
    nature::{Nature, VariableTokenStream, RustTypeName},
};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

#[derive(Clone, Debug)]
pub enum Refered {
    // Name, Context, Fields
    Struct(String, Context, Vec<Nature>),
    // Name, Context, Variants
    Enum(String, Context, Vec<Nature>),
    // name, context, values, is_flat
    EnumVariant(String, Context, Vec<Nature>, bool),
    // Name, Context, FuncNature
    Func(String, Context, Box<Nature>),
    // Name, Context, Nature, Binding
    Field(String, Context, Box<Nature>, Option<String>),
    // Name, Context, Nature, Binding
    FuncArg(String, Context, Box<Nature>, Option<String>),
    // Name
    Ref(String),
    // Alias, Context, Nature
    Generic(String, Context, Box<Nature>)
}

impl Refered {
    pub fn is_flat_varians(variants: &[Nature]) -> Result<bool, E> {
        for variant in variants {
            if let Nature::Refered(Refered::EnumVariant(_, _, values, _)) = variant {
                if !values.is_empty() {
                    return Ok(false);
                }
            } else {
                return Err(E::Parsing(String::from("Given Nature isn't enum varian")));
            }
        }
        Ok(true)
    }

    pub fn is_enum_flat(&self) -> Result<bool, E> {
        if let Refered::Enum(_, _, variants) = self {
            Refered::is_flat_varians(variants)
        } else {
            Err(E::Parsing(String::from("Given Nature isn't enum")))
        }
    }
}

impl VariableTokenStream for Refered {
    fn variable_token_stream(&self, var_name: &str, err: Option<&Nature>) -> Result<TokenStream, E> {
        let var_name = format_ident!("{}", var_name);
        match self {   
            Self::Ref(_) => {
                Ok(if let Some(nature) = err {
                    let err_type_ref = format_ident!("{}", nature.rust_type_name()?);
                    quote! {
                        serde_json::to_string(&#var_name).map_err(|e| Into::<#err_type_ref>::into(e))?
                    }
                } else {
                    quote! {
                        serde_json::to_string(&#var_name).expect("Converting to JSON string")
                    }
                })
            },
            _ => {
                Err(E::Parsing(format!("Only reference to entity (struct / enum) can be convert into JSON string (var: {var_name})")))
            }
        }
    }
}

impl RustTypeName for Refered {
    fn rust_type_name(&self) -> Result<String, E> {
        match self {   
            Self::Ref(ref_name) => {
                Ok(ref_name.to_owned())
            },
            _ => {
                Err(E::Parsing("Only reference to entity (struct / enum) can be convert String".to_string()))
            }
        }
    }
}
