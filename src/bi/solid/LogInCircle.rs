#[cfg(feature = "BiSolidLogInCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidLogInCircle")]
/// *This icon requires the feature* `BiSolidLogInCircle` *to be enabled*.
#[component]
pub fn LogInCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 3c-4.625 0-8.442 3.507-8.941 8.001H10v-3l5 4-5 4v-3H3.06C3.56 17.494 7.376 21 12 21c4.963 0 9-4.037 9-9s-4.037-9-9-9z" /></svg>
   }
}