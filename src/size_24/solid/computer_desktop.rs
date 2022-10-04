use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2024%2024%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20fill%2Drule%3D%22evenodd%22%20d%3D%22M2%2E25%205%2E25a3%203%200%20013%2D3h13%2E5a3%203%200%20013%203V15a3%203%200%2001%2D3%203h%2D3v%2E257c0%20%2E597%2E237%201%2E17%2E659%201%2E591l%2E621%2E622a%2E75%2E75%200%2001%2D%2E53%201%2E28h%2D9a%2E75%2E75%200%2001%2D%2E53%2D1%2E28l%2E621%2D%2E622a2%2E25%202%2E25%200%2000%2E659%2D1%2E59V18h%2D3a3%203%200%2001%2D3%2D3V5%2E25zm1%2E5%200v7%2E5a1%2E5%201%2E5%200%20001%2E5%201%2E5h13%2E5a1%2E5%201%2E5%200%20001%2E5%2D1%2E5v%2D7%2E5a1%2E5%201%2E5%200%2000%2D1%2E5%2D1%2E5H5%2E25a1%2E5%201%2E5%200%2000%2D1%2E5%201%2E5z%22%20clip%2Drule%3D%22evenodd%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ComputerDesktopIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
  <path fill-rule="evenodd" d="M2.25 5.25a3 3 0 013-3h13.5a3 3 0 013 3V15a3 3 0 01-3 3h-3v.257c0 .597.237 1.17.659 1.591l.621.622a.75.75 0 01-.53 1.28h-9a.75.75 0 01-.53-1.28l.621-.622a2.25 2.25 0 00.659-1.59V18h-3a3 3 0 01-3-3V5.25zm1.5 0v7.5a1.5 1.5 0 001.5 1.5h13.5a1.5 1.5 0 001.5-1.5v-7.5a1.5 1.5 0 00-1.5-1.5H5.25a1.5 1.5 0 00-1.5 1.5z" clip-rule="evenodd"/>
</svg>
  }
}