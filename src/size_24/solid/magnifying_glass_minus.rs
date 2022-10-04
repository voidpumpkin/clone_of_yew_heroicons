use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2E5%203%2E75a6%2E75%206%2E75%200%20100%2013%2E5%206%2E75%206%2E75%200%20000%2D13%2E5zM2%2E25%2010%2E5a8%2E25%208%2E25%200%201114%2E59%205%2E28l4%2E69%204%2E69a%2E75%2E75%200%2011%2D1%2E06%201%2E06l%2D4%2E69%2D4%2E69A8%2E25%208%2E25%200%20012%2E25%2010%2E5zm4%2E5%200a%2E75%2E75%200%2001%2E75%2D%2E75h6a%2E75%2E75%200%20010%201%2E5h%2D6a%2E75%2E75%200%2001%2D%2E75%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MagnifyingGlassMinusIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10.5 3.75a6.75 6.75 0 100 13.5 6.75 6.75 0 000-13.5zM2.25 10.5a8.25 8.25 0 1114.59 5.28l4.69 4.69a.75.75 0 11-1.06 1.06l-4.69-4.69A8.25 8.25 0 012.25 10.5zm4.5 0a.75.75 0 01.75-.75h6a.75.75 0 010 1.5h-6a.75.75 0 01-.75-.75z" clip-rule="evenodd"/>
</svg>
  }
}