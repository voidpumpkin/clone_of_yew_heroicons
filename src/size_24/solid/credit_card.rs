use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M4%2E5%203%2E75a3%203%200%2000%2D3%203v%2E75h21v%2D%2E75a3%203%200%2000%2D3%2D3h%2D15z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M22%2E5%209%2E75h%2D21v7%2E5a3%203%200%20003%203h15a3%203%200%20003%2D3v%2D7%2E5zm%2D18%203%2E75a%2E75%2E75%200%2001%2E75%2D%2E75h6a%2E75%2E75%200%20010%201%2E5h%2D6a%2E75%2E75%200%2001%2D%2E75%2D%2E75zm%2E75%202%2E25a%2E75%2E75%200%20000%201%2E5h3a%2E75%2E75%200%20000%2D1%2E5h%2D3z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CreditCardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M4.5 3.75a3 3 0 00-3 3v.75h21v-.75a3 3 0 00-3-3h-15z"/>
  <path fill-rule="evenodd" d="M22.5 9.75h-21v7.5a3 3 0 003 3h15a3 3 0 003-3v-7.5zm-18 3.75a.75.75 0 01.75-.75h6a.75.75 0 010 1.5h-6a.75.75 0 01-.75-.75zm.75 2.25a.75.75 0 000 1.5h3a.75.75 0 000-1.5h-3z" clip-rule="evenodd"/>
</svg>
  }
}
