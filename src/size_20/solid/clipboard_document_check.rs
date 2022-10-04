use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M18%205%2E25a2%2E25%202%2E25%200%2000%2D2%2E012%2D2%2E238A2%2E25%202%2E25%200%200013%2E75%201h%2D1%2E5a2%2E25%202%2E25%200%2000%2D2%2E238%202%2E012c%2D%2E875%2E092%2D1%2E6%2E686%2D1%2E884%201%2E488H11A2%2E5%202%2E5%200%200113%2E5%207v7h2%2E25A2%2E25%202%2E25%200%200018%2011%2E75v%2D6%2E5zM12%2E25%202%2E5a%2E75%2E75%200%2000%2D%2E75%2E75v%2E25h3v%2D%2E25a%2E75%2E75%200%2000%2D%2E75%2D%2E75h%2D1%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M3%206a1%201%200%2000%2D1%201v10a1%201%200%20001%201h8a1%201%200%20001%2D1V7a1%201%200%2000%2D1%2D1H3zm6%2E874%204%2E166a%2E75%2E75%200%2010%2D1%2E248%2D%2E832l%2D2%2E493%203%2E739%2D%2E853%2D%2E853a%2E75%2E75%200%2000%2D1%2E06%201%2E06l1%2E5%201%2E5a%2E75%2E75%200%20001%2E154%2D%2E114l3%2D4%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ClipboardDocumentCheckIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M18 5.25a2.25 2.25 0 00-2.012-2.238A2.25 2.25 0 0013.75 1h-1.5a2.25 2.25 0 00-2.238 2.012c-.875.092-1.6.686-1.884 1.488H11A2.5 2.5 0 0113.5 7v7h2.25A2.25 2.25 0 0018 11.75v-6.5zM12.25 2.5a.75.75 0 00-.75.75v.25h3v-.25a.75.75 0 00-.75-.75h-1.5z" clip-rule="evenodd"/>
  <path fill-rule="evenodd" d="M3 6a1 1 0 00-1 1v10a1 1 0 001 1h8a1 1 0 001-1V7a1 1 0 00-1-1H3zm6.874 4.166a.75.75 0 10-1.248-.832l-2.493 3.739-.853-.853a.75.75 0 00-1.06 1.06l1.5 1.5a.75.75 0 001.154-.114l3-4.5z" clip-rule="evenodd"/>
</svg>
  }
}