#[cfg(feature = "BiSolidChart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidChart")]
/// *This icon requires the feature* `BiSolidChart` *to be enabled*.
#[component]
pub fn Chart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 21c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14zM9.553 9.658l4 2 1.553-3.105 1.789.895-2.447 4.895-4-2-1.553 3.105-1.789-.895 2.447-4.895z" /></svg>
   }
}