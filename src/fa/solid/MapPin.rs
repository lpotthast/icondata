#[cfg(feature = "FaSolidMapPin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidMapPin")]
/// *This icon requires the feature* `FaSolidMapPin` *to be enabled*.
#[component]
pub fn MapPin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M32 144a144 144 0 1 1 288 0A144 144 0 1 1 32 144zM176 80c8.8 0 16-7.2 16-16s-7.2-16-16-16c-53 0-96 43-96 96c0 8.8 7.2 16 16 16s16-7.2 16-16c0-35.3 28.7-64 64-64zM144 480V317.1c10.4 1.9 21.1 2.9 32 2.9s21.6-1 32-2.9V480c0 17.7-14.3 32-32 32s-32-14.3-32-32z" /></svg>
   }
}