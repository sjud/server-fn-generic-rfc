pub mod app;
// Generic with a single implementation of an SSR only type
pub mod example_1;
// Generic with multiple implementations of an SSR only type
pub mod example_2;
// Generic with wih a single implementation but it uses a SSR only type and a SHARED type.
pub mod example_3;
// Generic with multiple implementations, each use multiple SSR only types and SHARED types.
pub mod example_4;
// Generic with a single implementation, it uses SSR only type and SHARED type, and each of which have trait bounds in the server function that are
// both shared and not shared.
pub mod example_5;
// Generic return type
pub mod example_6;
// generic return type and error type
pub mod example_7;
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
