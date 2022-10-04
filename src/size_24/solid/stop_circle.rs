use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%2012c0%2D5%2E385%204%2E365%2D9%2E75%209%2E75%2D9%2E75s9%2E75%204%2E365%209%2E75%209%2E75%2D4%2E365%209%2E75%2D9%2E75%209%2E75S2%2E25%2017%2E385%202%2E25%2012zm6%2D2%2E438c0%2D%2E724%2E588%2D1%2E312%201%2E313%2D1%2E312h4%2E874c%2E725%200%201%2E313%2E588%201%2E313%201%2E313v4%2E874c0%20%2E725%2D%2E588%201%2E313%2D1%2E313%201%2E313H9%2E564a1%2E312%201%2E312%200%2001%2D1%2E313%2D1%2E313V9%2E564z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn StopCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm6-2.438c0-.724.588-1.312 1.313-1.312h4.874c.725 0 1.313.588 1.313 1.313v4.874c0 .725-.588 1.313-1.313 1.313H9.564a1.312 1.312 0 01-1.313-1.313V9.564z" clip-rule="evenodd"/>
</svg>
  }
}