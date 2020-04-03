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
                Err(e) => {
                    let msg = format!("Attempt to Unwind out of `rust` with err: {:?}", e);
                    #[cfg(not(target_os = "android"))]
                    eprintln!("{}", msg);
                    #[cfg(target_os = "android")]
                    old_glue::write_log(&msg);
                    std::process::abort()
                }
            }
        }

        #[no_mangle]
        #[inline(never)]
        pub extern "C" fn start_app() {
            #func
            stop_unwind(|| #name());
        }

        #[cfg(target_os = "android")]
        #[no_mangle]
        #[inline(never)]
        pub extern "C" fn android_main(app: *mut std::os::raw::c_void) {
            stop_unwind(|| {
                old_glue::android_main2(app as *mut _, move |_, _| {
                    start_app();
                });
            })
        }

        #[cfg(target_os = "android")]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn native_activity_on_create(
            activity: *mut std::os::raw::c_void,
            saved_state: *mut std::os::raw::c_void,
            saved_state_size: usize,
        ) {
            old_glue::ANativeActivity_onCreate(activity, saved_state, saved_state_size);
        }
    };
    //panic!("{}", expanded.to_string());
    expanded.into()
}
