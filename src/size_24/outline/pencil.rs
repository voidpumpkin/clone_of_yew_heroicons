use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M16%2E862%204%2E487l1%2E687%2D1%2E688a1%2E875%201%2E875%200%20112%2E652%202%2E652L6%2E832%2019%2E82a4%2E5%204%2E5%200%2001%2D1%2E897%201%2E13l%2D2%2E685%2E8%2E8%2D2%2E685a4%2E5%204%2E5%200%20011%2E13%2D1%2E897L16%2E863%204%2E487zm0%200L19%2E5%207%2E125%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn PencilIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125"/>
</svg>
  }
}