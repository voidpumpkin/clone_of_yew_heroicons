use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M13%2E75%207h%2D3V3%2E66l1%2E95%202%2E1a%2E75%2E75%200%20101%2E1%2D1%2E02l%2D3%2E25%2D3%2E5a%2E75%2E75%200%2000%2D1%2E1%200L6%2E2%204%2E74a%2E75%2E75%200%20001%2E1%201%2E02l1%2E95%2D2%2E1V7h%2D3A2%2E25%202%2E25%200%20004%209%2E25v7%2E5A2%2E25%202%2E25%200%20006%2E25%2019h7%2E5A2%2E25%202%2E25%200%200016%2016%2E75v%2D7%2E5A2%2E25%202%2E25%200%200013%2E75%207zm%2D3%200h%2D1%2E5v5%2E25a%2E75%2E75%200%20001%2E5%200V7z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowUpOnSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M13.75 7h-3V3.66l1.95 2.1a.75.75 0 101.1-1.02l-3.25-3.5a.75.75 0 00-1.1 0L6.2 4.74a.75.75 0 001.1 1.02l1.95-2.1V7h-3A2.25 2.25 0 004 9.25v7.5A2.25 2.25 0 006.25 19h7.5A2.25 2.25 0 0016 16.75v-7.5A2.25 2.25 0 0013.75 7zm-3 0h-1.5v5.25a.75.75 0 001.5 0V7z" clip-rule="evenodd"/>
</svg>
  }
}
