use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M13%2E2%202%2E24a%2E75%2E75%200%2000%2E04%201%2E06l2%2E1%201%2E95H6%2E75a%2E75%2E75%200%20000%201%2E5h8%2E59l%2D2%2E1%201%2E95a%2E75%2E75%200%20101%2E02%201%2E1l3%2E5%2D3%2E25a%2E75%2E75%200%20000%2D1%2E1l%2D3%2E5%2D3%2E25a%2E75%2E75%200%2000%2D1%2E06%2E04zm%2D6%2E4%208a%2E75%2E75%200%2000%2D1%2E06%2D%2E04l%2D3%2E5%203%2E25a%2E75%2E75%200%20000%201%2E1l3%2E5%203%2E25a%2E75%2E75%200%20101%2E02%2D1%2E1l%2D2%2E1%2D1%2E95h8%2E59a%2E75%2E75%200%20000%2D1%2E5H4%2E66l2%2E1%2D1%2E95a%2E75%2E75%200%2000%2E04%2D1%2E06z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowsRightLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M13.2 2.24a.75.75 0 00.04 1.06l2.1 1.95H6.75a.75.75 0 000 1.5h8.59l-2.1 1.95a.75.75 0 101.02 1.1l3.5-3.25a.75.75 0 000-1.1l-3.5-3.25a.75.75 0 00-1.06.04zm-6.4 8a.75.75 0 00-1.06-.04l-3.5 3.25a.75.75 0 000 1.1l3.5 3.25a.75.75 0 101.02-1.1l-2.1-1.95h8.59a.75.75 0 000-1.5H4.66l2.1-1.95a.75.75 0 00.04-1.06z" clip-rule="evenodd"/>
</svg>
  }
}