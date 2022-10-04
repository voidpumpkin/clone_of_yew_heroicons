use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M9%2E674%202%2E075a%2E75%2E75%200%2001%2E652%200l7%2E25%203%2E5A%2E75%2E75%200%200117%206%2E957V16%2E5h%2E25a%2E75%2E75%200%20010%201%2E5H2%2E75a%2E75%2E75%200%20010%2D1%2E5H3V6%2E957a%2E75%2E75%200%2001%2D%2E576%2D1%2E382l7%2E25%2D3%2E5zM11%206a1%201%200%2011%2D2%200%201%201%200%20012%200zM7%2E5%209%2E75a%2E75%2E75%200%2000%2D1%2E5%200v5%2E5a%2E75%2E75%200%20001%2E5%200v%2D5%2E5zm3%2E25%200a%2E75%2E75%200%2000%2D1%2E5%200v5%2E5a%2E75%2E75%200%20001%2E5%200v%2D5%2E5zm3%2E25%200a%2E75%2E75%200%2000%2D1%2E5%200v5%2E5a%2E75%2E75%200%20001%2E5%200v%2D5%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn BuildingLibraryIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M9.674 2.075a.75.75 0 01.652 0l7.25 3.5A.75.75 0 0117 6.957V16.5h.25a.75.75 0 010 1.5H2.75a.75.75 0 010-1.5H3V6.957a.75.75 0 01-.576-1.382l7.25-3.5zM11 6a1 1 0 11-2 0 1 1 0 012 0zM7.5 9.75a.75.75 0 00-1.5 0v5.5a.75.75 0 001.5 0v-5.5zm3.25 0a.75.75 0 00-1.5 0v5.5a.75.75 0 001.5 0v-5.5zm3.25 0a.75.75 0 00-1.5 0v5.5a.75.75 0 001.5 0v-5.5z" clip-rule="evenodd"/>
</svg>
  }
}