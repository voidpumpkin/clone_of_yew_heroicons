use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M5%201a3%203%200%2000%2D3%203v12a3%203%200%20003%203h10a3%203%200%20003%2D3V4a3%203%200%2000%2D3%2D3H5zM3%2E5%204A1%2E5%201%2E5%200%20015%202%2E5h10A1%2E5%201%2E5%200%200116%2E5%204v12a1%2E5%201%2E5%200%2001%2D1%2E5%201%2E5H5A1%2E5%201%2E5%200%20013%2E5%2016V4zm5%2E25%2011%2E5a%2E75%2E75%200%20000%201%2E5h2%2E5a%2E75%2E75%200%20000%2D1%2E5h%2D2%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DeviceTabletIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M5 1a3 3 0 00-3 3v12a3 3 0 003 3h10a3 3 0 003-3V4a3 3 0 00-3-3H5zM3.5 4A1.5 1.5 0 015 2.5h10A1.5 1.5 0 0116.5 4v12a1.5 1.5 0 01-1.5 1.5H5A1.5 1.5 0 013.5 16V4zm5.25 11.5a.75.75 0 000 1.5h2.5a.75.75 0 000-1.5h-2.5z" clip-rule="evenodd"/>
</svg>
  }
}