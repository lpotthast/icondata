#[cfg(feature = "IoFileTrayOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFileTrayOutline")]
/// *This icon requires the feature* `IoFileTrayOutline` *to be enabled*.
#[component]
pub fn FileTrayOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M384,80H128c-26,0-43,14-48,40L48,272V384a48.14,48.14,0,0,0,48,48H416a48.14,48.14,0,0,0,48-48V272L432,120C427,93,409,80,384,80Z" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><line x1="48" y1="272" x2="192" y2="272" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><line x1="320" y1="272" x2="464" y2="272" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M192,272a64,64,0,0,0,128,0" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}