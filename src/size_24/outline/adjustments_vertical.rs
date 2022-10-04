use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M6%2013%2E5V3%2E75m0%209%2E75a1%2E5%201%2E5%200%20010%203m0%2D3a1%2E5%201%2E5%200%20000%203m0%203%2E75V16%2E5m12%2D3V3%2E75m0%209%2E75a1%2E5%201%2E5%200%20010%203m0%2D3a1%2E5%201%2E5%200%20000%203m0%203%2E75V16%2E5m%2D6%2D9V3%2E75m0%203%2E75a1%2E5%201%2E5%200%20010%203m0%2D3a1%2E5%201%2E5%200%20000%203m0%209%2E75V10%2E5%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn AdjustmentsVerticalIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M6 13.5V3.75m0 9.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 3.75V16.5m12-3V3.75m0 9.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 3.75V16.5m-6-9V3.75m0 3.75a1.5 1.5 0 010 3m0-3a1.5 1.5 0 000 3m0 9.75V10.5"/>
</svg>
  }
}
