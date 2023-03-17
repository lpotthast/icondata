#[cfg(feature = "IoRemoveCircleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoRemoveCircleOutline")]
/// *This icon requires the feature* `IoRemoveCircleOutline` *to be enabled*.
#[component]
pub fn RemoveCircleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M448,256c0-106-86-192-192-192S64,150,64,256s86,192,192,192S448,362,448,256Z" style="fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" /><line x1="336" y1="256" x2="176" y2="256" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}