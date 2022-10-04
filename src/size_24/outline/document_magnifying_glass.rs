use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M19%2E5%2014%2E25v%2D2%2E625a3%2E375%203%2E375%200%2000%2D3%2E375%2D3%2E375h%2D1%2E5A1%2E125%201%2E125%200%200113%2E5%207%2E125v%2D1%2E5a3%2E375%203%2E375%200%2000%2D3%2E375%2D3%2E375H8%2E25m5%2E231%2013%2E481L15%2017%2E25m%2D4%2E5%2D15H5%2E625c%2D%2E621%200%2D1%2E125%2E504%2D1%2E125%201%2E125v16%2E5c0%20%2E621%2E504%201%2E125%201%2E125%201%2E125h12%2E75c%2E621%200%201%2E125%2D%2E504%201%2E125%2D1%2E125V11%2E25a9%209%200%2000%2D9%2D9zm3%2E75%2011%2E625a2%2E625%202%2E625%200%2011%2D5%2E25%200%202%2E625%202%2E625%200%20015%2E25%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DocumentMagnifyingGlassIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m5.231 13.481L15 17.25m-4.5-15H5.625c-.621 0-1.125.504-1.125 1.125v16.5c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9zm3.75 11.625a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"/>
</svg>
  }
}