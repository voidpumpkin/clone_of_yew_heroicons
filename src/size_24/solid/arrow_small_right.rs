use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%2E75%2012a%2E75%2E75%200%2001%2E75%2D%2E75h13%2E19l%2D5%2E47%2D5%2E47a%2E75%2E75%200%20011%2E06%2D1%2E06l6%2E75%206%2E75a%2E75%2E75%200%20010%201%2E06l%2D6%2E75%206%2E75a%2E75%2E75%200%2011%2D1%2E06%2D1%2E06l5%2E47%2D5%2E47H4%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowSmallRightIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M3.75 12a.75.75 0 01.75-.75h13.19l-5.47-5.47a.75.75 0 011.06-1.06l6.75 6.75a.75.75 0 010 1.06l-6.75 6.75a.75.75 0 11-1.06-1.06l5.47-5.47H4.5a.75.75 0 01-.75-.75z" clip-rule="evenodd"/>
</svg>
  }
}