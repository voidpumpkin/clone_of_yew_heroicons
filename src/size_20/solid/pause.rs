use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M5%2E75%203a%2E75%2E75%200%2000%2D%2E75%2E75v12%2E5c0%20%2E414%2E336%2E75%2E75%2E75h1%2E5a%2E75%2E75%200%2000%2E75%2D%2E75V3%2E75A%2E75%2E75%200%20007%2E25%203h%2D1%2E5zM12%2E75%203a%2E75%2E75%200%2000%2D%2E75%2E75v12%2E5c0%20%2E414%2E336%2E75%2E75%2E75h1%2E5a%2E75%2E75%200%2000%2E75%2D%2E75V3%2E75a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D1%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PauseIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M5.75 3a.75.75 0 00-.75.75v12.5c0 .414.336.75.75.75h1.5a.75.75 0 00.75-.75V3.75A.75.75 0 007.25 3h-1.5zM12.75 3a.75.75 0 00-.75.75v12.5c0 .414.336.75.75.75h1.5a.75.75 0 00.75-.75V3.75a.75.75 0 00-.75-.75h-1.5z"/>
</svg>
  }
}