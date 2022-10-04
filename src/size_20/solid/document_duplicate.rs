use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M7%203%2E5A1%2E5%201%2E5%200%20018%2E5%202h3%2E879a1%2E5%201%2E5%200%20011%2E06%2E44l3%2E122%203%2E12A1%2E5%201%2E5%200%200117%206%2E622V12%2E5a1%2E5%201%2E5%200%2001%2D1%2E5%201%2E5h%2D1v%2D3%2E379a3%203%200%2000%2D%2E879%2D2%2E121L10%2E5%205%2E379A3%203%200%20008%2E379%204%2E5H7v%2D1z%22%2F%3E%20%3Cpath%20d%3D%22M4%2E5%206A1%2E5%201%2E5%200%20003%207%2E5v9A1%2E5%201%2E5%200%20004%2E5%2018h7a1%2E5%201%2E5%200%20001%2E5%2D1%2E5v%2D5%2E879a1%2E5%201%2E5%200%2000%2D%2E44%2D1%2E06L9%2E44%206%2E439A1%2E5%201%2E5%200%20008%2E378%206H4%2E5z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DocumentDuplicateIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M7 3.5A1.5 1.5 0 018.5 2h3.879a1.5 1.5 0 011.06.44l3.122 3.12A1.5 1.5 0 0117 6.622V12.5a1.5 1.5 0 01-1.5 1.5h-1v-3.379a3 3 0 00-.879-2.121L10.5 5.379A3 3 0 008.379 4.5H7v-1z"/>
  <path d="M4.5 6A1.5 1.5 0 003 7.5v9A1.5 1.5 0 004.5 18h7a1.5 1.5 0 001.5-1.5v-5.879a1.5 1.5 0 00-.44-1.06L9.44 6.439A1.5 1.5 0 008.378 6H4.5z"/>
</svg>
  }
}
