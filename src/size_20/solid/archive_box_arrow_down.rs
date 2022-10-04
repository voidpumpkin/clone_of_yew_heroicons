use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%203a1%201%200%2000%2D1%201v1a1%201%200%20001%201h16a1%201%200%20001%2D1V4a1%201%200%2000%2D1%2D1H2zm0%204%2E5h16l%2D%2E811%207%2E71a2%202%200%2001%2D1%2E99%201%2E79H4%2E802a2%202%200%2001%2D1%2E99%2D1%2E79L2%207%2E5zM10%209a%2E75%2E75%200%2001%2E75%2E75v2%2E546l%2E943%2D1%2E048a%2E75%2E75%200%20111%2E114%201%2E004l%2D2%2E25%202%2E5a%2E75%2E75%200%2001%2D1%2E114%200l%2D2%2E25%2D2%2E5a%2E75%2E75%200%20111%2E114%2D1%2E004l%2E943%201%2E048V9%2E75A%2E75%2E75%200%200110%209z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArchiveBoxArrowDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2 3a1 1 0 00-1 1v1a1 1 0 001 1h16a1 1 0 001-1V4a1 1 0 00-1-1H2zm0 4.5h16l-.811 7.71a2 2 0 01-1.99 1.79H4.802a2 2 0 01-1.99-1.79L2 7.5zM10 9a.75.75 0 01.75.75v2.546l.943-1.048a.75.75 0 111.114 1.004l-2.25 2.5a.75.75 0 01-1.114 0l-2.25-2.5a.75.75 0 111.114-1.004l.943 1.048V9.75A.75.75 0 0110 9z" clip-rule="evenodd"/>
</svg>
  }
}