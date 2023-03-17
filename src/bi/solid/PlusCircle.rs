#[cfg(feature = "BiSolidPlusCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPlusCircle")]
/// *This icon requires the feature* `BiSolidPlusCircle` *to be enabled*.
#[component]
pub fn PlusCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm5 11h-4v4h-2v-4H7v-2h4V7h2v4h4v2z" /></svg>
   }
}