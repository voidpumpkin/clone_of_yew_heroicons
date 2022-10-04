use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E232%2012%2E207a%2E75%2E75%200%20011%2E06%2E025l3%2E958%204%2E146V6%2E375a5%2E375%205%2E375%200%200110%2E75%200V9%2E25a%2E75%2E75%200%2001%2D1%2E5%200V6%2E375a3%2E875%203%2E875%200%2000%2D7%2E75%200v10%2E003l3%2E957%2D4%2E146a%2E75%2E75%200%20011%2E085%201%2E036l%2D5%2E25%205%2E5a%2E75%2E75%200%2001%2D1%2E085%200l%2D5%2E25%2D5%2E5a%2E75%2E75%200%2001%2E025%2D1%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowUturnDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.232 12.207a.75.75 0 011.06.025l3.958 4.146V6.375a5.375 5.375 0 0110.75 0V9.25a.75.75 0 01-1.5 0V6.375a3.875 3.875 0 00-7.75 0v10.003l3.957-4.146a.75.75 0 011.085 1.036l-5.25 5.5a.75.75 0 01-1.085 0l-5.25-5.5a.75.75 0 01.025-1.06z" clip-rule="evenodd"/>
</svg>
  }
}