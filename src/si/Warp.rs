#[cfg(feature = "SiWarp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiWarp")]
/// *This icon requires the feature* `SiWarp` *to be enabled*.
#[component]
pub fn Warp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12.035 2.723h9.253A2.712 2.712 0 0 1 24 5.435v10.529a2.712 2.712 0 0 1-2.712 2.713H8.047Zm-1.681 2.6L6.766 19.677h5.598l-.399 1.6H2.712A2.712 2.712 0 0 1 0 18.565V8.036a2.712 2.712 0 0 1 2.712-2.712Z" /></svg>
   }
}