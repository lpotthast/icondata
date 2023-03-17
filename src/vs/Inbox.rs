#[cfg(feature = "VsInbox")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsInbox")]
/// *This icon requires the feature* `VsInbox` *to be enabled*.
#[component]
pub fn Inbox(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 14h13l.5-.5V9l-2.77-7.66-.47-.34H4.27l-.47.33L1 8.74v4.76l.5.5zM14 13H2v-2.98h2.55l.74 1.25.43.24h4.57l.44-.26.69-1.23H14V13zm-.022-3.98H11.12l-.43.26-.69 1.23H6.01l-.75-1.25-.43-.24H2V9l2.62-7h6.78l2.578 7.02z" /></svg>
   }
}