use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M9%206a%2E75%2E75%200%2001%2E75%2E75v1%2E5h1%2E5a%2E75%2E75%200%20010%201%2E5h%2D1%2E5v1%2E5a%2E75%2E75%200%2001%2D1%2E5%200v%2D1%2E5h%2D1%2E5a%2E75%2E75%200%20010%2D1%2E5h1%2E5v%2D1%2E5A%2E75%2E75%200%20019%206z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%209a7%207%200%201112%2E452%204%2E391l3%2E328%203%2E329a%2E75%2E75%200%2011%2D1%2E06%201%2E06l%2D3%2E329%2D3%2E328A7%207%200%20012%209zm7%2D5%2E5a5%2E5%205%2E5%200%20100%2011%205%2E5%205%2E5%200%20000%2D11z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MagnifyingGlassPlusIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M9 6a.75.75 0 01.75.75v1.5h1.5a.75.75 0 010 1.5h-1.5v1.5a.75.75 0 01-1.5 0v-1.5h-1.5a.75.75 0 010-1.5h1.5v-1.5A.75.75 0 019 6z"/>
  <path fill-rule="evenodd" d="M2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9zm7-5.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11z" clip-rule="evenodd"/>
</svg>
  }
}