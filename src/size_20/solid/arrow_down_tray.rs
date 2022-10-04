use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%2E75%202%2E75a%2E75%2E75%200%2000%2D1%2E5%200v8%2E614L6%2E295%208%2E235a%2E75%2E75%200%2010%2D1%2E09%201%2E03l4%2E25%204%2E5a%2E75%2E75%200%20001%2E09%200l4%2E25%2D4%2E5a%2E75%2E75%200%2000%2D1%2E09%2D1%2E03l%2D2%2E955%203%2E129V2%2E75z%22%2F%3E%20%3Cpath%20d%3D%22M3%2E5%2012%2E75a%2E75%2E75%200%2000%2D1%2E5%200v2%2E5A2%2E75%202%2E75%200%20004%2E75%2018h10%2E5A2%2E75%202%2E75%200%200018%2015%2E25v%2D2%2E5a%2E75%2E75%200%2000%2D1%2E5%200v2%2E5c0%20%2E69%2D%2E56%201%2E25%2D1%2E25%201%2E25H4%2E75c%2D%2E69%200%2D1%2E25%2D%2E56%2D1%2E25%2D1%2E25v%2D2%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowDownTrayIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M10.75 2.75a.75.75 0 00-1.5 0v8.614L6.295 8.235a.75.75 0 10-1.09 1.03l4.25 4.5a.75.75 0 001.09 0l4.25-4.5a.75.75 0 00-1.09-1.03l-2.955 3.129V2.75z"/>
  <path d="M3.5 12.75a.75.75 0 00-1.5 0v2.5A2.75 2.75 0 004.75 18h10.5A2.75 2.75 0 0018 15.25v-2.5a.75.75 0 00-1.5 0v2.5c0 .69-.56 1.25-1.25 1.25H4.75c-.69 0-1.25-.56-1.25-1.25v-2.5z"/>
</svg>
  }
}
