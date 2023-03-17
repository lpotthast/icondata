#[cfg(feature = "RiUserFillGenderless")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiUserFillGenderless")]
/// *This icon requires the feature* `RiUserFillGenderless` *to be enabled*.
#[component]
pub fn Genderless(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M11 7.066V1h2v6.066A7.501 7.501 0 0 1 12 22a7.5 7.5 0 0 1-1-14.934z" /></g></svg>
   }
}