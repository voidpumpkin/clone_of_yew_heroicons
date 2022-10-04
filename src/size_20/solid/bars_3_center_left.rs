use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%204%2E75A%2E75%2E75%200%20012%2E75%204h14%2E5a%2E75%2E75%200%20010%201%2E5H2%2E75A%2E75%2E75%200%20012%204%2E75zm0%2010%2E5a%2E75%2E75%200%2001%2E75%2D%2E75h14%2E5a%2E75%2E75%200%20010%201%2E5H2%2E75a%2E75%2E75%200%2001%2D%2E75%2D%2E75zM2%2010a%2E75%2E75%200%2001%2E75%2D%2E75h7%2E5a%2E75%2E75%200%20010%201%2E5h%2D7%2E5A%2E75%2E75%200%20012%2010z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn Bars3CenterLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5A.75.75 0 012 10z" clip-rule="evenodd"/>
</svg>
  }
}
