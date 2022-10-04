use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M12%209v3%2E75m%2D9%2E303%203%2E376c%2D%2E866%201%2E5%2E217%203%2E374%201%2E948%203%2E374h14%2E71c1%2E73%200%202%2E813%2D1%2E874%201%2E948%2D3%2E374L13%2E949%203%2E378c%2D%2E866%2D1%2E5%2D3%2E032%2D1%2E5%2D3%2E898%200L2%2E697%2016%2E126zM12%2015%2E75h%2E007v%2E008H12v%2D%2E008z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn ExclamationTriangleIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"/>
</svg>
  }
}
