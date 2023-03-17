#[cfg(feature = "TbMoodHeart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMoodHeart")]
/// *This icon requires the feature* `TbMoodHeart` *to be enabled*.
#[component]
pub fn MoodHeart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mood-heart" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M21 12a9 9 0 1 0 -8.012 8.946" /><path d="M9 10h.01" /><path d="M15 10h.01" /><path d="M9.5 15a3.59 3.59 0 0 0 2.774 .99" /><path d="M18.994 21.5l2.518 -2.58a1.74 1.74 0 0 0 .004 -2.413a1.627 1.627 0 0 0 -2.346 -.005l-.168 .172l-.168 -.172a1.627 1.627 0 0 0 -2.346 -.004a1.74 1.74 0 0 0 -.004 2.412l2.51 2.59z" /></svg>
   }
}