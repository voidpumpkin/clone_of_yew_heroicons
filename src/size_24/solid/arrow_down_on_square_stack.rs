use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M9%2E75%206%2E75h%2D3a3%203%200%2000%2D3%203v7%2E5a3%203%200%20003%203h7%2E5a3%203%200%20003%2D3v%2D7%2E5a3%203%200%2000%2D3%2D3h%2D3V1%2E5a%2E75%2E75%200%2000%2D1%2E5%200v5%2E25zm0%200h1%2E5v5%2E69l1%2E72%2D1%2E72a%2E75%2E75%200%20111%2E06%201%2E06l%2D3%203a%2E75%2E75%200%2001%2D1%2E06%200l%2D3%2D3a%2E75%2E75%200%20111%2E06%2D1%2E06l1%2E72%201%2E72V6%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20d%3D%22M7%2E151%2021%2E75a2%2E999%202%2E999%200%20002%2E599%201%2E5h7%2E5a3%203%200%20003%2D3v%2D7%2E5c0%2D1%2E11%2D%2E603%2D2%2E08%2D1%2E5%2D2%2E599v7%2E099a4%2E5%204%2E5%200%2001%2D4%2E5%204%2E5H7%2E151z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowDownOnSquareStackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M9.75 6.75h-3a3 3 0 00-3 3v7.5a3 3 0 003 3h7.5a3 3 0 003-3v-7.5a3 3 0 00-3-3h-3V1.5a.75.75 0 00-1.5 0v5.25zm0 0h1.5v5.69l1.72-1.72a.75.75 0 111.06 1.06l-3 3a.75.75 0 01-1.06 0l-3-3a.75.75 0 111.06-1.06l1.72 1.72V6.75z" clip-rule="evenodd"/>
  <path d="M7.151 21.75a2.999 2.999 0 002.599 1.5h7.5a3 3 0 003-3v-7.5c0-1.11-.603-2.08-1.5-2.599v7.099a4.5 4.5 0 01-4.5 4.5H7.151z"/>
</svg>
  }
}