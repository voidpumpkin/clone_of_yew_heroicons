use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M16%2E5%206v%2E75m0%203v%2E75m0%203v%2E75m0%203V18m%2D9%2D5%2E25h5%2E25M7%2E5%2015h3M3%2E375%205%2E25c%2D%2E621%200%2D1%2E125%2E504%2D1%2E125%201%2E125v3%2E026a2%2E999%202%2E999%200%20010%205%2E198v3%2E026c0%20%2E621%2E504%201%2E125%201%2E125%201%2E125h17%2E25c%2E621%200%201%2E125%2D%2E504%201%2E125%2D1%2E125v%2D3%2E026a2%2E999%202%2E999%200%20010%2D5%2E198V6%2E375c0%2D%2E621%2D%2E504%2D1%2E125%2D1%2E125%2D1%2E125H3%2E375z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn TicketIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 6v.75m0 3v.75m0 3v.75m0 3V18m-9-5.25h5.25M7.5 15h3M3.375 5.25c-.621 0-1.125.504-1.125 1.125v3.026a2.999 2.999 0 010 5.198v3.026c0 .621.504 1.125 1.125 1.125h17.25c.621 0 1.125-.504 1.125-1.125v-3.026a2.999 2.999 0 010-5.198V6.375c0-.621-.504-1.125-1.125-1.125H3.375z"/>
</svg>
  }
}
