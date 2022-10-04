use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M1%2E5%208%2E67v8%2E58a3%203%200%20003%203h15a3%203%200%20003%2D3V8%2E67l%2D8%2E928%205%2E493a3%203%200%2001%2D3%2E144%200L1%2E5%208%2E67z%22%2F%3E%20%3Cpath%20d%3D%22M22%2E5%206%2E908V6%2E75a3%203%200%2000%2D3%2D3h%2D15a3%203%200%2000%2D3%203v%2E158l9%2E714%205%2E978a1%2E5%201%2E5%200%20001%2E572%200L22%2E5%206%2E908z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EnvelopeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M1.5 8.67v8.58a3 3 0 003 3h15a3 3 0 003-3V8.67l-8.928 5.493a3 3 0 01-3.144 0L1.5 8.67z"/>
  <path d="M22.5 6.908V6.75a3 3 0 00-3-3h-15a3 3 0 00-3 3v.158l9.714 5.978a1.5 1.5 0 001.572 0L22.5 6.908z"/>
</svg>
  }
}
