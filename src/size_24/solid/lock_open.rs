use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M18%201%2E5c2%2E9%200%205%2E25%202%2E35%205%2E25%205%2E25v3%2E75a%2E75%2E75%200%2001%2D1%2E5%200V6%2E75a3%2E75%203%2E75%200%2010%2D7%2E5%200v3a3%203%200%20013%203v6%2E75a3%203%200%2001%2D3%203H3%2E75a3%203%200%2001%2D3%2D3v%2D6%2E75a3%203%200%20013%2D3h9v%2D3c0%2D2%2E9%202%2E35%2D5%2E25%205%2E25%2D5%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn LockOpenIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M18 1.5c2.9 0 5.25 2.35 5.25 5.25v3.75a.75.75 0 01-1.5 0V6.75a3.75 3.75 0 10-7.5 0v3a3 3 0 013 3v6.75a3 3 0 01-3 3H3.75a3 3 0 01-3-3v-6.75a3 3 0 013-3h9v-3c0-2.9 2.35-5.25 5.25-5.25z"/>
</svg>
  }
}
