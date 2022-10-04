use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%203v2%2E25m6%2E364%2E386l%2D1%2E591%201%2E591M21%2012h%2D2%2E25m%2D%2E386%206%2E364l%2D1%2E591%2D1%2E591M12%2018%2E75V21m%2D4%2E773%2D4%2E227l%2D1%2E591%201%2E591M5%2E25%2012H3m4%2E227%2D4%2E773L5%2E636%205%2E636M15%2E75%2012a3%2E75%203%2E75%200%2011%2D7%2E5%200%203%2E75%203%2E75%200%20017%2E5%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn SunIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z"/>
</svg>
  }
}