#[cfg(feature = "BiSolidHotel")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidHotel")]
/// *This icon requires the feature* `BiSolidHotel` *to be enabled*.
#[component]
pub fn Hotel(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="8" cy="11" r="3" /><path d="M18.205 7H12v8H4V6H2v14h2v-3h16v3h2v-4c0-.009-.005-.016-.005-.024H22V11c0-2.096-1.698-4-3.795-4z" /></svg>
   }
}