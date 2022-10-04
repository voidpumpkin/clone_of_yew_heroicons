use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M14%206H6v8h8V6z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M9%2E25%203V1%2E75a%2E75%2E75%200%20011%2E5%200V3h1%2E5V1%2E75a%2E75%2E75%200%20011%2E5%200V3h%2E5A2%2E75%202%2E75%200%200117%205%2E75v%2E5h1%2E25a%2E75%2E75%200%20010%201%2E5H17v1%2E5h1%2E25a%2E75%2E75%200%20010%201%2E5H17v1%2E5h1%2E25a%2E75%2E75%200%20010%201%2E5H17v%2E5A2%2E75%202%2E75%200%200114%2E25%2017h%2D%2E5v1%2E25a%2E75%2E75%200%2001%2D1%2E5%200V17h%2D1%2E5v1%2E25a%2E75%2E75%200%2001%2D1%2E5%200V17h%2D1%2E5v1%2E25a%2E75%2E75%200%2001%2D1%2E5%200V17h%2D%2E5A2%2E75%202%2E75%200%20013%2014%2E25v%2D%2E5H1%2E75a%2E75%2E75%200%20010%2D1%2E5H3v%2D1%2E5H1%2E75a%2E75%2E75%200%20010%2D1%2E5H3v%2D1%2E5H1%2E75a%2E75%2E75%200%20010%2D1%2E5H3v%2D%2E5A2%2E75%202%2E75%200%20015%2E75%203h%2E5V1%2E75a%2E75%2E75%200%20011%2E5%200V3h1%2E5zM4%2E5%205%2E75c0%2D%2E69%2E56%2D1%2E25%201%2E25%2D1%2E25h8%2E5c%2E69%200%201%2E25%2E56%201%2E25%201%2E25v8%2E5c0%20%2E69%2D%2E56%201%2E25%2D1%2E25%201%2E25h%2D8%2E5c%2D%2E69%200%2D1%2E25%2D%2E56%2D1%2E25%2D1%2E25v%2D8%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CpuChipIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M14 6H6v8h8V6z"/>
  <path fill-rule="evenodd" d="M9.25 3V1.75a.75.75 0 011.5 0V3h1.5V1.75a.75.75 0 011.5 0V3h.5A2.75 2.75 0 0117 5.75v.5h1.25a.75.75 0 010 1.5H17v1.5h1.25a.75.75 0 010 1.5H17v1.5h1.25a.75.75 0 010 1.5H17v.5A2.75 2.75 0 0114.25 17h-.5v1.25a.75.75 0 01-1.5 0V17h-1.5v1.25a.75.75 0 01-1.5 0V17h-1.5v1.25a.75.75 0 01-1.5 0V17h-.5A2.75 2.75 0 013 14.25v-.5H1.75a.75.75 0 010-1.5H3v-1.5H1.75a.75.75 0 010-1.5H3v-1.5H1.75a.75.75 0 010-1.5H3v-.5A2.75 2.75 0 015.75 3h.5V1.75a.75.75 0 011.5 0V3h1.5zM4.5 5.75c0-.69.56-1.25 1.25-1.25h8.5c.69 0 1.25.56 1.25 1.25v8.5c0 .69-.56 1.25-1.25 1.25h-8.5c-.69 0-1.25-.56-1.25-1.25v-8.5z" clip-rule="evenodd"/>
</svg>
  }
}