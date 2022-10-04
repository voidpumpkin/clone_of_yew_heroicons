use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M20%2E25%207%2E5l%2D%2E625%2010%2E632a2%2E25%202%2E25%200%2001%2D2%2E247%202%2E118H6%2E622a2%2E25%202%2E25%200%2001%2D2%2E247%2D2%2E118L3%2E75%207%2E5m8%2E25%203v6%2E75m0%200l%2D3%2D3m3%203l3%2D3M3%2E375%207%2E5h17%2E25c%2E621%200%201%2E125%2D%2E504%201%2E125%2D1%2E125v%2D1%2E5c0%2D%2E621%2D%2E504%2D1%2E125%2D1%2E125%2D1%2E125H3%2E375c%2D%2E621%200%2D1%2E125%2E504%2D1%2E125%201%2E125v1%2E5c0%20%2E621%2E504%201%2E125%201%2E125%201%2E125z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArchiveBoxArrowDownIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M20.25 7.5l-.625 10.632a2.25 2.25 0 01-2.247 2.118H6.622a2.25 2.25 0 01-2.247-2.118L3.75 7.5m8.25 3v6.75m0 0l-3-3m3 3l3-3M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125z"/>
</svg>
  }
}
