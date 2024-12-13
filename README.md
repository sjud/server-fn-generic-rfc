# Generic Server Function RFC

### Comparing current attempts
There's a current ongoing PR
https://github.com/leptos-rs/leptos/pull/3008
that addresses this.
That solution proposes generic in server function as inputs only, and suggests creating explicit calls in the main to register the function.

My solution addresses using generics that are used anywhere.

in the body only

```rust
async fn generic_fn<T>() -> Result<(),ServerFnError>
```

in input

```rust
async fn generic_fn<T>(t:T) -> Result<(),ServerFnError>
```

in Result<T,ServerFnError<E>> as T or E

```rust
async fn generic_fn<T,E>() -> Result<T,ServerFnError<E>>
```

It solves for a SSR only T in a generic function, or for a trait that is only implemented on the server.

```rust
#[cfg(feature="ssr")]
pub struct SomeSsrOnlyType;
#[cfg(feature="ssr")]
pub trait SomeSsrOnlyTrait{}
#[cfg(feature="ssr")]
impl SsrOnlyTrait for SsrOnlyType;

#[server(
    register<SsrOnlyType>
)]
async fn generic_fn<T: SsrOnlyType + SomeSsrOnlyTrait + SsrOnlytrait>() -> Result<(),ServerFnError>

spawn(async move{ generic_fn::<SomeSsrOnlyTypePhantom>().await.unwrap()})
```

by auto generating shadow frontend types, traits and implementing a ServerTrait type which maps phantom types into their Server only types in the trait implementation.


or in any combination

```rust
#[cfg(feature="ssr")]
pub struct SomeSsrOnlyType;
#[cfg(feature="ssr")]
pub trait SomeSsrOnlyTrait{}
#[cfg(feature="ssr")]
impl SomeSsrOnlyTrait for SomeSsrOnlyType;
pub struct SharedType;
pub trait SharedTrait{}
#[cfg(feature="ssr")]
impl SomeSsrOnlyTrait for SharedType{}
impl SharedTrait for SharedType{}
impl SharedTrait for SomeSsrOnlyType{}

#[server(
    register<SomeSsrOnlyType,SharedType> = "endpoint"
)]
async fn generic_fn<T,T2>(t2:T2) -> Result<(),ServerFnError> where
    T:SsrOnlyTrait + SsrOnlyType + SomeSsrOnlyTrait, 
    T:SharedTrait
    T2: SsrOnlyTrait + SomeSsrOnlyTrait
    T2: SharedTrait {
    
}

spawn(async move {generic_fn::<SomeSsrOnlyTypePhantom,SharedType>(SharedType{}).await.unwrap()})

```

### Implications

This lets us specify our backend from our frontend, which sounds silly but bear with me. 

```rust
#[component]
pub fn ShowThing<Backend:BackendTraitConstraint>(id:usize) -> impl IntoView {
    Suspense::new(async move{ find_thing::<Backend>(id).await.unwrap().into_view()})
}
#[component]
pub fn ShowMultipleThings() -> impl IntoView {
    view!{
        <ShowThing<MockBackendPhantom> id = 0>
        <ShowThing<RealBackendPhantom> id = 0>
    }
}
#[server(register<MockBackend>,register<RealBackend>,)]
pub async fn find_thing::<Backend:SsrOnlyType + BackendTrait + SsrOnlyTrait>(id:usize) -> Result<Thing,ServerFnError> {
    Ok(Backend::find_thing(id).await?)
}
```

There's currently no way to have this type of functionality where your frontend code can (effectively) specify the behavior of the backend code it calls into.



### Constraints

There are specific constraints we are designing around which are as follows

- Generic Monomorphization happens after constants and statics are written.
- ServerFn::Path is 'static str
- Inventory requires 'static
So this code
```rust
impl<T> ServerFn for SomeType<T> {
    PATH :&'static str = const_format::concatp!("endpoint",stringify!(T))
}
```
will produce the same endpoint for all T
we could solve this setting path to be String and using the runtime lookup of type names 
but then we can't register our server functions via inventory

### Server Attribute extension

This solution proposes that the server macro have a `register` attribute which takes the generic parameters of the `register<...>` attribute and then creates
specific ServerFn implementation for each register item, and uses the types given as part of the unique hash for endpoint generation OR it uses the specified string after `'='` i.e `register<...> = "..."` as an endpoint and overrides the endpoint from `#[server(endpoint = "...")]`

## How it works

### Summary 
- Generic server functions must be annotated with `#[server(...)]`.
- Registrations via `#[server(register<T1,T2,...>)]` specify concrete type instantiations for generics and optionally a custom endpoint path.
- Endpoint uniqueness is derived from monomorphized types and optional custom endpoints.
- SSR-only traits and types in generics are transformed into phantom equivalents on the client and into monomorphized, fully resolved types on the server.
- Trait bounds are split into SSR-only constraint traits (server-only) and shared constraints (applied as-is on both sides). Phantom constraint traits stand in on the client.
- Code generation produces multiple `ServerFn` implementations, one per registered instantiation, plus associated `__generic_fn` helpers on the server side.
- Inventory integration is performed per registered instantiation, ensuring server functions are discoverable and invocable by the runtime.
- Return and error types that are generic follow the same phantom mapping and constraint propagation rules as inputs and parameters.
- Configuration via `#[cfg(feature="ssr")]` ensures that the correct code path (phantom substitution vs. real SSR types) is used depending on compilation mode.
- All original trait bounds are preserved with transformations for SSR-only traits as needed, ensuring the original function’s type-level contracts remain intact.

### Details


Suppose we have the following server function

```rust
#[server(
    register<SrrOnlyType,SharedType,ReturnType,ErrorType>
    register<SsrOnlyType2,SharedType,ReturnType,ErrorType> = "specific_endpoint"
)]
pub async fn generic_fn<T,T2,R,E>(t2:T2) -> Result<ReturnType,ServerFnError<ErrorType>>
    where
    T : SsrOnlyType,
    T : SomeServerTrait + SsrOnlyTrait,
    T : SharedTrait,
    T2: SharedTrait,
    T2: SsrOnlyTraitConsumedBySsrOnlyTrait + SsrOnly
    E: Default,
{
    let t = T::some_method(t2).await.map_err(|_|ServerFnError::WrappedServerError(E::default()))?;
    Ok(t)
}
```

`SsrOnlyType` and `SsrOnlyTrait`  trait is an empty trait that has a blanket implementation

```rust
impl<T> SsrOnlyTrait for T{}
impl<T> SsrOnlyType for T{}
```

for a given bound in the generic server function the following
if the type is bound by SsrOnlyType, 

i.e
```rust
#[server(register<SpecificT>)]
async fn generic_fn<T>() -> Result<(),ServerFnError> where T:SsrOnlyType {Ok(())}
spawn(async move{generic_fn::<SpecificT>().await;})
```

we generate

```rust
struct SpecificTPhantom;
#[cfg(feature="ssr")]
impl ServerType for SpecificTPhantom {
    type ServerType = SpecificT;
}
```

and we add to the generated function structure a marker

```rust
struct GenericFn<T> {
    _marker:PhantomData<T>
}
```

for the line 

```rust
T: SomeServerTrait + SsrOnlyTrait
```

We look at what traits are on the line with `SsrOnlyTrait` and we generate the following for each trait (except for the trait SsrOnlyType)

```rust
pub trait SsrOnlyTraitConstraint{}
```

and we generate implementations, 

if there exists a `PhantomTSpecific` such that for generic T bound by SsrOnlyType in the generic server fn (i.e `generic_fn<T:SsrOnlyType>()` ), there exists a SpecificT as in `register<SpecificT>` as attributed on the server procedural macro.

we generate

```rust
impl SomeServerTraitConstraint for SpecificTPhantom {}
```

or else we just use the specific type T as in `register<SpecificT>`

```rust
impl SomeServerTraitConstraint for SpecificT {}
```

and we extend the server function struct with the bound `SomeTraitConstraint` where the trait is bound on the same line SsrOnlyTrait at the generic function
i.e

```rust
struct GenericFn<T:SomeServerTraitConstraint> {
    _marker:PhantomData<T>
}
```

If there exists a trait bound for T that is not `SsrOnlyType` or on the line with the trait `SsrOnlyTrait`, we treat it as a trait bound for T without any modifications. For all trait X if X != SsrOnlyType && X != SsrOnlyTrait X maps to X on the server and the client.

If the generic appears in the result, we add it to the _marker PhantomData but don't create a phantom type.

and if the are multiple types in PhantomData we include them in a parentheses.

So finally we have 

```rust
struct GenericFn<T,T2,R,E> {
    _marker:PhantomData<(T,R,E)>,
    t2:T2,
}
let g = ServerAction::<GenericFn::<PhantomSpecifcT,SpecificT2,Result,Error>>::new();
g.dispatch(GenericFn{_marker:PhantomData,t2:SpecificT2::default()}) // or whatever
```

for each register attribute on server proc macro we generate a specific implementation of ServerFn for the server function structure

i.e

```rust
#[server(
    register<BackendType,String>,
    register<BackendType,usize> = "usize_endpoint",
)]
async fn generic_fn<T,S>(s:S) -> Result<(),ServerFnError>
    where T:SsrOnlyType + BackendTrait + SsrOnlyTrait {
    _ = s;
    Ok(())
}
```

->

```rust


cfg_if::cfg_if!{
    if #[cfg(feature="ssr")] {
impl ServerFn for GenericFn<BackendTypePhantom,String> {
    PATH = {
        // we add stringify!(String) to code to generate the function hash to it's unique per path
    }
    // if the output is generic we specify it here
    type Output = ();
    // if the error is generic we specify it here
    type Error = ::leptos::server_fn::error::NoCustomError;
    #[allow(clippy::manual_async_fn)]
    // change the output if needed
            fn run_body(self) -> impl std::future::Future<Output = Result<(), ServerFnError>> + Send {
                async move {
                    let GenericFn::<BackendTypePhantom,String> { _marker,s } = self;
                    __generic_fn::<<BackendTypePhantom as ServerType>::ServerType,String>(s).await
                }
            }
}
    } else {
        impl ServerFn for GenericFn<BackendTypePhantom,String> {
            // the only differences here are the ones specified by the server attributes not covered by this RFC, i.e Client, Req, Resp etc.
        }
    }
}

cfg_if::cfg_if!{
    if #[cfg(feature="ssr")] {
impl ServerFn for GenericFn<BackendTypePhantom,usize> {
    PATH = {
        // we ad stringify!(String) to code to generate the function hash to it's unique per path
    }
    // if the output is generic we specify it here
    type Output = ();
    // if the error is generic we specify it here
    type Error = ::leptos::server_fn::error::NoCustomError;
    #[allow(clippy::manual_async_fn)]
    // change the output if needed
            fn run_body(self) -> impl std::future::Future<Output = Result<(), ServerFnError>> + Send {
                async move {
                    let GenericFn::<BackendTypePhantom,usize> { _marker,s } = self;
                    __generic_fn::<<BackendTypePhantom as ServerType>::ServerType,usize>(s).await
                }
            }
}
    } else {
        impl ServerFn for GenericFn<PhantomBackendType,usize> {
            // the only differences here are the ones specified by the server attributes not covered by this RFC, i.e Client, Req, Resp etc.
        }
    }
}

// we generate inventory code for each
#[cfg(feature = "ssr")]
const _: () = {
    static __INVENTORY: ::inventory::Node =
        ::inventory::Node {
            value: &{
                {
                    use ::leptos::server_fn::{codec::Encoding, ServerFn};
                    ::leptos::server_fn::ServerFnTraitObj::new(
                    <GenericFn<BackendTypePhantom,usize> as ServerFn>::PATH,
                    <GenericFn<BackendTypePhantom,usize> as ServerFn>::InputEncoding::METHOD,
                    |req| Box::pin(<GenericFn<BackendTypePhantom,usize>>::run_on_server(req)),
                    GenericFn::<BackendTypePhantom,usize>::middlewares,
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
#[cfg(feature = "ssr")]
const _: () = {
    static __INVENTORY: ::inventory::Node =
        ::inventory::Node {
            value: &{
                {
                    use ::leptos::server_fn::{codec::Encoding, ServerFn};
                    ::leptos::server_fn::ServerFnTraitObj::new(
                    <GenericFn<BackendTypePhantom,String> as ServerFn>::PATH,
                    <GenericFn<BackendTypePhantom,String> as ServerFn>::InputEncoding::METHOD,
                    |req| Box::pin(<GenericFn<BackendTypePhantom,String>>::run_on_server(req)),
                    GenericFn::<BackendTypePhantom,String>::middlewares,
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
```

and for the actual function

```rust
// and we remove the original generic function and replace it with 

cfg_if::cfg_if! {
    if #[cfg(feature="ssr")] {
        pub async fn generic_fn<T,S>(
            s:S,
        ) -> Result<(), ServerFnError>
        where
            T: ServerType + BackendTraitConstraint
            <T as ServerType>::ServerType: BackendTrait,
        {
            __generic_fn::<<T as ServerType>::ServerType,S>(s).await
        }
    } else {
        pub async fn generic_fn<T,S>(
        s:S
    ) -> Result<(), ServerFnError>
        where
        T: BackendTraitConstraint + Clone + Send + 'static,
        GenericFn<T,S>:ServerFn<Output = (), Error = NoCustomError>,
        {
        use ::leptos::server_fn::ServerFn;
        let data = GenericFn::<T,S> {
            _marker: PhantomData,
            s,
        };
        data.run_on_client().await
    }
    }
}

#[cfg(feature = "ssr")]
pub async fn __generic_fn<T,S>(s:S) -> Result<(), ServerFnError>
    where
        T:BackendTrait {
    _ = s;
    Ok(())
}
```


If we have other trait bounds on our original server function we need to propagate those bounds to our new server function i.e

```rust
#[server(
    register<String>
)]
pub async fn do_default<T:Default>() -> Result<T,ServerFnError>{
    Ok(T::default())
}
```

->

```rust
cfg_if::cfg_if! {
    if #[cfg(feature="ssr")] {

        pub async fn generic_fn<T>() -> Result<T, ServerFnError> 
        where
            // here
            T : Default
         {
            __generic_fn::<T>().await
        }
    } else {
        pub async fn generic_fn<T>() -> Result<T, ServerFnError>
        where
        GenericFn<T>:ServerFn<Output=T,Error=NoCustomError>
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
pub async fn __generic_fn<T>() -> Result<T, ServerFnError> 
    where 
    // and here
    T: Default {
    Ok(T::default())
}
```

### Free stuff

Everyone likes free stuff. Because we are doing our own monomorphization process we can write any additional implementation that we want, so while we write `impl ServerFn for GenericFn<String>` we can also throw in (for free) `impl From<String> for GenericFn<String>`

### Additional Notes

TPhantom should derive default,
Deriving default on a struct that holds `PhantomData<T>` when `T` doesn't implement `Default` can erroneously cause https://github.com/rust-lang/rust/issues/26925
and instead of using https://docs.rs/derive-where/latest/derive_where/ on our server functions structure we can just always derive default for our phantom types.

### Complexity
We introduce two new blanket traits and a new server function attribute, but everything is additive. If you want to use generic server functions with only generic inputs it looks like this

```rust

#[server(register<String>,register<usize>)]
pub async fn generic_fn<S>(s:S) -> Result<(),ServerFnError> {
    _ = s;
    Ok(())
}
spawn(async move(generic_fn(String::from("Hello, world.")).await.unwrap();));
```

But when you want to start talking about the ssr/hydrate divide from your frontend type model then we now need to think about the addtional types. Which type is ssr only, what traits are ssr only etc. 

This doesn't change any other code and has no breaking changes. # server-fn-generic-rfc
