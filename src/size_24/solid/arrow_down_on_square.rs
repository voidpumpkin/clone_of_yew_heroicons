use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M12%201%2E5a%2E75%2E75%200%2001%2E75%2E75V7%2E5h%2D1%2E5V2%2E25A%2E75%2E75%200%200112%201%2E5zM11%2E25%207%2E5v5%2E69l%2D1%2E72%2D1%2E72a%2E75%2E75%200%2000%2D1%2E06%201%2E06l3%203a%2E75%2E75%200%20001%2E06%200l3%2D3a%2E75%2E75%200%2010%2D1%2E06%2D1%2E06l%2D1%2E72%201%2E72V7%2E5h3%2E75a3%203%200%20013%203v9a3%203%200%2001%2D3%203h%2D9a3%203%200%2001%2D3%2D3v%2D9a3%203%200%20013%2D3h3%2E75z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowDownOnSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M12 1.5a.75.75 0 01.75.75V7.5h-1.5V2.25A.75.75 0 0112 1.5zM11.25 7.5v5.69l-1.72-1.72a.75.75 0 00-1.06 1.06l3 3a.75.75 0 001.06 0l3-3a.75.75 0 10-1.06-1.06l-1.72 1.72V7.5h3.75a3 3 0 013 3v9a3 3 0 01-3 3h-9a3 3 0 01-3-3v-9a3 3 0 013-3h3.75z"/>
</svg>
  }
}