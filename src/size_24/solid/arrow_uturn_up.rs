use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M21%2E53%209%2E53a%2E75%2E75%200%2001%2D1%2E06%200l%2D4%2E72%2D4%2E72V15a6%2E75%206%2E75%200%2001%2D13%2E5%200v%2D3a%2E75%2E75%200%20011%2E5%200v3a5%2E25%205%2E25%200%201010%2E5%200V4%2E81L9%2E53%209%2E53a%2E75%2E75%200%2001%2D1%2E06%2D1%2E06l6%2D6a%2E75%2E75%200%20011%2E06%200l6%206a%2E75%2E75%200%20010%201%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowUturnUpIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M21.53 9.53a.75.75 0 01-1.06 0l-4.72-4.72V15a6.75 6.75 0 01-13.5 0v-3a.75.75 0 011.5 0v3a5.25 5.25 0 1010.5 0V4.81L9.53 9.53a.75.75 0 01-1.06-1.06l6-6a.75.75 0 011.06 0l6 6a.75.75 0 010 1.06z" clip-rule="evenodd"/>
</svg>
  }
}
