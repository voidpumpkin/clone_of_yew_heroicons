use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%208%2E25H7%2E5a2%2E25%202%2E25%200%2000%2D2%2E25%202%2E25v9a2%2E25%202%2E25%200%20002%2E25%202%2E25h9a2%2E25%202%2E25%200%20002%2E25%2D2%2E25v%2D9a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25H15m0%2D3l%2D3%2D3m0%200l%2D3%203m3%2D3V15%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowUpOnSquareIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9 8.25H7.5a2.25 2.25 0 00-2.25 2.25v9a2.25 2.25 0 002.25 2.25h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25H15m0-3l-3-3m0 0l-3 3m3-3V15"/>
</svg>
  }
}
