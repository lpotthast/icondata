#[cfg(feature = "TbHomeBolt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbHomeBolt")]
/// *This icon requires the feature* `TbHomeBolt` *to be enabled*.
#[component]
pub fn HomeBolt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-home-bolt" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M19 10l-7 -7l-9 9h2v7a2 2 0 0 0 2 2h7.5" /><path d="M9 21v-6a2 2 0 0 1 2 -2h2c.661 0 1.248 .32 1.612 .815" /><path d="M19 14l-2 4h4l-2 4" /></svg>
   }
}