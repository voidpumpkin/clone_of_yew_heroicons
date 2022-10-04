use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M9%204%2E5v15m6%2D15v15m%2D10%2E875%200h15%2E75c%2E621%200%201%2E125%2D%2E504%201%2E125%2D1%2E125V5%2E625c0%2D%2E621%2D%2E504%2D1%2E125%2D1%2E125%2D1%2E125H4%2E125C3%2E504%204%2E5%203%205%2E004%203%205%2E625v12%2E75c0%20%2E621%2E504%201%2E125%201%2E125%201%2E125z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ViewColumnsIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M9 4.5v15m6-15v15m-10.875 0h15.75c.621 0 1.125-.504 1.125-1.125V5.625c0-.621-.504-1.125-1.125-1.125H4.125C3.504 4.5 3 5.004 3 5.625v12.75c0 .621.504 1.125 1.125 1.125z"/>
</svg>
  }
}
