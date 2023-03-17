#[cfg(feature = "BiRegularArrowToBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularArrowToBottom")]
/// *This icon requires the feature* `BiRegularArrowToBottom` *to be enabled*.
#[component]
pub fn ArrowToBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 18h12v2H6zm5-14v8.586L6.707 8.293 5.293 9.707 12 16.414l6.707-6.707-1.414-1.414L13 12.586V4z" /></svg>
   }
}