use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::{error::NoCustomError, ServerFn};
use std::marker::PhantomData;

#[component]
pub fn GenericServerFunctionExample2() -> impl IntoView {
    let action = ServerAction::<
        GenericFn<SsrOnlyStructPhantom>,
    >::new();
    let action2 = ServerAction::<
        GenericFn<SsrOnlyStruct2Phantom>,
    >::new();

    Effect::new(move |_| {
        action.dispatch(
            GenericFn::<SsrOnlyStructPhantom> {
                _marker: PhantomData,
            },
        );
        action2.dispatch(GenericFn::<
            SsrOnlyStruct2Phantom,
        > {
            _marker: PhantomData,
        });
    });
}

#[cfg(feature = "ssr")]
pub struct SsrOnlyStruct;
#[cfg(feature = "ssr")]
pub struct SsrOnlyStruct2;

#[cfg(feature = "ssr")]
pub trait SsrOnlyTrait {}

#[cfg(feature = "ssr")]
impl SsrOnlyTrait for SsrOnlyStruct {}
#[cfg(feature = "ssr")]
impl SsrOnlyTrait for SsrOnlyStruct2 {}

#[cfg(feature = "ssr")]
pub trait ServerType {
    type ServerType;
}

#[derive(Clone)]
pub struct SsrOnlyStructPhantom;
#[derive(Clone)]
pub struct SsrOnlyStruct2Phantom;
pub trait SsrOnlyTraitConstraint {}
impl SsrOnlyTraitConstraint for SsrOnlyStructPhantom {}
impl SsrOnlyTraitConstraint for SsrOnlyStruct2Phantom {}

#[cfg(feature = "ssr")]
impl ServerType for SsrOnlyStructPhantom {
    type ServerType = SsrOnlyStruct;
}
#[cfg(feature = "ssr")]
impl ServerType for SsrOnlyStruct2Phantom {
    type ServerType = SsrOnlyStruct2;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericFn<
    T: SsrOnlyTraitConstraint + Clone + Send + 'static,
> {
    #[serde(skip)]
    _marker: PhantomData<T>,
}

cfg_if::cfg_if! {
    if #[cfg(feature="hydrate")] {
        impl ServerFn for GenericFn<SsrOnlyStructPhantom> {
            const PATH: &'static str = const_format::concatcp!("/api/generic_server_fn_2",stringify!(SsrOnlyStructPhantom));
            type Client = ::leptos::server_fn::client::browser::BrowserClient;
            type ServerRequest = ::leptos::server_fn::request::BrowserMockReq;
            type ServerResponse = ::leptos::server_fn::response::BrowserMockRes;
            type Output = ();
            type InputEncoding = ::leptos::server_fn::codec::PostUrl;
            type OutputEncoding = ::leptos::server_fn::codec::Json;
            type Error = ::leptos::server_fn::error::NoCustomError;
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
            async fn run_body(self) -> Result<(), ServerFnError> {
                panic!("internal error: entered unreachable code")
            }
        }
    } else {

        impl ServerFn for GenericFn<SsrOnlyStructPhantom>
        {
            const PATH: &'static str = const_format::concatcp!("/api/generic_server_fn_2",stringify!(SsrOnlyStructPhantom));

            type Client = ::leptos::server_fn::client::browser::BrowserClient;
            type ServerRequest =
                ::leptos::server_fn::http_export::Request<::leptos::server_fn::axum_export::body::Body>;
            type ServerResponse =
                ::leptos::server_fn::http_export::Response<::leptos::server_fn::axum_export::body::Body>;
            type Output = ();
            type InputEncoding = ::leptos::server_fn::codec::PostUrl;
            type OutputEncoding = ::leptos::server_fn::codec::Json;
            type Error = ::leptos::server_fn::error::NoCustomError;
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
            fn run_body(self) -> impl std::future::Future<Output = Result<(), ServerFnError>> + Send {
                async move {
                    let GenericFn::<SsrOnlyStructPhantom> { _marker } = self;
                    __generic_fn::<<SsrOnlyStructPhantom as ServerType>::ServerType>().await
                }
            }
        }

    }
}
cfg_if::cfg_if! {
    if #[cfg(feature="hydrate")] {
        impl ServerFn for GenericFn<SsrOnlyStruct2Phantom> {
            const PATH: &'static str = const_format::concatcp!("/api/generic_server_fn_2",stringify!(SsrOnlyStruct2Phantom));
            type Client = ::leptos::server_fn::client::browser::BrowserClient;
            type ServerRequest = ::leptos::server_fn::request::BrowserMockReq;
            type ServerResponse = ::leptos::server_fn::response::BrowserMockRes;
            type Output = ();
            type InputEncoding = ::leptos::server_fn::codec::PostUrl;
            type OutputEncoding = ::leptos::server_fn::codec::Json;
            type Error = ::leptos::server_fn::error::NoCustomError;
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
            async fn run_body(self) -> Result<(), ServerFnError> {
                panic!("internal error: entered unreachable code")
            }
        }
    } else {

        impl ServerFn for GenericFn<SsrOnlyStruct2Phantom>  {
            const PATH: &'static str = const_format::concatcp!("/api/generic_server_fn_2",stringify!(SsrOnlyStruct2Phantom));

            type Client = ::leptos::server_fn::client::browser::BrowserClient;
            type ServerRequest =
                ::leptos::server_fn::http_export::Request<::leptos::server_fn::axum_export::body::Body>;
            type ServerResponse =
                ::leptos::server_fn::http_export::Response<::leptos::server_fn::axum_export::body::Body>;
            type Output = ();
            type InputEncoding = ::leptos::server_fn::codec::PostUrl;
            type OutputEncoding = ::leptos::server_fn::codec::Json;
            type Error = ::leptos::server_fn::error::NoCustomError;
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
            fn run_body(self) -> impl std::future::Future<Output = Result<(), ServerFnError>> + Send {
                async move {
                    let GenericFn::<SsrOnlyStruct2Phantom> { _marker } = self;
                    __generic_fn::<<SsrOnlyStruct2Phantom as ServerType>::ServerType>().await
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
                    <GenericFn<SsrOnlyStructPhantom> as ServerFn>::PATH,
                    <GenericFn<SsrOnlyStructPhantom> as ServerFn>::InputEncoding::METHOD,
                    |req| Box::pin(<GenericFn<SsrOnlyStructPhantom>>::run_on_server(req)),
                    GenericFn::<SsrOnlyStructPhantom>::middlewares,
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
#[cfg(feature = "ssr")]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            {
                use ::leptos::server_fn::{codec::Encoding, ServerFn};
                ::leptos::server_fn::ServerFnTraitObj::new(
                    <GenericFn<SsrOnlyStruct2Phantom> as ServerFn>::PATH,
                    <GenericFn<SsrOnlyStruct2Phantom> as ServerFn>::InputEncoding::METHOD,
                    |req| Box::pin(<GenericFn<SsrOnlyStruct2Phantom>>::run_on_server(req)),
                    GenericFn::<SsrOnlyStruct2Phantom>::middlewares,
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
        pub async fn generic_fn<T: ServerType + SsrOnlyTraitConstraint>(
        ) -> Result<(), ServerFnError>
        where
            <T as ServerType>::ServerType: SsrOnlyTrait,
        {
            __generic_fn::<<T as ServerType>::ServerType>().await
        }
    } else {
        pub async fn generic_fn<
        T: SsrOnlyTraitConstraint + Clone + Send + 'static,
    >() -> Result<(), ServerFnError>
        where
        GenericFn<T>:ServerFn<Output = (), Error = NoCustomError>,
        {
        use ::leptos::server_fn::ServerFn;
        let data = GenericFn::<T> {
            _marker: PhantomData,
        };
        data.run_on_client().await
    }
    }
}

#[cfg(feature = "ssr")]
pub async fn __generic_fn<T: SsrOnlyTrait>(
) -> Result<(), ServerFnError> {
    println!("Type: {}", std::any::type_name::<T>());
    Ok(())
}
