use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M15%203%2E75A5%2E25%205%2E25%200%20009%2E75%209v10%2E19l4%2E72%2D4%2E72a%2E75%2E75%200%20111%2E06%201%2E06l%2D6%206a%2E75%2E75%200%2001%2D1%2E06%200l%2D6%2D6a%2E75%2E75%200%20111%2E06%2D1%2E06l4%2E72%204%2E72V9a6%2E75%206%2E75%200%200113%2E5%200v3a%2E75%2E75%200%2001%2D1%2E5%200V9c0%2D2%2E9%2D2%2E35%2D5%2E25%2D5%2E25%2D5%2E25z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowUturnDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M15 3.75A5.25 5.25 0 009.75 9v10.19l4.72-4.72a.75.75 0 111.06 1.06l-6 6a.75.75 0 01-1.06 0l-6-6a.75.75 0 111.06-1.06l4.72 4.72V9a6.75 6.75 0 0113.5 0v3a.75.75 0 01-1.5 0V9c0-2.9-2.35-5.25-5.25-5.25z" clip-rule="evenodd"/>
</svg>
  }
}
