use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M6%2E75%202%2E25A%2E75%2E75%200%20017%2E5%203v1%2E5h9V3A%2E75%2E75%200%200118%203v1%2E5h%2E75a3%203%200%20013%203v11%2E25a3%203%200%2001%2D3%203H5%2E25a3%203%200%2001%2D3%2D3V7%2E5a3%203%200%20013%2D3H6V3a%2E75%2E75%200%2001%2E75%2D%2E75zm13%2E5%209a1%2E5%201%2E5%200%2000%2D1%2E5%2D1%2E5H5%2E25a1%2E5%201%2E5%200%2000%2D1%2E5%201%2E5v7%2E5a1%2E5%201%2E5%200%20001%2E5%201%2E5h13%2E5a1%2E5%201%2E5%200%20001%2E5%2D1%2E5v%2D7%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CalendarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M6.75 2.25A.75.75 0 017.5 3v1.5h9V3A.75.75 0 0118 3v1.5h.75a3 3 0 013 3v11.25a3 3 0 01-3 3H5.25a3 3 0 01-3-3V7.5a3 3 0 013-3H6V3a.75.75 0 01.75-.75zm13.5 9a1.5 1.5 0 00-1.5-1.5H5.25a1.5 1.5 0 00-1.5 1.5v7.5a1.5 1.5 0 001.5 1.5h13.5a1.5 1.5 0 001.5-1.5v-7.5z" clip-rule="evenodd"/>
</svg>
  }
}
