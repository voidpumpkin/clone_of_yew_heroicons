use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M6%2E75%2012a%2E75%2E75%200%2011%2D1%2E5%200%20%2E75%2E75%200%20011%2E5%200zM12%2E75%2012a%2E75%2E75%200%2011%2D1%2E5%200%20%2E75%2E75%200%20011%2E5%200zM18%2E75%2012a%2E75%2E75%200%2011%2D1%2E5%200%20%2E75%2E75%200%20011%2E5%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EllipsisHorizontalIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 12a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM12.75 12a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM18.75 12a.75.75 0 11-1.5 0 .75.75 0 011.5 0z"/>
</svg>
  }
}
