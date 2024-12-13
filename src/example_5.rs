use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::{error::NoCustomError, ServerFn};
use std::marker::PhantomData;

#[component]
pub fn GenericServerFunctionExample5() -> impl IntoView {
    let action = ServerAction::<
        GenericFn<SsrOnlyStructPhantom, SharedStruct>,
    >::new();

    Effect::new(move |_| {
        action.dispatch(GenericFn::<
            SsrOnlyStructPhantom,
            SharedStruct,
        > {
            _marker: PhantomData,
            shared_type: SharedStruct {
                inner: String::from("Hello, world."),
            },
        });
    });
}

pub trait SsrOnly {}

impl<T> SsrOnly for T{}

#[derive(Serialize, Deserialize, Clone)]
pub struct SharedStruct {
    inner: String,
}
impl SharedTrait for SharedStruct {}
#[cfg(feature = "ssr")]
impl SsrOnlyTrait for SharedStruct {}

pub trait SharedTrait {}

#[cfg(feature = "ssr")]
pub struct SsrOnlyStruct;

#[cfg(feature = "ssr")]
pub trait SsrOnlyTrait {}

#[cfg(feature = "ssr")]
impl SsrOnlyTrait for SsrOnlyStruct {}

// If the trait is shared by the type is not shared the implementation needs to be ssr only
#[cfg(feature = "ssr")]
impl SharedTrait for SsrOnlyStruct {}

#[cfg(feature = "ssr")]
pub trait ServerType {
    type ServerType;
}

#[derive(Clone)]
pub struct SsrOnlyStructPhantom;

pub trait SsrOnlyTraitConstraint {}
impl SsrOnlyTraitConstraint for SsrOnlyStructPhantom {}

#[cfg(feature = "ssr")]
impl ServerType for SsrOnlyStructPhantom {
    type ServerType = SsrOnlyStruct;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericFn<
    T: SsrOnlyTraitConstraint + Clone + Send + 'static,
    S: SharedTrait,
> {
    #[serde(skip)]
    _marker: PhantomData<T>,
    shared_type: S,
}

cfg_if::cfg_if! {
    if #[cfg(feature="hydrate")] {
        impl ServerFn for GenericFn<SsrOnlyStructPhantom,SharedStruct> {
            const PATH: &'static str = const_format::concatcp!("/api/generic_server_fn",stringify!(SsrOnlyStructPhantom),stringify!(SharedStruct));

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

        impl ServerFn for GenericFn<SsrOnlyStructPhantom,SharedStruct> {

            const PATH: &'static str = const_format::concatcp!("/api/generic_server_fn",stringify!(SsrOnlyStructPhantom),stringify!(SharedStruct));

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
                    let GenericFn::<SsrOnlyStructPhantom,SharedStruct> { _marker, shared_type } = self;
                    __generic_fn::<<SsrOnlyStructPhantom as ServerType>::ServerType, SharedStruct>(shared_type).await
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
                    <GenericFn<
                        SsrOnlyStructPhantom,
                        SharedStruct,
                    > as ServerFn>::PATH,
                    <GenericFn<
                        SsrOnlyStructPhantom,
                        SharedStruct,
                    > as ServerFn>::InputEncoding::METHOD,
                    |req| {
                        Box::pin(<GenericFn<
                            SsrOnlyStructPhantom,
                            SharedStruct,
                        >>::run_on_server(req))
                    },
                    GenericFn::<
                        SsrOnlyStructPhantom,
                        SharedStruct,
                    >::middlewares,
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
        pub async fn generic_fn<T: ServerType + SsrOnlyTraitConstraint,S:SharedTrait + SsrOnlyTrait>(
            shared_type:S
        ) -> Result<(), ServerFnError>
        where
            <T as ServerType>::ServerType: SsrOnlyTrait + SharedTrait,
        {
            __generic_fn::<<T as ServerType>::ServerType,S>(shared_type).await
        }
    } else {
        pub async fn generic_fn<
        T: SsrOnlyTraitConstraint + Clone + Send + 'static,
        S: SharedTrait,
    >(shared_type:S) -> Result<(), ServerFnError>
    where
    // we need a where clause which specifies that the implementation of server fn returns the return type Result<T,ServerFnError<E>>
        GenericFn<T,S>:ServerFn<Output = (), Error = NoCustomError>,{
        use ::leptos::server_fn::ServerFn;
        let data = GenericFn::<T,S> {
            _marker: PhantomData,
            shared_type,
        };
        data.run_on_client().await
    }
    }
}

#[cfg(feature = "ssr")]
pub async fn __generic_fn<
    T: SsrOnlyTrait + SharedTrait + SsrOnly,
    S: SsrOnlyTrait + SharedTrait,
>(
    shared_type: S,
) -> Result<(), ServerFnError> {
    println!(
        "Type: {} AND {}",
        std::any::type_name::<T>(),
        std::any::type_name::<S>()
    );
    Ok(())
}
