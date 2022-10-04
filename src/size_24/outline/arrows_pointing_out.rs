use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M3%2E75%203%2E75v4%2E5m0%2D4%2E5h4%2E5m%2D4%2E5%200L9%209M3%2E75%2020%2E25v%2D4%2E5m0%204%2E5h4%2E5m%2D4%2E5%200L9%2015M20%2E25%203%2E75h%2D4%2E5m4%2E5%200v4%2E5m0%2D4%2E5L15%209m5%2E25%2011%2E25h%2D4%2E5m4%2E5%200v%2D4%2E5m0%204%2E5L15%2015%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowsPointingOutIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15"/>
</svg>
  }
}