use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M4%2E5%202A1%2E5%201%2E5%200%20003%203%2E5v13A1%2E5%201%2E5%200%20004%2E5%2018h11a1%2E5%201%2E5%200%20001%2E5%2D1%2E5V7%2E621a1%2E5%201%2E5%200%2000%2D%2E44%2D1%2E06l%2D4%2E12%2D4%2E122A1%2E5%201%2E5%200%200011%2E378%202H4%2E5zm2%2E25%208%2E5a%2E75%2E75%200%20000%201%2E5h6%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D6%2E5zm0%203a%2E75%2E75%200%20000%201%2E5h6%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D6%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DocumentTextIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M4.5 2A1.5 1.5 0 003 3.5v13A1.5 1.5 0 004.5 18h11a1.5 1.5 0 001.5-1.5V7.621a1.5 1.5 0 00-.44-1.06l-4.12-4.122A1.5 1.5 0 0011.378 2H4.5zm2.25 8.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5zm0 3a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-6.5z" clip-rule="evenodd"/>
</svg>
  }
}
