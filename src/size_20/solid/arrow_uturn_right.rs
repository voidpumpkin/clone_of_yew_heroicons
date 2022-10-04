use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%2E207%202%2E232a%2E75%2E75%200%2000%2E025%201%2E06l4%2E146%203%2E958H6%2E375a5%2E375%205%2E375%200%20000%2010%2E75H9%2E25a%2E75%2E75%200%20000%2D1%2E5H6%2E375a3%2E875%203%2E875%200%20010%2D7%2E75h10%2E003l%2D4%2E146%203%2E957a%2E75%2E75%200%20001%2E036%201%2E085l5%2E5%2D5%2E25a%2E75%2E75%200%20000%2D1%2E085l%2D5%2E5%2D5%2E25a%2E75%2E75%200%2000%2D1%2E06%2E025z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowUturnRightIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12.207 2.232a.75.75 0 00.025 1.06l4.146 3.958H6.375a5.375 5.375 0 000 10.75H9.25a.75.75 0 000-1.5H6.375a3.875 3.875 0 010-7.75h10.003l-4.146 3.957a.75.75 0 001.036 1.085l5.5-5.25a.75.75 0 000-1.085l-5.5-5.25a.75.75 0 00-1.06.025z" clip-rule="evenodd"/>
</svg>
  }
}