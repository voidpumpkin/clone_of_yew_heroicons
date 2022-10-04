use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%2012c0%2D5%2E385%204%2E365%2D9%2E75%209%2E75%2D9%2E75s9%2E75%204%2E365%209%2E75%209%2E75%2D4%2E365%209%2E75%2D9%2E75%209%2E75S2%2E25%2017%2E385%202%2E25%2012zM12%208%2E25a%2E75%2E75%200%2001%2E75%2E75v3%2E75a%2E75%2E75%200%2001%2D1%2E5%200V9a%2E75%2E75%200%2001%2E75%2D%2E75zm0%208%2E25a%2E75%2E75%200%20100%2D1%2E5%2E75%2E75%200%20000%201%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ExclamationCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zM12 8.25a.75.75 0 01.75.75v3.75a.75.75 0 01-1.5 0V9a.75.75 0 01.75-.75zm0 8.25a.75.75 0 100-1.5.75.75 0 000 1.5z" clip-rule="evenodd"/>
</svg>
  }
}