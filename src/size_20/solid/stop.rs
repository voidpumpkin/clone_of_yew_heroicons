use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M5%2E25%203A2%2E25%202%2E25%200%20003%205%2E25v9%2E5A2%2E25%202%2E25%200%20005%2E25%2017h9%2E5A2%2E25%202%2E25%200%200017%2014%2E75v%2D9%2E5A2%2E25%202%2E25%200%200014%2E75%203h%2D9%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn StopIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M5.25 3A2.25 2.25 0 003 5.25v9.5A2.25 2.25 0 005.25 17h9.5A2.25 2.25 0 0017 14.75v-9.5A2.25 2.25 0 0014.75 3h-9.5z"/>
</svg>
  }
}