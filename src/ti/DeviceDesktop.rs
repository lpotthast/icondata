#[cfg(feature = "TiDeviceDesktop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiDeviceDesktop")]
/// *This icon requires the feature* `TiDeviceDesktop` *to be enabled*.
#[component]
pub fn DeviceDesktop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><g><path d="M21 1h-18c-1.654 0-3 1.346-3 3v11c0 1.654 1.346 3 3 3h6v2h-3c-.552 0-1 .448-1 1s.448 1 1 1h12c.552 0 1-.448 1-1s-.448-1-1-1h-3v-2h6c1.654 0 3-1.346 3-3v-11c0-1.654-1.346-3-3-3zm-7 19h-4v-2h4v2zm8-5c0 .551-.449 1-1 1h-18c-.551 0-1-.449-1-1v-11c0-.551.449-1 1-1h18c.551 0 1 .449 1 1v11zM20 4h-16c-.55 0-1 .45-1 1v8c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-8c0-.55-.45-1-1-1zm0 9h-16v-8h16v8z" /></g></svg>
   }
}