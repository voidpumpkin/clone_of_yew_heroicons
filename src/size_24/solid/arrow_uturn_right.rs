use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M14%2E47%202%2E47a%2E75%2E75%200%20011%2E06%200l6%206a%2E75%2E75%200%20010%201%2E06l%2D6%206a%2E75%2E75%200%2011%2D1%2E06%2D1%2E06l4%2E72%2D4%2E72H9a5%2E25%205%2E25%200%20100%2010%2E5h3a%2E75%2E75%200%20010%201%2E5H9a6%2E75%206%2E75%200%20010%2D13%2E5h10%2E19l%2D4%2E72%2D4%2E72a%2E75%2E75%200%20010%2D1%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowUturnRightIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M14.47 2.47a.75.75 0 011.06 0l6 6a.75.75 0 010 1.06l-6 6a.75.75 0 11-1.06-1.06l4.72-4.72H9a5.25 5.25 0 100 10.5h3a.75.75 0 010 1.5H9a6.75 6.75 0 010-13.5h10.19l-4.72-4.72a.75.75 0 010-1.06z" clip-rule="evenodd"/>
</svg>
  }
}