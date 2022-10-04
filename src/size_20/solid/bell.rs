use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%202a6%206%200%2000%2D6%206c0%201%2E887%2D%2E454%203%2E665%2D1%2E257%205%2E234a%2E75%2E75%200%2000%2E515%201%2E076%2032%2E91%2032%2E91%200%20003%2E256%2E508%203%2E5%203%2E5%200%20006%2E972%200%2032%2E903%2032%2E903%200%20003%2E256%2D%2E508%2E75%2E75%200%2000%2E515%2D1%2E076A11%2E448%2011%2E448%200%200116%208a6%206%200%2000%2D6%2D6zM8%2E05%2014%2E943a33%2E54%2033%2E54%200%20003%2E9%200%202%202%200%2001%2D3%2E9%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BellIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10 2a6 6 0 00-6 6c0 1.887-.454 3.665-1.257 5.234a.75.75 0 00.515 1.076 32.91 32.91 0 003.256.508 3.5 3.5 0 006.972 0 32.903 32.903 0 003.256-.508.75.75 0 00.515-1.076A11.448 11.448 0 0116 8a6 6 0 00-6-6zM8.05 14.943a33.54 33.54 0 003.9 0 2 2 0 01-3.9 0z" clip-rule="evenodd"/>
</svg>
  }
}
