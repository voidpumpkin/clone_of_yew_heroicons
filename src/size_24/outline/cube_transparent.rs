use yew::prelude::*;
use crate::props::Props;

/// <img src="data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww%2Ew3%2Eorg%2F2000%2Fsvg%22%20fill%3D%22none%22%20viewBox%3D%220%200%2024%2024%22%20stroke%2Dwidth%3D%221%2E5%22%20stroke%3D%22currentColor%22%20aria%2Dhidden%3D%22true%22%3E%20%3Cpath%20stroke%2Dlinecap%3D%22round%22%20stroke%2Dlinejoin%3D%22round%22%20d%3D%22M21%207%2E5l%2D2%2E25%2D1%2E313M21%207%2E5v2%2E25m0%2D2%2E25l%2D2%2E25%201%2E313M3%207%2E5l2%2E25%2D1%2E313M3%207%2E5l2%2E25%201%2E313M3%207%2E5v2%2E25m9%203l2%2E25%2D1%2E313M12%2012%2E75l%2D2%2E25%2D1%2E313M12%2012%2E75V15m0%206%2E75l2%2E25%2D1%2E313M12%2021%2E75V19%2E5m0%202%2E25l%2D2%2E25%2D1%2E313m0%2D16%2E875L12%202%2E25l2%2E25%201%2E313M21%2014%2E25v2%2E25l%2D2%2E25%201%2E313m%2D13%2E5%200L3%2016%2E5v%2D2%2E25%22%2F%3E%20%3C%2Fsvg%3E">
#[function_component]
pub fn CubeTransparentIcon(props: &Props) -> Html {
    let Props { class } = props.clone();

  html! {
<svg {class} fill-rule="currentColor" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25"/>
</svg>
  }
}