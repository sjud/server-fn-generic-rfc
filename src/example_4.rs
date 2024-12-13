use leptos::{prelude::*, reactive::spawn};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use server_fn::{error::NoCustomError, ServerFn};
use std::marker::PhantomData;

#[component]
pub fn GenericServerFunctionExample4() -> impl IntoView {
    let action = ServerAction::<
        GenericFn<SsrOnlyStructPhantom, SsrOnlyStruct2Phantom, String, usize>,
    >::new();
    let action2 = ServerAction::<
        GenericFn<SsrOnlyStruct3Phantom, SsrOnlyStruct4Phantom, i8, CustomType>,
    >::new();
    spawn(async move {
        generic_fn::<SsrOnlyStructPhantom, SsrOnlyStruct2Phantom, _, _>(
            String::from("Hello world"),
            0,
        )
        .await
        .unwrap();
    });
    Effect::new(move |_| {
        action.dispatch(
            GenericFn::<SsrOnlyStructPhantom, SsrOnlyStruct2Phantom, String, usize> {
                _marker: PhantomData,
                shared_type: String::from("Hello world."),
                shared_type_2: 0,
            },
        );
        action2.dispatch(
            GenericFn::<SsrOnlyStruct3Phantom, SsrOnlyStruct4Phantom, i8, CustomType> {
                _marker: PhantomData,
                shared_type: 0,
                shared_type_2: CustomType {
                    inner: String::from("Hello world."),
                },
            },
        );
    });
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomType {
    inner: String,
}

#[cfg(feature = "ssr")]
pub struct SsrOnlyStruct;
#[cfg(feature = "ssr")]
pub struct SsrOnlyStruct2;
#[cfg(feature = "ssr")]
pub struct SsrOnlyStruct3;
#[cfg(feature = "ssr")]
pub struct SsrOnlyStruct4;
#[cfg(feature = "ssr")]
pub trait SsrOnlyTrait {}
#[cfg(feature = "ssr")]
pub trait SsrOnlyTrait2 {}

#[cfg(feature = "ssr")]
impl SsrOnlyTrait for SsrOnlyStruct {}
#[cfg(feature = "ssr")]
impl SsrOnlyTrait for SsrOnlyStruct3 {}
#[cfg(feature = "ssr")]
impl SsrOnlyTrait2 for SsrOnlyStruct2 {}
#[cfg(feature = "ssr")]
impl SsrOnlyTrait2 for SsrOnlyStruct4 {}
#[cfg(feature = "ssr")]
pub trait ServerType {
    type ServerType;
}

#[derive(Clone)]
pub struct SsrOnlyStructPhantom;
#[derive(Clone)]
pub struct SsrOnlyStruct2Phantom;
#[derive(Clone)]
pub struct SsrOnlyStruct3Phantom;
#[derive(Clone)]
pub struct SsrOnlyStruct4Phantom;

pub trait SsrOnlyTraitConstraint {}
impl SsrOnlyTraitConstraint for SsrOnlyStructPhantom {}
impl SsrOnlyTraitConstraint for SsrOnlyStruct3Phantom {}
pub trait SsrOnlyTrait2Constraint {}
impl SsrOnlyTrait2Constraint for SsrOnlyStruct2Phantom {}
impl SsrOnlyTrait2Constraint for SsrOnlyStruct4Phantom {}

#[cfg(feature = "ssr")]
impl ServerType for SsrOnlyStructPhantom {
    type ServerType = SsrOnlyStruct;
}
#[cfg(feature = "ssr")]
impl ServerType for SsrOnlyStruct2Phantom {
    type ServerType = SsrOnlyStruct2;
}
#[cfg(feature = "ssr")]
impl ServerType for SsrOnlyStruct3Phantom {
    type ServerType = SsrOnlyStruct3;
}
#[cfg(feature = "ssr")]
impl ServerType for SsrOnlyStruct4Phantom {
    type ServerType = SsrOnlyStruct4;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericFn<
    T: SsrOnlyTraitConstraint + Clone + Send + 'static,
    T2: SsrOnlyTrait2Constraint + Clone + Send + 'static,
    S,
    S2,
> {
    #[serde(skip)]
    _marker: PhantomData<(T, T2)>,
    shared_type: S,
    shared_type_2: S2,
}

cfg_if::cfg_if! {
    if #[cfg(feature="hydrate")] {
        impl ServerFn for GenericFn<SsrOnlyStructPhantom,SsrOnlyStruct2Phantom,String,usize>  {
            const PATH: &'static str = const_format::concatcp!("/api/generic_fn",
            stringify!(SsrOnlyStructPhantom),stringify!(SsrOnlyStruct2Phantom),stringify!(String),stringify!(usize));

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

        impl ServerFn for GenericFn<SsrOnlyStructPhantom,SsrOnlyStruct2Phantom,String,usize>  {

 const PATH: &'static str = const_format::concatcp!("/api/generic_fn",
            stringify!(SsrOnlyStructPhantom),stringify!(SsrOnlyStruct2Phantom),stringify!(String),stringify!(usize));

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
                    let GenericFn::<SsrOnlyStructPhantom,SsrOnlyStruct2Phantom,String,usize> { _marker,shared_type,shared_type_2 } = self;
                    __generic_fn::<<SsrOnlyStructPhantom as ServerType>::ServerType,<SsrOnlyStruct2Phantom as ServerType>::ServerType
                    ,String,usize>(shared_type,shared_type_2).await
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
                        SsrOnlyStruct2Phantom,
                        String,
                        usize,
                    > as ServerFn>::PATH,
                    <GenericFn<
                        SsrOnlyStructPhantom,
                        SsrOnlyStruct2Phantom,
                        String,
                        usize,
                    > as ServerFn>::InputEncoding::METHOD,
                    |req| {
                        Box::pin(<GenericFn<
                            SsrOnlyStructPhantom,
                            SsrOnlyStruct2Phantom,
                            String,
                            usize,
                        >>::run_on_server(req))
                    },
                    <GenericFn<
                        SsrOnlyStructPhantom,
                        SsrOnlyStruct2Phantom,
                        String,
                        usize,
                    > as ServerFn>::middlewares,
                )
            }
        },
        next: ::inventory::core::cell::UnsafeCell::new(::inventory::core::option::Option::None),
    };
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ctor() {
        unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
    }
    #[used]
    #[link_section = ".init_array"]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};

cfg_if::cfg_if! {
    if #[cfg(feature="hydrate")] {
        impl ServerFn for GenericFn<SsrOnlyStruct3Phantom,SsrOnlyStruct4Phantom,i8,CustomType>  {
            const PATH: &'static str = const_format::concatcp!("/api/generic_fn",
            stringify!(SsrOnlyStruct3Phantom),stringify!(SsrOnlyStruct4Phantom),stringify!(i8),stringify!(CustomType));

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

        impl ServerFn for GenericFn<SsrOnlyStruct3Phantom,SsrOnlyStruct4Phantom,i8,CustomType>  {

 const PATH: &'static str = const_format::concatcp!("/api/generic_fn",
            stringify!(SsrOnlyStruct3Phantom),stringify!(SsrOnlyStruct4Phantom),stringify!(i8),stringify!(CustomType));

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
                    let GenericFn::<SsrOnlyStruct3Phantom,SsrOnlyStruct4Phantom,i8,CustomType> { _marker,shared_type,shared_type_2 } = self;
                    __generic_fn::<<SsrOnlyStruct3Phantom as ServerType>::ServerType,<SsrOnlyStruct4Phantom as ServerType>::ServerType
                    ,i8,CustomType>(shared_type,shared_type_2).await
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
                        SsrOnlyStruct3Phantom,
                        SsrOnlyStruct4Phantom,
                        i8,
                        CustomType,
                    > as ServerFn>::PATH,
                    <GenericFn<
                        SsrOnlyStruct3Phantom,
                        SsrOnlyStruct4Phantom,
                        i8,
                        CustomType,
                    > as ServerFn>::InputEncoding::METHOD,
                    |req| {
                        Box::pin(<GenericFn<
                            SsrOnlyStruct3Phantom,
                            SsrOnlyStruct4Phantom,
                            i8,
                            CustomType,
                        >>::run_on_server(req))
                    },
                    <GenericFn<
                        SsrOnlyStruct3Phantom,
                        SsrOnlyStruct4Phantom,
                        i8,
                        CustomType,
                    > as ServerFn>::middlewares,
                )
            }
        },
        next: ::inventory::core::cell::UnsafeCell::new(::inventory::core::option::Option::None),
    };
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ctor() {
        unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
    }
    #[used]
    #[link_section = ".init_array"]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};

cfg_if::cfg_if! {
    if #[cfg(feature="ssr")] {
        pub async fn generic_fn<T: ServerType + SsrOnlyTraitConstraint,T2:ServerType + SsrOnlyTrait2Constraint, S, S2>(
            shared_type:S,
            shared_type_2:S2,
        ) -> Result<(), ServerFnError>
        where
            <T as ServerType>::ServerType: SsrOnlyTrait,
            <T2 as ServerType>::ServerType: SsrOnlyTrait2,
        {
            __generic_fn::<<T as ServerType>::ServerType,<T2 as ServerType>::ServerType,S,S2>(shared_type,shared_type_2).await
        }
    } else {
        pub async fn generic_fn<
        T: SsrOnlyTraitConstraint + Clone + Send + 'static,
        T2: SsrOnlyTrait2Constraint + Clone + Send + 'static,
        S,
        S2,
    >(
        shared_type:S,
        shared_type_2:S2,
    ) -> Result<(), ServerFnError>
        where
        GenericFn<T,T2,S,S2>:ServerFn<Output = (), Error = NoCustomError>,
        {
        use ::leptos::server_fn::ServerFn;
        let data = GenericFn::<T,T2,S,S2> {
            _marker: PhantomData,
            shared_type,
            shared_type_2,
        };
        data.run_on_client().await
    }
    }
}

#[cfg(feature = "ssr")]
pub async fn __generic_fn<T: SsrOnlyTrait, T2: SsrOnlyTrait2, S, S2>(
    shared_type: S,
    shared_type_2: S2,
) -> Result<(), ServerFnError> {
    _ = shared_type;
    _ = shared_type_2;
    println!(
        "Type: {} AND {} AND {} AND {}",
        std::any::type_name::<T>(),
        std::any::type_name::<T2>(),
        std::any::type_name::<S>(),
        std::any::type_name::<S2>(),
    );
    Ok(())
}
