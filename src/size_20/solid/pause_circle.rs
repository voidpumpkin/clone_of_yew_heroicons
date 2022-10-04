use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2010a8%208%200%201116%200%208%208%200%2001%2D16%200zm5%2D2%2E25A%2E75%2E75%200%20017%2E75%207h%2E5a%2E75%2E75%200%2001%2E75%2E75v4%2E5a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D4%2E5zm4%200a%2E75%2E75%200%2001%2E75%2D%2E75h%2E5a%2E75%2E75%200%2001%2E75%2E75v4%2E5a%2E75%2E75%200%2001%2D%2E75%2E75h%2D%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75v%2D4%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PauseCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2 10a8 8 0 1116 0 8 8 0 01-16 0zm5-2.25A.75.75 0 017.75 7h.5a.75.75 0 01.75.75v4.5a.75.75 0 01-.75.75h-.5a.75.75 0 01-.75-.75v-4.5zm4 0a.75.75 0 01.75-.75h.5a.75.75 0 01.75.75v4.5a.75.75 0 01-.75.75h-.5a.75.75 0 01-.75-.75v-4.5z" clip-rule="evenodd"/>
</svg>
  }
}
