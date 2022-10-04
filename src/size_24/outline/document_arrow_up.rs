use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M19%2E5%2014%2E25v%2D2%2E625a3%2E375%203%2E375%200%2000%2D3%2E375%2D3%2E375h%2D1%2E5A1%2E125%201%2E125%200%200113%2E5%207%2E125v%2D1%2E5a3%2E375%203%2E375%200%2000%2D3%2E375%2D3%2E375H8%2E25m6%2E75%2012l%2D3%2D3m0%200l%2D3%203m3%2D3v6m%2D1%2E5%2D15H5%2E625c%2D%2E621%200%2D1%2E125%2E504%2D1%2E125%201%2E125v17%2E25c0%20%2E621%2E504%201%2E125%201%2E125%201%2E125h12%2E75c%2E621%200%201%2E125%2D%2E504%201%2E125%2D1%2E125V11%2E25a9%209%200%2000%2D9%2D9z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DocumentArrowUpIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m6.75 12l-3-3m0 0l-3 3m3-3v6m-1.5-15H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"/>
</svg>
  }
}