use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M10%2E5%201%2E5H8%2E25A2%2E25%202%2E25%200%20006%203%2E75v16%2E5a2%2E25%202%2E25%200%20002%2E25%202%2E25h7%2E5A2%2E25%202%2E25%200%200018%2020%2E25V3%2E75a2%2E25%202%2E25%200%2000%2D2%2E25%2D2%2E25H13%2E5m%2D3%200V3h3V1%2E5m%2D3%200h3m%2D3%2018%2E75h3%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn DevicePhoneMobileIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 1.5H8.25A2.25 2.25 0 006 3.75v16.5a2.25 2.25 0 002.25 2.25h7.5A2.25 2.25 0 0018 20.25V3.75a2.25 2.25 0 00-2.25-2.25H13.5m-3 0V3h3V1.5m-3 0h3m-3 18.75h3"/>
</svg>
  }
}