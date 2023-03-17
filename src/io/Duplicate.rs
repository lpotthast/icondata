#[cfg(feature = "IoDuplicate")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoDuplicate")]
/// *This icon requires the feature* `IoDuplicate` *to be enabled*.
#[component]
pub fn Duplicate(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M408,112H184a72,72,0,0,0-72,72V408a72,72,0,0,0,72,72H408a72,72,0,0,0,72-72V184A72,72,0,0,0,408,112ZM375.55,312H312v63.55c0,8.61-6.62,16-15.23,16.43A16,16,0,0,1,280,376V312H216.45c-8.61,0-16-6.62-16.43-15.23A16,16,0,0,1,216,280h64V216.45c0-8.61,6.62-16,15.23-16.43A16,16,0,0,1,312,216v64h64a16,16,0,0,1,16,16.77C391.58,305.38,384.16,312,375.55,312Z" /><path d="M395.88,80A72.12,72.12,0,0,0,328,32H104a72,72,0,0,0-72,72V328a72.12,72.12,0,0,0,48,67.88V160a80,80,0,0,1,80-80Z" /></svg>
   }
}