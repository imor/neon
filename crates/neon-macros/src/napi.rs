pub(crate) fn init(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let name = &sig.ident;

    quote::quote!(
        #(#attrs) *
        #vis #sig {
            #[no_mangle]
            unsafe extern "C" fn napi_register_module_v1(
                env: ::neon::macro_internal::runtime::nodejs_sys::napi_env,
                m: ::neon::macro_internal::runtime::nodejs_sys::napi_value,
            ) -> ::neon::macro_internal::runtime::nodejs_sys::napi_value {
                // Suppress the default Rust panic hook, which prints diagnostics to stderr.
                #[cfg(not(feature = "default-panic-hook"))]
                ::std::panic::set_hook(::std::boxed::Box::new(|_| { }));
    
                ::neon::macro_internal::initialize_module(
                    env,
                    ::std::mem::transmute(m),
                    #name,
                );
    
                m
            }

            #block
        }
    )
    .into()
}
