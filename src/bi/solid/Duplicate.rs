#[cfg(feature = "BiSolidDuplicate")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDuplicate")]
/// *This icon requires the feature* `BiSolidDuplicate` *to be enabled*.
#[component]
pub fn Duplicate(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 22h12a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2zm2-9h3v-3h2v3h3v2h-3v3H9v-3H6v-2z" /><path d="M20 2H8v2h12v12h2V4c0-1.103-.897-2-2-2z" /></svg>
   }
}