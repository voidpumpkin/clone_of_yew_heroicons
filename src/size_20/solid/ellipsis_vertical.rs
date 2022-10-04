use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20viewBox%3D%220%200%2020%2020%22%20fill%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20d%3D%22M10%203a1%2E5%201%2E5%200%20110%203%201%2E5%201%2E5%200%20010%2D3zM10%208%2E5a1%2E5%201%2E5%200%20110%203%201%2E5%201%2E5%200%20010%2D3zM11%2E5%2015%2E5a1%2E5%201%2E5%200%2010%2D3%200%201%2E5%201%2E5%200%20003%200z%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn EllipsisVerticalIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
  <path d="M10 3a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM10 8.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM11.5 15.5a1.5 1.5 0 10-3 0 1.5 1.5 0 003 0z"/>
</svg>
  }
}