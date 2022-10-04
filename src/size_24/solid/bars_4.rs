use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%205%2E25a%2E75%2E75%200%2001%2E75%2D%2E75h16%2E5a%2E75%2E75%200%20010%201%2E5H3%2E75A%2E75%2E75%200%20013%205%2E25zm0%204%2E5A%2E75%2E75%200%20013%2E75%209h16%2E5a%2E75%2E75%200%20010%201%2E5H3%2E75A%2E75%2E75%200%20013%209%2E75zm0%204%2E5a%2E75%2E75%200%2001%2E75%2D%2E75h16%2E5a%2E75%2E75%200%20010%201%2E5H3%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75zm0%204%2E5a%2E75%2E75%200%2001%2E75%2D%2E75h16%2E5a%2E75%2E75%200%20010%201%2E5H3%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Bars4Icon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3 5.25a.75.75 0 01.75-.75h16.5a.75.75 0 010 1.5H3.75A.75.75 0 013 5.25zm0 4.5A.75.75 0 013.75 9h16.5a.75.75 0 010 1.5H3.75A.75.75 0 013 9.75zm0 4.5a.75.75 0 01.75-.75h16.5a.75.75 0 010 1.5H3.75a.75.75 0 01-.75-.75zm0 4.5a.75.75 0 01.75-.75h16.5a.75.75 0 010 1.5H3.75a.75.75 0 01-.75-.75z" clip-rule="evenodd"/>
</svg>
  }
}