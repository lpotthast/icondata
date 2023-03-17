#[cfg(feature = "IoPauseCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPauseCircleSharp")]
/// *This icon requires the feature* `IoPauseCircleSharp` *to be enabled*.
#[component]
pub fn PauseCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM224,336H192V176h32Zm96,0H288V176h32Z" /></svg>
   }
}