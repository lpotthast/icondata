#[cfg(feature = "SiKeras")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiKeras")]
/// *This icon requires the feature* `SiKeras` *to be enabled*.
#[component]
pub fn Keras(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M24 0H0v24h24V0zM8.45 5.16l.2.17v6.24l6.46-6.45h1.96l.2.4-5.14 5.1 5.47 7.94-.2.3h-1.94l-4.65-6.88-2.16 2.08v4.6l-.19.2H7l-.2-.2V5.33l.17-.17h1.48z" /></svg>
   }
}