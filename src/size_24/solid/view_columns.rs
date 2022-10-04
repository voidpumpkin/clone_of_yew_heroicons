use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M15%203%2E75H9v16%2E5h6V3%2E75zM16%2E5%2020%2E25h3%2E375c1%2E035%200%201%2E875%2D%2E84%201%2E875%2D1%2E875V5%2E625c0%2D1%2E036%2D%2E84%2D1%2E875%2D1%2E875%2D1%2E875H16%2E5v16%2E5zM4%2E125%203%2E75H7%2E5v16%2E5H4%2E125a1%2E875%201%2E875%200%2001%2D1%2E875%2D1%2E875V5%2E625c0%2D1%2E036%2E84%2D1%2E875%201%2E875%2D1%2E875z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ViewColumnsIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M15 3.75H9v16.5h6V3.75zM16.5 20.25h3.375c1.035 0 1.875-.84 1.875-1.875V5.625c0-1.036-.84-1.875-1.875-1.875H16.5v16.5zM4.125 3.75H7.5v16.5H4.125a1.875 1.875 0 01-1.875-1.875V5.625c0-1.036.84-1.875 1.875-1.875z"/>
</svg>
  }
}