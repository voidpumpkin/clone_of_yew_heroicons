use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M6%203a3%203%200%2000%2D3%203v1%2E5a%2E75%2E75%200%20001%2E5%200V6A1%2E5%201%2E5%200%20016%204%2E5h1%2E5a%2E75%2E75%200%20000%2D1%2E5H6zM16%2E5%203a%2E75%2E75%200%20000%201%2E5H18A1%2E5%201%2E5%200%200119%2E5%206v1%2E5a%2E75%2E75%200%20001%2E5%200V6a3%203%200%2000%2D3%2D3h%2D1%2E5zM12%208%2E25a3%2E75%203%2E75%200%20100%207%2E5%203%2E75%203%2E75%200%20000%2D7%2E5zM4%2E5%2016%2E5a%2E75%2E75%200%2000%2D1%2E5%200V18a3%203%200%20003%203h1%2E5a%2E75%2E75%200%20000%2D1%2E5H6A1%2E5%201%2E5%200%20014%2E5%2018v%2D1%2E5zM21%2016%2E5a%2E75%2E75%200%2000%2D1%2E5%200V18a1%2E5%201%2E5%200%2001%2D1%2E5%201%2E5h%2D1%2E5a%2E75%2E75%200%20000%201%2E5H18a3%203%200%20003%2D3v%2D1%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ViewfinderCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M6 3a3 3 0 00-3 3v1.5a.75.75 0 001.5 0V6A1.5 1.5 0 016 4.5h1.5a.75.75 0 000-1.5H6zM16.5 3a.75.75 0 000 1.5H18A1.5 1.5 0 0119.5 6v1.5a.75.75 0 001.5 0V6a3 3 0 00-3-3h-1.5zM12 8.25a3.75 3.75 0 100 7.5 3.75 3.75 0 000-7.5zM4.5 16.5a.75.75 0 00-1.5 0V18a3 3 0 003 3h1.5a.75.75 0 000-1.5H6A1.5 1.5 0 014.5 18v-1.5zM21 16.5a.75.75 0 00-1.5 0V18a1.5 1.5 0 01-1.5 1.5h-1.5a.75.75 0 000 1.5H18a3 3 0 003-3v-1.5z"/>
</svg>
  }
}