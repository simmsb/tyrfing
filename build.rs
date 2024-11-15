use std::env;
use std::fs;
use std::path::Path;

use proc_macro2::TokenStream;
use quote::quote;

fn main() {
    let croak = small_morse::encode("croak")
        .map(|c| {
            let duration = c.duration;
            let on = c.state == small_morse::State::On;

            quote! {
                Morse { duration: #duration, on: #on }
            }
        })
        .collect::<Vec<_>>();

    let croak_len = croak.len();

    let mut g = TokenStream::new();
    g.extend(quote! {
        #[derive(Copy, Clone)]
        pub struct Morse {
            pub duration: u8,
            pub on: bool,
        }

        const CROAK_C: [Morse; #croak_len] = [
            #(#croak),*
        ];

        avr_progmem::progmem! {
            pub static progmem CROAK: [Morse; #croak_len] = CROAK_C;
        }
    });

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("croak.rs");

    fs::write(&dest_path, g.to_string()).unwrap();
}
