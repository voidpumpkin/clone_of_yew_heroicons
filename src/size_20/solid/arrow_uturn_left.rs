use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M7%2E793%202%2E232a%2E75%2E75%200%2001%2D%2E025%201%2E06L3%2E622%207%2E25h10%2E003a5%2E375%205%2E375%200%20010%2010%2E75H10%2E75a%2E75%2E75%200%20010%2D1%2E5h2%2E875a3%2E875%203%2E875%200%20000%2D7%2E75H3%2E622l4%2E146%203%2E957a%2E75%2E75%200%2001%2D1%2E036%201%2E085l%2D5%2E5%2D5%2E25a%2E75%2E75%200%20010%2D1%2E085l5%2E5%2D5%2E25a%2E75%2E75%200%20011%2E06%2E025z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowUturnLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M7.793 2.232a.75.75 0 01-.025 1.06L3.622 7.25h10.003a5.375 5.375 0 010 10.75H10.75a.75.75 0 010-1.5h2.875a3.875 3.875 0 000-7.75H3.622l4.146 3.957a.75.75 0 01-1.036 1.085l-5.5-5.25a.75.75 0 010-1.085l5.5-5.25a.75.75 0 011.06.025z" clip-rule="evenodd"/>
</svg>
  }
}