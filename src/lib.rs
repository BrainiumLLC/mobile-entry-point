#![recursion_limit = "256"]

use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn mobile_entry_point(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func = parse_macro_input!(input as syn::ItemFn);
    let name = &func.sig.ident;

    let expanded = quote! {
        fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                Ok(t) => t,
                Err(err) => {
                    eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
                    std::process::abort()
                }
            }
        }

        fn _start_app() {
            #func
            stop_unwind(|| #name());
        }

        #[no_mangle]
        #[inline(never)]
        pub extern "C" fn start_app() {
            _start_app()
        }

        #[cfg(target_os = "android")]
        ndk_glue::ndk_glue!(_start_app);
    };
    //panic!("{}", expanded.to_string());
    expanded.into()
}
