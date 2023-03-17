#[cfg(feature = "VsSymbolMisc")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSymbolMisc")]
/// *This icon requires the feature* `VsSymbolMisc` *to be enabled*.
#[component]
pub fn SymbolMisc(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M4 2h8v4c.341.035.677.112 1 .23V1H3v8.48l1-1.75V2zm2.14 8L5 8 4 9.75 3.29 11 1 15h8l-2.29-4-.57-1zm-3.42 4l1.72-3L5 10l.56 1 1.72 3H2.72zm6.836-6.41a3.5 3.5 0 1 1 3.888 5.82 3.5 3.5 0 0 1-3.888-5.82zm.555 4.989a2.5 2.5 0 1 0 2.778-4.157 2.5 2.5 0 0 0-2.778 4.157z" /></svg>
   }
}