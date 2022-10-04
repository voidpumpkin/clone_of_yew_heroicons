use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%208%2E25c0%2D2%2E485%2D2%2E099%2D4%2E5%2D4%2E688%2D4%2E5%2D1%2E935%200%2D3%2E597%201%2E126%2D4%2E312%202%2E733%2D%2E715%2D1%2E607%2D2%2E377%2D2%2E733%2D4%2E313%2D2%2E733C5%2E1%203%2E75%203%205%2E765%203%208%2E25c0%207%2E22%209%2012%209%2012s9%2D4%2E78%209%2D12z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn HeartIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z"/>
</svg>
  }
}
