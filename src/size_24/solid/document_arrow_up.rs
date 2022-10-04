use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%2E625%201%2E5H9a3%2E75%203%2E75%200%20013%2E75%203%2E75v1%2E875c0%201%2E036%2E84%201%2E875%201%2E875%201%2E875H16%2E5a3%2E75%203%2E75%200%20013%2E75%203%2E75v7%2E875c0%201%2E035%2D%2E84%201%2E875%2D1%2E875%201%2E875H5%2E625a1%2E875%201%2E875%200%2001%2D1%2E875%2D1%2E875V3%2E375c0%2D1%2E036%2E84%2D1%2E875%201%2E875%2D1%2E875zm6%2E905%209%2E97a%2E75%2E75%200%2000%2D1%2E06%200l%2D3%203a%2E75%2E75%200%20101%2E06%201%2E06l1%2E72%2D1%2E72V18a%2E75%2E75%200%20001%2E5%200v%2D4%2E19l1%2E72%201%2E72a%2E75%2E75%200%20101%2E06%2D1%2E06l%2D3%2D3z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20d%3D%22M14%2E25%205%2E25a5%2E23%205%2E23%200%2000%2D1%2E279%2D3%2E434%209%2E768%209%2E768%200%20016%2E963%206%2E963A5%2E23%205%2E23%200%200016%2E5%207%2E5h%2D1%2E875a%2E375%2E375%200%2001%2D%2E375%2D%2E375V5%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DocumentArrowUpIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M5.625 1.5H9a3.75 3.75 0 013.75 3.75v1.875c0 1.036.84 1.875 1.875 1.875H16.5a3.75 3.75 0 013.75 3.75v7.875c0 1.035-.84 1.875-1.875 1.875H5.625a1.875 1.875 0 01-1.875-1.875V3.375c0-1.036.84-1.875 1.875-1.875zm6.905 9.97a.75.75 0 00-1.06 0l-3 3a.75.75 0 101.06 1.06l1.72-1.72V18a.75.75 0 001.5 0v-4.19l1.72 1.72a.75.75 0 101.06-1.06l-3-3z" clip-rule="evenodd"/>
  <path d="M14.25 5.25a5.23 5.23 0 00-1.279-3.434 9.768 9.768 0 016.963 6.963A5.23 5.23 0 0016.5 7.5h-1.875a.375.375 0 01-.375-.375V5.25z"/>
</svg>
  }
}
