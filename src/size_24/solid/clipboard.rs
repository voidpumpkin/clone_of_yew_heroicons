use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M10%2E5%203A1%2E501%201%2E501%200%20009%204%2E5h6A1%2E5%201%2E5%200%200013%2E5%203h%2D3zm%2D2%2E693%2E178A3%203%200%200110%2E5%201%2E5h3a3%203%200%20012%2E694%201%2E678c%2E497%2E042%2E992%2E092%201%2E486%2E15%201%2E497%2E173%202%2E57%201%2E46%202%2E57%202%2E929V19%2E5a3%203%200%2001%2D3%203H6%2E75a3%203%200%2001%2D3%2D3V6%2E257c0%2D1%2E47%201%2E073%2D2%2E756%202%2E57%2D2%2E93%2E493%2D%2E057%2E989%2D%2E107%201%2E487%2D%2E15z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ClipboardIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M10.5 3A1.501 1.501 0 009 4.5h6A1.5 1.5 0 0013.5 3h-3zm-2.693.178A3 3 0 0110.5 1.5h3a3 3 0 012.694 1.678c.497.042.992.092 1.486.15 1.497.173 2.57 1.46 2.57 2.929V19.5a3 3 0 01-3 3H6.75a3 3 0 01-3-3V6.257c0-1.47 1.073-2.756 2.57-2.93.493-.057.989-.107 1.487-.15z" clip-rule="evenodd"/>
</svg>
  }
}