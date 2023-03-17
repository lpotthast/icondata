#[cfg(feature = "BiSolidToggleRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidToggleRight")]
/// *This icon requires the feature* `BiSolidToggleRight` *to be enabled*.
#[component]
pub fn ToggleRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16 6H8c-3.296 0-5.982 2.682-6 5.986v.042A6.01 6.01 0 0 0 8 18h8c3.309 0 6-2.691 6-6s-2.691-6-6-6zm0 9c-1.627 0-3-1.373-3-3s1.373-3 3-3 3 1.373 3 3-1.373 3-3 3z" /></svg>
   }
}