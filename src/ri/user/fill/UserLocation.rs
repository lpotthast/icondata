#[cfg(feature = "RiUserFillUserLocation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiUserFillUserLocation")]
/// *This icon requires the feature* `RiUserFillUserLocation` *to be enabled*.
#[component]
pub fn UserLocation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 14v8H4a8 8 0 0 1 8-8zm0-1c-3.315 0-6-2.685-6-6s2.685-6 6-6 6 2.685 6 6-2.685 6-6 6zm8.828 7.828L18 23.657l-2.828-2.829a4 4 0 1 1 5.656 0zM18 17a1 1 0 1 0 0 2 1 1 0 0 0 0-2z" /></g></svg>
   }
}