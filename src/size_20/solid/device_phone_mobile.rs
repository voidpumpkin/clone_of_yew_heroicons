use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M8%2016%2E25a%2E75%2E75%200%2001%2E75%2D%2E75h2%2E5a%2E75%2E75%200%20010%201%2E5h%2D2%2E5a%2E75%2E75%200%2001%2D%2E75%2D%2E75z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M4%204a3%203%200%20013%2D3h6a3%203%200%20013%203v12a3%203%200%2001%2D3%203H7a3%203%200%2001%2D3%2D3V4zm4%2D1%2E5v%2E75c0%20%2E414%2E336%2E75%2E75%2E75h2%2E5a%2E75%2E75%200%2000%2E75%2D%2E75V2%2E5h1A1%2E5%201%2E5%200%200114%2E5%204v12a1%2E5%201%2E5%200%2001%2D1%2E5%201%2E5H7A1%2E5%201%2E5%200%20015%2E5%2016V4A1%2E5%201%2E5%200%20017%202%2E5h1z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DevicePhoneMobileIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M8 16.25a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5a.75.75 0 01-.75-.75z"/>
  <path fill-rule="evenodd" d="M4 4a3 3 0 013-3h6a3 3 0 013 3v12a3 3 0 01-3 3H7a3 3 0 01-3-3V4zm4-1.5v.75c0 .414.336.75.75.75h2.5a.75.75 0 00.75-.75V2.5h1A1.5 1.5 0 0114.5 4v12a1.5 1.5 0 01-1.5 1.5H7A1.5 1.5 0 015.5 16V4A1.5 1.5 0 017 2.5h1z" clip-rule="evenodd"/>
</svg>
  }
}
