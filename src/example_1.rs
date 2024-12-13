use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFn;
use std::marker::PhantomData;

#[component]
pub fn GenericServerFunctionExample1() -> impl IntoView {
    let action = ServerAction::<
        GenericServerFnGenericSsrOnly<SsrOnlyStructPhantom>,
    >::new();

    Effect::new(move |_| {
        action.dispatch(
            GenericServerFnGenericSsrOnly::<SsrOnlyStructPhantom> {
                _marker: PhantomData,
            },
        );
    });
}

#[cfg(feature = "ssr")]
pub struct SsrOnlyStruct;

#[cfg(feature = "ssr")]
pub trait SsrOnlyTrait {}

#[cfg(feature = "ssr")]
impl SsrOnlyTrait for SsrOnlyStruct {}

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
pub struct GenericServerFnGenericSsrOnly<
    T: SsrOnlyTraitConstraint + Clone + Send + 'static,
> {
    #[serde(skip)]
    _marker: PhantomData<T>,
}
/*
    The problem with making the impl of ServerFn generic over the generic paramters of our server function structure is that
    we can't make the path unique based on the generic types since const/static creations happens before generic monomorphization.
    The problem with switching PATH to a String, is that then we can't use the inventory crate.
    So instead we can specify the specific implementation of the server function via an attribute
    i.e

    ```
    #[server]
    #[register<SpecificT> = "endpoint"]
    #[register<OtherSpecificT>]
    async fn server_fn ...
    ```

    And for each registered T we do our own monomorphization in the macro where we optionally take an endpoint and
    if so we override the endpoint (like the normal endpoint attribute)
    otherwise we generate a unique endpoint for each implementation by adding stringify!(OtherSpecificT) (in this example above)
    to the hash
*/
cfg_if::cfg_if! {
    if #[cfg(feature="hydrate")] {
        impl<T> ServerFn for GenericServerFnGenericSsrOnly<T>
        where
            T: SsrOnlyTraitConstraint + Clone + Send + 'static,
        {
            // problem what would this be for any given T ?
            // Each T needs a unique endpoint, since each T requires a unique handler and each unique handler requires a unique route i.e endpoint.
            // lazy proof:
            // Suppose for all T, T didn't require a unique handler function, then there'd be some function server_fn<T> which could handle any given type T.
            // but this function doesn't exist, because each actual function needs a specific T.
            // In fact, a generic_function<T> really is the set f functions such that for each T specified in code elsewhere there exists a generic_function<T> in the
            // created set
            // now suppose that each unique handler didn't require a unique endpoint
            // then then you could have two endpoints "a" -> fn_a1(a:A1), "a" -> fn_a2(a:A2)
            // now suppose you had to handle some package routed to "a", where would you route it to?
            // you'd have to know that it was either an A1 or an A2. (Since each type requires its own function)
            // so you'd have to add the information to the route i.e "a1" -> fn_a1(a:A1), "a2" -> fn_a2(a:A2)
            // but then for each handler you've created a unique route.
            const PATH: &'static str = "/api/generic_server_fn_generic_ssr_only";

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

        impl<T> ServerFn for GenericServerFnGenericSsrOnly<T>
        where
            T: SsrOnlyTraitConstraint + ServerType + Clone + Send + 'static,
            <T as ServerType>::ServerType: SsrOnlyTrait,
        {
            const PATH: &'static str = "/api/generic_server_fn_generic_ssr_only";

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
                    let GenericServerFnGenericSsrOnly::<T> { _marker } = self;
                    __generic_server_fn_generic_ssr_only::<<T as ServerType>::ServerType>().await
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
                    <GenericServerFnGenericSsrOnly<SsrOnlyStructPhantom> as ServerFn>::PATH,
                    <GenericServerFnGenericSsrOnly<SsrOnlyStructPhantom> as ServerFn>::InputEncoding::METHOD,
                    |req| Box::pin(<GenericServerFnGenericSsrOnly<SsrOnlyStructPhantom>>::run_on_server(req)),
                    GenericServerFnGenericSsrOnly::<SsrOnlyStructPhantom>::middlewares,
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
        pub async fn generic_server_fn_generic_ssr_only<T: ServerType + SsrOnlyTraitConstraint>(
        ) -> Result<(), ServerFnError>
        where
            <T as ServerType>::ServerType: SsrOnlyTrait,
        {
            __generic_server_fn_generic_ssr_only::<<T as ServerType>::ServerType>().await
        }
    } else {
        pub async fn generic_server_fn_generic_ssr_only<
        T: SsrOnlyTraitConstraint + Clone + Send + 'static,
    >() -> Result<(), ServerFnError> {
        use ::leptos::server_fn::ServerFn;
        let data = GenericServerFnGenericSsrOnly::<T> {
            _marker: PhantomData,
        };
        data.run_on_client().await
    }
    }
}

#[cfg(feature = "ssr")]
pub async fn __generic_server_fn_generic_ssr_only<T: SsrOnlyTrait>(
) -> Result<(), ServerFnError> {
    println!("{}", stringify!(T));
    Ok(())
}
