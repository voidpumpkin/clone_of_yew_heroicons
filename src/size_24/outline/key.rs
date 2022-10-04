use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M15%2E75%205%2E25a3%203%200%20013%203m3%200a6%206%200%2001%2D7%2E029%205%2E912c%2D%2E563%2D%2E097%2D1%2E159%2E026%2D1%2E563%2E43L10%2E5%2017%2E25H8%2E25v2%2E25H6v2%2E25H2%2E25v%2D2%2E818c0%2D%2E597%2E237%2D1%2E17%2E659%2D1%2E591l6%2E499%2D6%2E499c%2E404%2D%2E404%2E527%2D1%20%2E43%2D1%2E563A6%206%200%201121%2E75%208%2E25z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn KeyIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"/>
</svg>
  }
}
