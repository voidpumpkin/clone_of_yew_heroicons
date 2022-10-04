use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M15%2E97%202%2E47a%2E75%2E75%200%20011%2E06%200l4%2E5%204%2E5a%2E75%2E75%200%20010%201%2E06l%2D4%2E5%204%2E5a%2E75%2E75%200%2011%2D1%2E06%2D1%2E06l3%2E22%2D3%2E22H7%2E5a%2E75%2E75%200%20010%2D1%2E5h11%2E69l%2D3%2E22%2D3%2E22a%2E75%2E75%200%20010%2D1%2E06zm%2D7%2E94%209a%2E75%2E75%200%20010%201%2E06l%2D3%2E22%203%2E22H16%2E5a%2E75%2E75%200%20010%201%2E5H4%2E81l3%2E22%203%2E22a%2E75%2E75%200%2011%2D1%2E06%201%2E06l%2D4%2E5%2D4%2E5a%2E75%2E75%200%20010%2D1%2E06l4%2E5%2D4%2E5a%2E75%2E75%200%20011%2E06%200z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowsRightLeftIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M15.97 2.47a.75.75 0 011.06 0l4.5 4.5a.75.75 0 010 1.06l-4.5 4.5a.75.75 0 11-1.06-1.06l3.22-3.22H7.5a.75.75 0 010-1.5h11.69l-3.22-3.22a.75.75 0 010-1.06zm-7.94 9a.75.75 0 010 1.06l-3.22 3.22H16.5a.75.75 0 010 1.5H4.81l3.22 3.22a.75.75 0 11-1.06 1.06l-4.5-4.5a.75.75 0 010-1.06l4.5-4.5a.75.75 0 011.06 0z" clip-rule="evenodd"/>
</svg>
  }
}
