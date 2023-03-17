#[cfg(feature = "BsTypeH1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsTypeH1")]
/// *This icon requires the feature* `BsTypeH1` *to be enabled*.
#[component]
pub fn TypeH1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-type-h1" viewBox="0 0 16 16"><path d="M8.637 13V3.669H7.379V7.62H2.758V3.67H1.5V13h1.258V8.728h4.62V13h1.259zm5.329 0V3.669h-1.244L10.5 5.316v1.265l2.16-1.565h.062V13h1.244z" /></svg>
   }
}