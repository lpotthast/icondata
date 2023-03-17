#[cfg(feature = "VsLinkExternal")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLinkExternal")]
/// *This icon requires the feature* `VsLinkExternal` *to be enabled*.
#[component]
pub fn LinkExternal(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M1.5 1H6v1H2v12h12v-4h1v4.5l-.5.5h-13l-.5-.5v-13l.5-.5z" /><path d="M15 1.5V8h-1V2.707L7.243 9.465l-.707-.708L13.293 2H8V1h6.5l.5.5z" /></svg>
   }
}