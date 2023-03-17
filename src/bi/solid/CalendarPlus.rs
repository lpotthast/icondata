#[cfg(feature = "BiSolidCalendarPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCalendarPlus")]
/// *This icon requires the feature* `BiSolidCalendarPlus` *to be enabled*.
#[component]
pub fn CalendarPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 22h14c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2h-2V2h-2v2H9V2H7v2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2zm11-6h-3v3h-2v-3H8v-2h3v-3h2v3h3v2zM5 7h14v2H5V7z" /></svg>
   }
}