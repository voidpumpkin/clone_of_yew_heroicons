use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M7%2E5%203%2E75A1%2E5%201%2E5%200%20006%205%2E25v13%2E5a1%2E5%201%2E5%200%20001%2E5%201%2E5h6a1%2E5%201%2E5%200%20001%2E5%2D1%2E5V15a%2E75%2E75%200%20011%2E5%200v3%2E75a3%203%200%2001%2D3%203h%2D6a3%203%200%2001%2D3%2D3V5%2E25a3%203%200%20013%2D3h6a3%203%200%20013%203V9A%2E75%2E75%200%200115%209V5%2E25a1%2E5%201%2E5%200%2000%2D1%2E5%2D1%2E5h%2D6zm5%2E03%204%2E72a%2E75%2E75%200%20010%201%2E06l%2D1%2E72%201%2E72h10%2E94a%2E75%2E75%200%20010%201%2E5H10%2E81l1%2E72%201%2E72a%2E75%2E75%200%2011%2D1%2E06%201%2E06l%2D3%2D3a%2E75%2E75%200%20010%2D1%2E06l3%2D3a%2E75%2E75%200%20011%2E06%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowLeftOnRectangleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M7.5 3.75A1.5 1.5 0 006 5.25v13.5a1.5 1.5 0 001.5 1.5h6a1.5 1.5 0 001.5-1.5V15a.75.75 0 011.5 0v3.75a3 3 0 01-3 3h-6a3 3 0 01-3-3V5.25a3 3 0 013-3h6a3 3 0 013 3V9A.75.75 0 0115 9V5.25a1.5 1.5 0 00-1.5-1.5h-6zm5.03 4.72a.75.75 0 010 1.06l-1.72 1.72h10.94a.75.75 0 010 1.5H10.81l1.72 1.72a.75.75 0 11-1.06 1.06l-3-3a.75.75 0 010-1.06l3-3a.75.75 0 011.06 0z" clip-rule="evenodd"/>
</svg>
  }
}