use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%2E5%2018a%2E75%2E75%200%20000%201%2E5h3a%2E75%2E75%200%20000%2D1%2E5h%2D3z%22%2F%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M7%2E125%201%2E5A3%2E375%203%2E375%200%20003%2E75%204%2E875v14%2E25A3%2E375%203%2E375%200%20007%2E125%2022%2E5h9%2E75a3%2E375%203%2E375%200%20003%2E375%2D3%2E375V4%2E875A3%2E375%203%2E375%200%200016%2E875%201%2E5h%2D9%2E75zM6%204%2E875c0%2D%2E621%2E504%2D1%2E125%201%2E125%2D1%2E125h9%2E75c%2E621%200%201%2E125%2E504%201%2E125%201%2E125v14%2E25c0%20%2E621%2D%2E504%201%2E125%2D1%2E125%201%2E125h%2D9%2E75A1%2E125%201%2E125%200%20016%2019%2E125V4%2E875z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DeviceTabletIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path d="M10.5 18a.75.75 0 000 1.5h3a.75.75 0 000-1.5h-3z"/>
  <path fill-rule="evenodd" d="M7.125 1.5A3.375 3.375 0 003.75 4.875v14.25A3.375 3.375 0 007.125 22.5h9.75a3.375 3.375 0 003.375-3.375V4.875A3.375 3.375 0 0016.875 1.5h-9.75zM6 4.875c0-.621.504-1.125 1.125-1.125h9.75c.621 0 1.125.504 1.125 1.125v14.25c0 .621-.504 1.125-1.125 1.125h-9.75A1.125 1.125 0 016 19.125V4.875z" clip-rule="evenodd"/>
</svg>
  }
}