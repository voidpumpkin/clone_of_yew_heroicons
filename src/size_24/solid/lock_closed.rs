use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%201%2E5a5%2E25%205%2E25%200%2000%2D5%2E25%205%2E25v3a3%203%200%2000%2D3%203v6%2E75a3%203%200%20003%203h10%2E5a3%203%200%20003%2D3v%2D6%2E75a3%203%200%2000%2D3%2D3v%2D3c0%2D2%2E9%2D2%2E35%2D5%2E25%2D5%2E25%2D5%2E25zm3%2E75%208%2E25v%2D3a3%2E75%203%2E75%200%2010%2D7%2E5%200v3h7%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn LockClosedIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12 1.5a5.25 5.25 0 00-5.25 5.25v3a3 3 0 00-3 3v6.75a3 3 0 003 3h10.5a3 3 0 003-3v-6.75a3 3 0 00-3-3v-3c0-2.9-2.35-5.25-5.25-5.25zm3.75 8.25v-3a3.75 3.75 0 10-7.5 0v3h7.5z" clip-rule="evenodd"/>
</svg>
  }
}
