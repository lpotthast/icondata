#[cfg(feature = "BiSolidHardHat")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidHardHat")]
/// *This icon requires the feature* `BiSolidHardHat` *to be enabled*.
#[component]
pub fn HardHat(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 18v-3a8 8 0 0 0-5-7.4V13h-1V5h-4v8H9V7.6A8 8 0 0 0 4 15v3H2v2h20v-2z" /></svg>
   }
}