//! Create `ffigen::RustType`s from code.
extern crate proc_macro;

use std::str::FromStr;

use ffigen::*;
use proc_macro::TokenStream;
use syn::*;
use quote::ToTokens;

#[proc_macro_derive(FfiGen)]
pub fn gen_ffi(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);
    let ty = convert(macro_input);
    let res = ty.to_string();

	//use std::io::Write;
	//let mut file = std::fs::OpenOptions::new().append(true).open("/tmp/ffigen-output.rs").unwrap();
	//write!(file, "{}", t).unwrap();
	//std::fs::write("/tmp/ffigen-output.rs", res.as_bytes()).unwrap();
    match proc_macro::TokenStream::from_str(&res) {
    	Ok(r) => r,
    	Err(e) => {
    		panic!("Failed to parse output as rust code: {:?}", e);
    	}
    }
}

fn fields_to_struct(fields: &Fields) -> Struct {
    let mut struc = Struct { fields: Vec::new() };
    match fields {
    	Fields::Named(n) => for f in n.named.iter() {
    		let n = f.ident.as_ref().map(|i| i.to_string()).unwrap_or_else(String::new);
    		let ty = f.ty.clone().into_token_stream().to_string();
    		struc.fields.push((n, convert_type(&ty).into()));
		}
    	Fields::Unnamed(n) => for f in n.unnamed.iter() {
    		let n = f.ident.as_ref().map(|i| i.to_string()).unwrap_or_else(String::new);
    		let ty = f.ty.clone().into_token_stream().to_string();
    		struc.fields.push((n, convert_type(&ty).into()));
		}
    	Fields::Unit => {}
	}
	struc
}

fn convert(input: DeriveInput) -> RustType {
    let name = input.ident.to_string();
    match input.data {
    	Data::Struct(s) => {
			RustType {
				name,
				content: TypeContent::Struct(fields_to_struct(&s.fields)),
			}
		}
    	Data::Enum(e) => {
    		let mut en = Enum { possibilities: Vec::new() };
    		for v in e.variants {
    			let prefix = v.ident.to_string();
    			en.possibilities.push((prefix, fields_to_struct(&v.fields)));
    		}
			RustType {
				name,
				content: TypeContent::Enum(en),
			}
		}
		_ => panic!("Only structs or enums are supported"),
	}
}

/// Replace types
fn convert_type(t: &str) -> String {
	match t {
		"ConnectionId" | "FutureHandle" => "u64".into(),
		"MessageTarget" => "u8".into(),
		s => s.into(),
	}
}
