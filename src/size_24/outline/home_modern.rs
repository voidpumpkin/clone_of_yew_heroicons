use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M8%2E25%2021v%2D4%2E875c0%2D%2E621%2E504%2D1%2E125%201%2E125%2D1%2E125h2%2E25c%2E621%200%201%2E125%2E504%201%2E125%201%2E125V21m0%200h4%2E5V3%2E545M12%2E75%2021h7%2E5V10%2E75M2%2E25%2021h1%2E5m18%200h%2D18M2%2E25%209l4%2E5%2D1%2E636M18%2E75%203l%2D1%2E5%2E545m0%206%2E205l3%201m1%2E5%2E5l%2D1%2E5%2D%2E5M6%2E75%207%2E364V3h%2D3v18m3%2D13%2E636l10%2E5%2D3%2E819%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn HomeModernIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 21v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21m0 0h4.5V3.545M12.75 21h7.5V10.75M2.25 21h1.5m18 0h-18M2.25 9l4.5-1.636M18.75 3l-1.5.545m0 6.205l3 1m1.5.5l-1.5-.5M6.75 7.364V3h-3v18m3-13.636l10.5-3.819"/>
</svg>
  }
}
