#[cfg(feature = "ImPhoneHangUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImPhoneHangUp")]
/// *This icon requires the feature* `ImPhoneHangUp` *to be enabled*.
#[component]
pub fn PhoneHangUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M15.897 9c0.125 0.867 0.207 2.053-0.182 2.507-0.643 0.751-4.714 0.751-4.714-0.751 0-0.756 0.67-1.252 0.027-2.003-0.632-0.738-1.766-0.75-3.027-0.751s-2.394 0.012-3.027 0.751c-0.643 0.751 0.027 1.247 0.027 2.003 0 1.501-4.071 1.501-4.714 0.751-0.389-0.454-0.307-1.64-0.182-2.507 0.096-0.579 0.339-1.203 1.118-2 0-0 0-0 0-0 1.168-1.090 2.935-1.98 6.716-2v-0c0.021 0 0.042 0 0.063 0s0.041-0 0.063-0v0c3.781 0.019 5.548 0.91 6.716 2 0 0 0 0 0 0 0.778 0.797 1.022 1.421 1.118 2z" /></svg>
   }
}