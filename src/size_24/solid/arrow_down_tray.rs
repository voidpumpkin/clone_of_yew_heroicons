use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M12%202%2E25a%2E75%2E75%200%2001%2E75%2E75v11%2E69l3%2E22%2D3%2E22a%2E75%2E75%200%20111%2E06%201%2E06l%2D4%2E5%204%2E5a%2E75%2E75%200%2001%2D1%2E06%200l%2D4%2E5%2D4%2E5a%2E75%2E75%200%20111%2E06%2D1%2E06l3%2E22%203%2E22V3a%2E75%2E75%200%2001%2E75%2D%2E75zm%2D9%2013%2E5a%2E75%2E75%200%2001%2E75%2E75v2%2E25a1%2E5%201%2E5%200%20001%2E5%201%2E5h13%2E5a1%2E5%201%2E5%200%20001%2E5%2D1%2E5V16%2E5a%2E75%2E75%200%20011%2E5%200v2%2E25a3%203%200%2001%2D3%203H5%2E25a3%203%200%2001%2D3%2D3V16%2E5a%2E75%2E75%200%2001%2E75%2D%2E75z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowDownTrayIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M12 2.25a.75.75 0 01.75.75v11.69l3.22-3.22a.75.75 0 111.06 1.06l-4.5 4.5a.75.75 0 01-1.06 0l-4.5-4.5a.75.75 0 111.06-1.06l3.22 3.22V3a.75.75 0 01.75-.75zm-9 13.5a.75.75 0 01.75.75v2.25a1.5 1.5 0 001.5 1.5h13.5a1.5 1.5 0 001.5-1.5V16.5a.75.75 0 011.5 0v2.25a3 3 0 01-3 3H5.25a3 3 0 01-3-3V16.5a.75.75 0 01.75-.75z" clip-rule="evenodd"/>
</svg>
  }
}