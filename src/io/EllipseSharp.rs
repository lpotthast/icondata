#[cfg(feature = "IoEllipseSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoEllipseSharp")]
/// *This icon requires the feature* `IoEllipseSharp` *to be enabled*.
#[component]
pub fn EllipseSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,464C141.31,464,48,370.69,48,256S141.31,48,256,48s208,93.31,208,208S370.69,464,256,464Z" /></svg>
   }
}