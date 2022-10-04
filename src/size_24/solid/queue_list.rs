use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M5%2E625%203%2E75a2%2E625%202%2E625%200%20100%205%2E25h12%2E75a2%2E625%202%2E625%200%20000%2D5%2E25H5%2E625zM3%2E75%2011%2E25a%2E75%2E75%200%20000%201%2E5h16%2E5a%2E75%2E75%200%20000%2D1%2E5H3%2E75zM3%2015%2E75a%2E75%2E75%200%2001%2E75%2D%2E75h16%2E5a%2E75%2E75%200%20010%201%2E5H3%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75zM3%2E75%2018%2E75a%2E75%2E75%200%20000%201%2E5h16%2E5a%2E75%2E75%200%20000%2D1%2E5H3%2E75z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn QueueListIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M5.625 3.75a2.625 2.625 0 100 5.25h12.75a2.625 2.625 0 000-5.25H5.625zM3.75 11.25a.75.75 0 000 1.5h16.5a.75.75 0 000-1.5H3.75zM3 15.75a.75.75 0 01.75-.75h16.5a.75.75 0 010 1.5H3.75a.75.75 0 01-.75-.75zM3.75 18.75a.75.75 0 000 1.5h16.5a.75.75 0 000-1.5H3.75z"/>
</svg>
  }
}
