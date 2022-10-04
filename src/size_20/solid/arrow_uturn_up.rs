use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M17%2E768%207%2E793a%2E75%2E75%200%2001%2D1%2E06%2D%2E025L12%2E75%203%2E622v10%2E003a5%2E375%205%2E375%200%2001%2D10%2E75%200V10%2E75a%2E75%2E75%200%20011%2E5%200v2%2E875a3%2E875%203%2E875%200%20007%2E75%200V3%2E622L7%2E293%207%2E768a%2E75%2E75%200%2001%2D1%2E086%2D1%2E036l5%2E25%2D5%2E5a%2E75%2E75%200%20011%2E085%200l5%2E25%205%2E5a%2E75%2E75%200%2001%2D%2E024%201%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowUturnUpIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M17.768 7.793a.75.75 0 01-1.06-.025L12.75 3.622v10.003a5.375 5.375 0 01-10.75 0V10.75a.75.75 0 011.5 0v2.875a3.875 3.875 0 007.75 0V3.622L7.293 7.768a.75.75 0 01-1.086-1.036l5.25-5.5a.75.75 0 011.085 0l5.25 5.5a.75.75 0 01-.024 1.06z" clip-rule="evenodd"/>
</svg>
  }
}
