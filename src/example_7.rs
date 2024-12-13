use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::{error::NoCustomError, ServerFn};
use std::marker::PhantomData;

#[component]
pub fn GenericServerFunctionExample7() -> impl IntoView {
    let action = ServerAction::<GenericFn<String, String>>::new();
    // let action2 = ServerAction::<GenericFn<u8>>::new();

    Effect::new(move |_| {
        action.dispatch(GenericFn::<String, String> {
            _marker: PhantomData,
        });
        /*action2.dispatch(GenericFn::<u8> {
            _marker: PhantomData,
        });*/
    });
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericFn<T, E> {
    #[serde(skip)]
    _marker: PhantomData<(T, E)>,
}

cfg_if::cfg_if! {
    if #[cfg(feature="hydrate")] {
        impl ServerFn for GenericFn<String,String> {
            const PATH: &'static str = const_format::concatcp!("/api/generic_server_fn",stringify!(String),stringify!(String));

            type Client = ::leptos::server_fn::client::browser::BrowserClient;
            type ServerRequest = ::leptos::server_fn::request::BrowserMockReq;
            type ServerResponse = ::leptos::server_fn::response::BrowserMockRes;
            type Output = String;
            type InputEncoding = ::leptos::server_fn::codec::PostUrl;
            type OutputEncoding = ::leptos::server_fn::codec::Json;
            type Error = String;
            fn middlewares() -> Vec<
                std::sync::Arc<
                    dyn ::leptos::server_fn::middleware::Layer<
                        ::leptos::server_fn::request::BrowserMockReq,
                        ::leptos::server_fn::response::BrowserMockRes,
                    >,
                >,
            > {
                Vec::new()
            }

            #[allow(unused_variables)]
            async fn run_body(self) -> Result<String, ServerFnError<String>> {
                panic!("internal error: entered unreachable code")
            }
        }
    } else {

        impl ServerFn for GenericFn<String,String> {

            const PATH: &'static str = const_format::concatcp!("/api/generic_server_fn",stringify!(String),stringify!(String));

            type Client = ::leptos::server_fn::client::browser::BrowserClient;
            type ServerRequest =
                ::leptos::server_fn::http_export::Request<::leptos::server_fn::axum_export::body::Body>;
            type ServerResponse =
                ::leptos::server_fn::http_export::Response<::leptos::server_fn::axum_export::body::Body>;
            type Output = String;
            type InputEncoding = ::leptos::server_fn::codec::PostUrl;
            type OutputEncoding = ::leptos::server_fn::codec::Json;
            type Error = String;
            fn middlewares() -> Vec<
                std::sync::Arc<
                    dyn ::leptos::server_fn::middleware::Layer<
                        ::leptos::server_fn::http_export::Request<
                            ::leptos::server_fn::axum_export::body::Body,
                        >,
                        ::leptos::server_fn::http_export::Response<
                            ::leptos::server_fn::axum_export::body::Body,
                        >,
                    >,
                >,
            > {
                Vec::new()
            }

            #[allow(clippy::manual_async_fn)]
            fn run_body(self) -> impl std::future::Future<Output = Result<String, ServerFnError<String>>> + Send {
                async move {
                    let GenericFn::<String,String> { _marker } = self;
                    __generic_fn::<String,String>().await
                }
            }
        }

    }
}

#[cfg(feature = "ssr")]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            {
                use ::leptos::server_fn::{codec::Encoding, ServerFn};
                ::leptos::server_fn::ServerFnTraitObj::new(
                    <GenericFn<String,String> as ServerFn>::PATH,
                    <GenericFn<String,String> as ServerFn>::InputEncoding::METHOD,
                    |req| Box::pin(<GenericFn<String,String>>::run_on_server(req)),
                    GenericFn::<String,String>::middlewares,
                )
            }
        },
        next: ::inventory::core::cell::UnsafeCell::new(
            ::inventory::core::option::Option::None,
        ),
    };
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ctor() {
        unsafe {
            ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
        }
    }
    #[used]
    #[link_section = ".init_array"]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};

cfg_if::cfg_if! {
    if #[cfg(feature="ssr")] {

        pub async fn generic_fn<T:Default,E:Default>(
        ) -> Result<T, ServerFnError<E>>  {
            __generic_fn::<T,E>().await
        }
    } else {
        pub async fn generic_fn<T,E>() -> Result<T, ServerFnError<E>>
        where
        GenericFn<T,E>:ServerFn<Output=T,Error=E>
        {
        use ::leptos::server_fn::ServerFn;
        let data = GenericFn::<T,E> {
            _marker: PhantomData,

        };
        data.run_on_client().await
    }
}
}

#[cfg(feature = "ssr")]
pub async fn __generic_fn<T: Default, E: Default>(
) -> Result<T, ServerFnError<E>> {
    println!(
        "Type: {} AND {}",
        std::any::type_name::<T>(),
        std::any::type_name::<E>(),
    );
    Err(ServerFnError::WrappedServerError(E::default()))
}