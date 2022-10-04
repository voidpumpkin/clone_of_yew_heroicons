use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M2%2E25%2012l8%2E954%2D8%2E955c%2E44%2D%2E439%201%2E152%2D%2E439%201%2E591%200L21%2E75%2012M4%2E5%209%2E75v10%2E125c0%20%2E621%2E504%201%2E125%201%2E125%201%2E125H9%2E75v%2D4%2E875c0%2D%2E621%2E504%2D1%2E125%201%2E125%2D1%2E125h2%2E25c%2E621%200%201%2E125%2E504%201%2E125%201%2E125V21h4%2E125c%2E621%200%201%2E125%2D%2E504%201%2E125%2D1%2E125V9%2E75M8%2E25%2021h8%2E25%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn HomeIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"/>
</svg>
  }
}