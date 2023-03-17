#[cfg(feature = "TbArrowIteration")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowIteration")]
/// *This icon requires the feature* `TbArrowIteration` *to be enabled*.
#[component]
pub fn ArrowIteration(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-iteration" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8.5 16a5.5 5.5 0 1 0 -5.5 -5.5v.5" /><path d="M3 16h18" /><path d="M18 13l3 3l-3 3" /></svg>
   }
}