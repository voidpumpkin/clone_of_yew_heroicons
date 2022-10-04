use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M9%2E528%201%2E718a%2E75%2E75%200%2001%2E162%2E819A8%2E97%208%2E97%200%20009%206a9%209%200%20009%209%208%2E97%208%2E97%200%20003%2E463%2D%2E69%2E75%2E75%200%2001%2E981%2E98%2010%2E503%2010%2E503%200%2001%2D9%2E694%206%2E46c%2D5%2E799%200%2D10%2E5%2D4%2E701%2D10%2E5%2D10%2E5%200%2D4%2E368%202%2E667%2D8%2E112%206%2E46%2D9%2E694a%2E75%2E75%200%2001%2E818%2E162z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn MoonIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M9.528 1.718a.75.75 0 01.162.819A8.97 8.97 0 009 6a9 9 0 009 9 8.97 8.97 0 003.463-.69.75.75 0 01.981.98 10.503 10.503 0 01-9.694 6.46c-5.799 0-10.5-4.701-10.5-10.5 0-4.368 2.667-8.112 6.46-9.694a.75.75 0 01.818.162z" clip-rule="evenodd"/>
</svg>
  }
}