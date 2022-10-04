use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M7%2E5%207%2E5h%2D%2E75A2%2E25%202%2E25%200%20004%2E5%209%2E75v7%2E5a2%2E25%202%2E25%200%20002%2E25%202%2E25h7%2E5a2%2E25%202%2E25%200%20002%2E25%2D2%2E25v%2D7%2E5a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25h%2D%2E75m%2D6%203%2E75l3%203m0%200l3%2D3m%2D3%203V1%2E5m6%209h%2E75a2%2E25%202%2E25%200%20012%2E25%202%2E25v7%2E5a2%2E25%202%2E25%200%2001%2D2%2E25%202%2E25h%2D7%2E5a2%2E25%202%2E25%200%2001%2D2%2E25%2D2%2E25v%2D%2E75%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ArrowDownOnSquareStackIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 7.5h-.75A2.25 2.25 0 004.5 9.75v7.5a2.25 2.25 0 002.25 2.25h7.5a2.25 2.25 0 002.25-2.25v-7.5a2.25 2.25 0 00-2.25-2.25h-.75m-6 3.75l3 3m0 0l3-3m-3 3V1.5m6 9h.75a2.25 2.25 0 012.25 2.25v7.5a2.25 2.25 0 01-2.25 2.25h-7.5a2.25 2.25 0 01-2.25-2.25v-.75"/>
</svg>
  }
}
