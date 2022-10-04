use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%2E75%206%2E75a%2E75%2E75%200%2000%2D1%2E5%200v2%2E5h%2D2%2E5a%2E75%2E75%200%20000%201%2E5h2%2E5v2%2E5a%2E75%2E75%200%20001%2E5%200v%2D2%2E5h2%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D2%2E5v%2D2%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PlusSmallIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M10.75 6.75a.75.75 0 00-1.5 0v2.5h-2.5a.75.75 0 000 1.5h2.5v2.5a.75.75 0 001.5 0v-2.5h2.5a.75.75 0 000-1.5h-2.5v-2.5z"/>
</svg>
  }
}
