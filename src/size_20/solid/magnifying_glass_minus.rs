use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M6%2E75%208%2E25a%2E75%2E75%200%20000%201%2E5h4%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D4%2E5z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M9%202a7%207%200%20104%2E391%2012%2E452l3%2E329%203%2E328a%2E75%2E75%200%20101%2E06%2D1%2E06l%2D3%2E328%2D3%2E329A7%207%200%20009%202zM3%2E5%209a5%2E5%205%2E5%200%201111%200%205%2E5%205%2E5%200%2001%2D11%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MagnifyingGlassMinusIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M6.75 8.25a.75.75 0 000 1.5h4.5a.75.75 0 000-1.5h-4.5z"/>
  <path fill-rule="evenodd" d="M9 2a7 7 0 104.391 12.452l3.329 3.328a.75.75 0 101.06-1.06l-3.328-3.329A7 7 0 009 2zM3.5 9a5.5 5.5 0 1111 0 5.5 5.5 0 01-11 0z" clip-rule="evenodd"/>
</svg>
  }
}
