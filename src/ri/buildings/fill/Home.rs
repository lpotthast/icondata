#[cfg(feature = "RiBuildingsFillHome")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBuildingsFillHome")]
/// *This icon requires the feature* `RiBuildingsFillHome` *to be enabled*.
#[component]
pub fn Home(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M21 20a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V9.49a1 1 0 0 1 .386-.79l8-6.222a1 1 0 0 1 1.228 0l8 6.222a1 1 0 0 1 .386.79V20z" /></g></svg>
   }
}