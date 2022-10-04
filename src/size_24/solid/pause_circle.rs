use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%2012c0%2D5%2E385%204%2E365%2D9%2E75%209%2E75%2D9%2E75s9%2E75%204%2E365%209%2E75%209%2E75%2D4%2E365%209%2E75%2D9%2E75%209%2E75S2%2E25%2017%2E385%202%2E25%2012zM9%208%2E25a%2E75%2E75%200%2000%2D%2E75%2E75v6c0%20%2E414%2E336%2E75%2E75%2E75h%2E75a%2E75%2E75%200%2000%2E75%2D%2E75V9a%2E75%2E75%200%2000%2D%2E75%2D%2E75H9zm5%2E25%200a%2E75%2E75%200%2000%2D%2E75%2E75v6c0%20%2E414%2E336%2E75%2E75%2E75H15a%2E75%2E75%200%2000%2E75%2D%2E75V9a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PauseCircleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zM9 8.25a.75.75 0 00-.75.75v6c0 .414.336.75.75.75h.75a.75.75 0 00.75-.75V9a.75.75 0 00-.75-.75H9zm5.25 0a.75.75 0 00-.75.75v6c0 .414.336.75.75.75H15a.75.75 0 00.75-.75V9a.75.75 0 00-.75-.75h-.75z" clip-rule="evenodd"/>
</svg>
  }
}