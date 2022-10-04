use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M10%2E125%202%2E25h%2D4%2E5c%2D%2E621%200%2D1%2E125%2E504%2D1%2E125%201%2E125v17%2E25c0%20%2E621%2E504%201%2E125%201%2E125%201%2E125h12%2E75c%2E621%200%201%2E125%2D%2E504%201%2E125%2D1%2E125v%2D9M10%2E125%202%2E25h%2E375a9%209%200%20019%209v%2E375M10%2E125%202%2E25A3%2E375%203%2E375%200%200113%2E5%205%2E625v1%2E5c0%20%2E621%2E504%201%2E125%201%2E125%201%2E125h1%2E5a3%2E375%203%2E375%200%20013%2E375%203%2E375M9%2015l2%2E25%202%2E25L15%2012%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DocumentCheckIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M10.125 2.25h-4.5c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125v-9M10.125 2.25h.375a9 9 0 019 9v.375M10.125 2.25A3.375 3.375 0 0113.5 5.625v1.5c0 .621.504 1.125 1.125 1.125h1.5a3.375 3.375 0 013.375 3.375M9 15l2.25 2.25L15 12"/>
</svg>
  }
}
