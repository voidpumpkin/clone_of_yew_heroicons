use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%2E75%202a%2E75%2E75%200%2001%2E75%2E75V4h7V2%2E75a%2E75%2E75%200%20011%2E5%200V4h%2E25A2%2E75%202%2E75%200%200118%206%2E75v8%2E5A2%2E75%202%2E75%200%200115%2E25%2018H4%2E75A2%2E75%202%2E75%200%20012%2015%2E25v%2D8%2E5A2%2E75%202%2E75%200%20014%2E75%204H5V2%2E75A%2E75%2E75%200%20015%2E75%202zm%2D1%205%2E5c%2D%2E69%200%2D1%2E25%2E56%2D1%2E25%201%2E25v6%2E5c0%20%2E69%2E56%201%2E25%201%2E25%201%2E25h10%2E5c%2E69%200%201%2E25%2D%2E56%201%2E25%2D1%2E25v%2D6%2E5c0%2D%2E69%2D%2E56%2D1%2E25%2D1%2E25%2D1%2E25H4%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CalendarIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M5.75 2a.75.75 0 01.75.75V4h7V2.75a.75.75 0 011.5 0V4h.25A2.75 2.75 0 0118 6.75v8.5A2.75 2.75 0 0115.25 18H4.75A2.75 2.75 0 012 15.25v-8.5A2.75 2.75 0 014.75 4H5V2.75A.75.75 0 015.75 2zm-1 5.5c-.69 0-1.25.56-1.25 1.25v6.5c0 .69.56 1.25 1.25 1.25h10.5c.69 0 1.25-.56 1.25-1.25v-6.5c0-.69-.56-1.25-1.25-1.25H4.75z" clip-rule="evenodd"/>
</svg>
  }
}
