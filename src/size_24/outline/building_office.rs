use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M3%2E75%2021h16%2E5M4%2E5%203h15M5%2E25%203v18m13%2E5%2D18v18M9%206%2E75h1%2E5m%2D1%2E5%203h1%2E5m%2D1%2E5%203h1%2E5m3%2D6H15m%2D1%2E5%203H15m%2D1%2E5%203H15M9%2021v%2D3%2E375c0%2D%2E621%2E504%2D1%2E125%201%2E125%2D1%2E125h3%2E75c%2E621%200%201%2E125%2E504%201%2E125%201%2E125V21%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BuildingOfficeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 21h16.5M4.5 3h15M5.25 3v18m13.5-18v18M9 6.75h1.5m-1.5 3h1.5m-1.5 3h1.5m3-6H15m-1.5 3H15m-1.5 3H15M9 21v-3.375c0-.621.504-1.125 1.125-1.125h3.75c.621 0 1.125.504 1.125 1.125V21"/>
</svg>
  }
}