#[cfg(feature = "OcSmRepoPull")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmRepoPull")]
/// *This icon requires the feature* `OcSmRepoPull` *to be enabled*.
#[component]
pub fn RepoPull(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M11 7h1v6c0 .55-.45 1-1 1H6v2l-1.5-1.5L3 16v-2H1c-.55 0-1-.45-1-1V1c0-.55.45-1 1-1h10c.55 0 1 .45 1 1v2h-1V1H2v9h9Zm2 1V6H7V4h6V2l3 3Zm-2 3H1v2h2v-1h3v1h5ZM4 2v1H3V2Zm0 2v1H3V4Zm0 2v1H3V6ZM3 9V8h1v1Z" /></svg>
   }
}