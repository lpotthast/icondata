#[cfg(feature = "IoIdCardSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoIdCardSharp")]
/// *This icon requires the feature* `IoIdCardSharp` *to be enabled*.
#[component]
pub fn IdCardSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" id="icons" viewBox="0 0 512 512"><path d="M408,16H104A24,24,0,0,0,80,40V472a24,24,0,0,0,24,24H408a24,24,0,0,0,24-24V40A24,24,0,0,0,408,16ZM346.9,312.77a43,43,0,1,1-40.71-40.71A43,43,0,0,1,346.9,312.77ZM192,64H320V96H192ZM384,448H224V423.4c0-32.72,53.27-49.21,80-49.21s80,16.49,80,49.21Z" /></svg>
   }
}