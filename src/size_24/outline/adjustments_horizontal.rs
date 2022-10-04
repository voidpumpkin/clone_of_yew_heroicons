use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M10%2E5%206h9%2E75M10%2E5%206a1%2E5%201%2E5%200%2011%2D3%200m3%200a1%2E5%201%2E5%200%2010%2D3%200M3%2E75%206H7%2E5m3%2012h9%2E75m%2D9%2E75%200a1%2E5%201%2E5%200%2001%2D3%200m3%200a1%2E5%201%2E5%200%2000%2D3%200m%2D3%2E75%200H7%2E5m9%2D6h3%2E75m%2D3%2E75%200a1%2E5%201%2E5%200%2001%2D3%200m3%200a1%2E5%201%2E5%200%2000%2D3%200m%2D9%2E75%200h9%2E75%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn AdjustmentsHorizontalIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6h9.75M10.5 6a1.5 1.5 0 11-3 0m3 0a1.5 1.5 0 10-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-9.75 0h9.75"/>
</svg>
  }
}