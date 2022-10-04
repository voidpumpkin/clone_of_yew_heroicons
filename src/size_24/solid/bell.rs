use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%2E25%209a6%2E75%206%2E75%200%200113%2E5%200v%2E75c0%202%2E123%2E8%204%2E057%202%2E118%205%2E52a%2E75%2E75%200%2001%2D%2E297%201%2E206c%2D1%2E544%2E57%2D3%2E16%2E99%2D4%2E831%201%2E243a3%2E75%203%2E75%200%2011%2D7%2E48%200%2024%2E585%2024%2E585%200%2001%2D4%2E831%2D1%2E244%2E75%2E75%200%2001%2D%2E298%2D1%2E205A8%2E217%208%2E217%200%20005%2E25%209%2E75V9zm4%2E502%208%2E9a2%2E25%202%2E25%200%20104%2E496%200%2025%2E057%2025%2E057%200%2001%2D4%2E496%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BellIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M5.25 9a6.75 6.75 0 0113.5 0v.75c0 2.123.8 4.057 2.118 5.52a.75.75 0 01-.297 1.206c-1.544.57-3.16.99-4.831 1.243a3.75 3.75 0 11-7.48 0 24.585 24.585 0 01-4.831-1.244.75.75 0 01-.298-1.205A8.217 8.217 0 005.25 9.75V9zm4.502 8.9a2.25 2.25 0 104.496 0 25.057 25.057 0 01-4.496 0z" clip-rule="evenodd"/>
</svg>
  }
}