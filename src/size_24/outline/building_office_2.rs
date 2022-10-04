use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M2%2E25%2021h19%2E5m%2D18%2D18v18m10%2E5%2D18v18m6%2D13%2E5V21M6%2E75%206%2E75h%2E75m%2D%2E75%203h%2E75m%2D%2E75%203h%2E75m3%2D6h%2E75m%2D%2E75%203h%2E75m%2D%2E75%203h%2E75M6%2E75%2021v%2D3%2E375c0%2D%2E621%2E504%2D1%2E125%201%2E125%2D1%2E125h2%2E25c%2E621%200%201%2E125%2E504%201%2E125%201%2E125V21M3%203h12m%2D%2E75%204%2E5H21m%2D3%2E75%203%2E75h%2E008v%2E008h%2D%2E008v%2D%2E008zm0%203h%2E008v%2E008h%2D%2E008v%2D%2E008zm0%203h%2E008v%2E008h%2D%2E008v%2D%2E008z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BuildingOffice2Icon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 21h19.5m-18-18v18m10.5-18v18m6-13.5V21M6.75 6.75h.75m-.75 3h.75m-.75 3h.75m3-6h.75m-.75 3h.75m-.75 3h.75M6.75 21v-3.375c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21M3 3h12m-.75 4.5H21m-3.75 3.75h.008v.008h-.008v-.008zm0 3h.008v.008h-.008v-.008zm0 3h.008v.008h-.008v-.008z"/>
</svg>
  }
}